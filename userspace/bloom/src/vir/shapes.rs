//! Shape expansion to paths
//!
//! Converts high-level shapes (circle, ellipse, rect) into VIR paths.

use crate::vir::{VirPath, VirPoint};

/// Expand a rectangle into a path
pub fn rect_to_path(x: f32, y: f32, width: f32, height: f32, rx: f32, ry: f32) -> VirPath {
    let mut path = VirPath::new();

    if rx <= 0.0 && ry <= 0.0 {
        // Simple rectangle (no rounded corners)
        path.move_to(x, y);
        path.line_to(x + width, y);
        path.line_to(x + width, y + height);
        path.line_to(x, y + height);
        path.close();
    } else {
        // Rounded rectangle
        // Clamp rx and ry to half dimensions
        let rx = rx.min(width / 2.0);
        let ry = ry.min(height / 2.0);

        // Start at top-left after the corner
        path.move_to(x + rx, y);

        // Top edge
        path.line_to(x + width - rx, y);

        // Top-right corner (approximate with cubic bezier)
        let cp = 0.55228; // Magic number for circular arc approximation
        path.cubic_to(
            x + width - rx + cp * rx,
            y,
            x + width,
            y + ry - cp * ry,
            x + width,
            y + ry,
        );

        // Right edge
        path.line_to(x + width, y + height - ry);

        // Bottom-right corner
        path.cubic_to(
            x + width,
            y + height - ry + cp * ry,
            x + width - rx + cp * rx,
            y + height,
            x + width - rx,
            y + height,
        );

        // Bottom edge
        path.line_to(x + rx, y + height);

        // Bottom-left corner
        path.cubic_to(
            x + rx - cp * rx,
            y + height,
            x,
            y + height - ry + cp * ry,
            x,
            y + height - ry,
        );

        // Left edge
        path.line_to(x, y + ry);

        // Top-left corner
        path.cubic_to(x, y + ry - cp * ry, x + rx - cp * rx, y, x + rx, y);

        path.close();
    }

    path
}

/// Expand a circle into a path
pub fn circle_to_path(cx: f32, cy: f32, r: f32) -> VirPath {
    ellipse_to_path(cx, cy, r, r)
}

/// Expand an ellipse into a path using cubic bezier approximation
pub fn ellipse_to_path(cx: f32, cy: f32, rx: f32, ry: f32) -> VirPath {
    let mut path = VirPath::new();

    // Use 4 cubic beziers to approximate a circle/ellipse
    // The magic constant for circular arc approximation: 4/3 * (√2 - 1)
    let k = 0.5522847498;

    let kx = k * rx;
    let ky = k * ry;

    // Start at rightmost point
    path.move_to(cx + rx, cy);

    // Top-right quadrant
    path.cubic_to(cx + rx, cy - ky, cx + kx, cy - ry, cx, cy - ry);

    // Top-left quadrant
    path.cubic_to(cx - kx, cy - ry, cx - rx, cy - ky, cx - rx, cy);

    // Bottom-left quadrant
    path.cubic_to(cx - rx, cy + ky, cx - kx, cy + ry, cx, cy + ry);

    // Bottom-right quadrant
    path.cubic_to(cx + kx, cy + ry, cx + rx, cy + ky, cx + rx, cy);

    path.close();
    path
}

/// Expand a line into a path
pub fn line_to_path(x1: f32, y1: f32, x2: f32, y2: f32) -> VirPath {
    let mut path = VirPath::new();
    path.move_to(x1, y1);
    path.line_to(x2, y2);
    path
}

/// Expand a polyline into a path
pub fn polyline_to_path(points: &[VirPoint]) -> VirPath {
    let mut path = VirPath::new();

    if let Some(first) = points.first() {
        path.move_to(first.x, first.y);

        for point in points.iter().skip(1) {
            path.line_to(point.x, point.y);
        }
    }

    path
}

/// Expand a polygon into a path (same as polyline but closed)
pub fn polygon_to_path(points: &[VirPoint]) -> VirPath {
    let mut path = polyline_to_path(points);
    path.close();
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vir::VirSegment;

    #[test]
    fn test_simple_rect() {
        let path = rect_to_path(10.0, 20.0, 30.0, 40.0, 0.0, 0.0);
        assert_eq!(path.segments.len(), 5); // M, L, L, L, Z

        match path.segments[0] {
            VirSegment::MoveTo(p) => {
                assert_eq!(p.x, 10.0);
                assert_eq!(p.y, 20.0);
            }
            _ => panic!("Expected MoveTo"),
        }
    }

    #[test]
    fn test_circle() {
        let path = circle_to_path(50.0, 50.0, 25.0);
        // Circle uses 1 MoveTo, 4 cubic beziers + close
        assert_eq!(path.segments.len(), 6);

        match path.segments[0] {
            VirSegment::MoveTo(p) => {
                // Starts at rightmost point
                assert_eq!(p.x, 75.0); // cx + r
                assert_eq!(p.y, 50.0); // cy
            }
            _ => panic!("Expected MoveTo"),
        }
    }

    #[test]
    fn test_line() {
        let path = line_to_path(0.0, 0.0, 10.0, 10.0);
        assert_eq!(path.segments.len(), 2); // M, L
    }

    #[test]
    fn test_polygon() {
        let points = alloc::vec![
            VirPoint::new(0.0, 0.0),
            VirPoint::new(10.0, 0.0),
            VirPoint::new(10.0, 10.0),
        ];
        let path = polygon_to_path(&points);
        assert_eq!(path.segments.len(), 4); // M, L, L, Z
    }
}
