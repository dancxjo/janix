//! High-precision transforms for VIR
//!
//! Transforms use f64 internally to avoid precision loss during
//! composition, then convert to f32 for final application.

use serde::{Deserialize, Serialize};
use super::VirPoint;

/// 2D affine transform with high precision
///
/// Represents the transformation matrix:
/// ```text
/// [ a  c  e ]   [ x ]
/// [ b  d  f ] × [ y ]
/// [ 0  0  1 ]   [ 1 ]
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VirTransform {
    pub a: f64,  // scale_x / cos(rotation)
    pub b: f64,  // sin(rotation)
    pub c: f64,  // -sin(rotation)
    pub d: f64,  // scale_y / cos(rotation)
    pub e: f64,  // translate_x
    pub f: f64,  // translate_y
}

impl VirTransform {
    /// Create an identity transform (no change)
    pub const fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Create a translation transform
    pub fn translate(tx: f64, ty: f64) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: tx,
            f: ty,
        }
    }

    /// Create a scaling transform
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Create a uniform scaling transform
    pub fn scale_uniform(s: f64) -> Self {
        Self::scale(s, s)
    }

    /// Create a rotation transform (angle in radians)
    pub fn rotate(angle_rad: f64) -> Self {
        let cos = libm::cos(angle_rad);
        let sin = libm::sin(angle_rad);
        Self {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Compose two transforms: self * other
    /// The result applies 'other' first, then 'self'
    pub fn compose(&self, other: &Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            e: self.a * other.e + self.c * other.f + self.e,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }

    /// Transform a point
    pub fn transform_point(&self, p: VirPoint) -> VirPoint {
        VirPoint {
            x: (self.a * p.x as f64 + self.c * p.y as f64 + self.e) as f32,
            y: (self.b * p.x as f64 + self.d * p.y as f64 + self.f) as f32,
        }
    }

    /// Transform a point given as coordinates
    pub fn transform_xy(&self, x: f32, y: f32) -> (f32, f32) {
        let p = self.transform_point(VirPoint::new(x, y));
        (p.x, p.y)
    }

    /// Get the scale factor (maximum of x and y scale)
    /// Used for adjusting tolerances
    pub fn max_scale(&self) -> f64 {
        let sx = libm::sqrt(self.a * self.a + self.b * self.b);
        let sy = libm::sqrt(self.c * self.c + self.d * self.d);
        if sx > sy { sx } else { sy }
    }

    /// Check if this is the identity transform
    pub fn is_identity(&self) -> bool {
        const EPSILON: f64 = 1e-9;
        (self.a - 1.0).abs() < EPSILON
            && self.b.abs() < EPSILON
            && self.c.abs() < EPSILON
            && (self.d - 1.0).abs() < EPSILON
            && self.e.abs() < EPSILON
            && self.f.abs() < EPSILON
    }

    /// Convert to f32 representation for final use
    pub fn to_f32_matrix(&self) -> [f32; 6] {
        [
            self.a as f32,
            self.b as f32,
            self.c as f32,
            self.d as f32,
            self.e as f32,
            self.f as f32,
        ]
    }
}

impl Default for VirTransform {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = VirTransform::identity();
        let p = VirPoint::new(10.0, 20.0);
        let tp = t.transform_point(p);
        assert_eq!(tp.x, 10.0);
        assert_eq!(tp.y, 20.0);
        assert!(t.is_identity());
    }

    #[test]
    fn test_translate() {
        let t = VirTransform::translate(5.0, 10.0);
        let p = VirPoint::new(10.0, 20.0);
        let tp = t.transform_point(p);
        assert_eq!(tp.x, 15.0);
        assert_eq!(tp.y, 30.0);
    }

    #[test]
    fn test_scale() {
        let t = VirTransform::scale(2.0, 3.0);
        let p = VirPoint::new(10.0, 20.0);
        let tp = t.transform_point(p);
        assert_eq!(tp.x, 20.0);
        assert_eq!(tp.y, 60.0);
    }

    #[test]
    fn test_compose() {
        // Scale then translate
        let scale = VirTransform::scale(2.0, 2.0);
        let translate = VirTransform::translate(10.0, 10.0);
        let combined = translate.compose(&scale);

        let p = VirPoint::new(5.0, 5.0);
        let tp = combined.transform_point(p);
        // 5 * 2 + 10 = 20
        assert_eq!(tp.x, 20.0);
        assert_eq!(tp.y, 20.0);
    }

    #[test]
    fn test_max_scale() {
        let t = VirTransform::scale(3.0, 2.0);
        let max = t.max_scale();
        assert!((max - 3.0).abs() < 1e-6);
    }
}
