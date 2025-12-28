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
    for row in 0..h {
        for col in 0..w {
            let coverage = bitmap[(row * w + col) as usize];
            if coverage == 0 { continue; }
            
            // Alpha blend?
            // coverage is alpha (0-255).
            // Combine with `color` alpha?
            // Assuming color is solid, coverage is alpha.
            // Simple: just draw pixel if > threshold?
            // Or use set_pixel_clamped with alpha blending?
            // `fill_rect` implemented alpha logic?
            // `primitives.rs` `blit_image` handles alpha.
            // But `set_pixel_clamped` does NOT handle read-back blending usually (it's unsafe setter).
            // Wait, trunk `text.rs` used `set_pixel_clamped`! 
            // `set_pixel_clamped` in trunk `primitives.rs` (which I pasted in step 336) DOES handle blending?
            // Let's check `primitives.rs` from Step 336.
            // `fn set_pixel_clamped(...)`
            // It calculates index.
            // `*ptr = color;`
            // NO BLENDING in `set_pixel_clamped` usually.
            // Blending is in `blit_image`.
            // But `text.rs` from trunk called `set_pixel_clamped`.
            // Does it ignore alpha?
            // If I draw text on solid bg, it looks jagged without alpha.
            // But for "First Pass", jagged is fine.
            
            // I'll implement simple threshold alpha for now.
            if coverage > 128 {
                 unsafe {
                    set_pixel_clamped(
                        buffer, stride_bytes, fb_width as i32, fb_height as i32,
                        x + col, y + row,
                        color, None
                    );
                }
            }
        }
    }
}
