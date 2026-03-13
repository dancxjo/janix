//! VIR Pipeline Examples and Demos
//!
//! Demonstrates the complete VIR pipeline for common use cases.

use crate::drawlist::DrawList;
use crate::tessellate::TessellateConfig;
use crate::vir::*;
use alloc::sync::Arc;

/// Create a simple filled rectangle using VIR
pub fn example_filled_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    r: u8,
    g: u8,
    b: u8,
) -> VirDocument {
    let mut doc = VirDocument::new();

    let rect_path = shapes::rect_to_path(x, y, width, height, 0.0, 0.0);

    doc.elements.push(VirElement {
        path: Arc::new(rect_path),
        fill: Some(FillStyle {
            paint: Paint::Solid(VirColor::rgb(r, g, b)),
            rule: FillRule::NonZero,
        }),
        stroke: None,
        transform: VirTransform::identity(),
        opacity: 1.0,
    });

    doc
}

/// Create a filled circle using VIR
pub fn example_filled_circle(cx: f32, cy: f32, radius: f32, r: u8, g: u8, b: u8) -> VirDocument {
    let mut doc = VirDocument::new();

    let circle_path = shapes::circle_to_path(cx, cy, radius);

    doc.elements.push(VirElement {
        path: Arc::new(circle_path),
        fill: Some(FillStyle {
            paint: Paint::Solid(VirColor::rgb(r, g, b)),
            rule: FillRule::NonZero,
        }),
        stroke: None,
        transform: VirTransform::identity(),
        opacity: 1.0,
    });

    doc
}

/// Create a stroked line using VIR
pub fn example_stroked_line(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    width: f32,
    r: u8,
    g: u8,
    b: u8,
) -> VirDocument {
    let mut doc = VirDocument::new();

    let line_path = shapes::line_to_path(x1, y1, x2, y2);

    doc.elements.push(VirElement {
        path: Arc::new(line_path),
        fill: None,
        stroke: Some(StrokeStyle {
            paint: Paint::Solid(VirColor::rgb(r, g, b)),
            width,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            miter_limit: 4.0,
        }),
        transform: VirTransform::identity(),
        opacity: 1.0,
    });

    doc
}

/// Create a complex path with cubic bezier curves
pub fn example_bezier_path() -> VirDocument {
    let mut doc = VirDocument::new();

    let mut path = VirPath::new();

    // Create an S-curve
    path.move_to(10.0, 50.0);
    path.cubic_to(10.0, 10.0, 90.0, 10.0, 90.0, 50.0);
    path.cubic_to(90.0, 90.0, 10.0, 90.0, 10.0, 50.0);
    path.close();

    doc.elements.push(VirElement {
        path: Arc::new(path),
        fill: Some(FillStyle {
            paint: Paint::Solid(VirColor::rgb(0, 128, 255)),
            rule: FillRule::NonZero,
        }),
        stroke: None,
        transform: VirTransform::identity(),
        opacity: 1.0,
    });

    doc
}

/// Render a VIR document to a DrawList with default settings
pub fn render_vir_default(doc: &VirDocument) -> DrawList {
    let config = TessellateConfig::default();
    vir_to_drawlist(doc, &config)
}

/// Render a VIR document with high quality settings
pub fn render_vir_high_quality(doc: &VirDocument) -> DrawList {
    let config = TessellateConfig {
        tolerance: 0.1, // Higher quality (smaller tolerance)
        apply_transform: true,
    };
    vir_to_drawlist(doc, &config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_rect() {
        let doc = example_filled_rect(10.0, 10.0, 100.0, 50.0, 255, 0, 0);
        assert_eq!(doc.elements.len(), 1);

        let mut drawlist = render_vir_default(&doc);
        assert!(!drawlist.commands().is_empty());
    }

    #[test]
    fn test_example_circle() {
        let doc = example_filled_circle(50.0, 50.0, 25.0, 0, 255, 0);
        assert_eq!(doc.elements.len(), 1);

        let mut drawlist = render_vir_default(&doc);
        assert!(!drawlist.commands().is_empty());
    }

    #[test]
    fn test_example_line() {
        let doc = example_stroked_line(0.0, 0.0, 100.0, 100.0, 2.0, 0, 0, 255);
        assert_eq!(doc.elements.len(), 1);

        let mut drawlist = render_vir_default(&doc);
        assert!(!drawlist.commands().is_empty());
    }

    #[test]
    fn test_quality_comparison() {
        let doc = example_bezier_path();

        let mut default_list = render_vir_default(&doc);
        let mut hq_list = render_vir_high_quality(&doc);

        // High quality should potentially have more commands due to finer tessellation
        assert!(!default_list.commands().is_empty());
        assert!(!hq_list.commands().is_empty());
    }
}
