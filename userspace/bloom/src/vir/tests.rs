//! Integration tests for the VIR pipeline

#[cfg(test)]
mod tests {
    use crate::tessellate::{tessellate_fill, TessellateConfig};
    use crate::vir::*;
    use alloc::sync::Arc;

    #[test]
    fn test_simple_rect_pipeline() {
        // Create a simple rectangle path
        let mut path = VirPath::new();
        path.move_to(10.0, 10.0);
        path.line_to(50.0, 10.0);
        path.line_to(50.0, 50.0);
        path.line_to(10.0, 50.0);
        path.close();

        // Create VIR element
        let element = VirElement {
            path: Arc::new(path),
            fill: Some(FillStyle {
                paint: Paint::Solid(VirColor::rgb(255, 0, 0)),
                rule: FillRule::NonZero,
            }),
            stroke: None,
            transform: VirTransform::identity(),
            opacity: 1.0,
        };

        // Create document
        let mut doc = VirDocument::new();
        doc.elements.push(element);

        // Convert to DrawList
        let config = TessellateConfig::default();
        let mut drawlist = vir_to_drawlist(&doc, &config);

        // Should have at least one command
        assert!(!drawlist.commands().is_empty());
    }

    #[test]
    fn test_circle_pipeline() {
        // Create a circle using shape expansion
        let circle_path = shapes::circle_to_path(50.0, 50.0, 25.0);

        let element = VirElement {
            path: Arc::new(circle_path),
            fill: Some(FillStyle {
                paint: Paint::Solid(VirColor::rgb(0, 0, 255)),
                rule: FillRule::NonZero,
            }),
            stroke: None,
            transform: VirTransform::identity(),
            opacity: 1.0,
        };

        let mut doc = VirDocument::new();
        doc.elements.push(element);

        let config = TessellateConfig::default();
        let mut drawlist = vir_to_drawlist(&doc, &config);

        assert!(!drawlist.commands().is_empty());
    }

    #[test]
    fn test_stroked_line() {
        // Create a simple line
        let mut path = VirPath::new();
        path.move_to(10.0, 10.0);
        path.line_to(100.0, 100.0);

        let element = VirElement {
            path: Arc::new(path),
            fill: None,
            stroke: Some(StrokeStyle {
                paint: Paint::Solid(VirColor::BLACK),
                width: 2.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                miter_limit: 4.0,
            }),
            transform: VirTransform::identity(),
            opacity: 1.0,
        };

        let mut doc = VirDocument::new();
        doc.elements.push(element);

        let config = TessellateConfig::default();
        let mut drawlist = vir_to_drawlist(&doc, &config);

        assert!(!drawlist.commands().is_empty());
    }

    #[test]
    fn test_transform_scaling() {
        // Test that transforms are applied correctly
        let mut path = VirPath::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 0.0);
        path.line_to(10.0, 10.0);
        path.close();

        // Apply 2x scale
        let transform = VirTransform::scale(2.0, 2.0);

        let config = TessellateConfig::default();
        let tessellated = tessellate_fill(&path, &transform, &config);

        // Check that vertices are scaled
        assert!(!tessellated.vertices.is_empty());

        // The second point should be at (20, 0) after 2x scale
        if tessellated.vertices.len() > 1 {
            let p = tessellated.vertices[1];
            assert!((p.x - 20.0).abs() < 0.1);
            assert!(p.y.abs() < 0.1);
        }
    }

    #[test]
    fn test_svg_to_vir_conversion() {
        use crate::geometry::{Color, Transform};
        use crate::svg::ir::{
            FillRule as SvgFillRule, Paint as SvgPaint, Path2D, PathCommand as SvgPathCommand,
            PointF, SvgIrDocument, SvgOp,
        };

        // Create a simple SVG IR
        let mut svg = SvgIrDocument {
            width: Some(100.0),
            height: Some(100.0),
            view_box: None,
            ops: alloc::vec![],
        };

        let mut svg_path = Path2D {
            verbs: alloc::vec![],
        };
        svg_path
            .verbs
            .push(SvgPathCommand::MoveTo(PointF::new(0.0, 0.0)));
        svg_path
            .verbs
            .push(SvgPathCommand::LineTo(PointF::new(10.0, 0.0)));
        svg_path
            .verbs
            .push(SvgPathCommand::LineTo(PointF::new(10.0, 10.0)));
        svg_path.verbs.push(SvgPathCommand::Close);

        svg.ops.push(SvgOp::FillPath {
            path: Arc::new(svg_path),
            paint: SvgPaint::Solid(Color::rgb(255, 0, 0)),
            transform: Transform::identity(),
            fill_rule: SvgFillRule::NonZero,
            opacity: 1.0,
        });

        // Convert to VIR
        let vir = svg_to_vir(&svg);

        assert_eq!(vir.width, Some(100.0));
        assert_eq!(vir.height, Some(100.0));
        assert_eq!(vir.elements.len(), 1);
    }

    #[test]
    fn test_quadratic_curve_flattening() {
        // Test that quadratic curves are flattened correctly
        let mut path = VirPath::new();
        path.move_to(0.0, 0.0);
        path.quad_to(50.0, 100.0, 100.0, 0.0); // Parabolic arc

        let transform = VirTransform::identity();
        let config = TessellateConfig {
            tolerance: 0.5,
            apply_transform: true,
        };

        let tessellated = tessellate_fill(&path, &transform, &config);

        // Should have multiple segments (flattened curve)
        assert!(tessellated.vertices.len() > 2);
    }

    #[test]
    fn test_cubic_curve_flattening() {
        // Test that cubic curves are flattened correctly
        let mut path = VirPath::new();
        path.move_to(0.0, 0.0);
        path.cubic_to(0.0, 50.0, 100.0, 50.0, 100.0, 0.0); // S-curve

        let transform = VirTransform::identity();
        let config = TessellateConfig {
            tolerance: 0.5,
            apply_transform: true,
        };

        let tessellated = tessellate_fill(&path, &transform, &config);

        // Should have multiple segments (flattened curve)
        assert!(tessellated.vertices.len() > 2);
    }
}
