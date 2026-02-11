//! SVG to VIR conversion module
//!
//! This module converts SVG documents into VIR (Vector Intermediate Representation).
//! The conversion process:
//! 1. Parse SVG into a DOM
//! 2. Resolve transforms and styles (compose to world space)
//! 3. Expand shapes into paths
//! 4. Emit VIR elements with complete, explicit state

use crate::geometry::{Color, RectF, Transform};
use crate::svg::ir::{
    FillRule as SvgFillRule, LineCap as SvgLineCap, LineJoin as SvgLineJoin, Paint as SvgPaint,
    Path2D, PathCommand, SvgIrDocument, SvgOp,
};
use crate::vir::*;
use alloc::sync::Arc;
use alloc::vec::Vec;

/// Convert an SVG IR document to VIR
pub fn svg_to_vir(svg: &SvgIrDocument) -> VirDocument {
    let mut doc = VirDocument::new();

    // Transfer metadata
    doc.width = svg.width;
    doc.height = svg.height;
    doc.view_box = svg
        .view_box
        .as_ref()
        .map(|vb| ViewBox::new(vb.x(), vb.y(), vb.width(), vb.height()));

    // Convert each operation to VIR elements
    for op in &svg.ops {
        match op {
            SvgOp::FillPath {
                path,
                paint,
                transform,
                fill_rule,
                opacity,
            } => {
                let vir_path = convert_path(path);
                let vir_paint = convert_paint(paint);
                let vir_transform = convert_transform(transform);
                let vir_fill_rule = convert_fill_rule(fill_rule);

                doc.elements.push(VirElement {
                    path: Arc::new(vir_path),
                    fill: Some(FillStyle {
                        paint: vir_paint,
                        rule: vir_fill_rule,
                    }),
                    stroke: None,
                    transform: vir_transform,
                    opacity: *opacity,
                });
            }
            SvgOp::StrokePath {
                path,
                paint,
                transform,
                width,
                line_cap,
                line_join,
                miter_limit,
                opacity,
            } => {
                let vir_path = convert_path(path);
                let vir_paint = convert_paint(paint);
                let vir_transform = convert_transform(transform);
                let vir_line_cap = convert_line_cap(line_cap);
                let vir_line_join = convert_line_join(line_join);

                doc.elements.push(VirElement {
                    path: Arc::new(vir_path),
                    fill: None,
                    stroke: Some(StrokeStyle {
                        paint: vir_paint,
                        width: *width,
                        line_cap: vir_line_cap,
                        line_join: vir_line_join,
                        miter_limit: *miter_limit,
                    }),
                    transform: vir_transform,
                    opacity: *opacity,
                });
            }
        }
    }

    doc
}

fn convert_path(path: &Path2D) -> VirPath {
    let mut vir_path = VirPath::new();

    for verb in &path.verbs {
        match verb {
            PathCommand::MoveTo(p) => {
                vir_path.move_to(p.x, p.y);
            }
            PathCommand::LineTo(p) => {
                vir_path.line_to(p.x, p.y);
            }
            PathCommand::QuadTo(cp, p) => {
                vir_path.quad_to(cp.x, cp.y, p.x, p.y);
            }
            PathCommand::CubicTo(cp1, cp2, p) => {
                vir_path.cubic_to(cp1.x, cp1.y, cp2.x, cp2.y, p.x, p.y);
            }
            PathCommand::Close => {
                vir_path.close();
            }
        }
    }

    vir_path
}

fn convert_paint(paint: &SvgPaint) -> Paint {
    match paint {
        SvgPaint::Solid(color) => Paint::Solid(convert_color(color)),
    }
}

fn convert_color(color: &Color) -> VirColor {
    VirColor::new(color.r, color.g, color.b, color.a)
}

fn convert_transform(t: &Transform) -> VirTransform {
    // Convert from f32 Transform to f64 VirTransform
    VirTransform {
        a: t.m11 as f64,
        b: t.m12 as f64,
        c: t.m21 as f64,
        d: t.m22 as f64,
        e: t.dx as f64,
        f: t.dy as f64,
    }
}

fn convert_fill_rule(rule: &SvgFillRule) -> FillRule {
    match rule {
        SvgFillRule::NonZero => FillRule::NonZero,
        SvgFillRule::EvenOdd => FillRule::EvenOdd,
    }
}

fn convert_line_cap(cap: &SvgLineCap) -> LineCap {
    match cap {
        SvgLineCap::Butt => LineCap::Butt,
        SvgLineCap::Round => LineCap::Round,
        SvgLineCap::Square => LineCap::Square,
    }
}

fn convert_line_join(join: &SvgLineJoin) -> LineJoin {
    match join {
        SvgLineJoin::Miter => LineJoin::Miter,
        SvgLineJoin::Round => LineJoin::Round,
        SvgLineJoin::Bevel => LineJoin::Bevel,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::ir::PointF;

    #[test]
    fn test_convert_simple_path() {
        let mut path = Path2D {
            verbs: alloc::vec![],
        };
        path.verbs.push(PathCommand::MoveTo(PointF::new(0.0, 0.0)));
        path.verbs
            .push(PathCommand::LineTo(PointF::new(10.0, 10.0)));
        path.verbs.push(PathCommand::Close);

        let vir_path = convert_path(&path);
        assert_eq!(vir_path.segments.len(), 3);

        match vir_path.segments[0] {
            VirSegment::MoveTo(p) => {
                assert_eq!(p.x, 0.0);
                assert_eq!(p.y, 0.0);
            }
            _ => panic!("Expected MoveTo"),
        }
    }

    #[test]
    fn test_convert_fill_rule() {
        assert!(matches!(
            convert_fill_rule(&SvgFillRule::NonZero),
            FillRule::NonZero
        ));
        assert!(matches!(
            convert_fill_rule(&SvgFillRule::EvenOdd),
            FillRule::EvenOdd
        ));
    }
}
