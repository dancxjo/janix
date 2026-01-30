//! Pan/zoom controller state machine.

use super::{inertia::InertiaState, Viewport, ViewportIntent};

/// Constraints for viewport zoom and panning.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportConstraints {
    /// Minimum zoom level (e.g., 0.1 = 10%)
    pub min_zoom: f32,
    /// Maximum zoom level (e.g., 10.0 = 1000%)
    pub max_zoom: f32,
    /// Optional world-space bounds to clamp panning.
    /// Format: (x, y, width, height). None means infinite canvas.
    pub bounds: Option<(f32, f32, f32, f32)>,
}

impl Default for ViewportConstraints {
    fn default() -> Self {
        Self {
            min_zoom: 0.1,
            max_zoom: 10.0,
            bounds: None,
        }
    }
}

/// Pan/zoom controller with drag state and constraints.
///
/// This is the main integration point for apps. It holds the viewport state,
/// handles input events, and applies constraints.
#[derive(Clone, Debug)]
pub struct PanZoomController {
    /// Current viewport state.
    pub viewport: Viewport,
    /// Zoom and pan constraints.
    pub constraints: ViewportConstraints,
    /// Drag state: Some((last_x, last_y)) if dragging.
    drag_anchor: Option<(f32, f32)>,
    /// Inertia state for smooth scrolling.
    pub inertia: InertiaState,
}

impl PanZoomController {
    /// Create a new controller with given viewport and constraints.
    pub fn new(viewport: Viewport, constraints: ViewportConstraints) -> Self {
        Self {
            viewport,
            constraints,
            drag_anchor: None,
            inertia: InertiaState::default(),
        }
    }

    /// Apply a viewport intent.
    pub fn apply(&mut self, intent: ViewportIntent) {
        match intent {
            ViewportIntent::PanByScreen { dx, dy } => {
                self.viewport.pan_by_screen(dx, dy);
                self.clamp_to_bounds();
            }
            ViewportIntent::ZoomAbout {
                screen_x,
                screen_y,
                factor,
            } => {
                self.viewport.zoom_about(screen_x, screen_y, factor);
                self.clamp_zoom();
                self.clamp_to_bounds();
            }
            ViewportIntent::BeginGesture => {}
            ViewportIntent::EndGesture => {}
        }
    }

    /// Zoom about a point, respecting constraints.
    pub fn zoom_about(&mut self, screen_x: f32, screen_y: f32, factor: f32) {
        self.apply(ViewportIntent::ZoomAbout {
            screen_x,
            screen_y,
            factor,
        });
    }

    /// Pan by screen pixels, respecting constraints.
    pub fn pan_by_screen(&mut self, dx: f32, dy: f32) {
        self.apply(ViewportIntent::PanByScreen { dx, dy });
    }

    /// Begin a drag gesture at the given screen position.
    pub fn begin_drag(&mut self, screen_x: f32, screen_y: f32) {
        self.drag_anchor = Some((screen_x, screen_y));
    }

    /// Update drag with new position, returning true if dragging.
    pub fn update_drag(&mut self, screen_x: f32, screen_y: f32) -> bool {
        if let Some((last_x, last_y)) = self.drag_anchor {
            let dx = screen_x - last_x;
            let dy = screen_y - last_y;
            self.pan_by_screen(dx, dy);
            self.drag_anchor = Some((screen_x, screen_y));
            true
        } else {
            false
        }
    }

    /// End the current drag gesture.
    pub fn end_drag(&mut self) {
        self.drag_anchor = None;
    }

    /// Returns true if currently dragging.
    pub fn is_dragging(&self) -> bool {
        self.drag_anchor.is_some()
    }

    /// Handle mouse wheel input for zooming.
    ///
    /// `delta` is the wheel delta (positive = zoom in, negative = zoom out).
    /// Common values: +1/-1 for notched wheels, fractional for smooth scroll.
    pub fn handle_wheel(&mut self, screen_x: f32, screen_y: f32, delta: f32) {
        // Sensitivity: each notch zooms by ~10%
        let factor = if delta > 0.0 {
            1.0 + 0.1 * delta.min(3.0)
        } else {
            1.0 / (1.0 + 0.1 * (-delta).min(3.0))
        };
        self.zoom_about(screen_x, screen_y, factor);
    }

    /// Tick for inertia and smooth scrolling.
    ///
    /// Call this every frame to apply inertia physics.
    /// Returns true if the viewport was updated (needs redraw).
    pub fn tick(&mut self, dt_seconds: f32) -> bool {
        let (pan_dx, pan_dy, zoom_factor, updated) = self.inertia.tick(dt_seconds);

        if updated {
            // Apply pan
            if pan_dx.abs() > 0.01 || pan_dy.abs() > 0.01 {
                self.viewport.pan_by_screen(pan_dx, pan_dy);
                self.clamp_to_bounds();
            }

            // Apply zoom (about screen center for inertia zoom)
            if (zoom_factor - 1.0).abs() > 0.001 {
                let center_x = self.viewport.screen_size.0 / 2.0;
                let center_y = self.viewport.screen_size.1 / 2.0;
                self.viewport.zoom_about(center_x, center_y, zoom_factor);
                self.clamp_zoom();
                self.clamp_to_bounds();
            }
        }

        updated
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Keyboard handling (WCAG accessibility)
    // ─────────────────────────────────────────────────────────────────────────

    /// Pan step size in screen pixels for arrow key navigation.
    const ARROW_PAN_STEP: f32 = 40.0;
    /// Zoom factor for keyboard zoom (Alt+Plus / Alt+Minus).
    const KEYBOARD_ZOOM_FACTOR: f32 = 1.15;

    /// Handle arrow key for panning.
    ///
    /// Pass true for the direction(s) pressed. Returns true if handled.
    pub fn handle_arrow_key(&mut self, up: bool, down: bool, left: bool, right: bool) -> bool {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if up {
            dy -= Self::ARROW_PAN_STEP;
        }
        if down {
            dy += Self::ARROW_PAN_STEP;
        }
        if left {
            dx -= Self::ARROW_PAN_STEP;
        }
        if right {
            dx += Self::ARROW_PAN_STEP;
        }

        if dx != 0.0 || dy != 0.0 {
            self.pan_by_screen(dx, dy);
            true
        } else {
            false
        }
    }

    /// Handle keyboard zoom (Alt+Plus / Alt+Minus).
    ///
    /// `zoom_in`: true for zoom in, false for zoom out.
    /// Zooms about the screen center.
    pub fn handle_keyboard_zoom(&mut self, zoom_in: bool) {
        let center_x = self.viewport.screen_size.0 / 2.0;
        let center_y = self.viewport.screen_size.1 / 2.0;

        let factor = if zoom_in {
            Self::KEYBOARD_ZOOM_FACTOR
        } else {
            1.0 / Self::KEYBOARD_ZOOM_FACTOR
        };

        self.zoom_about(center_x, center_y, factor);
    }

    /// Handle Home key: reset to default view (zoom 1.0, center at origin).
    pub fn handle_home(&mut self) {
        self.viewport.zoom = 1.0;
        self.viewport.center_world = (0.0, 0.0);
    }

    /// Handle Page Up/Down for large vertical scroll.
    pub fn handle_page(&mut self, page_up: bool) {
        let page_step = self.viewport.screen_size.1 * 0.8; // 80% of screen height
        let dy = if page_up { -page_step } else { page_step };
        self.pan_by_screen(0.0, dy);
    }

    /// Clamp zoom to constraints.
    fn clamp_zoom(&mut self) {
        if self.viewport.zoom < self.constraints.min_zoom {
            self.viewport.zoom = self.constraints.min_zoom;
        }
        if self.viewport.zoom > self.constraints.max_zoom {
            self.viewport.zoom = self.constraints.max_zoom;
        }
    }

    /// Clamp pan to bounds if set.
    fn clamp_to_bounds(&mut self) {
        if let Some((bx, by, bw, bh)) = self.constraints.bounds {
            let (vx, vy, vw, vh) = self.viewport.visible_bounds();

            // Clamp center so visible area stays within bounds
            if vw < bw {
                // Viewport is smaller than bounds, keep it inside
                if vx < bx {
                    self.viewport.center_world.0 += bx - vx;
                } else if vx + vw > bx + bw {
                    self.viewport.center_world.0 -= (vx + vw) - (bx + bw);
                }
            } else {
                // Viewport is larger than bounds, center on bounds
                self.viewport.center_world.0 = bx + bw / 2.0;
            }

            if vh < bh {
                if vy < by {
                    self.viewport.center_world.1 += by - vy;
                } else if vy + vh > by + bh {
                    self.viewport.center_world.1 -= (vy + vh) - (by + bh);
                }
            } else {
                self.viewport.center_world.1 = by + bh / 2.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drag_pans_viewport() {
        let mut ctrl =
            PanZoomController::new(Viewport::new(800.0, 600.0), ViewportConstraints::default());

        let original = ctrl.viewport.center_world;
        ctrl.begin_drag(100.0, 100.0);
        ctrl.update_drag(150.0, 120.0); // Drag right 50px, down 20px
        ctrl.end_drag();

        // View panned right, so center moved left
        assert!(ctrl.viewport.center_world.0 < original.0);
        assert!(ctrl.viewport.center_world.1 < original.1);
    }

    #[test]
    fn wheel_zooms() {
        let mut ctrl =
            PanZoomController::new(Viewport::new(800.0, 600.0), ViewportConstraints::default());

        let original_zoom = ctrl.viewport.zoom;
        ctrl.handle_wheel(400.0, 300.0, 1.0); // Zoom in
        assert!(ctrl.viewport.zoom > original_zoom);
    }

    #[test]
    fn zoom_respects_constraints() {
        let mut ctrl = PanZoomController::new(
            Viewport::new(800.0, 600.0),
            ViewportConstraints {
                min_zoom: 0.5,
                max_zoom: 2.0,
                bounds: None,
            },
        );

        // Try to zoom out past min
        for _ in 0..20 {
            ctrl.handle_wheel(400.0, 300.0, -2.0);
        }
        assert!(ctrl.viewport.zoom >= 0.5);

        // Try to zoom in past max
        for _ in 0..20 {
            ctrl.handle_wheel(400.0, 300.0, 2.0);
        }
        assert!(ctrl.viewport.zoom <= 2.0);
    }
}
