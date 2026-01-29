use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Point> for PointF {
    fn from(p: Point) -> Self {
        Self::new(p.x as f32, p.y as f32)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub const fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SizeF {
    pub width: f32,
    pub height: f32,
}

impl SizeF {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RectF {
    pub origin: PointF,
    pub size: SizeF,
}

impl RectF {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            origin: PointF::new(x, y),
            size: SizeF::new(width, height),
        }
    }

    pub fn x(&self) -> f32 {
        self.origin.x
    }
    pub fn y(&self) -> f32 {
        self.origin.y
    }
    pub fn width(&self) -> f32 {
        self.size.width
    }
    pub fn height(&self) -> f32 {
        self.size.height
    }

    pub fn intersection(&self, other: &RectF) -> Option<RectF> {
        let x0 = self.x().max(other.x());
        let y0 = self.y().max(other.y());
        let x1 = (self.x() + self.width()).min(other.x() + other.width());
        let y1 = (self.y() + self.height()).min(other.y() + other.height());

        if x1 > x0 && y1 > y0 {
            Some(RectF::new(x0, y0, x1 - x0, y1 - y0))
        } else {
            None
        }
    }
}

impl Rect {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn x(&self) -> i32 {
        self.origin.x
    }
    pub fn y(&self) -> i32 {
        self.origin.y
    }
    pub fn width(&self) -> i32 {
        self.size.width
    }
    pub fn height(&self) -> i32 {
        self.size.height
    }


    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x0 = self.x().max(other.x());
        let y0 = self.y().max(other.y());
        let x1 = (self.x() + self.width()).min(other.x() + other.width());
        let y1 = (self.y() + self.height()).min(other.y() + other.height());

        if x1 > x0 && y1 > y0 {
            Some(Rect::new(x0, y0, x1 - x0, y1 - y0))
        } else {
            None
        }
    }

    /// Create a rectangle representing full screen bounds.
    #[inline]
    pub const fn full(width: i32, height: i32) -> Self {
        Self {
            origin: Point::new(0, 0),
            size: Size::new(width, height),
        }
    }

    /// Check if the rectangle is empty (zero or negative area).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.size.width <= 0 || self.size.height <= 0
    }

    /// Normalize the rectangle to ensure positive dimensions.
    /// Handles negative width/height by adjusting x/y.
    #[inline]
    pub fn normalize(self) -> Self {
        let (x, w) = if self.size.width < 0 {
            (self.origin.x + self.size.width, -self.size.width)
        } else {
            (self.origin.x, self.size.width)
        };
        let (y, h) = if self.size.height < 0 {
            (self.origin.y + self.size.height, -self.size.height)
        } else {
            (self.origin.y, self.size.height)
        };
        Self::new(x, y, w, h)
    }

    /// Clip this rectangle to the given bounds.
    /// Returns an empty rect if completely outside bounds.
    #[inline]
    pub fn clip(self, bounds: Rect) -> Self {
        let x0 = self.x().max(bounds.x());
        let y0 = self.y().max(bounds.y());
        let x1 = (self.x() + self.width()).min(bounds.x() + bounds.width());
        let y1 = (self.y() + self.height()).min(bounds.y() + bounds.height());

        if x1 <= x0 || y1 <= y0 {
            Self::default()
        } else {
            Self::new(x0, y0, x1 - x0, y1 - y0)
        }
    }

    /// Compute the union (bounding box) of two rectangles.
    #[inline]
    pub fn union(self, other: Rect) -> Self {
        if self.is_empty() {
            return other;
        }
        if other.is_empty() {
            return self;
        }

        let x0 = self.x().min(other.x());
        let y0 = self.y().min(other.y());
        let x1 = (self.x() + self.width()).max(other.x() + other.width());
        let y1 = (self.y() + self.height()).max(other.y() + other.height());

        Self::new(x0, y0, x1 - x0, y1 - y0)
    }

    /// Compute the intersection of two rectangles.
    /// Returns an empty rect if they don't overlap.
    #[inline]
    pub fn intersect(self, other: Rect) -> Self {
        self.intersection(&other).unwrap_or_default()
    }

    /// Expand the rectangle by `px` pixels in all directions.
    #[inline]
    pub fn expand(self, px: i32) -> Self {
        Self::new(
            self.x() - px,
            self.y() - px,
            self.width() + px * 2,
            self.height() + px * 2,
        )
    }

    /// Check if this rectangle touches or overlaps another.
    /// Touching means edges are adjacent (within 1 pixel).
    #[inline]
    pub fn touches_or_overlaps(self, other: Rect) -> bool {
        // Expand by 1 to catch adjacent edges
        let a = self.expand(1);
        !a.intersect(other).is_empty()
    }

    /// Compute the area of the rectangle.
    #[inline]
    pub fn area(self) -> i64 {
        if self.is_empty() {
            0
        } else {
            self.width() as i64 * self.height() as i64
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn to_u32(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub const fn from_u32(val: u32) -> Self {
        // Assume 0xAARRGGBB format
        let a = ((val >> 24) & 0xFF) as u8;
        let r = ((val >> 16) & 0xFF) as u8;
        let g = ((val >> 8) & 0xFF) as u8;
        let b = (val & 0xFF) as u8;
        Self { r, g, b, a }
    }

    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform {
    pub const fn identity() -> Self {
        Self {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub fn translate(dx: f32, dy: f32) -> Self {
        Self {
            dx,
            dy,
            ..Self::identity()
        }
    }

    pub fn rotate_degrees(angle: f32) -> Self {
        let rad = angle.to_radians();
        let c = libm::cosf(rad);
        let s = libm::sinf(rad);
        Self {
            m11: c,
            m12: -s,
            m21: s,
            m22: c,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            m11: sx,
            m22: sy,
            ..Self::identity()
        }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            m11: self.m11 * other.m11 + self.m12 * other.m21,
            m12: self.m11 * other.m12 + self.m12 * other.m22,
            m21: self.m21 * other.m11 + self.m22 * other.m21,
            m22: self.m21 * other.m12 + self.m22 * other.m22,
            dx: self.m11 * other.dx + self.m12 * other.dy + self.dx,
            dy: self.m21 * other.dx + self.m22 * other.dy + self.dy,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeAA {
    #[default]
    None,
    Coverage8,
}
