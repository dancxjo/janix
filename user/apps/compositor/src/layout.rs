#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

pub fn center_text(bounds: Rect, text_size: Size) -> Point {
    let center_x = bounds.x + (bounds.width as i32 / 2);
    let center_y = bounds.y + (bounds.height as i32 / 2);
    
    let text_half_w = text_size.width as i32 / 2;
    let text_half_h = text_size.height as i32 / 2;
    
    Point {
        x: center_x - text_half_w,
        y: center_y - text_half_h,
    }
}
