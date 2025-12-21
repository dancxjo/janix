use thing_models::PropValue;
use kernel::console::{ConsoleSink, register_sink};
use kernel::{graph, graph_kinds, symbols};
use limine::framebuffer::Framebuffer;
use spin::Mutex;
use thing_models::MODE_INDEX_CONSOLE;

// Will be provided by build.rs:
include!(concat!(env!("OUT_DIR"), "/unifont.rs"));

// Temporary kill-switch so logs only go out over serial until the FB console stabilizes.
pub const FRAMEBUFFER_CONSOLE_ENABLED: bool = true;

static CONSOLE: Mutex<Option<Console>> = Mutex::new(None);

struct FramebufferSink;
unsafe impl Sync for FramebufferSink {}
unsafe impl Send for FramebufferSink {}

impl ConsoleSink for FramebufferSink {
    fn write_str(&self, s: &str) {
        if !FRAMEBUFFER_CONSOLE_ENABLED {
            return;
        }
        if !console_mode_should_draw() {
            return;
        }
        if let Some(console) = CONSOLE.lock().as_mut() {
            console.write_str(s);
        }
    }
}

static SINK: FramebufferSink = FramebufferSink;

pub struct Console {
    fb_ptr: *mut u8,
    width: u64,
    height: u64,
    pitch: u64,
    bpp: u16, // assume 32 for now
    cursor_x: u32,
    cursor_y: u32,
    cols: u32,
    rows: u32,
    bg_color: u32,
    fg_color: u32,
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}

impl Console {
    pub unsafe fn from_framebuffer(fb: &Framebuffer) -> Self {
        let width = fb.width() as u64;
        let height = fb.height() as u64;
        let pitch = fb.pitch() as u64;
        let bpp = fb.bpp();
        let fb_ptr = fb.addr() as *mut u8;

        let cols = (width as u32 / GLYPH_WIDTH).max(1);
        let rows = (height as u32 / GLYPH_HEIGHT).max(1);

        Console {
            fb_ptr,
            width,
            height,
            pitch,
            bpp,
            cursor_x: 0,
            cursor_y: 0,
            cols,
            rows,
            bg_color: 0x00000000,
            fg_color: 0xFFFFFFFF,
        }
    }

    pub fn set_colors(&mut self, bg: u32, fg: u32) {
        self.bg_color = bg;
        self.fg_color = fg;
    }

    pub fn repaint(&mut self) {
        self.clear();
        self.cursor_x = 0;
        self.cursor_y = 0;

        let logs = kernel::log::get_logs();
        // Only print the last N logs that fit on the screen to avoid instant scrolling
        // Leave some space for the progress bar at the bottom
        let max_lines = self.rows.saturating_sub(4) as usize;
        let start = logs.len().saturating_sub(max_lines);

        for entry in &logs[start..] {
            if let Some(msg) = entry {
                self.write_str(msg);
                self.put_char('\n');
            }
        }
    }

    pub fn draw_progress(&mut self, percent: f32) {
        if self.bpp != 32 { return; }

        let bar_height = 8;
        let padding_bottom = 10;
        let padding_side = 50;

        if self.height < (bar_height + padding_bottom) as u64 {
            return;
        }

        let y_start = self.height as u32 - bar_height - padding_bottom;
        let total_width = (self.width as u32).saturating_sub(padding_side * 2);
        let x_start = padding_side;

        let percent = percent.max(0.0).min(1.0);
        let filled_width = (total_width as f32 * percent) as u32;

        // Draw background of bar (slightly lighter than bg or fixed dark grey)
        let bar_bg = 0xFF444444;
        self.fill_rect(x_start, y_start, total_width, bar_height, bar_bg);

        // Draw filled part
        let bar_fg = 0xFFFFFFFF; // White
        self.fill_rect(x_start, y_start, filled_width, bar_height, bar_fg);
    }

    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn clear(&mut self) {
        // Fill the framebuffer with bg_color; assume 32 bpp.
        // If bg_color is 0, we can use write_bytes which is faster (memset)
        // Otherwise, we must loop.
        if self.bg_color == 0 {
            let bytes = (self.pitch * self.height) as usize;
            unsafe {
                core::ptr::write_bytes(self.fb_ptr, 0, bytes);
            }
        } else {
            self.fill_rect(0, 0, self.width as u32, self.height as u32, self.bg_color);
        }
    }

    pub fn put_char(&mut self, ch: char) {
        match ch {
            '\n' => {
                self.cursor_x = 0;
                self.advance_line();
            }
            _ => {
                if let Some(bitmap) = lookup_glyph(ch) {
                    self.draw_glyph(self.cursor_x, self.cursor_y, bitmap);
                }
                self.cursor_x += 1;
                if self.cursor_x >= self.cols {
                    self.cursor_x = 0;
                    self.advance_line();
                }
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.put_char(ch);
        }
    }

    fn advance_line(&mut self) {
        if self.cursor_y < self.rows.saturating_sub(1) {
            self.cursor_y += 1;
        } else {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        // Simple “scroll up one row”: memmove framebuffer up by one glyph row,
        // then clear the bottom row.
        let row_bytes = (self.pitch * GLYPH_HEIGHT as u64) as usize;
        let total_bytes = (self.pitch * self.height) as usize;
        let copy_bytes = total_bytes - row_bytes;

        unsafe {
            // Move everything up by one row height
            core::ptr::copy(self.fb_ptr.add(row_bytes), self.fb_ptr, copy_bytes);

            // Clear the last row if background is black (fast path)
            if self.bg_color == 0 {
                core::ptr::write_bytes(self.fb_ptr.add(copy_bytes), 0, row_bytes);
            }
        }

        // Fill the bottom cleared area with bg_color (if not black)
        if self.bg_color != 0 {
             let start_y = (self.height as u32).saturating_sub(GLYPH_HEIGHT as u32);
             self.fill_rect(0, start_y, self.width as u32, GLYPH_HEIGHT as u32, self.bg_color);
        }

        self.cursor_y = self.rows.saturating_sub(1);
    }

    fn draw_glyph(&mut self, col: u32, row: u32, bitmap: &[u8; GLYPH_HEIGHT as usize]) {
        // Convert (col,row) to pixel origin:
        let x0 = col * GLYPH_WIDTH;
        let y0 = row * GLYPH_HEIGHT;

        // For each row of glyph:
        for (dy, byte) in bitmap.iter().enumerate() {
            let y = y0 + dy as u32;
            if y >= self.height as u32 {
                break;
            }
            // Each bit in `byte` is a pixel.
            for bit in 0..GLYPH_WIDTH {
                let x = x0 + bit;
                if x >= self.width as u32 {
                    break;
                }
                let on = (byte >> (7 - bit)) & 1 != 0;
                if on {
                    self.set_pixel(x, y, self.fg_color);
                } else {
                    self.set_pixel(x, y, self.bg_color);
                }
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, argb: u32) {
        if self.bpp != 32 {
            // For now, ignore non-32bpp modes.
            return;
        }
        let offset = y as u64 * self.pitch + x as u64 * 4;
        unsafe {
            let ptr = self.fb_ptr.add(offset as usize).cast::<u32>();
            ptr.write_volatile(argb);
        }
    }
}

use core::fmt;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

pub unsafe fn init_global(fb: &Framebuffer) {
    // SAFETY: We are in single-threaded boot context.
    let console = unsafe { Console::from_framebuffer(fb) };
    *CONSOLE.lock() = Some(console);
    register_sink(&SINK);
}

pub fn print(s: &str) {
    kernel::console::print(s);
}

pub fn clear_screen() {
    if let Some(console) = CONSOLE.lock().as_mut() {
        console.clear();
    }
}

pub fn update_theme(bg: u32, fg: u32) {
    if let Some(console) = CONSOLE.lock().as_mut() {
        console.set_colors(bg, fg);
        console.repaint();
    }
}

pub fn draw_progress(percent: f32) {
    if let Some(console) = CONSOLE.lock().as_mut() {
        console.draw_progress(percent);
    }
}

pub fn with_console<F, R>(f: F) -> R
where
    F: FnOnce(&mut Console) -> R,
{
    let mut guard = CONSOLE.lock();
    if let Some(console) = guard.as_mut() {
        f(console)
    } else {
        panic!("Console not initialized");
    }
}

fn console_mode_should_draw() -> bool {
    if !symbols::is_initialized() || kernel::graph::store::things_slab().lock().is_none() {
        return true;
    }

    let mut seen_mode = false;
    let mut console_active = false;
    
    // Intern symbols for lookup
    let kind_mode = symbols::intern(graph_kinds::KIND_MODE);
    let prop_mode_idx = symbols::intern(graph_kinds::PROP_MODE_INDEX);
    let prop_mode_active = symbols::intern(graph_kinds::PROP_MODE_ACTIVE);

    graph::iter_things(|thing| {
        if thing.kind == kind_mode {
            seen_mode = true;

            let mut index: Option<u8> = None;
            let mut active = false;

            for (key, value) in thing.props.iter() {
                if *key == prop_mode_idx {
                    if let PropValue::U64(v) = value {
                        index = Some(*v as u8);
                    }
                } else if *key == prop_mode_active {
                    if let PropValue::Bool(flag) = value {
                        active = *flag;
                    }
                }
            }

            if active {
                if let Some(idx) = index {
                    if idx == MODE_INDEX_CONSOLE {
                        console_active = true;
                    }
                }
            }
        }
    });

    if console_active {
        true
    } else {
        // Before userland seeds Mode Things, allow the console to draw.
        !seen_mode
    }
}
