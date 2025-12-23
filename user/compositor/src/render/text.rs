#![allow(dead_code)]

extern crate alloc;

use fontdue::{Font, FontSettings};
use spin::Once;

use crate::config::LINE_HEIGHT;
use crate::fonts::NOTO_SANS_REGULAR;
use crate::render::primitives::set_pixel_clamped;

pub struct FontEngine {
    font: Font,
}

impl FontEngine {
    fn new() -> Self {
        let font = Font::from_bytes(NOTO_SANS_REGULAR, FontSettings::default())
            .expect("failed to construct fontdue Font");
        Self { font }
    }

    fn global() -> &'static FontEngine {
        static FONT_ENGINE: Once<FontEngine> = Once::new();
        FONT_ENGINE.call_once(FontEngine::new)
    }
}

/// Draw UTF-8 text using fontdue with simple wrapping and newline handling.
pub fn draw_text(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    max_width: i32,
    max_height: i32,
    text: &str,
    color: u32,
) {
    if max_width <= 0 || max_height <= 0 {
        return;
    }

    let engine = FontEngine::global();
    let size_px: f32 = LINE_HEIGHT as f32;

    let mut cursor_x = x;
    let mut cursor_y = y;
    let max_x = x + max_width;
    let max_y = y + max_height;

    for ch in text.chars() {
        if cursor_y + LINE_HEIGHT > max_y {
            break;
        }

        if ch == '\n' {
            cursor_x = x;
            cursor_y += LINE_HEIGHT;
            if cursor_y >= max_y {
                break;
            }
            continue;
        }

        if cursor_x >= max_x {
            cursor_x = x;
            cursor_y += LINE_HEIGHT;
            if cursor_y >= max_y {
                break;
            }
        }

        let (metrics, bitmap) = engine.font.rasterize(ch, size_px);

        if cursor_x + metrics.width as i32 > max_x && cursor_x != x {
            cursor_x = x;
            cursor_y += LINE_HEIGHT;
            if cursor_y >= max_y {
                break;
            }
        }

        let advance = metrics.advance_width.max(1.0) as i32;

        if metrics.width == 0 || metrics.height == 0 {
            cursor_x += advance;
            continue;
        }

        let advance = metrics.advance_width as i32;
        let glyph_x = cursor_x + metrics.xmin;
        let glyph_y = cursor_y + LINE_HEIGHT - metrics.height as i32;

        blit_glyph_bitmap(
            buffer,
            stride_bytes,
            fb_width,
            fb_height,
            glyph_x,
            glyph_y,
            &bitmap,
            metrics.width as i32,
            metrics.height as i32,
            color,
        );

        cursor_x += advance.max(1);
        if cursor_x >= max_x {
            cursor_x = x;
            cursor_y += LINE_HEIGHT;
            if cursor_y >= max_y {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn draw_text_no_space_is_noop() {
        let mut buf = vec![0u32; 16];
        draw_text(buf.as_mut_ptr(), 16, 4, 4, 0, 0, 0, 0, "", 0xFF00FF00);
        assert!(buf.iter().all(|p| *p == 0));
    }

    #[test]
    fn draw_text_renders_glyphs() {
        let mut buf = vec![0u32; 64 * 64];
        draw_text(
            buf.as_mut_ptr(),
            64 * 4,
            64,
            64,
            2,
            2,
            40,
            20,
            "Hi",
            0xFF00FF00,
        );
        assert!(
            buf.iter().any(|p| *p == 0xFF00FF00),
            "expected text rendering to modify buffer"
        );
    }
}

fn blit_glyph_bitmap(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    bitmap: &[u8],
    glyph_width: i32,
    glyph_height: i32,
    color: u32,
) {
    if glyph_width <= 0 || glyph_height <= 0 {
        return;
    }

    for row in 0..glyph_height {
        for col in 0..glyph_width {
            let idx = (row * glyph_width + col) as usize;
            let coverage = bitmap.get(idx).copied().unwrap_or(0);
            if coverage == 0 {
                continue;
            }

            let px = x + col;
            let py = y + row;
            unsafe {
                set_pixel_clamped(
                    buffer,
                    stride_bytes,
                    fb_width as i32,
                    fb_height as i32,
                    px,
                    py,
                    color,
                    None, // TODO: Pass clip through draw_text if needed
                );
            }
        }
    }
}
