//! Unified pan/zoom viewport system ("Loupes").
//!
//! Provides a single, reusable camera transform for canvas-like UIs.
//! Use this for graph viewers, scroll views, maps, SVG editors, etc.
//!
//! # Example
//!
//! ```
//! use stem::petals::viewport::{Viewport, PanZoomController, ViewportConstraints};
//!
//! let mut controller = PanZoomController::new(
//!     Viewport::new(800.0, 600.0),
//!     ViewportConstraints::default(),
//! );
//!
//! // Handle mouse wheel zoom
//! controller.zoom_about(400.0, 300.0, 1.1); // Zoom in 10% around center
//!
//! // Handle drag pan
//! controller.pan_by_screen(10.0, 5.0); // Pan right and down
//!
//! // Convert screen coordinates to world coordinates for hit testing
//! let click_x = 100.0;
//! let click_y = 100.0;
//! let world_pos = controller.viewport.screen_to_world(click_x, click_y);
//! ```

pub mod cache;
mod controller;
pub mod inertia;
mod intent;
pub mod snap;

pub use cache::{QuantizationSettings, RasterHandle, ViewportRenderBridge};
pub use controller::{PanZoomController, ViewportConstraints};
pub use inertia::InertiaState;
pub use intent::ViewportIntent;
pub use snap::{snap_pan, snap_zoom, PanSnapPolicy, SnapSettings, ZoomSnapPolicy};

/// Camera transform representing zoom level and pan position.
///
/// The viewport uses a world-space center point and uniform zoom factor.
/// Screen coordinates are pixels; world coordinates are the original
/// content coordinate system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    /// Uniform zoom factor (1.0 = 100%, 2.0 = 200%, etc.)
    pub zoom: f32,
    /// World-space center point (what's at the screen center)
    pub center_world: (f32, f32),
    /// Screen/widget size in pixels
    pub screen_size: (f32, f32),
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            center_world: (0.0, 0.0),
            screen_size: (800.0, 600.0),
        }
    }
}

impl Viewport {
    /// Create a new viewport with given screen dimensions.
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            zoom: 1.0,
            center_world: (0.0, 0.0),
            screen_size: (screen_width, screen_height),
        }
    }

    /// Convert screen coordinates to world coordinates.
    ///
    /// Use this for hit testing: convert mouse position to world space
    /// before checking against content bounds.
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let half_w = self.screen_size.0 / 2.0;
        let half_h = self.screen_size.1 / 2.0;

        // Screen origin is top-left, center is at (half_w, half_h)
        let rel_x = (screen_x - half_w) / self.zoom;
        let rel_y = (screen_y - half_h) / self.zoom;

        (self.center_world.0 + rel_x, self.center_world.1 + rel_y)
    }

    /// Convert world coordinates to screen coordinates.
    ///
    /// Use this for rendering: convert content positions to screen space
    /// for drawing.
    pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> (f32, f32) {
        let half_w = self.screen_size.0 / 2.0;
        let half_h = self.screen_size.1 / 2.0;

        let rel_x = (world_x - self.center_world.0) * self.zoom;
        let rel_y = (world_y - self.center_world.1) * self.zoom;

        (half_w + rel_x, half_h + rel_y)
    }

    /// Zoom about a screen-space anchor point.
    ///
    /// The world point under the anchor stays fixed on screen after zooming.
    /// Use this for zoom-to-cursor behavior.
    pub fn zoom_about(&mut self, screen_anchor_x: f32, screen_anchor_y: f32, factor: f32) {
        // Find the world point under the anchor before zoom
        let (anchor_world_x, anchor_world_y) =
            self.screen_to_world(screen_anchor_x, screen_anchor_y);

        // Apply zoom
        self.zoom *= factor;

        // After zoom, find where the anchor world point ended up
        let half_w = self.screen_size.0 / 2.0;
        let half_h = self.screen_size.1 / 2.0;
        let new_screen_x = half_w + (anchor_world_x - self.center_world.0) * self.zoom;
        let new_screen_y = half_h + (anchor_world_y - self.center_world.1) * self.zoom;

        // Adjust center so the anchor stays at its original screen position
        let drift_x = (new_screen_x - screen_anchor_x) / self.zoom;
        let drift_y = (new_screen_y - screen_anchor_y) / self.zoom;
        self.center_world.0 += drift_x;
        self.center_world.1 += drift_y;
    }

    /// Pan by screen-space delta (pixels).
    ///
    /// Positive dx moves the view right (content moves left).
    pub fn pan_by_screen(&mut self, dx: f32, dy: f32) {
        // Convert screen delta to world delta (divide by zoom)
        self.center_world.0 -= dx / self.zoom;
        self.center_world.1 -= dy / self.zoom;
    }

    /// Pan by world-space delta.
    pub fn pan_by_world(&mut self, dx: f32, dy: f32) {
        self.center_world.0 += dx;
        self.center_world.1 += dy;
    }

    /// Set the viewport to fit a world-space rectangle with padding.
    ///
    /// The rectangle will be centered and scaled to fit within the screen.
    pub fn fit_rect(&mut self, x: f32, y: f32, w: f32, h: f32, padding: f32) {
        if w <= 0.0 || h <= 0.0 {
            return;
        }

        // Center on the rectangle
        self.center_world = (x + w / 2.0, y + h / 2.0);

        // Calculate zoom to fit with padding
        let available_w = self.screen_size.0 - padding * 2.0;
        let available_h = self.screen_size.1 - padding * 2.0;

        if available_w > 0.0 && available_h > 0.0 {
            let zoom_x = available_w / w;
            let zoom_y = available_h / h;
            self.zoom = zoom_x.min(zoom_y);
        }
    }

    /// Update screen size (call when window resizes).
    pub fn set_screen_size(&mut self, width: f32, height: f32) {
        self.screen_size = (width, height);
    }

    /// Get the world-space visible bounds.
    pub fn visible_bounds(&self) -> (f32, f32, f32, f32) {
        let half_w = self.screen_size.0 / (2.0 * self.zoom);
        let half_h = self.screen_size.1 / (2.0 * self.zoom);
        (
            self.center_world.0 - half_w,
            self.center_world.1 - half_h,
            half_w * 2.0,
            half_h * 2.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_to_world_roundtrip() {
        let vp = Viewport::new(800.0, 600.0);
        let (wx, wy) = vp.screen_to_world(100.0, 200.0);
        let (sx, sy) = vp.world_to_screen(wx, wy);
        assert!((sx - 100.0).abs() < 0.001);
        assert!((sy - 200.0).abs() < 0.001);
    }

    #[test]
    fn zoom_about_keeps_anchor_stable() {
        let mut vp = Viewport::new(800.0, 600.0);
        let anchor_x = 200.0;
        let anchor_y = 150.0;

        // Get world point under anchor before zoom
        let (world_x, world_y) = vp.screen_to_world(anchor_x, anchor_y);

        // Zoom in 2x
        vp.zoom_about(anchor_x, anchor_y, 2.0);

        // The same world point should still be at the anchor position
        let (new_sx, new_sy) = vp.world_to_screen(world_x, world_y);
        assert!((new_sx - anchor_x).abs() < 0.001);
        assert!((new_sy - anchor_y).abs() < 0.001);
    }

    #[test]
    fn pan_by_screen_moves_view() {
        let mut vp = Viewport::new(800.0, 600.0);
        let original_center = vp.center_world;

        // Pan right by 100 screen pixels
        vp.pan_by_screen(100.0, 0.0);

        // Center should have moved left in world space
        assert!(vp.center_world.0 < original_center.0);
    }

    #[test]
    fn fit_rect_centers_and_scales() {
        let mut vp = Viewport::new(800.0, 600.0);
        vp.fit_rect(100.0, 100.0, 200.0, 100.0, 50.0);

        // Should be centered on rect
        assert!((vp.center_world.0 - 200.0).abs() < 0.001);
        assert!((vp.center_world.1 - 150.0).abs() < 0.001);
    }
}
