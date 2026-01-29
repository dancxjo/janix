//! Curve flattening algorithms
//!
//! Converts bezier curves into line segments using adaptive subdivision.

use crate::vir::VirPoint;
use alloc::vec::Vec;

/// Flatten a quadratic bezier curve into line segments
pub fn flatten_quad(
    p0: VirPoint,
    cp: VirPoint,
    p1: VirPoint,
    tolerance: f32,
    output: &mut Vec<VirPoint>,
) {
    // Use recursive subdivision
    flatten_quad_recursive(p0, cp, p1, tolerance, output, 0);
}

fn flatten_quad_recursive(
    p0: VirPoint,
    cp: VirPoint,
    p1: VirPoint,
    tolerance: f32,
    output: &mut Vec<VirPoint>,
    depth: u32,
) {
    const MAX_DEPTH: u32 = 16;
    
    if depth > MAX_DEPTH {
        // Prevent infinite recursion
        output.push(p1);
        return;
    }
    
    // Calculate midpoint of curve
    let t = 0.5;
    let mid = eval_quad(p0, cp, p1, t);
    
    // Calculate midpoint of chord
    let chord_mid = VirPoint::new(
        (p0.x + p1.x) * 0.5,
        (p0.y + p1.y) * 0.5,
    );
    
    // Check if curve is flat enough
    let dx = mid.x - chord_mid.x;
    let dy = mid.y - chord_mid.y;
    let dist_sq = dx * dx + dy * dy;
    
    if dist_sq <= tolerance * tolerance {
        // Flat enough, add endpoint
        output.push(p1);
    } else {
        // Split curve at midpoint
        let cp0 = VirPoint::new(
            (p0.x + cp.x) * 0.5,
            (p0.y + cp.y) * 0.5,
        );
        let cp1 = VirPoint::new(
            (cp.x + p1.x) * 0.5,
            (cp.y + p1.y) * 0.5,
        );
        
        // Recursively flatten both halves
        flatten_quad_recursive(p0, cp0, mid, tolerance, output, depth + 1);
        flatten_quad_recursive(mid, cp1, p1, tolerance, output, depth + 1);
    }
}

/// Evaluate a quadratic bezier at parameter t
fn eval_quad(p0: VirPoint, cp: VirPoint, p1: VirPoint, t: f32) -> VirPoint {
    let t2 = 1.0 - t;
    let b0 = t2 * t2;
    let b1 = 2.0 * t2 * t;
    let b2 = t * t;
    
    VirPoint::new(
        b0 * p0.x + b1 * cp.x + b2 * p1.x,
        b0 * p0.y + b1 * cp.y + b2 * p1.y,
    )
}

/// Flatten a cubic bezier curve into line segments
pub fn flatten_cubic(
    p0: VirPoint,
    cp1: VirPoint,
    cp2: VirPoint,
    p1: VirPoint,
    tolerance: f32,
    output: &mut Vec<VirPoint>,
) {
    flatten_cubic_recursive(p0, cp1, cp2, p1, tolerance, output, 0);
}

fn flatten_cubic_recursive(
    p0: VirPoint,
    cp1: VirPoint,
    cp2: VirPoint,
    p1: VirPoint,
    tolerance: f32,
    output: &mut Vec<VirPoint>,
    depth: u32,
) {
    const MAX_DEPTH: u32 = 16;
    
    if depth > MAX_DEPTH {
        output.push(p1);
        return;
    }
    
    // Calculate midpoint of curve
    let t = 0.5;
    let mid = eval_cubic(p0, cp1, cp2, p1, t);
    
    // Calculate midpoint of chord
    let chord_mid = VirPoint::new(
        (p0.x + p1.x) * 0.5,
        (p0.y + p1.y) * 0.5,
    );
    
    // Check if curve is flat enough
    let dx = mid.x - chord_mid.x;
    let dy = mid.y - chord_mid.y;
    let dist_sq = dx * dx + dy * dy;
    
    if dist_sq <= tolerance * tolerance {
        output.push(p1);
    } else {
        // Use de Casteljau's algorithm to split at t=0.5
        let q0 = lerp_point(p0, cp1, 0.5);
        let q1 = lerp_point(cp1, cp2, 0.5);
        let q2 = lerp_point(cp2, p1, 0.5);
        
        let r0 = lerp_point(q0, q1, 0.5);
        let r1 = lerp_point(q1, q2, 0.5);
        
        // mid is the point at t=0.5, which we already calculated
        // But for symmetry in subdivision:
        let split = lerp_point(r0, r1, 0.5);
        
        flatten_cubic_recursive(p0, q0, r0, split, tolerance, output, depth + 1);
        flatten_cubic_recursive(split, r1, q2, p1, tolerance, output, depth + 1);
    }
}

/// Evaluate a cubic bezier at parameter t
fn eval_cubic(p0: VirPoint, cp1: VirPoint, cp2: VirPoint, p1: VirPoint, t: f32) -> VirPoint {
    let t2 = 1.0 - t;
    let b0 = t2 * t2 * t2;
    let b1 = 3.0 * t2 * t2 * t;
    let b2 = 3.0 * t2 * t * t;
    let b3 = t * t * t;
    
    VirPoint::new(
        b0 * p0.x + b1 * cp1.x + b2 * cp2.x + b3 * p1.x,
        b0 * p0.y + b1 * cp1.y + b2 * cp2.y + b3 * p1.y,
    )
}

/// Linear interpolation between two points
fn lerp_point(p0: VirPoint, p1: VirPoint, t: f32) -> VirPoint {
    VirPoint::new(
        p0.x + (p1.x - p0.x) * t,
        p0.y + (p1.y - p0.y) * t,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_line() {
        // A degenerate "curve" that's actually a line
        let p0 = VirPoint::new(0.0, 0.0);
        let cp = VirPoint::new(5.0, 0.0);
        let p1 = VirPoint::new(10.0, 0.0);
        
        let mut output = Vec::new();
        flatten_quad(p0, cp, p1, 0.25, &mut output);
        
        // Should add just the endpoint since it's flat
        assert!(!output.is_empty());
    }

    #[test]
    fn test_eval_quad() {
        let p0 = VirPoint::new(0.0, 0.0);
        let cp = VirPoint::new(5.0, 10.0);
        let p1 = VirPoint::new(10.0, 0.0);
        
        let mid = eval_quad(p0, cp, p1, 0.5);
        
        // At t=0.5, the curve should pass through (5, 5) for this symmetric case
        assert!((mid.x - 5.0).abs() < 0.1);
        assert!((mid.y - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_eval_cubic() {
        let p0 = VirPoint::new(0.0, 0.0);
        let cp1 = VirPoint::new(0.0, 10.0);
        let cp2 = VirPoint::new(10.0, 10.0);
        let p1 = VirPoint::new(10.0, 0.0);
        
        // At t=0, should be at p0
        let start = eval_cubic(p0, cp1, cp2, p1, 0.0);
        assert_eq!(start.x, 0.0);
        assert_eq!(start.y, 0.0);
        
        // At t=1, should be at p1
        let end = eval_cubic(p0, cp1, cp2, p1, 1.0);
        assert_eq!(end.x, 10.0);
        assert_eq!(end.y, 0.0);
    }
}
