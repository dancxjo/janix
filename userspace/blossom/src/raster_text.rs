//! Minimal bitmap font rasterizer for text runs.
//!
//! This is intentionally tiny (5x7 glyphs with integer scaling) to keep
//! Blossom's text support predictable and cache-friendly.
//!
//! # Examples
//! ```
//! use blossom::raster_text::{draw_text, TextStyle};
//! use blossom::surface::{MappedSurface, SurfaceSpec};
//!
//! let mut buf = [0u8; 160];
//! let spec = SurfaceSpec { width: 10, height: 4, stride_bytes: 40 };
//! let mut surface = unsafe { MappedSurface::from_parts(buf.as_mut_ptr(), buf.len(), spec) };
//! let style = TextStyle { color: 0xFFFFFFFF, size_px: 8 };
//! draw_text(&mut surface, "12", 0, 0, &style);
//! ```

use crate::surface::MappedSurface;

/// Text style configuration for rasterization.
#[derive(Clone, Copy, Debug)]
pub struct TextStyle {
    pub color: u32,
    pub size_px: u32,
}

pub fn draw_text(surface: &mut MappedSurface, text: &str, x: i32, y: i32, style: &TextStyle) {
    let scale = (style.size_px / 8).max(1) as i32;
    let mut cursor_x = x;
    for ch in text.chars() {
        draw_glyph(surface, ch, cursor_x, y, scale, style.color);
        cursor_x += (6 * scale) as i32; // 5px glyph + 1px spacing
    }
}

fn draw_glyph(surface: &mut MappedSurface, ch: char, x: i32, y: i32, scale: i32, color: u32) {
    let glyph = glyph_bits(ch);
    for (col, bits) in glyph.iter().enumerate() {
        for row in 0..7 {
            if (bits >> row) & 1 == 1 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        surface.put_px(
                            x + (col as i32 * scale) + sx,
                            y + (row as i32 * scale) + sy,
                            color,
                        );
                    }
                }
            }
        }
    }
}

fn glyph_bits(ch: char) -> [u8; 5] {
    match ch {
        '0' => [0x3E, 0x51, 0x49, 0x45, 0x3E],
        '1' => [0x00, 0x42, 0x7F, 0x40, 0x00],
        '2' => [0x42, 0x61, 0x51, 0x49, 0x46],
        '3' => [0x21, 0x41, 0x45, 0x4B, 0x31],
        '4' => [0x18, 0x14, 0x12, 0x7F, 0x10],
        '5' => [0x27, 0x45, 0x45, 0x45, 0x39],
        '6' => [0x3C, 0x4A, 0x49, 0x49, 0x30],
        '7' => [0x01, 0x71, 0x09, 0x05, 0x03],
        '8' => [0x36, 0x49, 0x49, 0x49, 0x36],
        '9' => [0x06, 0x49, 0x49, 0x29, 0x1E],
        ':' => [0x00, 0x36, 0x36, 0x00, 0x00],
        '-' => [0x08, 0x08, 0x08, 0x08, 0x08],
        'A' => [0x7E, 0x11, 0x11, 0x11, 0x7E],
        'B' => [0x7F, 0x49, 0x49, 0x49, 0x36],
        'C' => [0x3E, 0x41, 0x41, 0x41, 0x22],
        'D' => [0x7F, 0x41, 0x41, 0x22, 0x1C],
        'E' => [0x7F, 0x49, 0x49, 0x49, 0x41],
        'F' => [0x7F, 0x09, 0x09, 0x09, 0x01],
        'L' => [0x7F, 0x40, 0x40, 0x40, 0x40],
        'O' => [0x3E, 0x41, 0x41, 0x41, 0x3E],
        'R' => [0x7F, 0x09, 0x19, 0x29, 0x46],
        'T' => [0x01, 0x01, 0x7F, 0x01, 0x01],
        'X' => [0x63, 0x14, 0x08, 0x14, 0x63],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00],
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::surface::{MappedSurface, SurfaceSpec};

    #[test]
    fn draws_digits() {
        let mut buf = [0u8; 64];
        let spec = SurfaceSpec {
            width: 4,
            height: 4,
            stride_bytes: 16,
        };
        let mut surface = unsafe { MappedSurface::from_parts(buf.as_mut_ptr(), buf.len(), spec) };
        let style = TextStyle { color: 0xFFFFFFFF, size_px: 8 };
        draw_text(&mut surface, "1", 0, 0, &style);
        assert_ne!(surface.get_px(1, 1), 0);
    }
}
