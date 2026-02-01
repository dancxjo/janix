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

// Colors
const COLOR_BG: Rgb888 = Rgb888::BLACK;
const COLOR_TIME: Rgb888 = Rgb888::CYAN;
const COLOR_SOURCE: Rgb888 = Rgb888::YELLOW;
const COLOR_MSG_DEFAULT: Rgb888 = Rgb888::WHITE;
const COLOR_MSG_ERROR: Rgb888 = Rgb888::RED;
const COLOR_MSG_WARN: Rgb888 = Rgb888::MAGENTA;
const COLOR_MSG_INFO: Rgb888 = Rgb888::WHITE;
const COLOR_MSG_DEBUG: Rgb888 = Rgb888::new(128, 128, 128); // Dim Gray

// Layout
const TEXT_PAD: i32 = 5;

pub struct BootUpDisplay<F: FramebufferTarget> {
    fb: F,
    last_msg_area: Option<Rectangle>,
}

impl<F: FramebufferTarget> BootUpDisplay<F> {
    pub fn new(mut fb: F) -> Self {
        {
            let mut drawer = FbDrawer { fb: &mut fb };
            drawer.clear(COLOR_BG).ok();
        }

        Self {
            fb,
            last_msg_area: None,
        }
    }

    pub fn into_inner(self) -> F {
        self.fb
    }

    pub fn render_log_line(&mut self, line: &str) {
        let mut drawer = FbDrawer { fb: &mut self.fb };

        // Clear previous message area
        if let Some(rect) = self.last_msg_area {
            rect.into_styled(PrimitiveStyle::with_fill(COLOR_BG))
                .draw(&mut drawer)
                .ok();
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
        if let Some(src) = parts.source {
            lines_to_draw[line_count] = Some((src, COLOR_SOURCE));
            line_count += 1;
        }

        // 3. Message (Bottom)
        let msg_color = get_msg_color(parts.message);
        lines_to_draw[line_count] = Some((parts.message, msg_color));
        line_count += 1;

        let total_h = (line_count as i32 * CHAR_HEIGHT as i32) +
                      ((line_count as i32 - 1).max(0) * TEXT_PAD);

        let start_y = center_y - (total_h / 2);

        let mut current_y = start_y;
        let mut bounding_box: Option<Rectangle> = None;

        for i in 0..line_count {
            if let Some((text, color)) = lines_to_draw[i] {
                let text_w = text.len() as i32 * CHAR_WIDTH as i32;
                let start_x = center_x - (text_w / 2);

                draw_string(&mut drawer, start_x, current_y, text, color, COLOR_BG);

                let line_rect = Rectangle::new(
                    Point::new(start_x, current_y),
                    Size::new(text_w as u32, CHAR_HEIGHT as u32)
                );

                bounding_box = match bounding_box {
                    Some(bb) => {
                         let min_x = bb.top_left.x.min(line_rect.top_left.x);
                         let min_y = bb.top_left.y.min(line_rect.top_left.y);
                         // Accessing top_left and size assuming they are public or available via methods.
                         // embedded_graphics Rectangle fields are public? Checking docs/memory.
                         // Rectangle has `top_left` and `size` fields which are public.
                         // Need to compute max extent.
                         let bb_right = bb.top_left.x + bb.size.width as i32;
                         let bb_bottom = bb.top_left.y + bb.size.height as i32;
                         let lr_right = line_rect.top_left.x + line_rect.size.width as i32;
                         let lr_bottom = line_rect.top_left.y + line_rect.size.height as i32;

                         let max_x = bb_right.max(lr_right);
                         let max_y = bb_bottom.max(lr_bottom);

                         Some(Rectangle::new(
                             Point::new(min_x, min_y),
                             Size::new((max_x - min_x) as u32, (max_y - min_y) as u32)
                         ))
                    },
                    None => Some(line_rect),
                };

                current_y += CHAR_HEIGHT as i32 + TEXT_PAD;
            }
        }

        self.last_msg_area = bounding_box;
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

fn get_msg_color(msg: &str) -> Rgb888 {
    if msg.contains("ERROR") { COLOR_MSG_ERROR }
    else if msg.contains("WARN") { COLOR_MSG_WARN }
    else if msg.contains("INFO") { COLOR_MSG_INFO }
    else if msg.contains("DEBUG") { COLOR_MSG_DEBUG }
    else { COLOR_MSG_DEFAULT }
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

        let width_bytes = info.width * bpp; // u32 * u32 -> u32
        let stride = if info.stride > 0 {
             if info.stride < width_bytes {
                 info.stride * bpp
             } else {
                 info.stride
             }
        } else {
            width_bytes
        };

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
