#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    /// An empty rectangle (zero dimensions).
    pub fn empty() -> Self {
        Rect { x: 0, y: 0, w: 0, h: 0 }
    }

    /// Returns true if this rectangle has zero area.
    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }

    /// Compute the intersection of two rectangles. Returns an empty Rect if disjoint.
    pub fn intersect(self, other: Rect) -> Rect {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x.saturating_add(self.w as i32)).min(other.x.saturating_add(other.w as i32));
        let y1 = (self.y.saturating_add(self.h as i32)).min(other.y.saturating_add(other.h as i32));
        let w = (x1 - x0).max(0) as u32;
        let h = (y1 - y0).max(0) as u32;
        Rect { x: x0, y: y0, w, h }
    }

    pub fn union(a: Rect, b: Rect) -> Rect {
        let x1 = a.x.min(b.x);
        let y1 = a.y.min(b.y);
        let x2 = (a.x + (a.w as i32)).max(b.x + (b.w as i32));
        let y2 = (a.y + (a.h as i32)).max(b.y + (b.h as i32));
        Rect {
            x: x1,
            y: y1,
            w: (x2 - x1) as u32,
            h: (y2 - y1) as u32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_disjoint() {
        let a = Rect { x: 0, y: 0, w: 10, h: 10 };
        let b = Rect { x: 20, y: 0, w: 10, h: 10 };
        let i = a.intersect(b);
        assert!(i.is_empty());
        assert_eq!(i.w, 0);
        assert_eq!(i.h, 0);
    }

    #[test]
    fn test_intersect_overlap() {
        let a = Rect { x: 0, y: 0, w: 20, h: 20 };
        let b = Rect { x: 10, y: 10, w: 20, h: 20 };
        let i = a.intersect(b);
        assert!(!i.is_empty());
        assert_eq!(i, Rect { x: 10, y: 10, w: 10, h: 10 });
    }

    #[test]
    fn test_intersect_contained() {
        let a = Rect { x: 0, y: 0, w: 100, h: 100 };
        let b = Rect { x: 10, y: 10, w: 10, h: 10 };
        let i = a.intersect(b);
        assert_eq!(i, b);
    }
}
