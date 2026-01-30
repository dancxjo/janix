//! Viewport inertia (kinetic scrolling).
//!
//! Provides smooth deceleration for pan and zoom gestures.

use libm::{exp2f, powf};

/// Kinetic state for smooth pan/zoom deceleration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InertiaState {
    /// Pan velocity in screen pixels per second.
    pub pan_velocity: (f32, f32),
    /// Zoom velocity (log scale) per second.
    /// Positive = zooming in, negative = zooming out.
    pub zoom_velocity: f32,
    /// Pan friction coefficient (0.0-1.0, higher = more friction).
    /// Typical: 0.9 to 0.95 for natural feel.
    pub pan_friction: f32,
    /// Zoom friction coefficient (0.0-1.0).
    pub zoom_friction: f32,
    /// Velocity threshold below which motion stops (screen px/s).
    pub pan_stop_threshold: f32,
    /// Zoom velocity stop threshold (log scale/s).
    pub zoom_stop_threshold: f32,
}

impl Default for InertiaState {
    fn default() -> Self {
        Self {
            pan_velocity: (0.0, 0.0),
            zoom_velocity: 0.0,
            pan_friction: 0.92,      // Velocity reduced to 8% per second
            zoom_friction: 0.88,     // Velocity reduced to 12% per second
            pan_stop_threshold: 1.0, // Stop below 1 px/s
            zoom_stop_threshold: 0.01,
        }
    }
}

impl InertiaState {
    /// Create a new inertia state with custom friction.
    pub fn new(pan_friction: f32, zoom_friction: f32) -> Self {
        Self {
            pan_friction,
            zoom_friction,
            ..Default::default()
        }
    }

    /// Apply inertia for one frame, returning (pan_dx, pan_dy, zoom_factor).
    ///
    /// `dt_seconds` is the time step (e.g., 1/60 for 60 FPS).
    /// Returns (pan_delta_x, pan_delta_y, zoom_factor).
    /// Returns true if any motion occurred.
    pub fn tick(&mut self, dt_seconds: f32) -> (f32, f32, f32, bool) {
        let mut updated = false;

        // Apply pan velocity
        let pan_dx = self.pan_velocity.0 * dt_seconds;
        let pan_dy = self.pan_velocity.1 * dt_seconds;

        if pan_dx.abs() > 0.01 || pan_dy.abs() > 0.01 {
            updated = true;
        }

        // Apply zoom velocity
        let zoom_factor = if self.zoom_velocity.abs() > self.zoom_stop_threshold {
            let factor = exp2f(self.zoom_velocity * dt_seconds);
            updated = true;
            factor
        } else {
            1.0
        };

        // Apply friction
        let friction_factor = powf(1.0 - self.pan_friction, dt_seconds);
        self.pan_velocity.0 *= friction_factor;
        self.pan_velocity.1 *= friction_factor;
        self.zoom_velocity *= powf(1.0 - self.zoom_friction, dt_seconds);

        // Stop if below threshold
        if self.pan_velocity.0.abs() < self.pan_stop_threshold {
            self.pan_velocity.0 = 0.0;
        }
        if self.pan_velocity.1.abs() < self.pan_stop_threshold {
            self.pan_velocity.1 = 0.0;
        }
        if self.zoom_velocity.abs() < self.zoom_stop_threshold {
            self.zoom_velocity = 0.0;
        }

        (pan_dx, pan_dy, zoom_factor, updated)
    }

    /// Apply an impulse to pan velocity (e.g., from a drag release).
    ///
    /// `velocity_px_per_sec` is the velocity in screen pixels per second.
    pub fn apply_pan_impulse(&mut self, vx: f32, vy: f32) {
        self.pan_velocity.0 += vx;
        self.pan_velocity.1 += vy;
    }

    /// Apply an impulse to zoom velocity.
    ///
    /// `velocity_log_scale_per_sec` is the zoom velocity (log2 scale).
    pub fn apply_zoom_impulse(&mut self, velocity: f32) {
        self.zoom_velocity += velocity;
    }

    /// Stop all inertia immediately.
    pub fn stop(&mut self) {
        self.pan_velocity = (0.0, 0.0);
        self.zoom_velocity = 0.0;
    }

    /// Returns true if any inertia is active.
    pub fn is_active(&self) -> bool {
        self.pan_velocity.0.abs() >= self.pan_stop_threshold
            || self.pan_velocity.1.abs() >= self.pan_stop_threshold
            || self.zoom_velocity.abs() >= self.zoom_stop_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inertia_decelerates() {
        let mut inertia = InertiaState::default();
        inertia.apply_pan_impulse(100.0, 0.0);

        let mut total_distance = 0.0;
        // Run for 2 seconds (120 frames at 60fps)
        for _ in 0..120 {
            let (dx, _, _, _) = inertia.tick(1.0 / 60.0);
            total_distance += dx;
        }

        // Should have moved a significant distance but velocity reduced significantly
        assert!(total_distance > 10.0);
        // After 2 seconds, velocity should be well below starting velocity
        assert!(inertia.pan_velocity.0.abs() < 10.0);
    }

    #[test]
    fn inertia_stops_below_threshold() {
        let mut inertia = InertiaState::default();
        inertia.apply_pan_impulse(0.5, 0.0); // Below threshold

        let (dx, _, _, updated) = inertia.tick(1.0 / 60.0);
        assert!(dx.abs() < 0.1);
        assert!(!updated || !inertia.is_active());
    }

    #[test]
    fn zoom_inertia_works() {
        let mut inertia = InertiaState::default();
        inertia.apply_zoom_impulse(1.0); // Zoom in at 2x per second

        let (_, _, factor, updated) = inertia.tick(0.1);
        assert!(updated);
        assert!(factor > 1.0);
    }

    #[test]
    fn stop_kills_all_velocity() {
        let mut inertia = InertiaState::default();
        inertia.apply_pan_impulse(100.0, 100.0);
        inertia.apply_zoom_impulse(2.0);

        inertia.stop();
        assert!(!inertia.is_active());
        assert_eq!(inertia.pan_velocity, (0.0, 0.0));
        assert_eq!(inertia.zoom_velocity, 0.0);
    }
}
