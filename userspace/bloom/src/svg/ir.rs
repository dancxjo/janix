use crate::geometry::{Rect, Color, Transform};
pub use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
pub use crate::isa::{Path2D, PathVerb as PathCommand, PointF, FillRule, LineCap, LineJoin};

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
