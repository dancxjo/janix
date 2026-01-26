use stem::xml::{parse_color, parse_f32, parse_length_px, parse_transform, Attributes, TransformCmd};
use alloc::vec::Vec;

use crate::geometry::Color;

use super::parser::SvgParser;
use super::state::{parse_line_cap, parse_line_join, SvgState};

impl SvgParser {
    pub(crate) fn apply_attributes(&mut self, attrs: &Attributes) {
        let mut transform_cmds = Vec::new();
        let state = self.current_state_mut();

        apply_paint_attr(attrs, "fill", state, PaintTarget::Fill);
        apply_paint_attr(attrs, "stroke", state, PaintTarget::Stroke);

        if let Some(sw) = attrs.get("stroke-width") {
            if let Some(w) = parse_length_px(sw) {
                state.stroke_width = w.max(0.0);
            }
        }
        if let Some(lc) = attrs.get("stroke-linecap") {
            if let Some(cap) = parse_line_cap(lc) {
                state.line_cap = cap;
            }
        }
        if let Some(lj) = attrs.get("stroke-linejoin") {
            if let Some(join) = parse_line_join(lj) {
                state.line_join = join;
            }
        }
        if let Some(ml) = attrs.get("stroke-miterlimit") {
            if let Some(limit) = parse_f32(ml) {
                state.miter_limit = limit.max(0.1);
            }
        }
        if let Some(op) = attrs.get("opacity") {
            if let Some(f) = parse_f32(op) {
                state.opacity = clamp_opacity_to_u8(f);
            }
        }
        apply_alpha_attr(attrs, "fill-opacity", state, PaintTarget::Fill);
        apply_alpha_attr(attrs, "stroke-opacity", state, PaintTarget::Stroke);
        if let Some(fr) = attrs.get("fill-rule") {
            match fr {
                "evenodd" => state.fill_rule = crate::isa::FillRule::EvenOdd,
                _ => state.fill_rule = crate::isa::FillRule::NonZero,
            }
        }
        if let Some(trans) = attrs.get("transform") {
            transform_cmds.extend(parse_transform(trans));
        }

        if let Some(style) = attrs.get("style") {
            apply_style(style, state);
        }

        for cmd in transform_cmds {
            match cmd {
                TransformCmd::Translate(tx, ty) => {
                    let other = crate::geometry::Transform::translate(tx, ty);
                    state.transform = state.transform.multiply(&other);
                }
                TransformCmd::Scale(sx, sy) => {
                    let other = crate::geometry::Transform::scale(sx, sy);
                    state.transform = state.transform.multiply(&other);
                }
                TransformCmd::Rotate(angle) => {
                    let other = crate::geometry::Transform::rotate_degrees(angle);
                    state.transform = state.transform.multiply(&other);
                }
            }
        }
    }
}

enum PaintTarget {
    Fill,
    Stroke,
}

fn apply_paint_attr(attrs: &Attributes, key: &str, state: &mut SvgState, target: PaintTarget) {
    if let Some(value) = attrs.get(key) {
        apply_paint_value(value, state, target);
    }
}

fn apply_alpha_attr(attrs: &Attributes, key: &str, state: &mut SvgState, target: PaintTarget) {
    if let Some(value) = attrs.get(key) {
        if let Some(alpha) = parse_f32(value).map(clamp_opacity_to_u8) {
            apply_alpha(state, target, alpha);
        }
    }
}

fn apply_style(style: &str, state: &mut SvgState) {
    let pairs: Vec<(&str, &str)> = style
        .split(';')
        .filter_map(|part| {
            let mut pieces = part.splitn(2, ':');
            let key = pieces.next()?.trim();
            let value = pieces.next()?.trim();
            if key.is_empty() || value.is_empty() {
                None
            } else {
                Some((key, value))
            }
        })
        .collect();

    for (key, value) in &pairs {
        match key {
            "fill" => apply_paint_value(value, state, PaintTarget::Fill),
            "stroke" => apply_paint_value(value, state, PaintTarget::Stroke),
            "stroke-width" => {
                if let Some(w) = parse_length_px(value) {
                    state.stroke_width = w.max(0.0);
                }
            }
            "stroke-linecap" => {
                if let Some(cap) = parse_line_cap(value) {
                    state.line_cap = cap;
                }
            }
            "stroke-linejoin" => {
                if let Some(join) = parse_line_join(value) {
                    state.line_join = join;
                }
            }
            "stroke-miterlimit" => {
                if let Some(limit) = parse_f32(value) {
                    state.miter_limit = limit.max(0.1);
                }
            }
            "fill-rule" => {
                state.fill_rule = match value {
                    "evenodd" => crate::isa::FillRule::EvenOdd,
                    _ => crate::isa::FillRule::NonZero,
                };
            }
            _ => {}
        }
    }

    for (key, value) in &pairs {
        match key {
            "opacity" => {
                if let Some(f) = parse_f32(value) {
                    state.opacity = clamp_opacity_to_u8(f);
                }
            }
            "fill-opacity" => {
                if let Some(alpha) = parse_f32(value).map(clamp_opacity_to_u8) {
                    apply_alpha(state, PaintTarget::Fill, alpha);
                }
            }
            "stroke-opacity" => {
                if let Some(alpha) = parse_f32(value).map(clamp_opacity_to_u8) {
                    apply_alpha(state, PaintTarget::Stroke, alpha);
                }
            }
            _ => {}
        }
    }
}

fn apply_paint_value(value: &str, state: &mut SvgState, target: PaintTarget) {
    let trimmed = value.trim();
    let paint = if trimmed.eq_ignore_ascii_case("none") {
        Some(Color::TRANSPARENT)
    } else {
        parse_color(trimmed).map(|c| Color::new(c.r, c.g, c.b, c.a))
    };

    if let Some(color) = paint {
        match target {
            PaintTarget::Fill => state.fill = color,
            PaintTarget::Stroke => state.stroke = color,
        }
    }
}

fn apply_alpha(state: &mut SvgState, target: PaintTarget, alpha: u8) {
    match target {
        PaintTarget::Fill => state.fill.a = alpha,
        PaintTarget::Stroke => state.stroke.a = alpha,
    }
}

fn clamp_opacity_to_u8(value: f32) -> u8 {
    (value * 255.0).clamp(0.0, 255.0) as u8
}
