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
    let stride_pixels = (stride / 4) as usize;

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
    force_opaque: bool,
) {
    if img_w <= 0 || img_h <= 0 {
        return;
    }

    let stride_pixels = (stride / 4) as usize;

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
                    let a = if force_opaque {
                        0xFF
                    } else {
                        *pixel_ptr.add(3) as u32
                    };
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

    let stride_pixels = (stride / 4) as usize;
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

    let draw_w = (end_x - start_x) as usize;
    if draw_w == 0 {
        return;
    }

    // Optimization: Check if we can do a memcpy.
    // Conditions:
    // 1. Format matches (Bgra8888 dst, so src must be Bgra8888).
    // 2. We don't need alpha blending (assuming opaque source for now, or if we knew it was opaque).
    //    For now, only if format matches AND we assume opacity or check it?
    //    Actually, if the caller knows it's opaque, it would be great.
    //    Let's check pixel format. If it's Bgra8888, we might still have alpha = 0..255.
    //    But if the source is known to be fully opaque (e.g. wallpapers), we could skip.
    //    For now, let's just stick to the inner loop optimizations.

    let is_bgra = matches!(pixel_format, abi::PixelFormat::Bgra8888);
    let is_rgba = matches!(pixel_format, abi::PixelFormat::Rgba8888);

    for dest_y in start_y..end_y {
        let src_y = dest_y - y;
        if src_y < 0 || src_y >= img_h {
            continue;
        }

        let src_row_start = unsafe { img_ptr.add(src_y as usize * img_stride_bytes as usize) };
        let dest_row_idx = dest_y as usize * stride_pixels;
        let dest_row_ptr = unsafe { buffer.add(dest_row_idx) };

        // Inner loop start source offset
        let start_src_x = start_x - x;
        
        // Fast path: BGRA -> BGRA copy (if we assume source is opaque / we treat it as such or if we just want raw speed for testing)
        // Ideally we check per-pixel alpha, but for large opaque surfaces (windows without transparency) this matters.
        // Let's implement the per-pixel blending but optimized.

        for i in 0..draw_w {
            let src_x = start_src_x + i as i32;
            let dest_x = start_x + i as i32;

            let src_offset = src_x as usize * 4;
            
            unsafe {
                let pixel_ptr = src_row_start.add(src_offset);
                let src_val = *(pixel_ptr as *const u32);
                let dest_ptr = dest_row_ptr.add(dest_x as usize);

                // Decode source to BGRA
                let src_bgra = if is_rgba {
                    // R G B A -> B G R A
                    // 0xAABBGGRR -> 0xAARRGGBB
                    (src_val & 0xFF00FF00)
                        | ((src_val & 0xFF) << 16)
                        | ((src_val >> 16) & 0xFF)
                } else {
                    // Already BGRA (or unknown treated as such)
                    src_val
                };

                // Check Alpha
                let sa = (src_bgra >> 24) & 0xFF;
                
                if sa == 255 {
                    // Opaque: Memory Write
                    *dest_ptr = src_bgra;
                } else if sa != 0 {
                    // Blending needed
                    // Optimize: Cheap blend.
                    // out = src * alpha + dst * (1 - alpha)
                    //     = (src * alpha + dst * (255 - alpha)) / 255
                    // Fast div 255: (x + 1 + (x >> 8)) >> 8
                    
                    let dest_bgra = *dest_ptr;
                    
                    let inv_sa = 255 - sa;
                    
                    let rb_s = src_bgra & 0x00FF00FF;
                    let g_s = src_bgra & 0x0000FF00;
                    
                    let rb_d = dest_bgra & 0x00FF00FF;
                    let g_d = dest_bgra & 0x0000FF00;
                    
                    let rb = (rb_s * sa + rb_d * inv_sa) >> 8;
                    let g = (g_s * sa + g_d * inv_sa) >> 8;
                    
                    // Mask and reassemble
                    let rb_out = rb & 0x00FF00FF;
                    let g_out = g & 0x0000FF00;
                    let a_out = 0xFF000000; // Force full opacity destination? Or blend alpha?
                                            // Generally screen is opaque.
                    
                    *dest_ptr = a_out | rb_out | g_out;
                }
                // If sa == 0, skip
            }
        }
    }
}
