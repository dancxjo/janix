use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

pub mod compile;
pub mod ir;
pub mod parse;
pub mod walk;

use crate::drawlist::DrawCmd;
use crate::geometry::{Color, Point, Rect, Transform};
use stem::xml::{
    parse_color, parse_f32, parse_length_px, parse_transform, Attributes, Event, TransformCmd,
    XmlReader,
};

#[derive(Debug, Clone)]
struct SvgState {
    transform: Transform,
    fill: Color,
    stroke: Color,
    stroke_width: f32,
    opacity: u8,
    fill_rule: crate::isa::FillRule,
    line_cap: crate::isa::LineCap,
    line_join: crate::isa::LineJoin,
    miter_limit: f32,
}

impl Default for SvgState {
    fn default() -> Self {
        Self {
            transform: Transform::identity(),
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
struct StrokeStyle {
    color: Color,
    width: i32,
    cap: crate::isa::LineCap,
    join: crate::isa::LineJoin,
    miter_limit: f32,
}

const SNAP_EPS: f32 = 0.01;

pub struct SvgParser {
    state_stack: Vec<SvgState>,
    cmds: Vec<DrawCmd>,
    width: i32,
    height: i32,
    view_box: Option<Rect>,
}

impl SvgParser {
    pub fn new() -> Self {
        Self {
            state_stack: vec![SvgState::default()],
            cmds: Vec::new(),
            width: 0,
            height: 0,
            view_box: None,
        }
    }

    fn current_state(&self) -> &SvgState {
        self.state_stack.last().expect("State stack empty")
    }

    fn current_state_mut(&mut self) -> &mut SvgState {
        self.state_stack.last_mut().expect("State stack empty")
    }

    fn push_state(&mut self) {
        let current = self.current_state().clone();
        self.state_stack.push(current);
    }

    fn pop_state(&mut self) {
        if self.state_stack.len() > 1 {
            self.state_stack.pop();
        }
    }

    fn apply_attributes(&mut self, attrs: &Attributes) {
        // Collect transform cmds first so we can apply them
        let mut transform_cmds = Vec::new();

        let state = self.current_state_mut();

        // Attributes
        if let Some(fill) = attrs.get("fill") {
            if let Some(c) = parse_color(fill) {
                state.fill = Color::new(c.r, c.g, c.b, c.a);
            }
        }
        if let Some(stroke) = attrs.get("stroke") {
            if let Some(c) = parse_color(stroke) {
                state.stroke = Color::new(c.r, c.g, c.b, c.a);
            }
        }
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
                state.opacity = (f * 255.0).clamp(0.0, 255.0) as u8;
            }
        }
        if let Some(fop) = attrs.get("fill-opacity") {
            if let Some(f) = parse_f32(fop) {
                state.fill.a = (f * 255.0).clamp(0.0, 255.0) as u8;
            }
        }
        if let Some(sop) = attrs.get("stroke-opacity") {
            if let Some(f) = parse_f32(sop) {
                state.stroke.a = (f * 255.0).clamp(0.0, 255.0) as u8;
            }
        }
        if let Some(fr) = attrs.get("fill-rule") {
            match fr {
                "evenodd" => state.fill_rule = crate::isa::FillRule::EvenOdd,
                _ => state.fill_rule = crate::isa::FillRule::NonZero,
            }
        }
        if let Some(trans) = attrs.get("transform") {
            transform_cmds.extend(parse_transform(trans));
        }

        // Inline style
        if let Some(style) = attrs.get("style") {
            for part in style.split(';') {
                let kv: Vec<&str> = part.split(':').collect();
                if kv.len() == 2 {
                    let k = kv[0].trim();
                    let v = kv[1].trim();
                    match k {
                        "fill" => {
                            if let Some(c) = parse_color(v) {
                                state.fill = Color::new(c.r, c.g, c.b, c.a);
                            }
                        }
                        "stroke" => {
                            if let Some(c) = parse_color(v) {
                                state.stroke = Color::new(c.r, c.g, c.b, c.a);
                            }
                        }
                        "stroke-width" => {
                            if let Some(w) = parse_length_px(v) {
                                state.stroke_width = w.max(0.0);
                            }
                        }
                        "stroke-linecap" => {
                            if let Some(cap) = parse_line_cap(v) {
                                state.line_cap = cap;
                            }
                        }
                        "stroke-linejoin" => {
                            if let Some(join) = parse_line_join(v) {
                                state.line_join = join;
                            }
                        }
                        "stroke-miterlimit" => {
                            if let Some(limit) = parse_f32(v) {
                                state.miter_limit = limit.max(0.1);
                            }
                        }
                        "opacity" => {
                            if let Some(f) = parse_f32(v) {
                                state.opacity = (f * 255.0).clamp(0.0, 255.0) as u8;
                            }
                        }
                        "fill-rule" => match v {
                            "evenodd" => state.fill_rule = crate::isa::FillRule::EvenOdd,
                            _ => state.fill_rule = crate::isa::FillRule::NonZero,
                        },
                        _ => {}
                    }
                }
            }
        }

        // Apply transform accumulation
        for cmd in transform_cmds {
            match cmd {
                TransformCmd::Translate(tx, ty) => {
                    let other = Transform::translate(tx, ty);
                    state.transform = state.transform.multiply(&other);
                }
                TransformCmd::Scale(sx, sy) => {
                    let other = Transform::scale(sx, sy);
                    state.transform = state.transform.multiply(&other);
                }
                TransformCmd::Rotate(angle) => {
                    let other = Transform::rotate_degrees(angle);
                    state.transform = state.transform.multiply(&other);
                }
            }
        }
    }

    fn current_paints(
        &self,
    ) -> (
        Transform,
        Option<Color>,
        Option<StrokeStyle>,
        crate::isa::FillRule,
    ) {
        let state = self.current_state();
        let fill = apply_opacity(state.fill, state.opacity);
        let stroke = apply_opacity(state.stroke, state.opacity);
        let fill = if fill.a > 0 { Some(fill) } else { None };
        let stroke_style = stroke_width_to_i32(state.stroke_width).and_then(|width| {
            if stroke.a > 0 {
                Some(StrokeStyle {
                    color: stroke,
                    width,
                    cap: state.line_cap,
                    join: state.line_join,
                    miter_limit: state.miter_limit,
                })
            } else {
                None
            }
        });
        (state.transform, fill, stroke_style, state.fill_rule)
    }

    pub fn parse(&mut self, xml: &str) -> Vec<DrawCmd> {
        self.cmds.clear();
        // Preserve initial transform (e.g. set by render_to_buffer for scaling)
        let initial_transform = self.state_stack[0].transform;
        self.state_stack.truncate(1);
        self.state_stack[0] = SvgState::default();
        self.state_stack[0].transform = initial_transform;

        let reader = XmlReader::new(xml.as_bytes());

        for event in reader {
            match event {
                Ok(Event::StartElement { name, attributes }) => {
                    self.handle_start_element(&name, &attributes);
                }
                Ok(Event::EndElement { name }) => {
                    self.handle_end_element(&name);
                }
                _ => {}
            }
        }

        self.cmds.clone()
    }

    fn handle_start_element(&mut self, name: &str, attrs: &Attributes) {
        match name {
            "svg" => {
                if let Some(w) = attrs.get("width").and_then(parse_length_px) {
                    self.width = w as i32;
                }
                if let Some(h) = attrs.get("height").and_then(parse_length_px) {
                    self.height = h as i32;
                }
                if let Some(vb) = attrs.get("viewBox") {
                    let parts: Vec<f32> = vb
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    if parts.len() == 4 {
                        let vbr = Rect::new(
                            parts[0] as i32,
                            parts[1] as i32,
                            parts[2] as i32,
                            parts[3] as i32,
                        );
                        self.view_box = Some(vbr);

                        // Apply viewbox scaling if we have width/height
                        if self.width > 0 && self.height > 0 {
                            let sx = self.width as f32 / parts[2];
                            let sy = self.height as f32 / parts[3];
                            let tx = -parts[0] * sx;
                            let ty = -parts[1] * sy;
                            let state = self.current_state_mut();
                            state.transform =
                                state.transform.multiply(&Transform::translate(tx, ty));
                            state.transform = state.transform.multiply(&Transform::scale(sx, sy));
                        }
                    }
                }
            }
            "g" => {
                self.push_state();
                self.apply_attributes(attrs);
            }
            "rect" => {
                self.push_state();
                self.apply_attributes(attrs);

                let x = attrs.get("x").and_then(parse_length_px).unwrap_or(0.0);
                let y = attrs.get("y").and_then(parse_length_px).unwrap_or(0.0);
                let w = attrs.get("width").and_then(parse_length_px).unwrap_or(0.0);
                let h = attrs.get("height").and_then(parse_length_px).unwrap_or(0.0);

                let mut rx = attrs.get("rx").and_then(parse_length_px).unwrap_or(0.0);
                let mut ry = attrs.get("ry").and_then(parse_length_px).unwrap_or(0.0);

                // SVG spec: if one is missing, use the other. If both missing, 0.
                if rx == 0.0 && ry > 0.0 {
                    rx = ry;
                }
                if ry == 0.0 && rx > 0.0 {
                    ry = rx;
                }

                self.emit_rect(x, y, w, h, rx, ry);
            }
            "line" => {
                // Line conversion to path or use DrawCmd::Line
                // DrawCmd::Line exists but is integer coords. SVG line can be float/subpixel.
                self.push_state();
                self.apply_attributes(attrs);
                let x1 = attrs.get("x1").and_then(parse_length_px).unwrap_or(0.0);
                let y1 = attrs.get("y1").and_then(parse_length_px).unwrap_or(0.0);
                let x2 = attrs.get("x2").and_then(parse_length_px).unwrap_or(0.0);
                let y2 = attrs.get("y2").and_then(parse_length_px).unwrap_or(0.0);
                self.emit_line(x1, y1, x2, y2);
            }
            "circle" => {
                self.push_state();
                self.apply_attributes(attrs);

                let cx = attrs.get("cx").and_then(parse_length_px).unwrap_or(0.0);
                let cy = attrs.get("cy").and_then(parse_length_px).unwrap_or(0.0);
                let r = attrs.get("r").and_then(parse_length_px).unwrap_or(0.0);

                self.emit_circle(cx, cy, r);
            }
            "ellipse" => {
                self.push_state();
                self.apply_attributes(attrs);

                let cx = attrs.get("cx").and_then(parse_length_px).unwrap_or(0.0);
                let cy = attrs.get("cy").and_then(parse_length_px).unwrap_or(0.0);
                let rx = attrs.get("rx").and_then(parse_length_px).unwrap_or(0.0);
                let ry = attrs.get("ry").and_then(parse_length_px).unwrap_or(0.0);

                self.emit_ellipse(cx, cy, rx, ry);
            }
            "path" => {
                self.push_state();
                self.apply_attributes(attrs);
                if let Some(d) = attrs.get("d") {
                    let path = crate::svg::parse::parse_path_d(d);
                    self.emit_path(path);
                }
            }
            "polyline" | "polygon" => {
                self.push_state();
                self.apply_attributes(attrs);
                if let Some(pts_str) = attrs.get("points") {
                    let pts: Vec<f32> = pts_str
                        .split(|c| c == ',' || c == ' ' || c == '\n')
                        .filter_map(parse_f32)
                        .collect();

                    if !pts.is_empty() {
                        use crate::svg::ir::{PathCommand, PointF};
                        let mut verbs = Vec::with_capacity(pts.len() / 2 + 1);
                        if pts.len() >= 2 {
                            verbs.push(PathCommand::MoveTo(PointF {
                                x: pts[0],
                                y: pts[1],
                            }));
                            for i in (2..pts.len()).step_by(2) {
                                if i + 1 < pts.len() {
                                    verbs.push(PathCommand::LineTo(PointF {
                                        x: pts[i],
                                        y: pts[i + 1],
                                    }));
                                }
                            }
                        }
                        if name == "polygon" {
                            verbs.push(PathCommand::Close);
                        }
                        let path = crate::isa::Path2D { verbs };
                        self.emit_path(path);
                    }
                }
            }
            _ => {
                self.push_state();
            }
        }
    }

    fn handle_end_element(&mut self, _name: &str) {
        self.pop_state();
    }

    fn emit_rect(&mut self, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32) {
        use crate::svg::ir::{PathCommand, PointF};

        // Clamp radius
        let rx = rx.min(w * 0.5).max(0.0);
        let ry = ry.min(h * 0.5).max(0.0);

        let (transform, fill, stroke, fill_rule) = self.current_paints();
        let mut used_fill = false;
        let mut used_stroke = false;

        if rx == 0.0 && ry == 0.0 {
            if let Some(rect) = snapped_rect(x, y, w, h, &transform) {
                if let Some(color) = fill {
                    self.cmds.push(DrawCmd::FillRect {
                        rect,
                        color,
                        aa: crate::geometry::EdgeAA::Coverage8,
                    });
                    used_fill = true;
                }
                if let Some(stroke_style) = stroke {
                    if can_use_rect_stroke(&stroke_style) {
                        self.cmds.push(DrawCmd::StrokeRect {
                            rect,
                            color: stroke_style.color,
                            width: stroke_style.width,
                        });
                        used_stroke = true;
                    }
                }
            }
        }

        if used_fill && used_stroke {
            return;
        }

        let mut verbs = Vec::with_capacity(10);

        if rx > 0.0 || ry > 0.0 {
            // Rounded rect
            const K: f32 = 0.55228475;
            let kx = rx * K;
            let ky = ry * K;

            verbs.push(PathCommand::MoveTo(PointF { x: x + rx, y }));

            // Top edge
            verbs.push(PathCommand::LineTo(PointF { x: x + w - rx, y }));
            // TR Corner
            verbs.push(PathCommand::CubicTo(
                PointF {
                    x: x + w - rx + kx,
                    y,
                },
                PointF {
                    x: x + w,
                    y: y + ry - ky,
                },
                PointF {
                    x: x + w,
                    y: y + ry,
                },
            ));

            // Right edge
            verbs.push(PathCommand::LineTo(PointF {
                x: x + w,
                y: y + h - ry,
            }));
            // BR Corner
            verbs.push(PathCommand::CubicTo(
                PointF {
                    x: x + w,
                    y: y + h - ry + ky,
                },
                PointF {
                    x: x + w - rx + kx,
                    y: y + h,
                },
                PointF {
                    x: x + w - rx,
                    y: y + h,
                },
            ));

            // Bottom edge
            verbs.push(PathCommand::LineTo(PointF {
                x: x + rx,
                y: y + h,
            }));
            // BL Corner
            verbs.push(PathCommand::CubicTo(
                PointF {
                    x: x + rx - kx,
                    y: y + h,
                },
                PointF {
                    x: x,
                    y: y + h - ry + ky,
                },
                PointF {
                    x: x,
                    y: y + h - ry,
                },
            ));

            // Left edge
            verbs.push(PathCommand::LineTo(PointF { x, y: y + ry }));
            // TL Corner
            verbs.push(PathCommand::CubicTo(
                PointF { x, y: y + ry - ky },
                PointF { x: x + rx - kx, y },
                PointF { x: x + rx, y },
            ));
        } else {
            verbs.push(PathCommand::MoveTo(PointF { x, y }));
            verbs.push(PathCommand::LineTo(PointF { x: x + w, y }));
            verbs.push(PathCommand::LineTo(PointF { x: x + w, y: y + h }));
            verbs.push(PathCommand::LineTo(PointF { x, y: y + h }));
        }
        verbs.push(PathCommand::Close);

        let fill = if used_fill { None } else { fill };
        let stroke = if used_stroke { None } else { stroke };
        if fill.is_none() && stroke.is_none() {
            return;
        }
        let path = crate::isa::Path2D { verbs };
        self.emit_path_with_style(path, transform, fill, stroke, fill_rule);
    }

    fn emit_circle(&mut self, cx: f32, cy: f32, r: f32) {
        if r <= 0.0 {
            return;
        }
        let (transform, fill, stroke, fill_rule) = self.current_paints();
        let mut used_fill = false;
        if let Some(center) = snapped_point(cx, cy, &transform) {
            if let Some(radius) = snap_to_i32(r) {
                if let Some(color) = fill {
                    self.cmds.push(DrawCmd::FillCircle {
                        center,
                        radius,
                        color,
                    });
                    used_fill = true;
                }
            }
        }

        if used_fill && stroke.is_none() {
            return;
        }

        use crate::svg::ir::{PathCommand, PointF};

        // Kappa for cubic bezier circle approximation
        const KAPPA: f32 = 0.55228475;
        let k = r * KAPPA;

        let mut verbs = Vec::with_capacity(6);
        // Start right
        verbs.push(PathCommand::MoveTo(PointF { x: cx + r, y: cy }));
        // Q1 (Right -> Bottom)
        verbs.push(PathCommand::CubicTo(
            PointF {
                x: cx + r,
                y: cy + k,
            },
            PointF {
                x: cx + k,
                y: cy + r,
            },
            PointF { x: cx, y: cy + r },
        ));
        // Q2 (Bottom -> Left)
        verbs.push(PathCommand::CubicTo(
            PointF {
                x: cx - k,
                y: cy + r,
            },
            PointF {
                x: cx - r,
                y: cy + k,
            },
            PointF { x: cx - r, y: cy },
        ));
        // Q3 (Left -> Top)
        verbs.push(PathCommand::CubicTo(
            PointF {
                x: cx - r,
                y: cy - k,
            },
            PointF {
                x: cx - k,
                y: cy - r,
            },
            PointF { x: cx, y: cy - r },
        ));
        // Q4 (Top -> Right)
        verbs.push(PathCommand::CubicTo(
            PointF {
                x: cx + k,
                y: cy - r,
            },
            PointF {
                x: cx + r,
                y: cy - k,
            },
            PointF { x: cx + r, y: cy },
        ));
        verbs.push(PathCommand::Close);

        let fill = if used_fill { None } else { fill };
        if fill.is_none() && stroke.is_none() {
            return;
        }
        let path = crate::isa::Path2D { verbs };
        self.emit_path_with_style(path, transform, fill, stroke, fill_rule);
    }

    fn emit_ellipse(&mut self, cx: f32, cy: f32, rx: f32, ry: f32) {
        if rx <= 0.0 || ry <= 0.0 {
            return;
        }

        use crate::svg::ir::{PathCommand, PointF};
        const KAPPA: f32 = 0.55228475;
        let kx = rx * KAPPA;
        let ky = ry * KAPPA;

        let mut verbs = Vec::with_capacity(6);
        verbs.push(PathCommand::MoveTo(PointF { x: cx + rx, y: cy }));
        verbs.push(PathCommand::CubicTo(
            PointF {
                x: cx + rx,
                y: cy + ky,
            },
            PointF {
                x: cx + kx,
                y: cy + ry,
            },
            PointF { x: cx, y: cy + ry },
        ));
        verbs.push(PathCommand::CubicTo(
            PointF {
                x: cx - kx,
                y: cy + ry,
            },
            PointF {
                x: cx - rx,
                y: cy + ky,
            },
            PointF { x: cx - rx, y: cy },
        ));
        verbs.push(PathCommand::CubicTo(
            PointF {
                x: cx - rx,
                y: cy - ky,
            },
            PointF {
                x: cx - kx,
                y: cy - ry,
            },
            PointF { x: cx, y: cy - ry },
        ));
        verbs.push(PathCommand::CubicTo(
            PointF {
                x: cx + kx,
                y: cy - ry,
            },
            PointF {
                x: cx + rx,
                y: cy - ky,
            },
            PointF { x: cx + rx, y: cy },
        ));
        verbs.push(PathCommand::Close);

        let path = crate::isa::Path2D { verbs };
        self.emit_path(path);
    }

    fn emit_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        use crate::svg::ir::{PathCommand, PointF};
        let (transform, _fill, stroke, fill_rule) = self.current_paints();

        if let Some(stroke_style) = stroke {
            if stroke_style.width == 1 && stroke_style.cap == crate::isa::LineCap::Butt {
                if let Some(from) = snapped_point(x1, y1, &transform) {
                    if let Some(to) = snapped_point(x2, y2, &transform) {
                        self.cmds.push(DrawCmd::Line {
                            from,
                            to,
                            color: stroke_style.color,
                            width: stroke_style.width,
                        });
                        return;
                    }
                }
            }
        }

        if stroke.is_none() {
            return;
        }
        let verbs = vec![
            PathCommand::MoveTo(PointF { x: x1, y: y1 }),
            PathCommand::LineTo(PointF { x: x2, y: y2 }),
        ];
        let path = crate::isa::Path2D { verbs };
        self.emit_path_with_style(path, transform, None, stroke, fill_rule);
    }

    fn emit_path(&mut self, path: crate::isa::Path2D) {
        let (transform, fill, stroke, fill_rule) = self.current_paints();
        self.emit_path_with_style(path, transform, fill, stroke, fill_rule);
    }

    fn emit_path_with_style(
        &mut self,
        path: crate::isa::Path2D,
        transform: Transform,
        fill: Option<Color>,
        stroke: Option<StrokeStyle>,
        fill_rule: crate::isa::FillRule,
    ) {
        self.cmds.push(DrawCmd::PushTransform { transform });

        let path_arc = alloc::sync::Arc::new(path);

        if let Some(color) = fill {
            self.cmds.push(DrawCmd::FillPath {
                path: path_arc.clone(),
                color,
                fill_rule,
                aa: crate::geometry::EdgeAA::Coverage8,
            });
        }

        if let Some(stroke_style) = stroke {
            self.cmds.push(DrawCmd::StrokePath {
                path: path_arc,
                color: stroke_style.color,
                width: stroke_style.width,
                cap: stroke_style.cap,
                join: stroke_style.join,
                miter_limit: stroke_style.miter_limit,
                aa: crate::geometry::EdgeAA::Coverage8,
            });
        }

        self.cmds.push(DrawCmd::PopTransform);
    }
}

fn apply_opacity(c: Color, opacity: u8) -> Color {
    crate::geometry::Color::new(c.r, c.g, c.b, ((c.a as u16 * opacity as u16) / 255) as u8)
}

fn parse_line_cap(value: &str) -> Option<crate::isa::LineCap> {
    match value.trim() {
        "butt" => Some(crate::isa::LineCap::Butt),
        "round" => Some(crate::isa::LineCap::Round),
        "square" => Some(crate::isa::LineCap::Square),
        _ => None,
    }
}

fn parse_line_join(value: &str) -> Option<crate::isa::LineJoin> {
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
fn stroke_width_to_i32(width: f32) -> Option<i32> {
    if width <= 0.0 {
        None
    } else {
        Some(libm::roundf(width.max(1.0)) as i32)
    }
}

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() <= SNAP_EPS
}

fn snap_to_i32(value: f32) -> Option<i32> {
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

fn snapped_point(x: f32, y: f32, transform: &Transform) -> Option<Point> {
    let (dx, dy) = translation_only(transform)?;
    let sx = snap_to_i32(x + dx as f32)?;
    let sy = snap_to_i32(y + dy as f32)?;
    Some(Point::new(sx, sy))
}

fn snapped_rect(x: f32, y: f32, w: f32, h: f32, transform: &Transform) -> Option<Rect> {
    let (dx, dy) = translation_only(transform)?;
    let sx = snap_to_i32(x + dx as f32)?;
    let sy = snap_to_i32(y + dy as f32)?;
    let sw = snap_to_i32(w)?;
    let sh = snap_to_i32(h)?;
    Some(Rect::new(sx, sy, sw, sh))
}

fn can_use_rect_stroke(stroke: &StrokeStyle) -> bool {
    stroke.cap == crate::isa::LineCap::Butt
        && stroke.join == crate::isa::LineJoin::Miter
        && approx_eq(stroke.miter_limit, 4.0)
}

/// Render SVG string to a raw buffer (ARGB pre-allocation)
/// Useful for asset loading.
pub fn render_to_buffer(xml: &str, width: i32, height: i32, scale: f32) -> alloc::vec::Vec<u32> {
    let mut pixels = alloc::vec![0u32; (width * height) as usize];
    let ptr = pixels.as_mut_ptr() as *mut u8;
    // Create unsafe surface. Ensure lifetime of pixels outlives surface usage.
    let mut surface = unsafe {
        crate::surface::Surface::new(
            ptr,
            (width * height * 4) as usize,
            width as u32,
            height as u32,
            (width * 4) as u32,
        )
    };

    // Parse
    let mut parser = SvgParser::new();
    // let cmds = parser.parse(xml); // Removed unused call

    // Create DrawList
    let mut list = crate::drawlist::DrawList::new();

    // Wrap cmds in PushTransform/PopTransform for scale?
    // We already baked transforms into the primitives in SvgParser (because we are flattening).
    // So we just need a GLOBAL scale transform if requested.
    // Wait, SvgParser starts with Identity.
    // If we want detailed rendering, we should maybe set the initial state of the parser?
    // But `render_to_buffer` takes a `scale`.
    // If I use DrawCmd::PushTransform, my `FillRect`s (scanlines) will get scaled.
    // Since `LowLevelOp` ONLY supports translation, `PushTransform` with scale will BE IGNORED by `lower()`!
    //
    // CRITICAL: `lower()` drops scale.
    // So I MUST apply the global scale INSIDE `SvgParser` or wrap the input SVG in a `<g transform="scale(...)">` ...
    // Or just modify `state_stack` init.
    //
    // Let's modify `parser.parse` to allow injecting initial transform?
    // Or just prepend a group tag? String manipulation is alloc-heavy.
    //
    // Let's manually apply scale to the generated cmds?
    // No, that's hard.
    //
    // Best way: Initialize SvgParser state with the scale.
    // I'll add `set_initial_transform` to SvgParser.

    parser.state_stack[0].transform = Transform::default();
    parser.state_stack[0].transform.m11 = scale;
    parser.state_stack[0].transform.m22 = scale;

    // Re-parse with scale
    let cmds = parser.parse(xml);
    list.commands().extend(cmds);

    crate::raster::execute(&mut surface, &list, false);

    pixels
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawlist::DrawCmd;
    use crate::isa::{LineCap, LineJoin};

    #[test]
    fn parses_stroke_attributes() {
        let xml = r###"
            <svg width="10" height="10">
                <path d="M0 0 L10 0"
                      fill="none"
                      stroke="#000000"
                      stroke-width="2"
                      stroke-linecap="round"
                      stroke-linejoin="bevel"
                      stroke-miterlimit="2" />
            </svg>
        "###;
        let mut parser = SvgParser::new();
        let cmds = parser.parse(xml);
        let mut found = None;
        for cmd in cmds {
            if let DrawCmd::StrokePath {
                cap,
                join,
                miter_limit,
                width,
                ..
            } = cmd
            {
                found = Some((cap, join, miter_limit, width));
                break;
            }
        }
        let (cap, join, miter_limit, width) = found.expect("missing stroke path");
        assert_eq!(cap, LineCap::Round);
        assert_eq!(join, LineJoin::Bevel);
        assert_eq!(width, 2);
        assert!((miter_limit - 2.0).abs() < 0.01);
    }

    #[test]
    fn uses_fill_rect_primitive_for_aligned_rects() {
        let xml = r###"
            <svg width="10" height="10">
                <rect x="1" y="2" width="3" height="4" fill="#ff0000" />
            </svg>
        "###;
        let mut parser = SvgParser::new();
        let cmds = parser.parse(xml);
        let mut found = None;
        for cmd in cmds {
            if let DrawCmd::FillRect { rect, .. } = cmd {
                found = Some(rect);
                break;
            }
        }
        let rect = found.expect("missing fill rect");
        assert_eq!(rect, Rect::new(1, 2, 3, 4));
    }
}
