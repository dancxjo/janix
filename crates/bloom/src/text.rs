//! Text rendering using fontdue (no_std compatible).
//!
//! Inspired by v0.2 compositor text rendering.

use fontdue::{Font, FontSettings};
use spin::Once;

/// Embed the Hack font for simple, reliable rendering.
static FONT_DATA: &[u8] = include_bytes!("../../../assets/fonts/Hack-Regular.ttf");

static FONT_ENGINE: Once<FontEngine> = Once::new();

pub struct FontEngine {
    font: Font,
}

impl FontEngine {
    fn new() -> Self {
        let font = Font::from_bytes(FONT_DATA, FontSettings::default())
            .expect("Failed to parse embedded font");
        Self { font }
    }

    pub fn global() -> &'static FontEngine {
        FONT_ENGINE.call_once(FontEngine::new)
    }

    pub fn font(&self) -> &Font {
        &self.font
    }
}

/// Draw text to a framebuffer.
///
/// # Arguments
/// * `buffer` - Destination framebuffer (ARGB u32 pixels)
/// * `fb_width` - Framebuffer width in pixels
/// * `fb_height` - Framebuffer height in pixels
/// * `x` - X position (left edge of text)
/// * `y` - Y position (top of text line)
/// * `text` - The string to render
/// * `color` - Text color as ARGB u32
/// * `size_px` - Font size in pixels
pub fn draw_text(
    buffer: *mut u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    text: &str,
    color: u32,
    size_px: f32,
) {
    let engine = FontEngine::global();
    let font = engine.font();

    let mut cursor_x = x as f32;

    for ch in text.chars() {
        if ch == '\n' {
            break; // Title bars are single-line
        }

        let (metrics, bitmap) = font.rasterize(ch, size_px);

        let glyph_x = cursor_x as i32 + metrics.xmin;
        // Vertically position: y is the top of the line, so we offset by ascent
        let glyph_y = y + (size_px as i32 - metrics.height as i32 - metrics.ymin);

        blit_glyph(
            buffer,
            fb_width,
            fb_height,
            glyph_x,
            glyph_y,
            &bitmap,
            metrics.width as i32,
            metrics.height as i32,
            color,
        );

        cursor_x += metrics.advance_width;
    }
}

/// Measure the width of text at a given size.
pub fn measure_text_width(text: &str, size_px: f32) -> i32 {
    let engine = FontEngine::global();
    let font = engine.font();

    let mut width = 0.0f32;
    for ch in text.chars() {
        if ch == '\n' {
            break;
        }
        let metrics = font.metrics(ch, size_px);
        width += metrics.advance_width;
    }
    width as i32
}

fn blit_glyph(
    buffer: *mut u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    bitmap: &[u8],
    w: i32,
    h: i32,
    color: u32,
) {
    let color_a = (color >> 24) & 0xFF;
    let color_r = (color >> 16) & 0xFF;
    let color_g = (color >> 8) & 0xFF;
    let color_b = color & 0xFF;

    for row in 0..h {
        let draw_y = y + row;
        if draw_y < 0 || draw_y >= fb_height as i32 {
            continue;
        }

        for col in 0..w {
            let draw_x = x + col;
            if draw_x < 0 || draw_x >= fb_width as i32 {
                continue;
            }

            let coverage = bitmap[(row * w + col) as usize] as u32;
            if coverage == 0 {
                continue;
            }

            // Combine font coverage with text color alpha
            let final_alpha = (coverage * color_a) / 255;
            if final_alpha == 0 {
                continue;
            }

            unsafe {
                let idx = draw_y as usize * fb_width as usize + draw_x as usize;
                let pixel_ptr = buffer.add(idx);

                if final_alpha == 255 {
                    *pixel_ptr = color;
                } else {
                    let dst = *pixel_ptr;
                    let dst_r = (dst >> 16) & 0xFF;
                    let dst_g = (dst >> 8) & 0xFF;
                    let dst_b = dst & 0xFF;

                    let inv_a = 255 - final_alpha;

                    let out_r = (color_r * final_alpha + dst_r * inv_a) / 255;
                    let out_g = (color_g * final_alpha + dst_g * inv_a) / 255;
                    let out_b = (color_b * final_alpha + dst_b * inv_a) / 255;

                    *pixel_ptr = (0xFF << 24) | (out_r << 16) | (out_g << 8) | out_b;
                }
            }
        }
    }
}
