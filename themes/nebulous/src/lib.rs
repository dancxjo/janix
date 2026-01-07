#![no_std]
//! Nebulous theme

// ─── Dominant Color (#2e80d2) ─────────────────────────────────────
pub const DOMINANT_R: u8 = 0x2E;
pub const DOMINANT_G: u8 = 0x80;
pub const DOMINANT_B: u8 = 0xD2;
pub const DOMINANT_COLOR_ARGB: u32 = 0xFF2E80D2;

// ─── Wallpaper ────────────────────────────────────────────────────
pub const WALLPAPER_BYTESPACE: &str = "bytespace.asset.clouds.bmp";

// ─── Window Chrome ────────────────────────────────────────────────
pub const TITLEBAR_ACTIVE_TOP: u32 = 0xFF4A90C8;
pub const TITLEBAR_ACTIVE_BOTTOM: u32 = 0xFF2E80D2;
pub const TITLEBAR_INACTIVE: u32 = 0xFF606060;

// ─── Shadows ──────────────────────────────────────────────────────
pub const SHADOW_BLUR_RADIUS: u8 = 8;
pub const SHADOW_COLOR: u32 = 0x40000000;
