use core::str;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::prelude::Point;
use fb_common::calc_stride_bytes;
use crate::framebuffer::{FramebufferTarget, PixelFormat};
use crate::font::{SimpleFont, CHAR_WIDTH, CHAR_HEIGHT};
use crate::parser::parse_log_line;

// Colors
const COLOR_BG: Rgb888 = Rgb888::BLACK;
const COLOR_TIME: Rgb888 = Rgb888::CYAN;
const COLOR_SOURCE: Rgb888 = Rgb888::YELLOW;
const COLOR_MSG_DEFAULT: Rgb888 = Rgb888::WHITE;
const COLOR_MSG_ERROR: Rgb888 = Rgb888::RED;
const COLOR_MSG_WARN: Rgb888 = Rgb888::MAGENTA;
const COLOR_MSG_INFO: Rgb888 = Rgb888::new(200, 200, 200); // Light Gray
const COLOR_MSG_DEBUG: Rgb888 = Rgb888::new(100, 100, 100); // Darker Gray

// Dashboard Layout
const MAX_SOURCES: usize = 32;
const SOURCE_NAME_LEN: usize = 24;
const DASHBOARD_COLS: i32 = 4;
const DASHBOARD_Y: i32 = 20;
const DASHBOARD_COL_WIDTH: i32 = 200;
const COLOR_DASHBOARD_TEXT: Rgb888 = Rgb888::GREEN;

// Layout
const TEXT_PAD: i32 = 5;

pub struct BootUpDisplay<F: FramebufferTarget> {
    fb: F,
    last_msg_area: Option<Rect>,
    sources: [[u8; SOURCE_NAME_LEN]; MAX_SOURCES],
    source_count: usize,
}

#[derive(Clone, Copy)]
struct Rect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl<F: FramebufferTarget> BootUpDisplay<F> {
    pub fn new(mut fb: F) -> Self {
        {
            let mut drawer = FbDrawer { fb: &mut fb };
            drawer.clear(COLOR_BG);
            // Draw a debug circle to help visualize stride/format issues.
            let info = drawer.fb.info();
            let cx = (info.width / 2) as i32;
            let cy = (info.height / 2) as i32;
            drawer.draw_circle(Point::new(cx, cy), 40, Rgb888::GREEN);
        }

        Self {
            fb,
            last_msg_area: None,
            sources: [[0; SOURCE_NAME_LEN]; MAX_SOURCES],
            source_count: 0,
        }
    }

    /// Create with a vertical offset for log rendering (theme header area)
    pub fn new_with_offset(mut fb: F, _header_height: u32) -> Self {
        // Don't clear - theme has already drawn header
        Self {
            fb,
            last_msg_area: None,
            sources: [[0; SOURCE_NAME_LEN]; MAX_SOURCES],
            source_count: 0,
        }
    }

    pub fn into_inner(self) -> F {
        self.fb
    }

    /// Get mutable access to the underlying framebuffer
    pub fn fb_mut(&mut self) -> &mut F {
        &mut self.fb
    }

    pub fn render_log_line(&mut self, line: &str) {
        let mut drawer = FbDrawer { fb: &mut self.fb };

        // Clear previous message area
        if let Some(rect) = self.last_msg_area {
            drawer.fill_rect(rect, COLOR_BG);
        }

        let parts = parse_log_line(line);
        let info = drawer.fb.info();
        let center_x = (info.width / 2) as i32;
        let center_y = (info.height / 2) as i32;

        let mut lines_to_draw: [Option<(&str, Rgb888)>; 3] = [None; 3];
        let mut line_count = 0;
        let mut time_buf = [0u8; 24]; // Buffer for timestamp string

        // 1. Timestamp (Top)
        if let Some(t) = parts.time {
             let s = u64_to_str_buf(t, &mut time_buf);
             lines_to_draw[line_count] = Some((s, COLOR_TIME));
             line_count += 1;
        }

        // 2. Source (Middle)
        // We show the source (3rd bracket) in the middle.
        if let Some(src) = parts.source {
            lines_to_draw[line_count] = Some((src, COLOR_SOURCE));
            line_count += 1;

            // Track unique source (matrix at the top)
            // Only consider the first part of the :: path
            let domain = src.split("::").next().unwrap_or(src);
            let domain_bytes = domain.as_bytes();
            let mut found = false;
            for i in 0..self.source_count {
                let existing = &self.sources[i];
                let len = existing.iter().position(|&b| b == 0).unwrap_or(SOURCE_NAME_LEN);
                if &existing[..len] == domain_bytes {
                    found = true;
                    break;
                }
            }

            if !found && self.source_count < MAX_SOURCES {
                let name = &mut self.sources[self.source_count];
                let len = domain_bytes.len().min(SOURCE_NAME_LEN);
                name[..len].copy_from_slice(&domain_bytes[..len]);
                self.source_count += 1;
            }
        }

        // 3. Message (Bottom)
        let msg_color = get_level_color(parts.level);
        lines_to_draw[line_count] = Some((parts.message, msg_color));
        line_count += 1;

        // Draw dashboard
        Self::draw_dashboard(&self.sources, self.source_count, &mut drawer);

        let total_h = (line_count as i32 * CHAR_HEIGHT as i32) +
                      ((line_count as i32 - 1).max(0) * TEXT_PAD);

        let start_y = center_y - (total_h / 2);

        let mut current_y = start_y;
        let mut bounding_box: Option<Rect> = None;

        for i in 0..line_count {
            if let Some((text, color)) = lines_to_draw[i] {
                let text_w = text.len() as i32 * CHAR_WIDTH as i32;
                let start_x = center_x - (text_w / 2);

                draw_string(&mut drawer, start_x, current_y, text, color, COLOR_BG);

                let line_rect = Rect {
                    x: start_x,
                    y: current_y,
                    w: text_w as u32,
                    h: CHAR_HEIGHT as u32,
                };

                bounding_box = match bounding_box {
                    Some(bb) => {
                         let min_x = bb.x.min(line_rect.x);
                         let min_y = bb.y.min(line_rect.y);
                         let bb_right = bb.x + bb.w as i32;
                         let bb_bottom = bb.y + bb.h as i32;
                         let lr_right = line_rect.x + line_rect.w as i32;
                         let lr_bottom = line_rect.y + line_rect.h as i32;

                         let max_x = bb_right.max(lr_right);
                         let max_y = bb_bottom.max(lr_bottom);

                         Some(Rect {
                             x: min_x,
                             y: min_y,
                             w: (max_x - min_x) as u32,
                             h: (max_y - min_y) as u32,
                         })
                    },
                    None => Some(line_rect),
                };

                current_y += CHAR_HEIGHT as i32 + TEXT_PAD;
            }
        }

        self.last_msg_area = bounding_box;
    }

    fn draw_dashboard(sources: &[[u8; SOURCE_NAME_LEN]; MAX_SOURCES], count: usize, drawer: &mut FbDrawer<'_, F>) {
        let info = drawer.fb.info();
        let total_w = DASHBOARD_COLS * DASHBOARD_COL_WIDTH;
        let start_x = (info.width as i32 - total_w) / 2;

        for i in 0..count {
            let row = (i as i32) / DASHBOARD_COLS;
            let col = (i as i32) % DASHBOARD_COLS;

            let x = start_x + (col * DASHBOARD_COL_WIDTH);
            let y = DASHBOARD_Y + (row * (CHAR_HEIGHT as i32 + TEXT_PAD));

            let name_bytes = &sources[i];
            let len = name_bytes.iter().position(|&b| b == 0).unwrap_or(SOURCE_NAME_LEN);
            if let Ok(name) = str::from_utf8(&name_bytes[..len]) {
                draw_string(drawer, x, y, name, COLOR_DASHBOARD_TEXT, COLOR_BG);
            }
        }
    }
}

fn u64_to_str_buf(val: u64, buf: &mut [u8]) -> &str {
    let mut i = buf.len();
    let mut n = val;
    if n == 0 {
        i -= 1;
        buf[i] = b'0';
        return core::str::from_utf8(&buf[i..]).unwrap();
    }
    while n > 0 && i > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    core::str::from_utf8(&buf[i..]).unwrap()
}

fn draw_string<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, s: &str, color: Rgb888, bg_color: Rgb888) {
    let mut cur_x = x;
    for c in s.bytes() {
         draw_char(drawer, cur_x, y, c, color, bg_color);
         cur_x += CHAR_WIDTH as i32;
    }
}

fn get_level_color(level: Option<&str>) -> Rgb888 {
    match level {
        Some(l) if l.contains("ERROR") => COLOR_MSG_ERROR,
        Some(l) if l.contains("WARN") => COLOR_MSG_WARN,
        Some(l) if l.contains("INFO") => COLOR_MSG_INFO,
        Some(l) if l.contains("DEBUG") => COLOR_MSG_DEBUG,
        _ => COLOR_MSG_DEFAULT,
    }
}

fn draw_char<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, c: u8, color: Rgb888, _bg_color: Rgb888) {
    let glyph = SimpleFont::get_glyph(c);
    for gy in 0..CHAR_HEIGHT {
        let row_byte = glyph[gy];
        for gx in 0..CHAR_WIDTH {
             let px = x + gx as i32;
             let py = y + gy as i32;
             if (row_byte >> (7 - gx)) & 1 != 0 {
                 drawer.put_pixel(Point::new(px, py), color);
             }
             // Additive drawing: Skip background pixels
        }
    }
}

struct FbDrawer<'a, F: FramebufferTarget> {
    fb: &'a mut F,
}

impl<'a, F: FramebufferTarget> FbDrawer<'a, F> {
    fn draw_circle(&mut self, center: Point, radius: i32, color: Rgb888) {
        if radius <= 0 {
            return;
        }

        let mut x = radius;
        let mut y = 0;
        let mut err = 1 - x;

        while x >= y {
            let points = [
                Point::new(center.x + x, center.y + y),
                Point::new(center.x + y, center.y + x),
                Point::new(center.x - y, center.y + x),
                Point::new(center.x - x, center.y + y),
                Point::new(center.x - x, center.y - y),
                Point::new(center.x - y, center.y - x),
                Point::new(center.x + y, center.y - x),
                Point::new(center.x + x, center.y - y),
            ];
            for p in points {
                self.put_pixel(p, color);
            }

            y += 1;
            if err < 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x) + 1;
            }
        }
    }

    fn fill_rect(&mut self, rect: Rect, color: Rgb888) {
        let info = self.fb.info();
        let bpp = match info.format {
            PixelFormat::Bgrx8888 | PixelFormat::Rgbx8888 => 4,
            PixelFormat::Rgb888 | PixelFormat::Bgr888 => 3,
            PixelFormat::Rgb565 => 2,
            _ => return,
        };
        let stride = calc_stride_bytes(info.width, bpp, info.stride) as usize;
        let buffer = self.fb.buffer_mut();

        let x0 = rect.x.max(0) as usize;
        let y0 = rect.y.max(0) as usize;
        let x1 = (rect.x + rect.w as i32).min(info.width as i32) as usize;
        let y1 = (rect.y + rect.h as i32).min(info.height as i32) as usize;

        for y in y0..y1 {
            let row_start = y * stride;
            for x in x0..x1 {
                let offset = row_start + x * bpp as usize;
                if offset + bpp as usize > buffer.len() {
                    return;
                }
                match info.format {
                    PixelFormat::Bgrx8888 => {
                        buffer[offset] = color.b();
                        buffer[offset + 1] = color.g();
                        buffer[offset + 2] = color.r();
                        buffer[offset + 3] = 0;
                    }
                    PixelFormat::Rgbx8888 => {
                        buffer[offset] = color.r();
                        buffer[offset + 1] = color.g();
                        buffer[offset + 2] = color.b();
                        buffer[offset + 3] = 0;
                    }
                    PixelFormat::Bgr888 => {
                        buffer[offset] = color.b();
                        buffer[offset + 1] = color.g();
                        buffer[offset + 2] = color.r();
                    }
                    PixelFormat::Rgb888 => {
                        buffer[offset] = color.r();
                        buffer[offset + 1] = color.g();
                        buffer[offset + 2] = color.b();
                    }
                    PixelFormat::Rgb565 => {
                        let r5 = (color.r() >> 3) as u16;
                        let g6 = (color.g() >> 2) as u16;
                        let b5 = (color.b() >> 3) as u16;
                        let packed = (r5 << 11) | (g6 << 5) | b5;
                        buffer[offset] = (packed & 0xFF) as u8;
                        buffer[offset + 1] = (packed >> 8) as u8;
                    }
                    PixelFormat::Unknown => {}
                }
            }
        }
    }

    fn put_pixel(&mut self, point: Point, color: Rgb888) {
        let info = self.fb.info();
        if point.x < 0 || point.y < 0 || point.x >= info.width as i32 || point.y >= info.height as i32 {
            return;
        }

        let buffer = self.fb.buffer_mut();
        let bpp = match info.format {
            PixelFormat::Bgrx8888 | PixelFormat::Rgbx8888 => 4,
            PixelFormat::Rgb888 | PixelFormat::Bgr888 => 3,
            PixelFormat::Rgb565 => 2,
            _ => return, // Unknown
        };

        // Trust the platform-reported stride when present; fall back to tight rows.
        let stride = calc_stride_bytes(info.width, bpp, info.stride);

        let offset = (point.y as usize * stride as usize) + (point.x as usize * bpp as usize);
        if offset + (bpp as usize) > buffer.len() {
            return;
        }

        match info.format {
            PixelFormat::Bgrx8888 => {
                buffer[offset] = color.b();
                buffer[offset + 1] = color.g();
                buffer[offset + 2] = color.r();
                buffer[offset + 3] = 0;
            }
            PixelFormat::Rgbx8888 => {
                buffer[offset] = color.r();
                buffer[offset + 1] = color.g();
                buffer[offset + 2] = color.b();
                buffer[offset + 3] = 0;
            }
             PixelFormat::Bgr888 => {
                buffer[offset] = color.b();
                buffer[offset + 1] = color.g();
                buffer[offset + 2] = color.r();
            }
            PixelFormat::Rgb888 => {
                buffer[offset] = color.r();
                buffer[offset + 1] = color.g();
                buffer[offset + 2] = color.b();
            }
            PixelFormat::Rgb565 => {
                // Convert 8-bit channels to 5-6-5, little-endian
                let r5 = (color.r() >> 3) as u16;
                let g6 = (color.g() >> 2) as u16;
                let b5 = (color.b() >> 3) as u16;
                let packed = (r5 << 11) | (g6 << 5) | b5;
                buffer[offset] = (packed & 0xFF) as u8;
                buffer[offset + 1] = (packed >> 8) as u8;
            }
            _ => {}
        }
    }
}

impl<'a, F: FramebufferTarget> FbDrawer<'a, F> {
    fn clear(&mut self, color: Rgb888) {
        let info = self.fb.info();
        match info.format {
            PixelFormat::Bgrx8888 => {
                let c = u32::from_le_bytes([color.b(), color.g(), color.r(), 0]);
                self.fb.clear(c);
            }
            PixelFormat::Rgbx8888 => {
                let c = u32::from_le_bytes([color.r(), color.g(), color.b(), 0]);
                self.fb.clear(c);
            }
            PixelFormat::Rgb888 => {
                let c = u32::from_le_bytes([color.r(), color.g(), color.b(), 0]);
                self.fb.clear(c);
            }
            PixelFormat::Bgr888 => {
                let c = u32::from_le_bytes([color.b(), color.g(), color.r(), 0]);
                self.fb.clear(c);
            }
            PixelFormat::Rgb565 => {
                let r5 = (color.r() >> 3) as u16;
                let g6 = (color.g() >> 2) as u16;
                let b5 = (color.b() >> 3) as u16;
                let packed = (r5 << 11) | (g6 << 5) | b5;
                self.fb.clear(packed as u32);
            }
            PixelFormat::Unknown => {}
        };
    }
}
