#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn union(&self, other: &Rect) -> Rect {
        if self.w == 0 || self.h == 0 {
            return *other;
        }
        if other.w == 0 || other.h == 0 {
            return *self;
        }

        let x0 = core::cmp::min(self.x, other.x);
        let y0 = core::cmp::min(self.y, other.y);
        let x1 = core::cmp::max(self.x + self.w, other.x + other.w);
        let y1 = core::cmp::max(self.y + self.h, other.y + other.h);

        Rect {
            x: x0,
            y: y0,
            w: x1 - x0,
            h: y1 - y0,
        }
    }

    pub fn inflate(&self, amount: u32) -> Rect {
        if self.w == 0 || self.h == 0 {
            return *self;
        }
        let x = self.x.saturating_sub(amount);
        let y = self.y.saturating_sub(amount);
        let w = self.w + amount * 2;
        let h = self.h + amount * 2;
        Rect { x, y, w, h }
    }

    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
         let x0 = core::cmp::max(self.x, other.x);
         let y0 = core::cmp::max(self.y, other.y);
         let x1 = core::cmp::min(self.x + self.w, other.x + other.w);
         let y1 = core::cmp::min(self.y + self.h, other.y + other.h);

         if x1 > x0 && y1 > y0 {
             Some(Rect {
                 x: x0,
                 y: y0,
                 w: x1 - x0,
                 h: y1 - y0,
             })
         } else {
             None
         }
    }
}
