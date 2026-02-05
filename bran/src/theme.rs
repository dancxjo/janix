//! Genie Circles Boot Theme
//!
//! A creative themed boot display with:
//! - 5 connected circles showing boot phase progression
//! - Phase name display
//! - Scrolling log area with colored messages
//!
//! The genie's gift to ThingOS! ✨

use bulb::theme_api::{BootPhase, LogLevel};
use bulb::font::{SimpleFont, CHAR_WIDTH, CHAR_HEIGHT};
use bulb::framebuffer::FramebufferTarget;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};

/// Flag to disable theme when compositor takes over
pub static THEME_DISABLED: AtomicBool = AtomicBool::new(false);

/// Global themed console
pub static THEMED_CONSOLE: Mutex<Option<GenieCirclesTheme>> = Mutex::new(None);

// Layout constants
const HEADER_HEIGHT: u32 = 60;
const LOG_AREA_TOP: u32 = 70;
const MAX_LOG_LINES: usize = 12;
const LOG_LINE_HEIGHT: u32 = 14;

// Back buffer for tear-free rendering
// Sized for 1024x768 @ 32bpp = 3MB (fits in early boot heap)
const BACK_BUFFER_SIZE: usize = 1024 * 768 * 4;
static mut BACK_BUFFER: [u8; BACK_BUFFER_SIZE] = [0; BACK_BUFFER_SIZE];

// Colors - A genie's palette ✨
const COLOR_BG: Rgb888 = Rgb888::new(0x08, 0x08, 0x12);
const COLOR_SPARK: Rgb888 = Rgb888::new(0x44, 0x88, 0xFF);     // Blue
const COLOR_MEMORY: Rgb888 = Rgb888::new(0x44, 0xFF, 0x88);    // Green
const COLOR_CPU: Rgb888 = Rgb888::new(0xFF, 0xDD, 0x44);       // Yellow/Gold
const COLOR_SERVICES: Rgb888 = Rgb888::new(0xFF, 0x88, 0xDD);  // Pink
const COLOR_AWAKE: Rgb888 = Rgb888::new(0xFF, 0xFF, 0xFF);     // White
const COLOR_DIM: Rgb888 = Rgb888::new(0x28, 0x28, 0x38);       // Dim circles
const COLOR_LINE: Rgb888 = Rgb888::new(0x40, 0x40, 0x50);      // Connecting line
const COLOR_LINE_ACTIVE: Rgb888 = Rgb888::new(0x80, 0xA0, 0xFF); // Active line
const COLOR_TEXT: Rgb888 = Rgb888::new(0xCC, 0xCC, 0xCC);      // Regular text
const COLOR_SOURCE: Rgb888 = Rgb888::new(0x44, 0xCC, 0xCC);    // Cyan sources
const COLOR_PHASE_NAME: Rgb888 = Rgb888::new(0xAA, 0xAA, 0xFF); // Phase name
const COLOR_ERROR: Rgb888 = Rgb888::new(0xFF, 0x44, 0x44);
const COLOR_WARN: Rgb888 = Rgb888::new(0xFF, 0xAA, 0x44);
const COLOR_DEBUG: Rgb888 = Rgb888::new(0x66, 0x66, 0x88);

/// Log line storage
#[derive(Clone)]
struct LogLine {
    source: [u8; 24],
    source_len: usize,
    message: [u8; 80],
    message_len: usize,
    level: LogLevel,
}

impl Default for LogLine {
    fn default() -> Self {
        Self {
            source: [0; 24],
            source_len: 0,
            message: [0; 80],
            message_len: 0,
            level: LogLevel::Info,
        }
    }
}

/// The Genie Circles theme - full boot display control
pub struct GenieCirclesTheme {
    fb: crate::framebuffer::Framebuffer,
    width: u32,
    height: u32,
    stride: u32,
    phase: BootPhase,
    logs: [LogLine; MAX_LOG_LINES],
    log_count: usize,
    log_head: usize,
    /// Current tick count for animation
    tick_count: u64,
    /// Last render timestamp
    last_render: u64,
    /// Dirty flag - logs have been added since last render
    dirty: bool,
}

// Safety: Framebuffer is only accessed from the boot CPU during early init
unsafe impl Send for GenieCirclesTheme {}

impl GenieCirclesTheme {
    pub fn new(fb: crate::framebuffer::Framebuffer) -> Self {
        let info = fb.info();
        
        let mut theme = Self {
            fb,
            width: info.width,
            height: info.height,
            stride: info.stride,
            phase: BootPhase::Spark,
            logs: core::array::from_fn(|_| LogLine::default()),
            log_count: 0,
            log_head: 0,
            tick_count: 0,
            last_render: 0,
            dirty: true,
        };
        
        // Initial render
        theme.render_full();
        theme
    }
    
    /// Called when a structured log event arrives (replaces put_char)
    pub fn on_log_event(&mut self, event: bulb::theme_api::LogEvent<'_>) {
        // Detect phase change from message content
        let new_phase = BootPhase::detect_from_log(event.message, self.phase);
        if new_phase != self.phase {
            self.phase = new_phase;
        }
        
        // Add to log buffer (ring buffer)
        let idx = (self.log_head + self.log_count) % MAX_LOG_LINES;
        if self.log_count < MAX_LOG_LINES {
            self.log_count += 1;
        } else {
            self.log_head = (self.log_head + 1) % MAX_LOG_LINES;
        }
        
        let log = &mut self.logs[idx];
        
        // Copy source
        if let Some(src) = event.source {
            let bytes = src.as_bytes();
            let len = bytes.len().min(24);
            log.source[..len].copy_from_slice(&bytes[..len]);
            log.source_len = len;
        } else {
            log.source_len = 0;
        }
        
        // Copy message
        let msg_bytes = event.message.as_bytes();
        let msg_len = msg_bytes.len().min(80);
        log.message[..msg_len].copy_from_slice(&msg_bytes[..msg_len]);
        log.message_len = msg_len;
        log.level = event.level;
        
        self.dirty = true;
    }
    
    /// Called on timer tick for animation (~30Hz recommended)
    pub fn tick(&mut self, now_ms: u64) {
        self.tick_count = now_ms;
        
        // Render if dirty or time for animation frame (~30 FPS)
        const RENDER_INTERVAL_MS: u64 = 33;
        if self.dirty || now_ms.saturating_sub(self.last_render) >= RENDER_INTERVAL_MS {
            self.render_full();
            self.dirty = false;
            self.last_render = now_ms;
        }
    }
    
    fn render_full(&mut self) {
        let bpp: usize = 4;
        let stride = fb_common::calc_stride_bytes(self.width, bpp as u32, self.stride) as usize;
        let width = self.width as i32;
        let height = self.height as i32;
        
        // Copy state we need for rendering
        let phase = self.phase;
        let logs: [LogLine; MAX_LOG_LINES] = self.logs.clone();
        let log_count = self.log_count;
        let log_head = self.log_head;
        
        // Render to back buffer first (tear-free)
        let back_buf = unsafe { &mut BACK_BUFFER[..] };
        let back_size = (height as usize * stride).min(BACK_BUFFER_SIZE);
        let back = &mut back_buf[..back_size];
        
        // Clear back buffer
        fill_rect(back, stride, width, height, 0, 0, width, height, COLOR_BG);
        
        // Draw circles and connecting lines
        draw_header(back, stride, width, height, phase);
        
        // Draw log area
        draw_logs(back, stride, width, height, &logs, log_count, log_head);
        
        // Fast blit: copy back buffer to framebuffer in one go
        let fb_buf = self.fb.buffer_mut();
        let copy_len = back_size.min(fb_buf.len());
        fb_buf[..copy_len].copy_from_slice(&back[..copy_len]);
    }
}

// Standalone drawing functions (to avoid borrow conflicts)

fn fill_rect(buf: &mut [u8], stride: usize, max_w: i32, max_h: i32, x: i32, y: i32, w: i32, h: i32, color: Rgb888) {
    let bpp: usize = 4;
    for py in y.max(0)..(y + h).min(max_h) {
        for px in x.max(0)..(x + w).min(max_w) {
            let offset = (py as usize * stride) + (px as usize * bpp);
            if offset + 2 < buf.len() {
                buf[offset] = color.b();
                buf[offset + 1] = color.g();
                buf[offset + 2] = color.r();
            }
        }
    }
}

fn draw_filled_circle(buf: &mut [u8], stride: usize, max_w: i32, max_h: i32, cx: i32, cy: i32, r: i32, color: Rgb888) {
    let bpp: usize = 4;
    let r2 = r * r;
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r2 {
                let px = cx + dx;
                let py = cy + dy;
                if px >= 0 && px < max_w && py >= 0 && py < max_h {
                    let offset = (py as usize * stride) + (px as usize * bpp);
                    if offset + 2 < buf.len() {
                        buf[offset] = color.b();
                        buf[offset + 1] = color.g();
                        buf[offset + 2] = color.r();
                    }
                }
            }
        }
    }
}

fn draw_circle_outline(buf: &mut [u8], stride: usize, max_w: i32, max_h: i32, cx: i32, cy: i32, r: i32, color: Rgb888) {
    let bpp: usize = 4;
    let r2_outer = r * r;
    let r2_inner = (r - 1) * (r - 1);
    for dy in -r..=r {
        for dx in -r..=r {
            let d2 = dx * dx + dy * dy;
            if d2 <= r2_outer && d2 >= r2_inner {
                let px = cx + dx;
                let py = cy + dy;
                if px >= 0 && px < max_w && py >= 0 && py < max_h {
                    let offset = (py as usize * stride) + (px as usize * bpp);
                    if offset + 2 < buf.len() {
                        buf[offset] = color.b();
                        buf[offset + 1] = color.g();
                        buf[offset + 2] = color.r();
                    }
                }
            }
        }
    }
}

fn draw_line(buf: &mut [u8], stride: usize, max_w: i32, max_h: i32, x1: i32, y1: i32, x2: i32, thickness: i32, color: Rgb888) {
    let bpp: usize = 4;
    for py in (y1 - thickness/2)..=(y1 + thickness/2) {
        for px in x1.min(x2)..=x1.max(x2) {
            if px >= 0 && px < max_w && py >= 0 && py < max_h {
                let offset = (py as usize * stride) + (px as usize * bpp);
                if offset + 2 < buf.len() {
                    buf[offset] = color.b();
                    buf[offset + 1] = color.g();
                    buf[offset + 2] = color.r();
                }
            }
        }
    }
}

fn draw_text(buf: &mut [u8], stride: usize, max_w: i32, max_h: i32, x: i32, y: i32, text: &str, color: Rgb888) {
    let bpp: usize = 4;
    let mut cur_x = x;
    for c in text.bytes() {
        let glyph = SimpleFont::get_glyph(c);
        for gy in 0..CHAR_HEIGHT {
            let row_byte = glyph[gy];
            for gx in 0..CHAR_WIDTH {
                if (row_byte >> (7 - gx)) & 1 != 0 {
                    let px = cur_x + gx as i32;
                    let py = y + gy as i32;
                    if px >= 0 && px < max_w && py >= 0 && py < max_h {
                        let offset = (py as usize * stride) + (px as usize * bpp);
                        if offset + 2 < buf.len() {
                            buf[offset] = color.b();
                            buf[offset + 1] = color.g();
                            buf[offset + 2] = color.r();
                        }
                    }
                }
            }
        }
        cur_x += CHAR_WIDTH as i32;
    }
}

fn draw_header(buf: &mut [u8], stride: usize, width: i32, height: i32, phase: BootPhase) {
    let center_y = (HEADER_HEIGHT / 2) as i32;
    let radius: i32 = 16;
    let spacing: i32 = 70;
    let total_width = 5 * spacing;
    let start_x = (width - total_width) / 2 + spacing / 2;
    
    let phases = [
        (BootPhase::Spark, COLOR_SPARK),
        (BootPhase::Memory, COLOR_MEMORY),
        (BootPhase::Cpu, COLOR_CPU),
        (BootPhase::Services, COLOR_SERVICES),
        (BootPhase::Awake, COLOR_AWAKE),
    ];
    
    // Draw connecting lines first
    for i in 0..4 {
        let x1 = start_x + (i as i32) * spacing + radius + 2;
        let x2 = start_x + ((i + 1) as i32) * spacing - radius - 2;
        let line_color = if phases[i].0 <= phase && phases[i + 1].0 <= phase {
            COLOR_LINE_ACTIVE
        } else {
            COLOR_LINE
        };
        draw_line(buf, stride, width, height, x1, center_y, x2, 2, line_color);
    }
    
    // Draw circles
    for (i, (p, color)) in phases.iter().enumerate() {
        let cx = start_x + (i as i32) * spacing;
        let circle_color = if *p <= phase { *color } else { COLOR_DIM };
        draw_filled_circle(buf, stride, width, height, cx, center_y, radius, circle_color);
        
        // Add glow for current phase
        if *p == phase {
            draw_circle_outline(buf, stride, width, height, cx, center_y, radius + 3, *color);
        }
    }
    
    // Draw phase name below circles
    let phase_name = phase.name();
    let text_x = (width - (phase_name.len() as i32 * CHAR_WIDTH as i32)) / 2;
    let text_y = center_y + radius + 8;
    draw_text(buf, stride, width, height, text_x, text_y, phase_name, COLOR_PHASE_NAME);
}

fn draw_logs(buf: &mut [u8], stride: usize, width: i32, height: i32, logs: &[LogLine; MAX_LOG_LINES], log_count: usize, log_head: usize) {
    let log_x = 20;
    let mut y = LOG_AREA_TOP as i32;
    
    for i in 0..log_count {
        let idx = (log_head + i) % MAX_LOG_LINES;
        let log = &logs[idx];
        
        // Draw arrow indicator
        draw_text(buf, stride, width, height, log_x, y, ">", COLOR_DIM);
        
        // Draw source in cyan
        if log.source_len > 0 {
            if let Ok(src) = core::str::from_utf8(&log.source[..log.source_len]) {
                draw_text(buf, stride, width, height, log_x + 12, y, src, COLOR_SOURCE);
            }
        }
        
        // Draw message with level color
        let msg_x = log_x + 12 + 20 * CHAR_WIDTH as i32;
        let msg_color = match log.level {
            LogLevel::Error => COLOR_ERROR,
            LogLevel::Warn => COLOR_WARN,
            LogLevel::Debug => COLOR_DEBUG,
            _ => COLOR_TEXT,
        };
        
        if log.message_len > 0 {
            if let Ok(msg) = core::str::from_utf8(&log.message[..log.message_len]) {
                let max_chars = ((width - msg_x - 20) / CHAR_WIDTH as i32).max(0) as usize;
                let display_msg = if msg.len() > max_chars { &msg[..max_chars] } else { msg };
                draw_text(buf, stride, width, height, msg_x, y, display_msg, msg_color);
            }
        }
        
        y += LOG_LINE_HEIGHT as i32;
    }
}

/// Initialize the themed console
pub fn init(fb: crate::framebuffer::Framebuffer) {
    let theme = GenieCirclesTheme::new(fb);
    *THEMED_CONSOLE.lock() = Some(theme);
}

/// Disable the themed console
pub fn disable() {
    THEME_DISABLED.store(true, Ordering::Relaxed);
}

/// Send a structured log event to the theme
pub fn on_log_event(event: bulb::theme_api::LogEvent<'_>) {
    if THEME_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    if let Some(ref mut theme) = *THEMED_CONSOLE.lock() {
        theme.on_log_event(event);
    }
}

/// Tick the theme animation (call from timer interrupt)
pub fn tick(now_ms: u64) {
    if THEME_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    if let Some(ref mut theme) = *THEMED_CONSOLE.lock() {
        theme.tick(now_ms);
    }
}
