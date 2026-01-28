use crate::geometry::{Color, Rect, Transform};
pub use crate::isa::{FillRule, LineCap, LineJoin, Path2D, PathVerb as PathCommand, PointF};
pub use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SvgIrDocument {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub view_box: Option<Rect>,
    pub ops: Vec<SvgOp>,
}

#[derive(Debug, Clone)]
pub enum SvgOp {
    FillPath {
        path: alloc::sync::Arc<Path2D>,
        paint: Paint,
        transform: Transform,
        fill_rule: FillRule,
        opacity: f32,
    },
    StrokePath {
        path: alloc::sync::Arc<Path2D>,
        paint: Paint,
        transform: Transform,
        width: f32,
        line_cap: LineCap,
        line_join: LineJoin,
        miter_limit: f32,
        opacity: f32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Paint {
    Solid(Color),
}
