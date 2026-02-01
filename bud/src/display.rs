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

pub struct BootUpDisplay<F: FramebufferTarget> {
    fb: F,
    // Ring buffer
    lines: [[u8; MAX_LINE_LEN]; MAX_LINES],
    line_lens: [usize; MAX_LINES],
    head: usize, // Index where the NEXT line will be written
    count: usize, // Number of valid lines in buffer
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
            lines: [[0; MAX_LINE_LEN]; MAX_LINES],
            line_lens: [0; MAX_LINES],
            head: 0,
            count: 0,
        }
    }

    pub fn render_log_line(&mut self, line: &str) {
        // Add to buffer
        let idx = self.head;
        let bytes = line.as_bytes();
        let len = bytes.len().min(MAX_LINE_LEN);

        self.lines[idx][..len].copy_from_slice(&bytes[..len]);
        self.line_lens[idx] = len;

        self.head = (self.head + 1) % MAX_LINES;
        if self.count < MAX_LINES {
            self.count += 1;
        }

        self.redraw_console();
    }

    fn redraw_console(&mut self) {
        let mut drawer = FbDrawer { fb: &mut self.fb };
        let info = drawer.fb.info();
        let width = info.width as i32;
        let height = info.height as i32;

        // Define text area bounds
        let start_x = MARGIN + TEXT_PAD;
        let start_y = MARGIN + TEXT_PAD;
        let area_w = width - 2 * (MARGIN + TEXT_PAD);
        let area_h = height - 2 * (MARGIN + TEXT_PAD);
        let max_visible_lines = (area_h as usize) / CHAR_HEIGHT;

        // Clear text area (fill with BG)
        let rect = Rectangle::new(
            Point::new(start_x, start_y),
            Size::new(area_w as u32, area_h as u32),
        );
        rect.into_styled(PrimitiveStyle::with_fill(COLOR_BG)).draw(&mut drawer).ok();

        // Calculate start index in ring buffer to show the latest lines
        // We want to show up to `max_visible_lines` lines ending at `self.head - 1`.

        let lines_to_show = self.count.min(max_visible_lines);
        if lines_to_show == 0 {
            return;
        }

        // The newest line is at (self.head + MAX_LINES - 1) % MAX_LINES
        // The oldest line to show is at (self.head + MAX_LINES - lines_to_show) % MAX_LINES

        let first_idx = (self.head + MAX_LINES - lines_to_show) % MAX_LINES;

        for i in 0..lines_to_show {
            let idx = (first_idx + i) % MAX_LINES;
            let len = self.line_lens[idx];
            let buf = &self.lines[idx][..len];

            // Try to parse as UTF-8
            if let Ok(s) = str::from_utf8(buf) {
                 let y = start_y + (i as i32 * CHAR_HEIGHT as i32);
                 draw_log_line(&mut drawer, start_x, y, s);
            }
        }
    }
}

fn draw_log_line<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, line: &str) {
    let parts = parse_log_line(line);

    let mut current_x = x;

    // Time
    if let Some(_t) = parts.time {
         // Basic formatting: "[timestamp] "
         if let Some(end) = line.find(']') {
             let time_str = &line[0..=end];
             draw_string(drawer, current_x, y, time_str, COLOR_TIME);
             current_x += (time_str.len() * CHAR_WIDTH) as i32;
             draw_string(drawer, current_x, y, " ", COLOR_BG); // spacer
             current_x += CHAR_WIDTH as i32;
         }
    }

    // Source
    if let Some(_src) = parts.source {
        // Re-parse offsets:
        let rem = line;
        if let Some(_time_end) = rem.find(']') {
             // We already handled time drawing above? No, above was inside `if parts.time`.
             // Wait, if we handled time above, we shouldn't draw it again?
             // Actually, the block `if let Some(src) = parts.source` is handling the whole line drawing if source is present?
             // My previous logic was flawed. It drew time inside `if parts.time` AND then attempted to draw parts inside `if parts.source`.

             // Correct logic:
             // If we have time and source, we want to draw [time] [source] msg.
             // If we have just time, [time] msg.
             // If neither, just msg.

             // However, `parts.time` and `parts.source` are independent.
             // But my `parse_log_line` only finds source if time is found first.

             // Let's rewrite the flow.

             // Start from the beginning of string.
             let mut rem_line = line;

             // Draw Time if present
             if parts.time.is_some() {
                 if let Some(end) = rem_line.find(']') {
                     let time_part = &rem_line[0..=end];
                     draw_string(drawer, current_x, y, time_part, COLOR_TIME);
                     current_x += (time_part.len() * CHAR_WIDTH) as i32;

                     // Advance
                     let next_start = end + 1;
                     rem_line = &rem_line[next_start..];

                     // Draw spacing spaces? `rem_line` might have leading spaces.
                     // We should draw them or skip them?
                     // If we skip them, we normalize spacing.
                     // The original `line` has spaces.
                     // Let's mimic original spacing but just colorize parts.
                 }
             }

             // Determine if we have source at current `rem_line`
             // `parse_log_line` expects `[source]` after time.
             // We can check if `rem_line` trimmed start starts with `[`

             let trimmed_rem = rem_line.trim_start();
             let spaces_count = rem_line.len() - trimmed_rem.len();
             if spaces_count > 0 {
                 draw_string(drawer, current_x, y, &rem_line[..spaces_count], COLOR_DEFAULT);
                 current_x += (spaces_count * CHAR_WIDTH) as i32;
                 rem_line = trimmed_rem;
             }

             if parts.source.is_some() && rem_line.starts_with('[') {
                  if let Some(end) = rem_line.find(']') {
                      let src_part = &rem_line[0..=end];
                      draw_string(drawer, current_x, y, src_part, COLOR_SOURCE);
                      current_x += (src_part.len() * CHAR_WIDTH) as i32;

                      rem_line = &rem_line[end+1..];
                  }
             }

             // Draw remaining message
             let msg_color = get_msg_color(rem_line);
             draw_string(drawer, current_x, y, rem_line, msg_color);
             return;
        }
    }

    // Fallback if no source found (or time found but no source)
    // If time was found, we already partially drew it?
    // My previous block `if let Some(_src)` wrapped the rest.
    // If I split logic, I need to be careful.

    // Let's restart logic completely in this function.

    // Reset x
    current_x = x;
    let mut rem_line = line;

    // 1. Time
    if parts.time.is_some() {
         if let Some(end) = rem_line.find(']') {
             let chunk = &rem_line[0..=end];
             draw_string(drawer, current_x, y, chunk, COLOR_TIME);
             current_x += (chunk.len() * CHAR_WIDTH) as i32;
             rem_line = &rem_line[end+1..];
         }
    }

    // 2. Spacing + Source
    if parts.source.is_some() {
         // Draw intervening spaces
         let trimmed = rem_line.trim_start();
         let n_spaces = rem_line.len() - trimmed.len();
         if n_spaces > 0 {
              draw_string(drawer, current_x, y, &rem_line[..n_spaces], COLOR_DEFAULT);
              current_x += (n_spaces * CHAR_WIDTH) as i32;
              rem_line = trimmed;
         }

         if rem_line.starts_with('[') {
             if let Some(end) = rem_line.find(']') {
                 let chunk = &rem_line[0..=end];
                 draw_string(drawer, current_x, y, chunk, COLOR_SOURCE);
                 current_x += (chunk.len() * CHAR_WIDTH) as i32;
                 rem_line = &rem_line[end+1..];
             }
         }
    }

    // 3. Message
    let msg_color = get_msg_color(rem_line);
    draw_string(drawer, current_x, y, rem_line, msg_color);
}

fn draw_string<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, s: &str, color: Rgb888) {
    let mut cur_x = x;
    for c in s.bytes() {
         draw_char(drawer, cur_x, y, c, color);
         cur_x += CHAR_WIDTH as i32;
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

fn draw_char<F: FramebufferTarget>(drawer: &mut FbDrawer<F>, x: i32, y: i32, c: u8, color: Rgb888) {
    let glyph = SimpleFont::get_glyph(c);
    for gy in 0..CHAR_HEIGHT {
        let row_byte = glyph[gy];
        for gx in 0..CHAR_WIDTH {
             if (row_byte >> (7 - gx)) & 1 != 0 {
                 let px = x + gx as i32;
                 let py = y + gy as i32;
                 drawer.put_pixel(Point::new(px, py), color);
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

        let offset = (point.y as usize * info.stride as usize) + (point.x as usize * bpp);
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
}

impl<'a, F: FramebufferTarget> OriginDimensions for FbDrawer<'a, F> {
    fn size(&self) -> Size {
        let info = self.fb.info();
        Size::new(info.width, info.height)
    }
}
