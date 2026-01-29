use alloc::{vec, vec::Vec};

use stem::xml::{parse_f32, parse_length_px, Attributes, Event, XmlReader};

use crate::drawlist::DrawCmd;
use crate::geometry::{Color, Rect, RectF, Transform};

use super::state::{apply_opacity, stroke_width_to_i32, StrokeStyle, SvgState};

pub struct SvgParser {
    state_stack: Vec<SvgState>,
    cmds: Vec<DrawCmd>,
    width: i32,
    height: i32,
    view_box: Option<RectF>,
    viewport: Option<(i32, i32)>,
}

impl SvgParser {
    /// Create a new SVG parser with default state.
    pub fn new() -> Self {
        Self {
            state_stack: vec![SvgState::default()],
            cmds: Vec::new(),
            width: 0,
            height: 0,
            view_box: None,
            viewport: None,
        }
    }

    /// Provide a viewport for viewBox scaling when the SVG omits width/height.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use crate::svg::SvgParser;
    ///
    /// let mut parser = SvgParser::new();
    /// parser.set_viewport(128, 128);
    /// ```
    pub fn set_viewport(&mut self, width: i32, height: i32) {
        self.viewport = Some((width, height));
    }

    /// Set the initial transform applied to the entire document.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use crate::geometry::Transform;
    /// use crate::svg::SvgParser;
    ///
    /// let mut parser = SvgParser::new();
    /// parser.set_initial_transform(Transform::scale(2.0, 2.0));
    /// ```
    pub fn set_initial_transform(&mut self, transform: Transform) {
        self.state_stack[0].transform = transform;
    }

    pub(crate) fn current_state(&self) -> &SvgState {
        self.state_stack.last().expect("State stack empty")
    }

    pub(crate) fn current_state_mut(&mut self) -> &mut SvgState {
        self.state_stack.last_mut().expect("State stack empty")
    }

    pub(crate) fn push_state(&mut self) {
        let current = self.current_state().clone();
        self.state_stack.push(current);
    }

    pub(crate) fn pop_state(&mut self) {
        if self.state_stack.len() > 1 {
            self.state_stack.pop();
        }
    }

    fn reset_for_parse(&mut self) {
        self.cmds.clear();
        let initial_transform = self.state_stack[0].transform;
        self.state_stack.truncate(1);
        self.state_stack[0] = SvgState::default();
        self.state_stack[0].transform = initial_transform;
        self.width = 0;
        self.height = 0;
        self.view_box = None;
    }

    pub fn parse(&mut self, xml: &str) -> Vec<DrawCmd> {
        self.reset_for_parse();

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
                        let vbr = RectF::new(
                            parts[0],
                            parts[1],
                            parts[2],
                            parts[3],
                        );
                        self.view_box = Some(vbr);
                        self.apply_viewbox_transform(parts[0], parts[1], parts[2], parts[3]);
                    }
                }
                self.apply_attributes(attrs);
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

                if rx == 0.0 && ry > 0.0 {
                    rx = ry;
                }
                if ry == 0.0 && rx > 0.0 {
                    ry = rx;
                }

                self.emit_rect(x, y, w, h, rx, ry);
            }
            "line" => {
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
                self.apply_attributes(attrs);
            }
        }
    }

    fn handle_end_element(&mut self, _name: &str) {
        self.pop_state();
    }

    fn apply_viewbox_transform(&mut self, min_x: f32, min_y: f32, width: f32, height: f32) {
        let (mut target_w, mut target_h) = (self.width, self.height);
        if target_w == 0 || target_h == 0 {
            if let Some((viewport_w, viewport_h)) = self.viewport {
                target_w = viewport_w;
                target_h = viewport_h;
            }
        }
        if target_w == 0 || target_h == 0 {
            target_w = width as i32;
            target_h = height as i32;
        }
        if target_w > 0 && target_h > 0 {
            if self.width == 0 {
                self.width = target_w;
            }
            if self.height == 0 {
                self.height = target_h;
            }
            let sx = target_w as f32 / width;
            let sy = target_h as f32 / height;
            let tx = -min_x * sx;
            let ty = -min_y * sy;
            let state = self.current_state_mut();
            state.transform = state.transform.multiply(&Transform::translate(tx, ty));
            state.transform = state.transform.multiply(&Transform::scale(sx, sy));
        }
    }

    pub(crate) fn current_paints(
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

    pub(crate) fn push_cmd(&mut self, cmd: DrawCmd) {
        self.cmds.push(cmd);
    }
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

    #[test]
    fn honors_viewbox_with_viewport() {
        let xml = r###"
            <svg viewBox="0 0 10 10">
                <rect x="0" y="0" width="10" height="10" fill="#00ff00" />
            </svg>
        "###;
        let mut parser = SvgParser::new();
        parser.set_viewport(20, 20);
        let cmds = parser.parse(xml);
        let mut found_scale = None;
        for cmd in cmds {
            if let DrawCmd::PushTransform { transform } = cmd {
                found_scale = Some((transform.m11, transform.m22));
                break;
            }
        }
        let (sx, sy) = found_scale.expect("missing transform");
        assert!((sx - 2.0).abs() < 0.01);
        assert!((sy - 2.0).abs() < 0.01);
    }

    #[test]
    fn ignores_fill_when_set_to_none() {
        let xml = r###"
            <svg width="10" height="10">
                <rect x="0" y="0" width="10" height="10" fill="none" stroke="#000000" />
            </svg>
        "###;
        let mut parser = SvgParser::new();
        let cmds = parser.parse(xml);
        assert!(cmds
            .iter()
            .all(|cmd| !matches!(cmd, DrawCmd::FillRect { .. })));
        assert!(cmds
            .iter()
            .any(|cmd| { matches!(cmd, DrawCmd::StrokePath { .. } | DrawCmd::StrokeRect { .. }) }));
    }

    #[test]
    fn applies_style_opacity_to_fill() {
        let xml = r###"
            <svg width="10" height="10">
                <rect x="0" y="0" width="10" height="10"
                      style="fill:#ff0000;fill-opacity:0.5" />
            </svg>
        "###;
        let mut parser = SvgParser::new();
        let cmds = parser.parse(xml);
        let mut found = None;
        for cmd in cmds {
            if let DrawCmd::FillRect { color, .. } = cmd {
                found = Some(color);
                break;
            }
        }
        let color = found.expect("missing fill rect");
        assert_eq!(color.r, 255);
        assert!(color.a >= 120 && color.a <= 135);
    }
}
