extern crate alloc;

#[derive(Clone, Copy, Debug)]
pub struct CursorState {
    pub x: i32,
    pub y: i32,
}

impl CursorState {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn apply_move(&mut self, dx: i16, dy: i16, max_w: i32, max_h: i32) -> bool {
        let mut nx = self.x.saturating_add(dx as i32);
        let mut ny = self.y.saturating_add(dy as i32);
        if nx < 0 {
            nx = 0;
        }
        if ny < 0 {
            ny = 0;
        }
        if max_w > 0 {
            nx = nx.min(max_w - 1);
        }
        if max_h > 0 {
            ny = ny.min(max_h - 1);
        }
        let moved = nx != self.x || ny != self.y;
        self.x = nx;
        self.y = ny;
        moved
    }
}
