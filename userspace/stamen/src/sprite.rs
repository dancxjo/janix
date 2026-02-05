//! Built-in Cursor Sprites
//!
//! Simple, baked-in cursor sprites for immediate use without asset loading.

/// A simple 16x16 arrow cursor sprite (1-bit mask + BGRA pixels).
pub struct ArrowSprite;

impl ArrowSprite {
    /// Sprite width in pixels
    pub const WIDTH: u32 = 16;
    /// Sprite height in pixels
    pub const HEIGHT: u32 = 16;
    /// Hotspot X offset (tip of arrow)
    pub const HOTSPOT_X: i32 = 0;
    /// Hotspot Y offset (tip of arrow)
    pub const HOTSPOT_Y: i32 = 0;

    /// 16x16 arrow cursor as BGRA pixels (pre-multiplied alpha).
    /// White arrow with black outline.
    #[rustfmt::skip]
    pub const PIXELS: [u32; 256] = [
        // Row 0
        0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 1
        0xFF000000, 0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 2
        0xFF000000, 0xFFFFFFFF, 0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 3
        0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 4
        0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 5
        0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 6
        0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 7
        0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 8
        0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 9
        0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000, 0xFF000000,
        0xFF000000, 0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 10
        0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 11
        0xFF000000, 0xFFFFFFFF, 0xFF000000, 0x00000000, 0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 12
        0xFF000000, 0xFF000000, 0x00000000, 0x00000000, 0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFF000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 13
        0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 14
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFF000000, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFF000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        // Row 15
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFF000000, 0xFF000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    ];
}

/// A simple 16x16 crosshair cursor (fallback).
pub struct CrosshairSprite;

impl CrosshairSprite {
    /// Sprite width in pixels
    pub const WIDTH: u32 = 16;
    /// Sprite height in pixels  
    pub const HEIGHT: u32 = 16;
    /// Hotspot X offset (center)
    pub const HOTSPOT_X: i32 = 8;
    /// Hotspot Y offset (center)
    pub const HOTSPOT_Y: i32 = 8;

    /// Draw crosshair directly to buffer (no sprite data needed).
    pub fn draw(fb: &mut [u32], stride: u32, x: i32, y: i32, w: i32, h: i32, color: u32) {
        // Horizontal line
        for dx in -8..=8 {
            let px = x + dx;
            if px >= 0 && px < w && y >= 0 && y < h {
                fb[(y as u32 * stride + px as u32) as usize] = color;
            }
        }
        // Vertical line
        for dy in -8..=8 {
            let py = y + dy;
            if x >= 0 && x < w && py >= 0 && py < h {
                fb[(py as u32 * stride + x as u32) as usize] = color;
            }
        }
    }
}
