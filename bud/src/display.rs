use core::str;
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    pixelcolor::{Rgb888, RgbColor},
    draw_target::DrawTarget,
    geometry::Point,
};
use crate::framebuffer::{FramebufferTarget, PixelFormat};
use crate::font::{SimpleFont, CHAR_WIDTH, CHAR_HEIGHT};
use crate::parser::parse_log_line;

// Colors (DOS-ish)
const COLOR_BG: Rgb888 = Rgb888::new(0, 0, 128); // Deep Blue
const COLOR_BORDER: Rgb888 = Rgb888::WHITE;
const COLOR_TIME: Rgb888 = Rgb888::CYAN;
const COLOR_SOURCE: Rgb888 = Rgb888::YELLOW;
const COLOR_MSG_DEFAULT: Rgb888 = Rgb888::WHITE;
const COLOR_MSG_ERROR: Rgb888 = Rgb888::RED;
const COLOR_MSG_WARN: Rgb888 = Rgb888::MAGENTA;
const COLOR_MSG_INFO: Rgb888 = Rgb888::WHITE;
const COLOR_MSG_DEBUG: Rgb888 = Rgb888::new(128, 128, 128); // Dim Gray

// Layout
const MARGIN: i32 = 20;
const TEXT_PAD: i32 = 5;

// Buffer constants
const MAX_LINES: usize = 32;
const MAX_LINE_LEN: usize = 128;

struct LogBuffer {
    lines: [[u8; MAX_LINE_LEN]; MAX_LINES],
    line_lens: [usize; MAX_LINES],
    head: usize,
    count: usize,
}

impl LogBuffer {
    fn new() -> Self {
        Self {
            lines: [[0; MAX_LINE_LEN]; MAX_LINES],
            line_lens: [0; MAX_LINES],
            head: 0,
            count: 0,
        }
    }

    fn add_line(&mut self, line: &str) {
        let idx = self.head;
        let bytes = line.as_bytes();
        let len = bytes.len().min(MAX_LINE_LEN);

        self.lines[idx][..len].copy_from_slice(&bytes[..len]);
        self.line_lens[idx] = len;

        self.head = (self.head + 1) % MAX_LINES;
        if self.count < MAX_LINES {
            self.count += 1;
        }
    }
}

pub struct BootUpDisplay<F: FramebufferTarget> {
    fb: F,
    buffer: LogBuffer,
}

impl<F: FramebufferTarget> BootUpDisplay<F> {
    pub fn new(mut fb: F) -> Self {
        // Initial clear and border draw
        {
            let mut drawer = FbDrawer { fb: &mut fb };
            let info = drawer.fb.info();
            let width = info.width as i32;
            let height = info.height as i32;

            // Fill background
            drawer.clear(COLOR_BG).ok();

            // Draw border
            let rect = Rectangle::new(
                Point::new(MARGIN, MARGIN),
                Size::new((width - 2 * MARGIN) as u32, (height - 2 * MARGIN) as u32),
            );

            let style = PrimitiveStyle::with_stroke(COLOR_BORDER, 2);
            rect.into_styled(style).draw(&mut drawer).ok();
        }

        Self {
            fb,
            buffer: LogBuffer::new(),
        }
    }

    pub fn render_log_line(&mut self, line: &str) {
        self.buffer.add_line(line);

        let mut drawer = FbDrawer { fb: &mut self.fb };
        let info = drawer.fb.info();
        let height = info.height as i32;
        let area_h = height - 2 * (MARGIN + TEXT_PAD);
        let max_visible_lines = (area_h as usize) / CHAR_HEIGHT;

        // Optimization: If appending (scrolling not needed yet), just draw the new line
        if self.buffer.count <= max_visible_lines && self.buffer.count > 0 {
             let start_x = MARGIN + TEXT_PAD;
             let start_y = MARGIN + TEXT_PAD;
             let line_idx = self.buffer.count - 1; // 0-indexed position on screen
             let y = start_y + (line_idx as i32 * CHAR_HEIGHT as i32);

             // The added line is at previous head
             let added_idx = (self.buffer.head + MAX_LINES - 1) % MAX_LINES;
             let added_len = self.buffer.line_lens[added_idx];
             if let Ok(s) = str::from_utf8(&self.buffer.lines[added_idx][..added_len]) {
                 draw_log_line(&mut drawer, start_x, y, s, COLOR_BG);
             }
        } else {
             // Scrolling needed, redraw all
             redraw_console_impl(&self.buffer, &mut drawer, max_visible_lines);
        }
    }
}

fn redraw_console_impl<F: FramebufferTarget>(buffer: &LogBuffer, drawer: &mut FbDrawer<F>, max_visible_lines: usize) {
    let start_x = MARGIN + TEXT_PAD;
    let start_y = MARGIN + TEXT_PAD;

    let lines_to_show = buffer.count.min(max_visible_lines);
    if lines_to_show == 0 {
        return;
    }

    let first_idx = (buffer.head + MAX_LINES - lines_to_show) % MAX_LINES;

    for i in 0..lines_to_show {
        let idx = (first_idx + i) % MAX_LINES;
        let len = buffer.line_lens[idx];
        let buf = &buffer.lines[idx][..len];

        if let Ok(s) = str::from_utf8(buf) {
             let y = start_y + (i as i32 * CHAR_HEIGHT as i32);
             draw_log_line(drawer, start_x, y, s, COLOR_BG);
        }
    }
}

fn draw_log_line<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, line: &str, bg_color: Rgb888) {
    let parts = parse_log_line(line);
    let mut current_x = x;

    // Time
    if let Some(_t) = parts.time {
         if let Some(end) = line.find(']') {
             let time_str = &line[0..=end];
             draw_string(drawer, current_x, y, time_str, COLOR_TIME, bg_color);
             current_x += (time_str.len() * CHAR_WIDTH) as i32;
             draw_string(drawer, current_x, y, " ", COLOR_BG, bg_color); // spacer
             current_x += CHAR_WIDTH as i32;
         }
    }

    // Source
    if let Some(_src) = parts.source {
        let rem = line;
        if let Some(_time_end) = rem.find(']') {
             let mut rem_line = line;
             if parts.time.is_some() {
                 if let Some(end) = rem_line.find(']') {
                     let time_part = &rem_line[0..=end];
                     draw_string(drawer, current_x, y, time_part, COLOR_TIME, bg_color);
                     current_x += (time_part.len() * CHAR_WIDTH) as i32;
                     rem_line = &rem_line[end+1..];
                 }
             }

             let trimmed_rem = rem_line.trim_start();
             let spaces_count = rem_line.len() - trimmed_rem.len();
             if spaces_count > 0 {
                 draw_string(drawer, current_x, y, &rem_line[..spaces_count], COLOR_DEFAULT, bg_color);
                 current_x += (spaces_count * CHAR_WIDTH) as i32;
                 rem_line = trimmed_rem;
             }

             if parts.source.is_some() && rem_line.starts_with('[') {
                  if let Some(end) = rem_line.find(']') {
                      let src_part = &rem_line[0..=end];
                      draw_string(drawer, current_x, y, src_part, COLOR_SOURCE, bg_color);
                      current_x += (src_part.len() * CHAR_WIDTH) as i32;
                      rem_line = &rem_line[end+1..];
                  }
             }

             let msg_color = get_msg_color(rem_line);
             draw_string(drawer, current_x, y, rem_line, msg_color, bg_color);
             clear_line_end(drawer, current_x + (rem_line.len() * CHAR_WIDTH) as i32, y, bg_color);
             return;
        }
    }

    // Fallback
    current_x = x;
    let mut rem_line = line;

    if parts.time.is_some() {
         if let Some(end) = rem_line.find(']') {
             let chunk = &rem_line[0..=end];
             draw_string(drawer, current_x, y, chunk, COLOR_TIME, bg_color);
             current_x += (chunk.len() * CHAR_WIDTH) as i32;
             rem_line = &rem_line[end+1..];
         }
    }

    if parts.source.is_some() {
         let trimmed = rem_line.trim_start();
         let n_spaces = rem_line.len() - trimmed.len();
         if n_spaces > 0 {
              draw_string(drawer, current_x, y, &rem_line[..n_spaces], COLOR_DEFAULT, bg_color);
              current_x += (n_spaces * CHAR_WIDTH) as i32;
              rem_line = trimmed;
         }

         if rem_line.starts_with('[') {
             if let Some(end) = rem_line.find(']') {
                 let chunk = &rem_line[0..=end];
                 draw_string(drawer, current_x, y, chunk, COLOR_SOURCE, bg_color);
                 current_x += (chunk.len() * CHAR_WIDTH) as i32;
                 rem_line = &rem_line[end+1..];
             }
         }
    }

    let msg_color = get_msg_color(rem_line);
    draw_string(drawer, current_x, y, rem_line, msg_color, bg_color);
    clear_line_end(drawer, current_x + (rem_line.len() * CHAR_WIDTH) as i32, y, bg_color);
}

fn draw_string<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, s: &str, color: Rgb888, bg_color: Rgb888) {
    let mut cur_x = x;
    for c in s.bytes() {
         draw_char(drawer, cur_x, y, c, color, bg_color);
         cur_x += CHAR_WIDTH as i32;
    }
}

fn clear_line_end<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, bg_color: Rgb888) {
    let info = drawer.fb.info();
    let width = info.width as i32;
    let end_x = width - (MARGIN + TEXT_PAD);

    if x < end_x {
        let rect = Rectangle::new(
            Point::new(x, y),
            Size::new((end_x - x) as u32, CHAR_HEIGHT as u32)
        );
        rect.into_styled(PrimitiveStyle::with_fill(bg_color)).draw(drawer).ok();
    }
}

const COLOR_DEFAULT: Rgb888 = Rgb888::WHITE;

fn get_msg_color(msg: &str) -> Rgb888 {
    if msg.contains("ERROR") { COLOR_MSG_ERROR }
    else if msg.contains("WARN") { COLOR_MSG_WARN }
    else if msg.contains("INFO") { COLOR_MSG_INFO }
    else if msg.contains("DEBUG") { COLOR_MSG_DEBUG }
    else { COLOR_MSG_DEFAULT }
}

fn draw_char<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, c: u8, color: Rgb888, bg_color: Rgb888) {
    let glyph = SimpleFont::get_glyph(c);
    for gy in 0..CHAR_HEIGHT {
        let row_byte = glyph[gy];
        for gx in 0..CHAR_WIDTH {
             let px = x + gx as i32;
             let py = y + gy as i32;
             if (row_byte >> (7 - gx)) & 1 != 0 {
                 drawer.put_pixel(Point::new(px, py), color);
             } else {
                 drawer.put_pixel(Point::new(px, py), bg_color);
             }
        }
    }
}

struct FbDrawer<'a, F: FramebufferTarget> {
    fb: &'a mut F,
}

impl<'a, F: FramebufferTarget> FbDrawer<'a, F> {
    fn put_pixel(&mut self, point: Point, color: Rgb888) {
        let info = self.fb.info();
        if point.x < 0 || point.y < 0 || point.x >= info.width as i32 || point.y >= info.height as i32 {
            return;
        }

        let buffer = self.fb.buffer_mut();
        let bpp = match info.format {
            PixelFormat::Bgrx8888 | PixelFormat::Rgbx8888 => 4,
            PixelFormat::Rgb888 | PixelFormat::Bgr888 => 3,
            _ => return, // Unknown
        };

        let min_stride = match info.format {
            PixelFormat::Bgrx8888 | PixelFormat::Rgbx8888 => info.width * 4,
            PixelFormat::Rgb888 | PixelFormat::Bgr888 => info.width * 3,
            _ => info.width,
        };
        let stride = if info.stride < min_stride { min_stride } else { info.stride };

        let offset = (point.y as usize * stride as usize) + (point.x as usize * bpp);
        if offset + bpp > buffer.len() {
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
            _ => {}
        }
    }
}

impl<'a, F: FramebufferTarget> DrawTarget for FbDrawer<'a, F> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
             self.put_pixel(point, color);
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let info = self.fb.info();
        let c = match info.format {
            PixelFormat::Bgrx8888 => u32::from_le_bytes([color.b(), color.g(), color.r(), 0]),
            PixelFormat::Rgbx8888 => u32::from_le_bytes([color.r(), color.g(), color.b(), 0]),
            _ => u32::from_le_bytes([color.b(), color.g(), color.r(), 0]),
        };
        self.fb.clear(c);
        Ok(())
    }
}

impl<'a, F: FramebufferTarget> OriginDimensions for FbDrawer<'a, F> {
    fn size(&self) -> Size {
        let info = self.fb.info();
        Size::new(info.width, info.height)
    }
}
