use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }

    pub fn intersect(&self, other: Rect) -> Rect {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.w as i32).min(other.x + other.w as i32);
        let y2 = (self.y + self.h as i32).min(other.y + other.h as i32);

        if x2 > x1 && y2 > y1 {
            Rect {
                x: x1,
                y: y1,
                w: (x2 - x1) as u32,
                h: (y2 - y1) as u32,
            }
        } else {
            Rect { x: 0, y: 0, w: 0, h: 0 }
        }
    }

    pub fn union(a: Rect, b: Rect) -> Rect {
        if a.w == 0 || a.h == 0 { return b; }
        if b.w == 0 || b.h == 0 { return a; }

        let x1 = a.x.min(b.x);
        let y1 = a.y.min(b.y);
        let x2 = (a.x + a.w as i32).max(b.x + b.w as i32);
        let y2 = (a.y + a.h as i32).max(b.y + b.h as i32);

        Rect {
            x: x1,
            y: y1,
            w: (x2 - x1) as u32,
            h: (y2 - y1) as u32,
        }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.x < self.x + self.w as i32 &&
        p.y >= self.y && p.y < self.y + self.h as i32
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub buttons: u16,
}
