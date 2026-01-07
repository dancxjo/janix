#![no_std]
//! Without A Cause theme - Inspired by Windows 3.1's Black Leather Jacket

// ─── Dominant Color (Dark leather gray) ───────────────────────────
pub const DOMINANT_R: u8 = 0x40;
pub const DOMINANT_G: u8 = 0x40;
pub const DOMINANT_B: u8 = 0x40;
pub const DOMINANT_COLOR_ARGB: u32 = 0xFF404040;

// ─── Wallpaper ────────────────────────────────────────────────────
pub const WALLPAPER_BYTESPACE: &str = "bytespace.asset.leather.bmp";

// ─── Window Chrome (Teal/Cyan accents like Win3.1 BLJ) ────────────
pub const TITLEBAR_ACTIVE_TOP: u32 = 0xFF00A8A8;     // Cyan highlight
pub const TITLEBAR_ACTIVE_BOTTOM: u32 = 0xFF008080;  // Teal
pub const TITLEBAR_INACTIVE: u32 = 0xFF606060;       // Gray

// ─── Shadows ──────────────────────────────────────────────────────
pub const SHADOW_BLUR_RADIUS: u8 = 8;
pub const SHADOW_COLOR: u32 = 0x60000000;

// ─── Boot Fade Colors ─────────────────────────────────────────────
/// Half-brightness for boot fade start (where kernel left off)
pub const DOMINANT_HALF_ARGB: u32 = 0xFF202020;  // #404040 / 2
