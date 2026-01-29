use serde::{Deserialize, Serialize};

// Re-export geometry types for use in the ISA
pub use crate::geometry::{Color, EdgeAA, Point, Rect};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BlendMode {
    #[default]
    SrcOver, // Alpha blending (A over B)
    Src, // Copy (replace target)
    Add, // Additive blending
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FilterMode {
    #[default]
    Nearest,
    Linear,
}

/// 2D Affine Transform
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::identity()
    }
}

#[allow(dead_code)]
impl Transform2D {
    pub const fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }

    pub const fn translate(tx: f32, ty: f32) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx,
            ty,
        }
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            tx: 0.0,
            ty: 0.0,
        }
    }

    pub fn rotate(angle_rad: f32) -> Self {
        let c = libm::cosf(angle_rad);
        let s = libm::sinf(angle_rad);
        Self {
            a: c,
            b: s,
            c: -s,
            d: c,
            tx: 0.0,
            ty: 0.0,
        }
    }

    pub fn combine(&self, other: &Transform2D) -> Self {
        // self * other (apply other then self?)
        // Usually multiply: result = self x other.
        // If current is T1, new is T2. transform(p) = T1(T2(p)) if T2 applied first.
        // User said "current transform stack to all geometry".
        // Matrix mul:
        // [a b tx] * [oa ob otx]
        // [c d ty]   [oc od oty]
        // [0 0 1 ]   [0  0  1  ]
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            tx: self.a * other.tx + self.b * other.ty + self.tx,
            ty: self.c * other.tx + self.d * other.ty + self.ty,
        }
    }

    pub fn transform_point(&self, p: Point) -> Point {
        let x = p.x as f32;
        let y = p.y as f32;
        Point::new(
            (self.a * x + self.c * y + self.tx) as i32,
            (self.b * x + self.d * y + self.ty) as i32,
        )
    }

    pub fn transform_point_f(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.a * x + self.c * y + self.tx,
            self.b * x + self.d * y + self.ty,
        )
    }

    // Helper to transform a rect (affects origin and size approximates)
    // Note: Axis-aligned rect transform by general matrix results in polygon.
    // This returns bounding box of transformed corners.
    pub fn transform_rect(&self, r: Rect) -> Rect {
        let x = r.x() as f32;
        let y = r.y() as f32;
        let w = r.width() as f32;
        let h = r.height() as f32;

        let (x1, y1) = self.transform_point_f(x, y);
        let (x2, y2) = self.transform_point_f(x + w, y);
        let (x3, y3) = self.transform_point_f(x, y + h);
        let (x4, y4) = self.transform_point_f(x + w, y + h);

        let min_x = x1.min(x2).min(x3).min(x4);
        let max_x = x1.max(x2).max(x3).max(x4);
        let min_y = y1.min(y2).min(y3).min(y4);
        let max_y = y1.max(y2).max(y3).max(y4);

        Rect::new(
            min_x as i32,
            min_y as i32,
            (max_x - min_x) as i32,
            (max_y - min_y) as i32,
        )
    }
}

// Path Types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PathVerb {
    MoveTo(PointF),
    LineTo(PointF),
    QuadTo(PointF, PointF),          // control, end
    CubicTo(PointF, PointF, PointF), // control1, control2, end
    Close,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Path2D {
    pub verbs: alloc::vec::Vec<PathVerb>,
}

impl From<abi::drawlist::FillRule> for FillRule {
    fn from(f: abi::drawlist::FillRule) -> Self {
        match f {
            abi::drawlist::FillRule::NonZero => Self::NonZero,
            abi::drawlist::FillRule::EvenOdd => Self::EvenOdd,
        }
    }
}

impl From<abi::drawlist::PointF> for PointF {
    fn from(p: abi::drawlist::PointF) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<Point> for PointF {
    fn from(p: Point) -> Self {
        Self {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

impl From<abi::drawlist::PathVerb> for PathVerb {
    fn from(v: abi::drawlist::PathVerb) -> Self {
        match v {
            abi::drawlist::PathVerb::MoveTo(p) => Self::MoveTo(p.into()),
            abi::drawlist::PathVerb::LineTo(p) => Self::LineTo(p.into()),
            abi::drawlist::PathVerb::QuadTo(c, p) => Self::QuadTo(c.into(), p.into()),
            abi::drawlist::PathVerb::CubicTo(c1, c2, p) => {
                Self::CubicTo(c1.into(), c2.into(), p.into())
            }
            abi::drawlist::PathVerb::Close => Self::Close,
        }
    }
}
