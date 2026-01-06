//! Painter API for GPU-ready drawing abstraction.
//!
//! All pixel writes in Bloom flow through the `Painter` trait,
//! enabling future GPU backend swaps without touching UI/layout logic.

pub mod cpu;
pub mod gpu;

use crate::scene::Rect;
use crate::shadow::{ShadowMask, ShadowParams};
use crate::assets::cursor::CursorFrame;
use abi::ids::ThingId;
use crate::scene_cache::BytespaceMappingCache;

/// Scissor/clip region for drawing operations.
#[derive(Clone, Copy, Debug)]
pub struct Clip {
    pub rect: Rect,
}

impl Clip {
    /// Create a clip covering the entire screen.
    pub fn full(w: u32, h: u32) -> Self {
        Clip { rect: Rect { x: 0, y: 0, w, h } }
    }

    /// Create a clip from a rect.
    pub fn from_rect(rect: Rect) -> Self {
        Clip { rect }
    }
}

/// Backend-agnostic drawing interface.
///
/// All drawing operations respect the current clip region.
/// GPU implementations can map these to shaders/geometry later.
pub trait Painter {
    /// Returns (width, height) of the drawing surface.
    fn screen_size(&self) -> (u32, u32);

    /// Set the scissor/clip region.
    fn set_clip(&mut self, clip: Clip);

    /// Get the current clip region.
    fn clip(&self) -> Clip;

    /// Push a new clip region (intersected with current).
    fn push_clip(&mut self, rect: Rect);

    /// Pop the last clip region.
    fn pop_clip(&mut self);

    /// Mark a region as damaged (needing present).
    fn damage(&mut self, rect: Rect);

    /// Take accumulated damage, returning it and clearing the internal state.
    fn take_damage(&mut self) -> Option<Rect>;

    // ========== Core Primitives ==========

    /// Clear the entire surface with a solid color.
    fn clear(&mut self, color: u32);

    /// Fill a rectangle with a solid color.
    fn fill_rect(&mut self, rect: Rect, color: u32);

    /// Fill a rectangle with a vertical gradient.
    fn fill_rect_vgrad(&mut self, rect: Rect, radius: u16, top_color: u32, bottom_color: u32);

    /// Fill a rounded rectangle with a solid color.
    fn fill_rounded_rect(&mut self, rect: Rect, radius: u16, color: u32);

    /// Fill a rounded rect with gradient and optional title stripe.
    fn fill_panel(&mut self, rect: Rect, radius: u16, base_color: u32, stripe_height: Option<i32>);

    /// Stroke a rounded rectangle border.
    fn stroke_rounded_rect(&mut self, rect: Rect, radius: u16, thickness: u16, color: u32);

    /// Stroke a rounded rectangle border (top corners only).
    fn stroke_rounded_rect_top(&mut self, rect: Rect, radius: u16, thickness: u16, color: u32);

    // ========== Blits ==========

    /// Blit ARGB pixels (opaque copy).
    fn blit_rgba(&mut self, dst_x: i32, dst_y: i32, src: &[u32], src_w: u32, src_h: u32);

    /// Blit ARGB pixels with alpha blending.
    fn blit_rgba_alpha(&mut self, dst_x: i32, dst_y: i32, src: &[u32], src_w: u32, src_h: u32);

    /// Blit asset from bytespace (with ID for recording, slice for immediate).
    fn blit_asset(&mut self, dst_x: i32, dst_y: i32, id: ThingId, src_w: u32, src_h: u32, src_stride: u32, src_len: usize, cache: &mut BytespaceMappingCache);

    /// Blit ARGB pixels with alpha blending using a custom source stride and rectangle.
    fn blit_rgba_alpha_rect(&mut self, dst_x: i32, dst_y: i32, src: &[u32], src_stride: u32, w: u32, h: u32);

    /// Copy a rectangular region from a source buffer.
    fn copy_region(&mut self, src: &[u32], src_w: u32, region: Rect);

    // ========== Shadows ==========

    /// Draw a shadow using a mask shape.
    fn draw_shadow_mask(&mut self, x: i32, y: i32, mask: ShadowMask<'_>, params: ShadowParams);

    // ========== Cursor ==========

    /// Draw a cursor frame with alpha blending.
    fn draw_cursor_frame(&mut self, frame: &CursorFrame, px: i32, py: i32);

    /// Draw a fallback cursor (simple rectangle).
    fn draw_fallback_cursor(&mut self, px: i32, py: i32);

    /// Draw a text string.
    fn draw_text(&mut self, x: i32, y: i32, text: &str, color: u32, size_px: f32);
}

pub use cpu::CpuPainter;
