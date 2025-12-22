#![allow(dead_code)]

pub const CLEAR_COLOR: u32 = 0xFF4C89AA; // Dominant sky blue from clouds.bmp
pub const FRAME_BG: u32 = 0xFF15364A; // Deep navy interior
pub const FRAME_BORDER: u32 = 0xFF0E2433; // Dark outline for contrast
pub const TITLE_COLOR_ACTIVE: u32 = 0xFF76B4D1; // Light sky tint
pub const TITLE_COLOR_INACTIVE: u32 = 0xFF2F5E78; // Muted blue
pub const TITLE_TEXT_COLOR: u32 = 0xFFFFFFFF; // High contrast white
pub const CONTENT_BG: u32 = 0xFF0B1D27; // Shadowed panel interior
pub const TEXT_COLOR: u32 = 0xFFFFFFFF; // High contrast white
pub const CURSOR_COLOR: u32 = 0xFFF4FAFF; // Soft white highlight
pub const CURSOR_SHADOW: u32 = 0xFF000000;

pub const TITLE_BAR_HEIGHT: i32 = 26;
pub const FRAME_THICKNESS: i32 = 2;
pub const MIN_WINDOW_WIDTH: i32 = 80;
pub const MIN_WINDOW_HEIGHT: i32 = 60;
pub const LINE_HEIGHT: i32 = 10;
pub const GLYPH_WIDTH: i32 = 4;
pub const GLYPH_HEIGHT: i32 = 6;
pub const CHAR_ADVANCE: i32 = 6;
pub const FRAME_INTERVAL_NS: u64 = 16_000_000;

pub const MOUSE_SCALE_NUM: i32 = 2;
pub const MOUSE_SCALE_DEN: i32 = 3;
