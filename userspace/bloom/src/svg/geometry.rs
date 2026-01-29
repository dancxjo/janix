use crate::geometry::{Point, Rect, Transform};
use crate::isa::{LineCap, LineJoin};

use super::state::StrokeStyle;

const SNAP_EPS: f32 = 0.01;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() <= SNAP_EPS
}

pub(crate) fn snap_to_i32(value: f32) -> Option<i32> {
    let rounded = libm::roundf(value);
    if (value - rounded).abs() <= SNAP_EPS {
        Some(rounded as i32)
    } else {
        None
    }
}

fn translation_only(transform: &Transform) -> Option<(i32, i32)> {
    if approx_eq(transform.m11, 1.0)
        && approx_eq(transform.m22, 1.0)
        && approx_eq(transform.m12, 0.0)
        && approx_eq(transform.m21, 0.0)
    {
        Some((snap_to_i32(transform.dx)?, snap_to_i32(transform.dy)?))
    } else {
        None
    }
}

pub(crate) fn snapped_point(x: f32, y: f32, transform: &Transform) -> Option<Point> {
    let (dx, dy) = translation_only(transform)?;
    let sx = snap_to_i32(x + dx as f32)?;
    let sy = snap_to_i32(y + dy as f32)?;
    Some(Point::new(sx, sy))
}

pub(crate) fn snapped_rect(x: f32, y: f32, w: f32, h: f32, transform: &Transform) -> Option<Rect> {
    let (dx, dy) = translation_only(transform)?;
    let sx = snap_to_i32(x + dx as f32)?;
    let sy = snap_to_i32(y + dy as f32)?;
    let sw = snap_to_i32(w)?;
    let sh = snap_to_i32(h)?;
    Some(Rect::new(sx, sy, sw, sh))
}

pub(crate) fn can_use_rect_stroke(stroke: &StrokeStyle) -> bool {
    stroke.cap == LineCap::Butt
        && stroke.join == LineJoin::Miter
        && approx_eq(stroke.miter_limit, 4.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Transform;

    #[test]
    fn snaps_to_integer_grid() {
        assert_eq!(snap_to_i32(1.0), Some(1));
        assert_eq!(snap_to_i32(1.004), Some(1));
        assert_eq!(snap_to_i32(1.02), None);
    }

    #[test]
    fn detects_translation_only_transform() {
        let mut transform = Transform::identity();
        transform.dx = 2.0;
        transform.dy = -3.0;
        let point = snapped_point(1.0, 2.0, &transform).expect("point");
        assert_eq!(point, Point::new(3, -1));
    }
}
