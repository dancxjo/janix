//! Vector Intermediate Representation (VIR)
//!
//! VIR is a minimal, renderer-friendly representation that serves as the
//! canonical format for all vector graphics in Thing-OS. SVG and other
//! vector formats are converted to VIR before rendering.
//!
//! Design principles:
//! - Explicit, not implicit (no inherited state)
//! - High precision until tessellation (f64 for transforms)
//! - Deterministic (same input → same output)
//! - Renderer-agnostic (no pixel assumptions)

use alloc::sync::Arc;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

pub mod dump;
pub mod examples;
pub mod render;
pub mod shapes;
pub mod svg_convert;
pub mod transform;
pub mod types;

pub use render::vir_to_drawlist;
pub use svg_convert::svg_to_vir;
pub use transform::*;
pub use types::*;

/// A complete vector document ready for tessellation and rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirDocument {
    /// Optional width hint in internal units
    pub width: Option<f32>,
    /// Optional height hint in internal units
    pub height: Option<f32>,
    /// Optional viewBox defining coordinate space
    pub view_box: Option<ViewBox>,
    /// All drawable elements in document
    pub elements: Vec<VirElement>,
}

/// A single drawable element with its complete state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirElement {
    /// The path geometry
    pub path: Arc<VirPath>,
    /// Fill style (if any)
    pub fill: Option<FillStyle>,
    /// Stroke style (if any)
    pub stroke: Option<StrokeStyle>,
    /// Transform from element space to world space (pre-composed)
    pub transform: VirTransform,
    /// Element opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
}

impl VirDocument {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            view_box: None,
            elements: Vec::new(),
        }
    }
}

impl Default for VirDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_document() {
        let doc = VirDocument::new();
        assert_eq!(doc.elements.len(), 0);
        assert!(doc.width.is_none());
        assert!(doc.height.is_none());
        assert!(doc.view_box.is_none());
    }

    #[test]
    fn test_create_simple_rect() {
        let mut path = VirPath::new();
        path.move_to(0.0, 0.0);
        path.line_to(100.0, 0.0);
        path.line_to(100.0, 100.0);
        path.line_to(0.0, 100.0);
        path.close();

        let elem = VirElement {
            path: Arc::new(path),
            fill: Some(FillStyle {
                paint: Paint::Solid(VirColor::rgb(255, 0, 0)),
                rule: FillRule::NonZero,
            }),
            stroke: None,
            transform: VirTransform::identity(),
            opacity: 1.0,
        };

        assert_eq!(elem.path.segments.len(), 5);
    }
}
