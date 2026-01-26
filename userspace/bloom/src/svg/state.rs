use crate::geometry::Color;

#[derive(Debug, Clone)]
pub(crate) struct SvgState {
    pub(crate) transform: crate::geometry::Transform,
    pub(crate) fill: Color,
    pub(crate) stroke: Color,
    pub(crate) stroke_width: f32,
    pub(crate) opacity: u8,
    pub(crate) fill_rule: crate::isa::FillRule,
    pub(crate) line_cap: crate::isa::LineCap,
    pub(crate) line_join: crate::isa::LineJoin,
    pub(crate) miter_limit: f32,
}

impl Default for SvgState {
    fn default() -> Self {
        Self {
            transform: crate::geometry::Transform::identity(),
            fill: Color::BLACK,
            stroke: Color::TRANSPARENT,
            stroke_width: 1.0,
            opacity: 255,
            fill_rule: crate::isa::FillRule::NonZero,
            line_cap: crate::isa::LineCap::Butt,
            line_join: crate::isa::LineJoin::Miter,
            miter_limit: 4.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct StrokeStyle {
    pub(crate) color: Color,
    pub(crate) width: i32,
    pub(crate) cap: crate::isa::LineCap,
    pub(crate) join: crate::isa::LineJoin,
    pub(crate) miter_limit: f32,
}

pub(crate) fn apply_opacity(c: Color, opacity: u8) -> Color {
    crate::geometry::Color::new(c.r, c.g, c.b, ((c.a as u16 * opacity as u16) / 255) as u8)
}

pub(crate) fn parse_line_cap(value: &str) -> Option<crate::isa::LineCap> {
    match value.trim() {
        "butt" => Some(crate::isa::LineCap::Butt),
        "round" => Some(crate::isa::LineCap::Round),
        "square" => Some(crate::isa::LineCap::Square),
        _ => None,
    }
}

pub(crate) fn parse_line_join(value: &str) -> Option<crate::isa::LineJoin> {
    match value.trim() {
        "miter" => Some(crate::isa::LineJoin::Miter),
        "round" => Some(crate::isa::LineJoin::Round),
        "bevel" => Some(crate::isa::LineJoin::Bevel),
        _ => None,
    }
}

/// Convert an SVG stroke width to an integer pixel width.
/// The conversion rounds to the nearest whole pixel and
/// returns `None` for zero/negative widths.
pub(crate) fn stroke_width_to_i32(width: f32) -> Option<i32> {
    if width <= 0.0 {
        None
    } else {
        Some(libm::roundf(width.max(1.0)) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_line_cap_values() {
        assert_eq!(parse_line_cap("butt"), Some(crate::isa::LineCap::Butt));
        assert_eq!(parse_line_cap("round"), Some(crate::isa::LineCap::Round));
        assert_eq!(parse_line_cap("square"), Some(crate::isa::LineCap::Square));
        assert_eq!(parse_line_cap("other"), None);
    }

    #[test]
    fn parses_line_join_values() {
        assert_eq!(parse_line_join("miter"), Some(crate::isa::LineJoin::Miter));
        assert_eq!(parse_line_join("round"), Some(crate::isa::LineJoin::Round));
        assert_eq!(parse_line_join("bevel"), Some(crate::isa::LineJoin::Bevel));
        assert_eq!(parse_line_join("other"), None);
    }

    #[test]
    fn stroke_width_rounds_to_pixels() {
        assert_eq!(stroke_width_to_i32(-1.0), None);
        assert_eq!(stroke_width_to_i32(0.0), None);
        assert_eq!(stroke_width_to_i32(0.4), Some(1));
        assert_eq!(stroke_width_to_i32(1.49), Some(1));
        assert_eq!(stroke_width_to_i32(1.5), Some(2));
    }
}
