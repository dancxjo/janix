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

pub fn draw_tiled_image(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    img_ptr: *const u8,
    img_w: i32,
    img_h: i32,
    bpp: u16,
    offset_x: i32,
    offset_y: i32,
) {
    if img_w <= 0 || img_h <= 0 {
        return;
    }

    let stride_pixels = (stride_bytes / 4) as usize;

    // BMP row stride is aligned to 4 bytes
    let bytes_per_pixel = (bpp / 8) as usize;
    let row_stride = ((img_w as usize * bpp as usize + 31) / 32) * 4;

    for y in 0..fb_height {
        // Calculate texture Y coordinate with offset and wrapping
        // We want (y + offset) to map to texture space.
        // Also handle negative results from % operator if offset is negative.
        let tex_y = ((y as i32 + offset_y) % img_h + img_h) % img_h;

        // Standard BMP logic: positive height means bottom-up
        let row = if img_h > 0 {
            (img_h - 1 - tex_y) as usize
        } else {
            tex_y as usize
        };

        let row_start = unsafe { img_ptr.add(row * row_stride) };
        let dest_row_start = y as usize * stride_pixels;

        for x in 0..fb_width {
            let tex_x = ((x as i32 + offset_x) % img_w + img_w) % img_w;
            let src_offset = tex_x as usize * bytes_per_pixel;

            unsafe {
                let pixel_ptr = row_start.add(src_offset);
                let color = if bpp == 24 {
                    // BGR
                    let b = *pixel_ptr as u32;
                    let g = *pixel_ptr.add(1) as u32;
                    let r = *pixel_ptr.add(2) as u32;
                    0xFF000000 | (r << 16) | (g << 8) | b
                } else if bpp == 32 {
                    // BGRA
                    let b = *pixel_ptr as u32;
                    let g = *pixel_ptr.add(1) as u32;
                    let r = *pixel_ptr.add(2) as u32;
                    let a = *pixel_ptr.add(3) as u32;
                    (a << 24) | (r << 16) | (g << 8) | b
                } else {
                    0 // Unsupported
                };

                *buffer.add(dest_row_start + x as usize) = color;
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
        fill_rect(buf.as_mut_ptr(), 20, 5, 5, -1, -1, 4, 4, 0xCC);
        assert_eq!(buf[0], 0xCC, "clamps to framebuffer origin");
        assert_eq!(buf[1], 0xCC);
        assert_eq!(buf[6], 0xCC);
        assert_eq!(buf[12], 0xCC);
        assert_eq!(buf[24], 0, "should not write beyond rect area");
    }
}
