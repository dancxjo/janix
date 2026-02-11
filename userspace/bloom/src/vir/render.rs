//! VIR to DrawList bridge
//!
//! Converts tessellated VIR paths into DrawList commands for rasterization.

use crate::drawlist::{DrawCmd, DrawList};
use crate::geometry::{Color, Point, Rect};
use crate::isa::EdgeAA;
use crate::tessellate::{tessellate_fill, tessellate_stroke, TessellateConfig, TessellatedPath};
use crate::vir::{FillRule, Paint, VirColor, VirDocument, VirElement, VirTransform};
use alloc::vec::Vec;

/// Convert a VIR document to a DrawList
pub fn vir_to_drawlist(doc: &VirDocument, config: &TessellateConfig) -> DrawList {
    let mut list = DrawList::new();

    for element in &doc.elements {
        convert_element(element, config, &mut list);
    }

    list
}

fn convert_element(element: &VirElement, config: &TessellateConfig, list: &mut DrawList) {
    // Handle fill
    if let Some(fill) = &element.fill {
        let tessellated = tessellate_fill(&element.path, &element.transform, config);

        // Convert to polygon drawing commands
        for contour in &tessellated.contours {
            let start = contour.start;
            let end = start + contour.count;
            let points = &tessellated.vertices[start..end];

            if points.len() < 3 {
                continue; // Need at least 3 points for a polygon
            }

            // Convert VirPoints to Points with rounding for better accuracy
            let draw_points: Vec<Point> = points
                .iter()
                .map(|p| Point::new(libm::roundf(p.x) as i32, libm::roundf(p.y) as i32))
                .collect();

            // For now, use a simple polygon fill
            // In a full implementation, we'd respect fill rules properly
            let color = match fill.paint {
                Paint::Solid(c) => convert_color(&c, element.opacity),
            };

            // Create a bounding box for the polygon
            let (min_x, min_y, max_x, max_y) = bounding_box(&draw_points);
            let bbox = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);

            // For simplicity, draw as a filled rect for now
            // A proper implementation would tessellate to triangles
            list.commands().push(DrawCmd::FillRect {
                rect: bbox,
                color,
                aa: EdgeAA::Coverage8,
            });
        }
    }

    // Handle stroke
    if let Some(stroke) = &element.stroke {
        let stroke_outline = tessellate_stroke(&element.path, &element.transform, stroke, config);

        // Convert stroke outline to fill (it's already expanded)
        for contour in &stroke_outline.contours {
            let start = contour.start;
            let end = start + contour.count;
            let points = &stroke_outline.vertices[start..end];

            if points.len() < 3 {
                continue;
            }

            let draw_points: Vec<Point> = points
                .iter()
                .map(|p| Point::new(libm::roundf(p.x) as i32, libm::roundf(p.y) as i32))
                .collect();

            let color = match stroke.paint {
                Paint::Solid(c) => convert_color(&c, element.opacity),
            };

            let (min_x, min_y, max_x, max_y) = bounding_box(&draw_points);
            let bbox = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);

            list.commands().push(DrawCmd::FillRect {
                rect: bbox,
                color,
                aa: EdgeAA::Coverage8,
            });
        }
    }
}

fn convert_color(c: &VirColor, opacity: f32) -> Color {
    let alpha = ((c.a as f32) * opacity.min(1.0).max(0.0)).min(255.0) as u8;
    Color::new(c.r, c.g, c.b, alpha)
}

fn bounding_box(points: &[Point]) -> (i32, i32, i32, i32) {
    if points.is_empty() {
        return (0, 0, 0, 0);
    }

    let mut min_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_x = points[0].x;
    let mut max_y = points[0].y;

    for p in points {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }

    (min_x, min_y, max_x, max_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vir::{FillRule, FillStyle, VirColor, VirPath};
    use alloc::sync::Arc;

    #[test]
    fn test_vir_to_drawlist_simple() {
        let mut doc = VirDocument::new();

        let mut path = VirPath::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 0.0);
        path.line_to(10.0, 10.0);
        path.line_to(0.0, 10.0);
        path.close();

        doc.elements.push(VirElement {
            path: Arc::new(path),
            fill: Some(FillStyle {
                paint: Paint::Solid(VirColor::rgb(255, 0, 0)),
                rule: FillRule::NonZero,
            }),
            stroke: None,
            transform: VirTransform::identity(),
            opacity: 1.0,
        });

        let config = TessellateConfig::default();
        let list = vir_to_drawlist(&doc, &config);

        assert!(!list.commands().is_empty());
    }
}
