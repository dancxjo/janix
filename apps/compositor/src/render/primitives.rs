use core::cmp::{max, min};

pub unsafe fn set_pixel_clamped(
    buffer: *mut u32,
    stride_pixels: i32,
    fb_width: i32,
    fb_height: i32,
    x: i32,
    y: i32,
    color: u32,
) {
    if x < 0 || y < 0 || x >= fb_width || y >= fb_height {
        return;
    }
    let idx = y * stride_pixels + x;
    if idx < 0 {
        return;
    }
    unsafe {
        *buffer.add(idx as usize) = color;
    }
}

pub fn fill_rect(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
) {
    if w <= 0 || h <= 0 {
        return;
    }
    let stride_pixels = (stride_bytes / 4) as usize;
    let start_row = y.max(0) as usize;
    let end_row = max(min(y.saturating_add(h), fb_height as i32), 0) as usize;
    for row in start_row..end_row {
        let start_col = x.max(0) as usize;
        let end_col = max(min(x.saturating_add(w), fb_width as i32), 0) as usize;
        for col in start_col..end_col {
            let idx = row * stride_pixels + col;
            unsafe {
                *buffer.add(idx) = color;
            }
        }
    }
}
