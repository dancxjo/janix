use core::cmp::{max, min};
use thing_os::println;

// Helper to check intersection with clip rect
#[inline(always)]
fn clip_span(start: i32, len: i32, clip_start: i32, clip_len: i32) -> Option<(i32, i32)> {
    // (new_start, new_len)
    let end = start + len;
    let clip_end = clip_start + clip_len;

    let new_start = max(start, clip_start);
    let new_end = min(end, clip_end);

    if new_start >= new_end {
        None
    } else {
        Some((new_start, new_end - new_start))
    }
}

pub unsafe fn set_pixel_clamped(
    buffer: *mut u32,
    stride_pixels: i32,
    fb_width: i32,
    fb_height: i32,
    x: i32,
    y: i32,
    color: u32,
    clip: Option<(i32, i32, i32, i32)>,
) {
    if x < 0 || y < 0 || x >= fb_width || y >= fb_height {
        return;
    }
    if let Some((cx, cy, cw, ch)) = clip {
        if x < cx || y < cy || x >= cx + cw as i32 || y >= cy + ch as i32 {
            return;
        }
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
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
    clip: Option<(i32, i32, i32, i32)>,
) {
    if w <= 0 || h <= 0 {
        return;
    }
    let stride_pixels = stride as usize;

    // Apply clipping if provided
    let (cx, cy, cw, ch) = if let Some((cx, cy, cw, ch)) = clip {
        // Intersect requested rect with clip
        match (clip_span(x, w, cx, cw), clip_span(y, h, cy, ch)) {
            (Some((nx, nw)), Some((ny, nh))) => (nx, ny, nw, nh),
            _ => return, // No intersection
        }
    } else {
        (x, y, w, h)
    };

    let start_row = cy.max(0) as usize;
    let end_row = max(min(cy.saturating_add(ch), fb_height as i32), 0) as usize;
    for row in start_row..end_row {
        let start_col = cx.max(0) as usize;
        let end_col = max(min(cx.saturating_add(cw), fb_width as i32), 0) as usize;
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
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    img_ptr: *const u8,
    img_w: i32,
    img_h: i32,
    bpp: u16,
    offset_x: i32,
    offset_y: i32,
    clip: Option<(i32, i32, i32, i32)>,
) {
    if img_w <= 0 || img_h <= 0 {
        return;
    }

    let stride_pixels = stride as usize;

    let bytes_per_pixel = (bpp / 8) as usize;
    let row_stride = ((img_w as usize * bpp as usize + 31) / 32) * 4;

    // Apply clipping (only affects the loops, does not affect the pattern offset logic)
    let (cx, cy, cw, ch) = if let Some((cx, cy, cw, ch)) = clip {
        let irect = (0, 0, fb_width as i32, fb_height as i32);
        match (
            clip_span(irect.0, irect.2, cx, cw),
            clip_span(irect.1, irect.3, cy, ch),
        ) {
            (Some((nx, nw)), Some((ny, nh))) => (nx, ny, nw, nh),
            _ => return,
        }
    } else {
        (0, 0, fb_width as i32, fb_height as i32)
    };

    let start_y = cy.max(0);
    let end_y = (cy + ch).min(fb_height as i32);

    for y in start_y..end_y {
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

        let row_start = unsafe { img_ptr.add(row * row_stride) };
        let dest_row_start = y as usize * stride_pixels;

        let start_x = cx.max(0);
        let end_x = (cx + cw).min(fb_width as i32);

        for x in start_x..end_x {
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
            set_pixel_clamped(buf.as_mut_ptr(), 4, 4, 4, -1, 0, 0xAA, None);
            set_pixel_clamped(buf.as_mut_ptr(), 4, 4, 4, 0, -1, 0xAA, None);
            set_pixel_clamped(buf.as_mut_ptr(), 4, 4, 4, 10, 10, 0xAA, None);
            set_pixel_clamped(buf.as_mut_ptr(), 4, 4, 4, 1, 1, 0xBB, None);
        }
        assert_eq!(buf[0], 0);
        assert_eq!(buf[5], 0xBB);
    }

    #[test]
    fn fill_rect_writes_only_inside_bounds() {
        let mut buf = vec![0u32; 25];
        fill_rect(buf.as_mut_ptr(), 5, 5, 5, -1, -1, 4, 4, 0xCC, None);
        assert_eq!(buf[0], 0xCC, "clamps to framebuffer origin");
        assert_eq!(buf[1], 0xCC);
        assert_eq!(buf[6], 0xCC);
        assert_eq!(buf[12], 0xCC);
        assert_eq!(buf[24], 0, "should not write beyond rect area");
    }

    #[test]
    fn fill_rect_respects_clip() {
        let mut buf = vec![0u32; 25]; // 5x5
        // Draw 3x3 at 1,1 -> indices 6,7,8, 11,12,13, 16,17,18
        // Clip to 2x2 at 2,2 -> indices 12,13, 17,18
        fill_rect(
            buf.as_mut_ptr(),
            5,
            5,
            5,
            1,
            1,
            3,
            3,
            0xFF,
            Some((2, 2, 2, 2)),
        );

        assert_eq!(buf[6], 0);
        assert_eq!(buf[12], 0xFF);
        assert_eq!(buf[13], 0xFF);
        assert_eq!(buf[17], 0xFF);
        assert_eq!(buf[18], 0xFF);
    }

    #[test]
    fn blit_image_alpha_blending() {
        // Test basic alpha blending
        // Src: Red 50% (0x80FF0000 in BGRA)
        // Dst: White (0xFFFFFFFF in BGRA)
        // Expected: 0xFFFF7F7F (A R G B)

        // Setup: 1x1 buffer
        let mut dest_buf = vec![0xFFFFFFFFu32; 1];

        // Src: Rgba8888
        // Red in Rgba8888 is 0xFF, 0x00, 0x00, 0x80 (R G B A)
        // In u32 LE: 0x800000FF
        let src_pixel: u32 = 0x800000FF;
        let src_buf = vec![src_pixel; 1];

        blit_image(
            dest_buf.as_mut_ptr(),
            1, // stride
            1, // width
            1, // height
            src_buf.as_ptr() as *const u8,
            1, // img_w
            1, // img_h
            4, // img_stride_bytes
            abi::PixelFormat::Rgba8888,
            0,
            0, // x, y
            None,
        );

        assert_eq!(dest_buf[0], 0xFFFF7F7F);
    }
}
pub fn blit_image(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    img_ptr: *const u8,
    img_w: i32,
    img_h: i32,
    img_stride_bytes: u32,
    pixel_format: abi::PixelFormat, // Currently assume Rgba8888 or Bgra8888
    x: i32,
    y: i32,
    clip: Option<(i32, i32, i32, i32)>,
) {
    if img_w <= 0 || img_h <= 0 {
        return;
    }

    let stride_pixels = stride as usize;
    let (cx, cy, cw, ch) = if let Some((cx, cy, cw, ch)) = clip {
        // Intersect requested rect with clip
        // We are drawing at (x, y) with size (img_w, img_h)
        match (clip_span(x, img_w, cx, cw), clip_span(y, img_h, cy, ch)) {
            (Some((nx, nw)), Some((ny, nh))) => (nx, ny, nw, nh),
            _ => return,
        }
    } else {
        (x, y, img_w, img_h)
    };

    let start_y = cy.max(0);
    let end_y = (cy + ch).min(fb_height as i32);
    let start_x = cx.max(0);
    let end_x = (cx + cw).min(fb_width as i32);

    for dest_y in start_y..end_y {
        let src_y = dest_y - y;
        if src_y < 0 || src_y >= img_h {
            continue;
        } // Should be covered by clip logic but safety first

        let src_row_start = unsafe { img_ptr.add(src_y as usize * img_stride_bytes as usize) };
        let dest_row_idx = dest_y as usize * stride_pixels;

        // This inner loop could be optimized with copy_nonoverlapping if formats match and no alpha blending
        // For now, per-pixel copy to handle formats.
        // Assuming Rgba8888 source for raw buffers usually?

        for dest_x in start_x..end_x {
            let src_x = dest_x - x;
            if src_x < 0 || src_x >= img_w {
                continue;
            }

            let src_offset = src_x as usize * 4; // Assume 32bpp
            unsafe {
                let pixel_ptr = src_row_start.add(src_offset);
                let src_val = *(pixel_ptr as *const u32);

                let dest_ptr = buffer.add(dest_row_idx + dest_x as usize);

                let src_bgra_opt = if matches!(pixel_format, abi::PixelFormat::Rgba8888) {
                    // Source is RGBA. Swap R/B to get BGRA.
                    // RGBA in memory: R G B A. u32 = 0xAABBGGRR.
                    // BGRA in memory: B G R A. u32 = 0xAARRGGBB.
                    Some(
                        (src_val & 0xFF00FF00)
                            | ((src_val & 0xFF) << 16)
                            | ((src_val >> 16) & 0xFF),
                    )
                } else if matches!(pixel_format, abi::PixelFormat::Bgra8888) {
                    Some(src_val)
                } else {
                    None // Unknown format, treated as opaque copy
                };

                if let Some(src_bgra) = src_bgra_opt {
                    let sa = (src_bgra >> 24) & 0xFF;
                    if sa == 255 {
                        *dest_ptr = src_bgra;
                    } else if sa != 0 {
                        let dest_bgra = *dest_ptr;
                        let sr = (src_bgra >> 16) & 0xFF;
                        let sg = (src_bgra >> 8) & 0xFF;
                        let sb = src_bgra & 0xFF;

                        let da = (dest_bgra >> 24) & 0xFF;
                        let dr = (dest_bgra >> 16) & 0xFF;
                        let dg = (dest_bgra >> 8) & 0xFF;
                        let db = dest_bgra & 0xFF;

                        let inv_sa = 255 - sa;

                        let out_r = (sr * sa + dr * inv_sa) / 255;
                        let out_g = (sg * sa + dg * inv_sa) / 255;
                        let out_b = (sb * sa + db * inv_sa) / 255;
                        let out_a = sa + (da * inv_sa) / 255;

                        *dest_ptr = (out_a << 24) | (out_r << 16) | (out_g << 8) | out_b;
                    }
                } else {
                    // Fallback for unknown formats: just overwrite
                    *dest_ptr = src_val;
                }
            }
        }
    }
}
