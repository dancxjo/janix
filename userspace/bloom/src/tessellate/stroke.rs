//! Stroke expansion
//!
//! Converts a stroked path into a filled outline path.

use crate::tessellate::{TessellateConfig, TessellatedPath};
use crate::vir::{LineCap, LineJoin, StrokeStyle, VirPoint};
use alloc::vec::Vec;

/// Expand a stroked path into a filled outline
pub fn expand_stroke(
    path: &TessellatedPath,
    stroke: &StrokeStyle,
    _config: &TessellateConfig,
) -> TessellatedPath {
    let mut result = TessellatedPath::new();
    let half_width = stroke.width * 0.5;

    // For each contour in the path
    for contour in &path.contours {
        let start_idx = contour.start;
        let end_idx = start_idx + contour.count;

        if contour.count < 2 {
            continue; // Need at least 2 points
        }

        let points = &path.vertices[start_idx..end_idx];

        // Generate left and right offset curves
        let mut left_points = Vec::new();
        let mut right_points = Vec::new();

        for i in 0..points.len() {
            let curr = points[i];
            let prev = if i > 0 {
                points[i - 1]
            } else {
                // For open paths, use the first point
                curr
            };
            let next = if i < points.len() - 1 {
                points[i + 1]
            } else {
                // For closed paths, wrap around
                points[0]
            };

            // Calculate perpendicular offset
            let (left, right) = if i == 0 {
                // First point
                offset_point_cap(curr, next, half_width, stroke.line_cap)
            } else if i == points.len() - 1 {
                // Last point
                offset_point_cap(curr, prev, half_width, stroke.line_cap)
            } else {
                // Middle point - join
                offset_point_join(
                    prev,
                    curr,
                    next,
                    half_width,
                    stroke.line_join,
                    stroke.miter_limit,
                )
            };

            left_points.push(left);
            right_points.push(right);
        }

        // Build the outline: left side forward, right side backward
        let contour_start = result.vertices.len();

        // Add left side
        for p in left_points {
            result.vertices.push(p);
        }

        // Add right side in reverse
        for p in right_points.iter().rev() {
            result.vertices.push(*p);
        }

        // Close the contour
        result.contours.push(crate::tessellate::Contour {
            start: contour_start,
            count: result.vertices.len() - contour_start,
        });
    }

    result
}

/// Calculate offset points for a line cap
fn offset_point_cap(
    point: VirPoint,
    direction_point: VirPoint,
    offset: f32,
    _cap: LineCap,
) -> (VirPoint, VirPoint) {
    // Calculate perpendicular direction
    let dx = direction_point.x - point.x;
    let dy = direction_point.y - point.y;
    let len = libm::sqrtf(dx * dx + dy * dy);

    if len < 1e-6 {
        return (point, point);
    }

    // Perpendicular vector
    let px = -dy / len * offset;
    let py = dx / len * offset;

    let left = VirPoint::new(point.x + px, point.y + py);
    let right = VirPoint::new(point.x - px, point.y - py);

    (left, right)
}

/// Calculate offset points for a line join
fn offset_point_join(
    prev: VirPoint,
    curr: VirPoint,
    next: VirPoint,
    offset: f32,
    join: LineJoin,
    miter_limit: f32,
) -> (VirPoint, VirPoint) {
    // Calculate incoming direction
    let dx1 = curr.x - prev.x;
    let dy1 = curr.y - prev.y;
    let len1 = libm::sqrtf(dx1 * dx1 + dy1 * dy1);

    // Calculate outgoing direction
    let dx2 = next.x - curr.x;
    let dy2 = next.y - curr.y;
    let len2 = libm::sqrtf(dx2 * dx2 + dy2 * dy2);

    if len1 < 1e-6 || len2 < 1e-6 {
        // Degenerate case
        return (curr, curr);
    }

    // Normalize
    let ndx1 = dx1 / len1;
    let ndy1 = dy1 / len1;
    let ndx2 = dx2 / len2;
    let ndy2 = dy2 / len2;

    // Perpendiculars
    let px1 = -ndy1 * offset;
    let py1 = ndx1 * offset;
    let px2 = -ndy2 * offset;
    let py2 = ndx2 * offset;

    match join {
        LineJoin::Miter => {
            // Calculate miter join
            // For now, use simple average (proper miter requires more math)
            let px_avg = (px1 + px2) * 0.5;
            let py_avg = (py1 + py2) * 0.5;

            // Check miter limit
            let miter_len = libm::sqrtf(px_avg * px_avg + py_avg * py_avg);
            let should_bevel = miter_len > offset * miter_limit;

            if should_bevel {
                // Fall back to bevel
                let left = VirPoint::new(curr.x + px1, curr.y + py1);
                let right = VirPoint::new(curr.x - px1, curr.y - py1);
                (left, right)
            } else {
                let left = VirPoint::new(curr.x + px_avg, curr.y + py_avg);
                let right = VirPoint::new(curr.x - px_avg, curr.y - py_avg);
                (left, right)
            }
        }
        LineJoin::Bevel | LineJoin::Round => {
            // Simple bevel join (round would need arc segments)
            // Use the average for now
            let px_avg = (px1 + px2) * 0.5;
            let py_avg = (py1 + py2) * 0.5;
            let left = VirPoint::new(curr.x + px_avg, curr.y + py_avg);
            let right = VirPoint::new(curr.x - px_avg, curr.y - py_avg);
            (left, right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vir::Paint;

    #[test]
    fn test_stroke_expansion() {
        let mut path = TessellatedPath::new();

        // Simple line segment
        path.vertices.push(VirPoint::new(0.0, 0.0));
        path.vertices.push(VirPoint::new(10.0, 0.0));
        path.contours
            .push(crate::tessellate::Contour { start: 0, count: 2 });

        let stroke = StrokeStyle {
            paint: Paint::Solid(crate::vir::VirColor::BLACK),
            width: 2.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 4.0,
        };

        let config = TessellateConfig::default();
        let result = expand_stroke(&path, &stroke, &config);

        // Should produce a quadrilateral (4 points)
        assert!(result.vertices.len() >= 4);
    }
}
