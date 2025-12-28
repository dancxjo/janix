extern crate alloc;
// use alloc::vec::Vec;
use fontdue::{Font, FontSettings};
use spin::Once;

use crate::render::primitives::{set_pixel_clamped};

// Embed the font for now to ensure working rendering without complex graph discovery first.
// This aligns with "Reintroduce v0.1 features" (which had embedded fonts).
static FONT_DATA: &[u8] = include_bytes!("../../../../../assets/fonts/Hack-Regular.ttf");

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

pub fn draw_text(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    text: &str,
    color: u32
) {
    let engine = FontEngine::global();
    let font = engine.font();
    let size_px = 20.0; // Hardcoded size for now

    let mut cursor_x = x as f32;
    let mut cursor_y = y as f32;

    for ch in text.chars() {
        if ch == '\n' {
            cursor_x = x as f32;
            cursor_y += size_px; // Line height
            continue;
        }

        let (metrics, bitmap) = font.rasterize(ch, size_px);

        // Blit bitmap
        let glyph_x = cursor_x as i32 + metrics.xmin;
        // Fontdue coordinates: y is UP from baseline? No, rasterize returns bitmap.
        // We usually draw at baseline. 
        // cursor_y is top-left of line.
        // metrics.height is height of glyph.
        // metrics.ymin is y-offset from baseline.
        // Typically: draw_y = baseline - metrics.height - metrics.ymin ?
        // Simplification: draw at cursor_y + (ascent - bearing)?
        // For simple rendering: y + size - height - offset?
        
        // Let's just try drawing at cursor_y + (size - height).
        // Actually, just verify it appears first.
        let glyph_y = cursor_y as i32 + (size_px as i32 - metrics.height as i32 - metrics.ymin as i32); 

        blit_glyph(
            buffer, stride_bytes, fb_width, fb_height,
            glyph_x, glyph_y,
            &bitmap, metrics.width as i32, metrics.height as i32,
            color
        );

        cursor_x += metrics.advance_width;
    }
}

fn blit_glyph(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    bitmap: &[u8],
    w: i32,
    h: i32,
    color: u32
) {
    let color_a = (color >> 24) & 0xFF;
    let color_r = (color >> 16) & 0xFF;
    let color_g = (color >> 8) & 0xFF;
    let color_b = color & 0xFF;

    for row in 0..h {
        let draw_y = y + row;
        if draw_y < 0 || draw_y >= fb_height as i32 { continue; }

        for col in 0..w {
            let draw_x = x + col;
            if draw_x < 0 || draw_x >= fb_width as i32 { continue; }

            let coverage = bitmap[(row * w + col) as usize] as u32;
            if coverage == 0 { continue; }
            
            // Combine font coverage with text color alpha
            let final_alpha = (coverage * color_a) / 255;
            
            if final_alpha == 0 { continue; }

            unsafe {
                let row_ptr = (buffer as *mut u8).add(draw_y as usize * stride_bytes as usize) as *mut u32;
                let pixel_ptr = row_ptr.add(draw_x as usize);
                
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
