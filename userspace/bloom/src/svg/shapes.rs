use alloc::{sync::Arc, vec, vec::Vec};

use crate::drawlist::DrawCmd;
use crate::geometry::{Color, Transform};

use super::geometry::{can_use_rect_stroke, snapped_point, snapped_rect};
use super::parser::SvgParser;
use super::state::StrokeStyle;

impl SvgParser {
    pub(crate) fn emit_rect(&mut self, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32) {
        use crate::svg::ir::{PathCommand, PointF};

        let rx = rx.min(w * 0.5).max(0.0);
        let ry = ry.min(h * 0.5).max(0.0);

        let (transform, fill, stroke, fill_rule) = self.current_paints();
        let mut used_fill = false;
        let mut used_stroke = false;

        if rx == 0.0 && ry == 0.0 {
            if let Some(rect) = snapped_rect(x, y, w, h, &transform) {
                if let Some(color) = fill {
                    self.push_cmd(DrawCmd::FillRect {
                        rect,
                        color,
                        aa: crate::geometry::EdgeAA::Coverage8,
                    });
                    used_fill = true;
                }
                if let Some(stroke_style) = stroke {
                    if can_use_rect_stroke(&stroke_style) {
                        self.push_cmd(DrawCmd::StrokeRect {
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
            const K: f32 = 0.55228475;
            let kx = rx * K;
            let ky = ry * K;

            verbs.push(PathCommand::MoveTo(PointF { x: x + rx, y }));

            verbs.push(PathCommand::LineTo(PointF { x: x + w - rx, y }));
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

            verbs.push(PathCommand::LineTo(PointF {
                x: x + w,
                y: y + h - ry,
            }));
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

            verbs.push(PathCommand::LineTo(PointF {
                x: x + rx,
                y: y + h,
            }));
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

            verbs.push(PathCommand::LineTo(PointF { x, y: y + ry }));
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

    pub(crate) fn emit_circle(&mut self, cx: f32, cy: f32, r: f32) {
        if r <= 0.0 {
            return;
        }
        let (transform, fill, stroke, fill_rule) = self.current_paints();
        let mut used_fill = false;
        if let Some(center) = snapped_point(cx, cy, &transform) {
            if let Some(radius) = super::geometry::snap_to_i32(r) {
                if let Some(color) = fill {
                    self.push_cmd(DrawCmd::FillCircle {
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

        const KAPPA: f32 = 0.55228475;
        let k = r * KAPPA;

        let mut verbs = Vec::with_capacity(6);
        verbs.push(PathCommand::MoveTo(PointF { x: cx + r, y: cy }));
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

    pub(crate) fn emit_ellipse(&mut self, cx: f32, cy: f32, rx: f32, ry: f32) {
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

    pub(crate) fn emit_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        use crate::svg::ir::{PathCommand, PointF};
        let (transform, _fill, stroke, fill_rule) = self.current_paints();

        if let Some(stroke_style) = stroke {
            if stroke_style.width == 1 && stroke_style.cap == crate::isa::LineCap::Butt {
                if let Some(from) = snapped_point(x1, y1, &transform) {
                    if let Some(to) = snapped_point(x2, y2, &transform) {
                        self.push_cmd(DrawCmd::Line {
                            from: from.into(),
                            to: to.into(),
                            color: stroke_style.color,
                            width: stroke_style.width as f32,
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

    pub(crate) fn emit_path(&mut self, path: crate::isa::Path2D) {
        let (transform, fill, stroke, fill_rule) = self.current_paints();
        self.emit_path_with_style(path, transform, fill, stroke, fill_rule);
    }

    pub(crate) fn emit_path_with_style(
        &mut self,
        path: crate::isa::Path2D,
        transform: Transform,
        fill: Option<Color>,
        stroke: Option<StrokeStyle>,
        fill_rule: crate::isa::FillRule,
    ) {
        self.push_cmd(DrawCmd::PushTransform { transform });

        let path_arc = Arc::new(path);

        if let Some(color) = fill {
            self.push_cmd(DrawCmd::FillPath {
                path: path_arc.clone(),
                color,
                fill_rule,
                aa: crate::geometry::EdgeAA::Coverage8,
            });
        }

        if let Some(stroke_style) = stroke {
            self.push_cmd(DrawCmd::StrokePath {
                path: path_arc,
                color: stroke_style.color,
                width: stroke_style.width as f32,
                cap: stroke_style.cap,
                join: stroke_style.join,
                miter_limit: stroke_style.miter_limit,
                aa: crate::geometry::EdgeAA::Coverage8,
            });
        }

        self.push_cmd(DrawCmd::PopTransform);
    }
}
