use core::cmp::{max, min};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    Bgra8888,
    Rgba8888
}

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

// Interpolate between two colors
// This matches the logic from v0.1 boot screen, allowing fast fades without readback.
pub fn lerp_color(start: u32, end: u32, step: usize, total_steps: usize) -> u32 {
    if total_steps == 0 {
        return end;
    }
    let t = step as u32;
    let total = total_steps as u32;
    
    let sr = (start >> 16) & 0xFF;
    let sg = (start >> 8) & 0xFF;
    let sb = start & 0xFF;

    let er = (end >> 16) & 0xFF;
    let eg = (end >> 8) & 0xFF;
    let eb = end & 0xFF;

    let lerp = |s, e| s + ((e as i32 - s as i32) * t as i32 / total as i32) as u32;

    (0xFF << 24) | (lerp(sr, er) << 16) | (lerp(sg, eg) << 8) | lerp(sb, eb)
}

pub unsafe fn set_pixel_clamped(
    buffer: *mut u32,
    stride_bytes: u32,
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

    // Safety: we checked bounds, so we assume we are inside the buffer.
    unsafe {
        let row_ptr = (buffer as *mut u8).add(y as usize * stride_bytes as usize) as *mut u32;
        *row_ptr.add(x as usize) = color;
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
    clip: Option<(i32, i32, i32, i32)>,
) {
    if w <= 0 || h <= 0 {
        return;
    }

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

    let start_col = cx.max(0) as usize;
    let end_col = max(min(cx.saturating_add(cw), fb_width as i32), 0) as usize;
    let width_to_fill = end_col.saturating_sub(start_col);

    if width_to_fill == 0 {
        return;
    }

    for row in start_row..end_row {
        unsafe {
            let row_ptr = (buffer as *mut u8).add(row * stride_bytes as usize) as *mut u32;
            let start_ptr = row_ptr.add(start_col);

            // Optimization: If we could use memset-like, but it's u32 fill.
            // Loop unrolling or slice fill is better.
            // Using a simple loop is fine for fill_rect usually, but we can do better.
            // slice::fill is not available on raw pointers easily without slice::from_raw_parts_mut.

            // Let's create a temporary slice for safety and speed
            let slice = core::slice::from_raw_parts_mut(start_ptr, width_to_fill);
            slice.fill(color);
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
    clip: Option<(i32, i32, i32, i32)>,
    force_opaque: bool,
) {
    if img_w <= 0 || img_h <= 0 {
        return;
    }

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
        // Calculate texture Y coordinate with offset and wrapping.
        // All compositor images are stored top-down (row 0 = top), so we read directly.
        let tex_y = ((y as i32 + offset_y) % img_h + img_h) % img_h;
        let row_start = unsafe { img_ptr.add(tex_y as usize * row_stride) };
        // Use byte-based stride for destination
        let dest_row_ptr = unsafe { (buffer as *mut u8).add(y as usize * stride_bytes as usize) as *mut u32 };

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
                    let a = if force_opaque {
                        0xFF
                    } else {
                        *pixel_ptr.add(3) as u32
                    };
                    (a << 24) | (r << 16) | (g << 8) | b
                } else {
                    0 // Unsupported
                };

                *dest_row_ptr.add(x as usize) = color;
            }
        }
    }
}


pub fn blit_image(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    img_ptr: *const u8,
    img_w: i32,
    img_h: i32,
    img_stride_bytes: u32,
    pixel_format: PixelFormat,
    x: i32,
    y: i32,
    clip: Option<(i32, i32, i32, i32)>,
) {
    if img_w <= 0 || img_h <= 0 {
        return;
    }

    let (cx, cy, cw, ch) = if let Some((cx, cy, cw, ch)) = clip {
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

    let draw_w = (end_x - start_x) as usize;
    if draw_w == 0 {
        return;
    }

    let is_bgra = matches!(pixel_format, PixelFormat::Bgra8888);
    let is_rgba = matches!(pixel_format, PixelFormat::Rgba8888);

    for dest_y in start_y..end_y {
        let src_y = dest_y - y;
        if src_y < 0 || src_y >= img_h {
            continue;
        }

        let src_row_start = unsafe { img_ptr.add(src_y as usize * img_stride_bytes as usize) };
        let dest_row_ptr = unsafe { (buffer as *mut u8).add(dest_y as usize * stride_bytes as usize) as *mut u32 };

        let start_src_x = start_x - x;
        
        unsafe {
            let dest_ptr_start = dest_row_ptr.add(start_x as usize);
            let src_ptr_start = src_row_start as *const u32;

            if is_bgra {
                 // Fast loop for BGRA
                 for i in 0..draw_w {
                     let src_val = *src_ptr_start.add(start_src_x as usize + i);
                     let sa = (src_val >> 24) & 0xFF;
                     let dest = dest_ptr_start.add(i);

                     if sa == 255 {
                         *dest = src_val;
                     } else if sa != 0 {
                         // Blend
                         let dest_val = *dest; // MMIO Read (Slow)
                         let inv_sa = 255 - sa;
                         let rb = ((src_val & 0x00FF00FF) * sa + (dest_val & 0x00FF00FF) * inv_sa) >> 8;
                         let g = ((src_val & 0x0000FF00) * sa + (dest_val & 0x0000FF00) * inv_sa) >> 8;
                         *dest = 0xFF000000 | (rb & 0x00FF00FF) | (g & 0x0000FF00);
                     }
                 }
            } else {
                // RGBA -> BGRA conversion loop
                 for i in 0..draw_w {
                     let src_val = *src_ptr_start.add(start_src_x as usize + i);
                     // R G B A -> B G R A
                     let src_bgra = (src_val & 0xFF00FF00)
                        | ((src_val & 0xFF) << 16)
                        | ((src_val >> 16) & 0xFF);

                     let sa = (src_bgra >> 24) & 0xFF;
                     let dest = dest_ptr_start.add(i);

                     if sa == 255 {
                         *dest = src_bgra;
                     } else if sa != 0 {
                         let dest_val = *dest;
                         let inv_sa = 255 - sa;
                         let rb = ((src_bgra & 0x00FF00FF) * sa + (dest_val & 0x00FF00FF) * inv_sa) >> 8;
                         let g = ((src_bgra & 0x0000FF00) * sa + (dest_val & 0x0000FF00) * inv_sa) >> 8;
                         *dest = 0xFF000000 | (rb & 0x00FF00FF) | (g & 0x0000FF00);
                     }
                 }
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
            PixelFormat::Rgba8888,
            0,
            0, // x, y
            None,
        );

        assert_eq!(dest_buf[0], 0xFFFF7F7F);
    }

    #[test]
    fn draw_tiled_image_32bpp_forces_alpha() {
        let mut buf = vec![0u32; 1];
        // Source: Blue=0x11, Green=0x22, Red=0x33, Alpha=0x00 (stored as B G R A)
        let src_bytes: [u8; 4] = [0x11, 0x22, 0x33, 0x00];

        draw_tiled_image(
            buf.as_mut_ptr(),
            4, // stride (1 pixel)
            1,
            1,
            src_bytes.as_ptr(),
            1,
            1,
            32, // bpp
            0,
            0,
            None,
            true, // force_opaque
        );

        // Expect Alpha to be forced to 0xFF.
        // 0xFF000000 | (0x33 << 16) | (0x22 << 8) | 0x11
        assert_eq!(buf[0], 0xFF332211, "Alpha should be forced to 0xFF for 32bpp images");
    }

    #[test]
    fn fill_rect_respects_byte_stride() {
        // 4x2 buffer. Width=4. Stride=20 bytes (1 extra pixel padding).
        // Row 0: 0,1,2,3, Pad
        // Row 1: 5,6,7,8, Pad
        let mut buf = vec![0u32; 10]; // 5 u32s per row equivalent
        let stride_bytes = 20;

        fill_rect(buf.as_mut_ptr(), stride_bytes, 4, 2, 0, 0, 4, 2, 0xFF, None);

        // Row 0
        assert_eq!(buf[0], 0xFF);
        assert_eq!(buf[3], 0xFF);
        assert_eq!(buf[4], 0); // Padding shouldn't be touched? fill_rect fills width 4.

        // Row 1 starts at byte offset 20 -> index 5
        assert_eq!(buf[5], 0xFF);
        assert_eq!(buf[8], 0xFF);
        assert_eq!(buf[9], 0);
    }

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
            PixelFormat::Rgba8888,
            0,
            0, // x, y
            None,
        );

        assert_eq!(dest_buf[0], 0xFFFF7F7F);
    }

    #[test]
    fn draw_tiled_image_32bpp_forces_alpha() {
        let mut buf = vec![0u32; 1];
        // Source: Blue=0x11, Green=0x22, Red=0x33, Alpha=0x00 (stored as B G R A)
        let src_bytes: [u8; 4] = [0x11, 0x22, 0x33, 0x00];

        draw_tiled_image(
            buf.as_mut_ptr(),
            4, // stride (1 pixel)
            1,
            1,
            src_bytes.as_ptr(),
            1,
            1,
            32, // bpp
            0,
            0,
            None,
            true, // force_opaque
        );

        // Expect Alpha to be forced to 0xFF.
        // 0xFF000000 | (0x33 << 16) | (0x22 << 8) | 0x11
        assert_eq!(buf[0], 0xFF332211, "Alpha should be forced to 0xFF for 32bpp images");
    }
}
