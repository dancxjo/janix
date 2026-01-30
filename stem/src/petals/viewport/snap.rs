//! Viewport snapping utilities.
//!
//! Provides optional grid and zoom snapping for viewports.

use libm::{log2f, powf, roundf};

/// Zoom snap policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZoomSnapPolicy {
    /// No snapping.
    None,
    /// Snap to powers of two (1.0, 2.0, 4.0, etc.).
    PowersOfTwo,
    /// Snap to nice ratios (1.0, 1.5, 2.0, 3.0, 4.0, etc.).
    NiceRatios,
}

/// Pan snap policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PanSnapPolicy {
    /// No snapping.
    None,
    /// Snap to grid in world space.
    Grid { cell_size: f32 },
}

/// Snap settings for viewport.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnapSettings {
    /// Zoom snap policy.
    pub zoom: ZoomSnapPolicy,
    /// Pan snap policy.
    pub pan: PanSnapPolicy,
    /// Snap threshold: how close to snap point before snapping (0.0-1.0).
    /// 0.1 means snap when within 10% of snap point.
    pub threshold: f32,
}

impl Default for SnapSettings {
    fn default() -> Self {
        Self {
            zoom: ZoomSnapPolicy::None,
            pan: PanSnapPolicy::None,
            threshold: 0.05, // 5% threshold
        }
    }
}

/// Snap a zoom value according to policy.
pub fn snap_zoom(zoom: f32, policy: ZoomSnapPolicy, threshold: f32) -> f32 {
    match policy {
        ZoomSnapPolicy::None => zoom,
        ZoomSnapPolicy::PowersOfTwo => {
            let log2 = log2f(zoom);
            let nearest = roundf(log2);
            let distance = (log2 - nearest).abs();

            if distance <= threshold {
                powf(2.0, nearest)
            } else {
                zoom
            }
        }
        ZoomSnapPolicy::NiceRatios => {
            // Nice ratios: 1, 1.5, 2, 3, 4, 6, 8, 12, 16, etc.
            const NICE_RATIOS: &[f32] = &[
                0.25, 0.33, 0.5, 0.67, 0.75, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0,
                10.0, 12.0, 16.0, 20.0,
            ];

            let mut best = zoom;
            let mut best_distance = f32::INFINITY;

            for &ratio in NICE_RATIOS {
                // Use log2 for consistent distance metric
                let distance = log2f(zoom / ratio).abs();
                if distance < best_distance {
                    best = ratio;
                    best_distance = distance;
                }
            }

            // Only snap if within threshold (in log2 space)
            let log_distance = log2f(zoom / best).abs();
            if log_distance <= threshold {
                best
            } else {
                zoom
            }
        }
    }
}

/// Snap a pan position according to policy.
pub fn snap_pan(pos: (f32, f32), policy: PanSnapPolicy, threshold: f32) -> (f32, f32) {
    match policy {
        PanSnapPolicy::None => pos,
        PanSnapPolicy::Grid { cell_size } => {
            let snap_coord = |v: f32| -> f32 {
                let nearest = roundf(v / cell_size) * cell_size;
                let distance = (v - nearest).abs();

                if distance <= cell_size * threshold {
                    nearest
                } else {
                    v
                }
            };

            (snap_coord(pos.0), snap_coord(pos.1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_snap_powers_of_two() {
        let policy = ZoomSnapPolicy::PowersOfTwo;

        // Close to 2.0 - should snap
        assert!((snap_zoom(1.95, policy, 0.1) - 2.0).abs() < 0.001);
        assert!((snap_zoom(2.05, policy, 0.1) - 2.0).abs() < 0.001);

        // Far from 2.0 - should not snap
        assert!((snap_zoom(1.5, policy, 0.1) - 1.5).abs() < 0.001);
    }

    #[test]
    fn zoom_snap_nice_ratios() {
        let policy = ZoomSnapPolicy::NiceRatios;

        // Close to 1.5 - should snap
        let result = snap_zoom(1.48, policy, 0.1);
        assert!((result - 1.5).abs() < 0.001);

        // Close to 2.0 - should snap
        let result = snap_zoom(1.98, policy, 0.1);
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn pan_snap_grid() {
        let policy = PanSnapPolicy::Grid { cell_size: 10.0 };

        // Close to (20, 30) - should snap
        let result = snap_pan((19.5, 30.2), policy, 0.1);
        assert!((result.0 - 20.0).abs() < 0.001);
        assert!((result.1 - 30.0).abs() < 0.001);

        // Far from grid - should not snap
        let result = snap_pan((15.0, 25.0), policy, 0.1);
        assert!((result.0 - 15.0).abs() < 0.001);
        assert!((result.1 - 25.0).abs() < 0.001);
    }

    #[test]
    fn no_snap_returns_original() {
        assert_eq!(snap_zoom(1.234, ZoomSnapPolicy::None, 0.1), 1.234);
        assert_eq!(snap_pan((1.2, 3.4), PanSnapPolicy::None, 0.1), (1.2, 3.4));
    }
}
