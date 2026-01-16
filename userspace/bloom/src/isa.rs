use serde::{Deserialize, Serialize};

// Re-export geometry types for use in the ISA
pub use crate::geometry::{Color, Point, Rect, EdgeAA};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BlendMode {
    #[default]
    SrcOver, // Alpha blending (A over B)
    Src,     // Copy (replace target)
    Add,     // Additive blending
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FilterMode {
    #[default]
    Nearest,
    Linear,
}

/// 2D Transform (Translation only for v0)
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub tx: f32,
    pub ty: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform2D {
    pub const fn identity() -> Self {
        Self { tx: 0.0, ty: 0.0 }
    }

    pub const fn translate(tx: f32, ty: f32) -> Self {
        Self { tx, ty }
    }
    
    // Helper to transform a point
    pub fn transform_point(&self, p: Point) -> Point {
        Point::new(
            (p.x as f32 + self.tx) as i32,
            (p.y as f32 + self.ty) as i32,
        )
    }
    
    // Helper to transform a rect (affects origin only for translation)
    pub fn transform_rect(&self, r: Rect) -> Rect {
        Rect::new(
            (r.x() as f32 + self.tx) as i32,
            (r.y() as f32 + self.ty) as i32,
            r.width(),
            r.height(),
        )
    }

    pub fn combine(&self, other: &Transform2D) -> Self {
        Self {
            tx: self.tx + other.tx,
            ty: self.ty + other.ty,
        }
    }
}
