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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn set_pixel_clamped_ignores_out_of_bounds() {
        let mut buf = vec![0u32; 16];
        unsafe {
            set_pixel_clamped(buf.as_mut_ptr(), 4, 4, 4, -1, 0, 0xAA);
            set_pixel_clamped(buf.as_mut_ptr(), 4, 4, 4, 0, -1, 0xAA);
            set_pixel_clamped(buf.as_mut_ptr(), 4, 4, 4, 10, 10, 0xAA);
            set_pixel_clamped(buf.as_mut_ptr(), 4, 4, 4, 1, 1, 0xBB);
        }
        assert_eq!(buf[0], 0);
        assert_eq!(buf[5], 0xBB);
    }

    #[test]
    fn fill_rect_writes_only_inside_bounds() {
        let mut buf = vec![0u32; 25];
        fill_rect(
            buf.as_mut_ptr(),
            20,
            5,
            5,
            -1,
            -1,
            4,
            4,
            0xCC,
        );
        assert_eq!(buf[0], 0xCC, "clamps to framebuffer origin");
        assert_eq!(buf[1], 0xCC);
        assert_eq!(buf[6], 0xCC);
        assert_eq!(buf[12], 0xCC);
        assert_eq!(buf[24], 0, "should not write beyond rect area");
    }
}
