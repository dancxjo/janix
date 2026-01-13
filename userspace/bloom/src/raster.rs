use crate::drawlist::{DrawCmd, DrawList};
use crate::surface::Surface;

pub fn execute(surface: &mut Surface, list: &DrawList) {
    for cmd in list.iter() {
        match *cmd {
            DrawCmd::Clear { xrgb } => clear(surface, xrgb),
            DrawCmd::Rect { x, y, w, h, xrgb } => fill_rect(surface, x, y, w, h, xrgb),
            DrawCmd::Line { x0, y0, x1, y1, xrgb } => line(surface, x0, y0, x1, y1, xrgb),
            DrawCmd::BlitImage { image, x, y } => {
                 blit_image(surface, &image, x, y);
            }
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

fn blit_image(surface: &mut Surface, image: &crate::asset::Image, dst_x: i32, dst_y: i32) {
    // Basic 1:1 copy
    // Clip to destination surface
    let img_w = image.width as i32;
    let img_h = image.height as i32;

    let mut start_x = dst_x;
    let mut start_y = dst_y;
    let mut end_x = dst_x + img_w;
    let mut end_y = dst_y + img_h;

    // Clipping
    let mut src_off_x = 0;
    let mut src_off_y = 0;

    if start_x < 0 {
        src_off_x = -start_x;
        start_x = 0;
    }
    if start_y < 0 {
        src_off_y = -start_y;
        start_y = 0;
    }
    
    if end_x > surface.width() { end_x = surface.width(); }
    if end_y > surface.height() { end_y = surface.height(); }

    if start_x >= end_x || start_y >= end_y {
        return;
    }

    let draw_w = (end_x - start_x) as usize;
    let draw_h = (end_y - start_y) as usize;

    for y in 0..draw_h {
        let sy = src_off_y as usize + y;
        let dy = start_y as usize + y;
        
        let src_row_start = sy * image.width as usize;
        let dst_row_start = dy * (surface.stride_bytes / 4) + start_x as usize; // Stride is bytes, we need u32 offset

        // Optimization: memcpy row if format matches (XRGB32)
        // Src is XRGB8888, Dst is Framebuffer (XRGB8888 usually)
        let sx = src_off_x as usize;
        let src_slice = &image.pixels[src_row_start + sx .. src_row_start + sx + draw_w];
        
        // Unsafe put_px equivalent but per row
        // surface.ptr is u8, so we cast. 
        // CAUTION: Surface stride is bytes.
        
        let dst_ptr = unsafe { (surface.ptr as *mut u32).add(dst_row_start) };
        unsafe {
            core::ptr::copy_nonoverlapping(src_slice.as_ptr(), dst_ptr, draw_w);
        }
    }
}
