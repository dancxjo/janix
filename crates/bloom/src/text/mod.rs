//! Text rendering modules for Bloom compositor.
//!
//! The text system has two tiers:
//! - Tier 0: Built-in bitmap font (`builtin_font`) - always available, no allocations, instant
//! - Tier 1: Full unifont/textd (`unifont`) - lazy loaded on demand
//!
//! At boot, text renders immediately using the builtin font. When unifont or textd
//! glyphs become available, they are used instead.

pub mod builtin_font;
mod unifont;

// Re-export the public API from unifont for backwards compatibility
pub use unifont::{
    draw_text,
    draw_text_on_painter,
    ensure_font_loaded,
    measure_text_width,
    register_textd_fonts_watch,
    check_textd_fonts_available,
    get_textd_fonts_watch_id,
};
