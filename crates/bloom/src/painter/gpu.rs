//! GPU backend stub for future implementation.
//!
//! This module defines the GPU painter interface that will eventually
//! map drawing operations to shader/geometry calls.

/// GPU painter stub.
///
/// # Future Implementation Notes
///
/// Each method will map to GPU operations:
/// - `fill_rect` -> solid color fragment shader
/// - `fill_rounded_rect` -> SDF shader with radius
/// - `fill_panel` -> gradient shader with bevel
/// - `stroke_rounded_rect` -> SDF outline shader  
/// - `blit_rgba` -> textured quad
/// - `blit_rgba_alpha` -> textured quad with alpha blend
/// - `draw_shadow_mask` -> blur shader or precomputed shadow texture
/// - `draw_cursor_frame` -> textured quad with hotspot offset
/// - Clip -> GPU scissor rect
/// - Damage -> swapchain present region
pub struct GpuPainter;

impl GpuPainter {
    /// Create a new GPU painter (unimplemented).
    pub fn new() -> Self {
        unimplemented!("GPU painter not yet implemented")
    }
}
