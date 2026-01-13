use crate::drawlist::{DrawCmd, DrawList};
use crate::surface::Surface;

pub fn execute(surface: &mut Surface, list: &DrawList) {
    for cmd in list.iter() {
        match *cmd {
            DrawCmd::Clear { xrgb } => clear(surface, xrgb),
            DrawCmd::Rect { x, y, w, h, xrgb } => fill_rect(surface, x, y, w, h, xrgb),
            DrawCmd::Line { x0, y0, x1, y1, xrgb } => line(surface, x0, y0, x1, y1, xrgb),
        }
    }
}

pub fn clear(surface: &mut Surface, xrgb: u32) {
    fill_rect(surface, 0, 0, surface.width(), surface.height(), xrgb);
}

pub fn fill_rect(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, xrgb: u32) {
    if w <= 0 || h <= 0 {
        return;
    }

    let mut x0 = x;
    let mut y0 = y;
    let mut x1 = x + w;
    let mut y1 = y + h;

    if x0 < 0 { x0 = 0; }
    if y0 < 0 { y0 = 0; }
    if x1 > surface.width() { x1 = surface.width(); }
    if y1 > surface.height() { y1 = surface.height(); }

    for yy in y0..y1 {
        for xx in x0..x1 {
            surface.put_px(xx, yy, xrgb);
        }
    }
}

pub fn line(surface: &mut Surface, mut x0: i32, mut y0: i32, x1: i32, y1: i32, xrgb: u32) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        surface.put_px(x0, y0, xrgb);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = err * 2;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
