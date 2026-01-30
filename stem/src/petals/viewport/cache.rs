//! Viewport render bridge for smooth raster zoom.
//!
//! Implements two-phase zoom: show scaled raster cache immediately while
//! vector scene re-renders asynchronously.

use super::Viewport;
use alloc::string::String;
use core::fmt;

/// Handle to a cached raster image.
///
/// This is an opaque identifier. Apps integrate this with their
/// actual image storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterHandle(pub u64);

impl RasterHandle {
    /// Create a null/invalid handle.
    pub const fn null() -> Self {
        Self(0)
    }

    /// Returns true if this is a valid handle.
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

/// Quantization settings for reraster debouncing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantizationSettings {
    /// Minimum zoom change (log2) to trigger reraster.
    /// e.g., 0.1 means only reraster when zoom changes by 2^0.1 ≈ 7%
    pub zoom_threshold_log2: f32,
    /// Minimum pan distance (in world units) to trigger reraster.
    pub pan_threshold_world: f32,
    /// Debounce time: milliseconds to wait after input settles before rerastering.
    pub debounce_ms: u64,
}

impl Default for QuantizationSettings {
    fn default() -> Self {
        Self {
            zoom_threshold_log2: 0.1,    // ~7% zoom change
            pan_threshold_world: 10.0,   // 10 world units
            debounce_ms: 150,            // 150ms after input settles
        }
    }
}

/// Viewport render bridge state.
///
/// Manages the two-phase zoom strategy:
/// 1. Immediately show scaled raster cache when viewport changes
/// 2. Request vector reraster when viewport has changed significantly
/// 3. Replace cache when new raster is ready
pub struct ViewportRenderBridge {
    /// Last rasterized viewport state.
    pub last_rasterized: Viewport,
    /// Current target viewport state (may differ from last_rasterized).
    pub target_viewport: Viewport,
    /// Handle to the last rasterized image.
    pub last_raster: RasterHandle,
    /// True if a vector render is pending.
    pub vector_render_pending: bool,
    /// Quantization settings.
    pub quantization: QuantizationSettings,
    /// Last input timestamp (nanoseconds) for debouncing.
    last_input_ns: u64,
    /// Scene identifier (for tracking which scene to reraster).
    pub scene_id: String,
}

impl fmt::Debug for ViewportRenderBridge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ViewportRenderBridge")
            .field("last_rasterized", &self.last_rasterized)
            .field("target_viewport", &self.target_viewport)
            .field("last_raster", &self.last_raster)
            .field("vector_render_pending", &self.vector_render_pending)
            .field("scene_id", &self.scene_id)
            .finish()
    }
}

impl ViewportRenderBridge {
    /// Create a new render bridge.
    pub fn new(scene_id: String, initial_viewport: Viewport) -> Self {
        Self {
            last_rasterized: initial_viewport,
            target_viewport: initial_viewport,
            last_raster: RasterHandle::null(),
            vector_render_pending: false,
            quantization: QuantizationSettings::default(),
            last_input_ns: 0,
            scene_id,
        }
    }

    /// Update the target viewport (called when user pans/zooms).
    ///
    /// Returns true if a reraster should be requested.
    pub fn update_target(&mut self, new_viewport: Viewport, current_ns: u64) -> bool {
        self.target_viewport = new_viewport;
        self.last_input_ns = current_ns;

        // Check if we should request a reraster
        self.should_reraster()
    }

    /// Check if a reraster should be requested based on quantization.
    pub fn should_reraster(&self) -> bool {
        if self.vector_render_pending {
            return false; // Already have a render in flight
        }

        // Check zoom change (log scale)
        let zoom_ratio = self.target_viewport.zoom / self.last_rasterized.zoom;
        let zoom_change_log2 = zoom_ratio.log2().abs();

        if zoom_change_log2 >= self.quantization.zoom_threshold_log2 {
            return true;
        }

        // Check pan distance (world space)
        let dx = self.target_viewport.center_world.0 - self.last_rasterized.center_world.0;
        let dy = self.target_viewport.center_world.1 - self.last_rasterized.center_world.1;
        let pan_distance = (dx * dx + dy * dy).sqrt();

        if pan_distance >= self.quantization.pan_threshold_world {
            return true;
        }

        false
    }

    /// Check if enough time has passed since last input for debounced reraster.
    pub fn should_debounced_reraster(&self, current_ns: u64) -> bool {
        if self.vector_render_pending {
            return false;
        }

        let elapsed_ms = (current_ns.saturating_sub(self.last_input_ns)) / 1_000_000;
        if elapsed_ms < self.quantization.debounce_ms {
            return false;
        }

        self.should_reraster()
    }

    /// Mark a vector render as started.
    pub fn start_render(&mut self) {
        self.vector_render_pending = true;
    }

    /// Complete a render with the new raster.
    pub fn complete_render(&mut self, new_raster: RasterHandle, viewport: Viewport) {
        self.last_raster = new_raster;
        self.last_rasterized = viewport;
        self.vector_render_pending = false;
    }

    /// Get the transform to apply to the cached raster for immediate display.
    ///
    /// Returns (scale_x, scale_y, translate_x, translate_y) or None if no cache.
    pub fn get_cache_transform(&self) -> Option<(f32, f32, f32, f32)> {
        if !self.last_raster.is_valid() {
            return None;
        }

        // Compute transform from last_rasterized to target_viewport
        let zoom_ratio = self.target_viewport.zoom / self.last_rasterized.zoom;

        // Center offset in world space
        let world_dx = self.target_viewport.center_world.0 - self.last_rasterized.center_world.0;
        let world_dy = self.target_viewport.center_world.1 - self.last_rasterized.center_world.1;

        // Convert to screen space in the target viewport
        let screen_dx = -world_dx * self.target_viewport.zoom;
        let screen_dy = -world_dy * self.target_viewport.zoom;

        Some((zoom_ratio, zoom_ratio, screen_dx, screen_dy))
    }

    /// Returns true if using cached raster (not at final quality).
    pub fn is_using_cache(&self) -> bool {
        self.last_raster.is_valid()
            && (self.target_viewport.zoom != self.last_rasterized.zoom
                || self.target_viewport.center_world != self.last_rasterized.center_world)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_detects_zoom_change() {
        let viewport = Viewport::new(800.0, 600.0);
        let mut bridge = ViewportRenderBridge::new("test".into(), viewport);
        bridge.last_raster = RasterHandle(1);

        // Small zoom change - should not trigger
        let mut new_viewport = viewport;
        new_viewport.zoom = 1.05;
        assert!(!bridge.update_target(new_viewport, 0));

        // Large zoom change - should trigger
        new_viewport.zoom = 1.5;
        assert!(bridge.update_target(new_viewport, 0));
    }

    #[test]
    fn bridge_detects_pan_change() {
        let viewport = Viewport::new(800.0, 600.0);
        let mut bridge = ViewportRenderBridge::new("test".into(), viewport);
        bridge.last_raster = RasterHandle(1);

        // Small pan - should not trigger
        let mut new_viewport = viewport;
        new_viewport.center_world = (5.0, 5.0);
        assert!(!bridge.update_target(new_viewport, 0));

        // Large pan - should trigger
        new_viewport.center_world = (50.0, 50.0);
        assert!(bridge.update_target(new_viewport, 0));
    }

    #[test]
    fn bridge_respects_pending_render() {
        let viewport = Viewport::new(800.0, 600.0);
        let mut bridge = ViewportRenderBridge::new("test".into(), viewport);
        bridge.last_raster = RasterHandle(1);

        let mut new_viewport = viewport;
        new_viewport.zoom = 2.0;

        bridge.update_target(new_viewport, 0);
        bridge.start_render();

        // Should not request another render while one is pending
        new_viewport.zoom = 3.0;
        assert!(!bridge.update_target(new_viewport, 1000));
    }

    #[test]
    fn cache_transform_computes_correctly() {
        let mut viewport = Viewport::new(800.0, 600.0);
        let mut bridge = ViewportRenderBridge::new("test".into(), viewport);
        bridge.last_raster = RasterHandle(1);
        bridge.last_rasterized = viewport;

        // Zoom in 2x
        viewport.zoom = 2.0;
        bridge.target_viewport = viewport;

        let transform = bridge.get_cache_transform().unwrap();
        assert!((transform.0 - 2.0).abs() < 0.001); // scale_x = 2.0
        assert!((transform.1 - 2.0).abs() < 0.001); // scale_y = 2.0
    }

    #[test]
    fn debounce_waits_for_settle() {
        let viewport = Viewport::new(800.0, 600.0);
        let mut bridge = ViewportRenderBridge::new("test".into(), viewport);
        bridge.last_raster = RasterHandle(1);

        let mut new_viewport = viewport;
        new_viewport.zoom = 2.0;
        bridge.update_target(new_viewport, 0);

        // Too soon - should not reraster
        assert!(!bridge.should_debounced_reraster(100_000_000)); // 100ms

        // After debounce time - should reraster
        assert!(bridge.should_debounced_reraster(200_000_000)); // 200ms
    }
}
