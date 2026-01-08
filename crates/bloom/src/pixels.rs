//! Pixel manipulation utilities for Bloom compositor.
//!
//! ## Alignment Safety Doctrine
//!
//! All raw pointer → u32 pixel slice conversions MUST go through:
//! - `pixels_u32()` for immutable slices
//! - `pixels_u32_mut()` for mutable slices
//!
//! These functions enforce alignment, null checks, and length validation.
//! If validation fails, they log an `ALIGN:` diagnostic and return `None`.

use crate::scene::Rect;
use crate::assets::cursor::CursorFrame;
use thing_std::log_info;
use alloc::format;

// =============================================================================
// CENTRALIZED ALIGNMENT-SAFE SLICE HELPERS
// =============================================================================

/// Create an immutable u32 slice from a raw byte pointer with full validation.
///
/// This is the ONLY function that should call `from_raw_parts` for `&[u32]`.
/// All other code paths MUST use this helper.
///
/// Returns `None` and logs `ALIGN:` if:
/// - `ptr` is null
/// - `ptr` is not 4-byte aligned
/// - `len_bytes` is not a multiple of 4
/// - `len_bytes / 4` exceeds `isize::MAX`
#[inline]
pub fn pixels_u32(ptr: *const u8, len_bytes: usize, label: &'static str) -> Option<&'static [u32]> {
    if ptr.is_null() {
        log_info(&format!("ALIGN: {} ptr=null (fallback)", label));
        return None;
    }
    if (ptr as usize) % 4 != 0 {
        log_info(&format!("ALIGN: {} ptr={:#x} misalign={} (fallback)", label, ptr as usize, (ptr as usize) & 3));
        return None;
    }
    if len_bytes % 4 != 0 {
        log_info(&format!("ALIGN: {} len_bytes={} not multiple of 4 (fallback)", label, len_bytes));
        return None;
    }
    let len_u32 = len_bytes / 4;
    if len_u32 > isize::MAX as usize {
        log_info(&format!("ALIGN: {} len_u32={} exceeds isize::MAX (fallback)", label, len_u32));
        return None;
    }
    Some(unsafe { core::slice::from_raw_parts(ptr as *const u32, len_u32) })
}

/// Create a mutable u32 slice from a raw byte pointer with full validation.
///
/// This is the ONLY function that should call `from_raw_parts_mut` for `&mut [u32]`.
/// All other code paths MUST use this helper.
#[inline]
pub fn pixels_u32_mut(ptr: *mut u8, len_bytes: usize, label: &'static str) -> Option<&'static mut [u32]> {
    if ptr.is_null() {
        log_info(&format!("ALIGN: {} ptr=null (fallback)", label));
        return None;
    }
    if (ptr as usize) % 4 != 0 {
        log_info(&format!("ALIGN: {} ptr={:#x} misalign={} (fallback)", label, ptr as usize, (ptr as usize) & 3));
        return None;
    }
    if len_bytes % 4 != 0 {
        log_info(&format!("ALIGN: {} len_bytes={} not multiple of 4 (fallback)", label, len_bytes));
        return None;
    }
    let len_u32 = len_bytes / 4;
    if len_u32 > isize::MAX as usize {
        log_info(&format!("ALIGN: {} len_u32={} exceeds isize::MAX (fallback)", label, len_u32));
        return None;
    }
    Some(unsafe { core::slice::from_raw_parts_mut(ptr as *mut u32, len_u32) })
}

/// Byte-wise copy for misaligned buffers (slow but safe fallback).
///
/// Use this when `pixels_u32` returns `None` but you still need to copy data.
#[inline]
pub unsafe fn blit_bytes(dst: *mut u8, src: *const u8, len_bytes: usize) {
    core::ptr::copy_nonoverlapping(src, dst, len_bytes);
}

/// Create an immutable u8 slice from a raw pointer with null and length validation.
///
/// For byte slices, alignment is not required, but we still check null and length.
#[inline]
pub fn bytes_checked(ptr: *const u8, len: usize, label: &'static str) -> Option<&'static [u8]> {
    if ptr.is_null() {
        log_info(&format!("ALIGN: {} ptr=null (fallback)", label));
        return None;
    }
    if len > isize::MAX as usize {
        log_info(&format!("ALIGN: {} len={} exceeds isize::MAX (fallback)", label, len));
        return None;
    }
    Some(unsafe { core::slice::from_raw_parts(ptr, len) })
}

// =============================================================================
// EXISTING PIXEL MANIPULATION FUNCTIONS
// =============================================================================

pub unsafe fn redraw_region(dest: *mut u32, src: *const u32, w: u32, h: u32, region: Rect) {
    let x1 = region.x.max(0) as u32;
    let y1 = region.y.max(0) as u32;
    let x2 = ((region.x + (region.w as i32)) as u32).min(w);
    let y2 = ((region.y + (region.h as i32)) as u32).min(h);
    for y in y1..y2 {
        let row_start = (y * w + x1) as usize;
        let row_len = (x2 - x1) as usize;
        core::ptr::copy_nonoverlapping(src.add(row_start), dest.add(row_start), row_len);
    }
}

pub unsafe fn draw_cursor_frame(
    dest: *mut u32,
    screen_w: u32,
    screen_h: u32,
    frame: &CursorFrame,
    px: i32,
    py: i32,
) {
    let cx = px - frame.hotspot_x;
    let cy = py - frame.hotspot_y;

    for row in 0..frame.height {
        let screen_y = cy + row as i32;
        if screen_y < 0 || screen_y >= screen_h as i32 {
            continue;
        }

        for col in 0..frame.width {
            let screen_x = cx + col as i32;
            if screen_x < 0 || screen_x >= screen_w as i32 {
                continue;
            }

            let cursor_idx = (row * frame.width + col) as usize;
            let src_pixel = frame.pixels[cursor_idx];
            let alpha = (src_pixel >> 24) & 0xFF;

            if alpha == 0 {
                continue;
            }

            let dest_idx = (screen_y as u32 * screen_w + screen_x as u32) as usize;

            if alpha == 255 {
                *dest.add(dest_idx) = src_pixel;
            } else {
                let dst_pixel = *dest.add(dest_idx);
                *dest.add(dest_idx) = blend_pixel(src_pixel, dst_pixel);
            }
        }
    }
}

#[inline]
pub fn blend_pixel(src: u32, dst: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    let inv_a = 255 - sa;
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;
    let or = sr + ((dr * inv_a + 127) / 255);
    let og = sg + ((dg * inv_a + 127) / 255);
    let ob = sb + ((db * inv_a + 127) / 255);
    0xFF000000 | (or.min(255) << 16) | (og.min(255) << 8) | ob.min(255)
}

pub unsafe fn draw_fallback_cursor(dest: *mut u32, w: u32, h: u32, px: i32, py: i32) {
    for dy in 0..8i32 {
        for dx in 0..8i32 {
            let x = px + dx + 2;
            let y = py + dy + 3;
            if x >= 0 && x < (w as i32) && y >= 0 && y < (h as i32) {
                let idx = (y as u32 * w + (x as u32)) as usize;
                let dst = *dest.add(idx);
                *dest.add(idx) = blend_pixel(0x66000000, dst);
            }
        }
    }
    for dy in 0..8i32 {
        for dx in 0..8i32 {
            let x = px + dx;
            let y = py + dy;
            if x >= 0 && x < (w as i32) && y >= 0 && y < (h as i32) {
                let idx = (y as u32 * w + (x as u32)) as usize;
                let pixel = if dx == 0 || dy == 0 || dx == 7 || dy == 7 {
                    0xFF000000
                } else {
                    0xFFFFFFFF
                };
                *dest.add(idx) = pixel;
            }
        }
    }
}

pub unsafe fn fill_rect(dest: *mut u32, screen_w: u32, screen_h: u32, rect: Rect, color: u32) {
    let x1 = rect.x.max(0) as u32;
    let y1 = rect.y.max(0) as u32;
    let x2 = ((rect.x + (rect.w as i32)) as u32).min(screen_w);
    let y2 = ((rect.y + (rect.h as i32)) as u32).min(screen_h);
    for y in y1..y2 {
        for x in x1..x2 {
            *dest.add((y * screen_w + x) as usize) = color;
        }
    }
}

pub unsafe fn draw_v_gradient(dest: *mut u32, w: u32, h: u32, c1: u32, c2: u32) {
    let r1 = (c1 >> 16) & 0xFF;
    let g1 = (c1 >> 8) & 0xFF;
    let b1 = c1 & 0xFF;
    let r2 = (c2 >> 16) & 0xFF;
    let g2 = (c2 >> 8) & 0xFF;
    let b2 = c2 & 0xFF;

    for y in 0..h {
        let r = (r1 as i32 + (r2 as i32 - r1 as i32) * y as i32 / h as i32) as u32;
        let g = (g1 as i32 + (g2 as i32 - g1 as i32) * y as i32 / h as i32) as u32;
        let b = (b1 as i32 + (b2 as i32 - b1 as i32) * y as i32 / h as i32) as u32;
        let color = 0xFF000000 | (r << 16) | (g << 8) | b;
        for x in 0..w {
            *dest.add((y * w + x) as usize) = color;
        }
    }
}

pub unsafe fn blend_wallpaper_rows(
    dest: *mut u32,
    dest_w: u32,
    dest_h: u32,
    wp: &crate::assets::bmp::Wallpaper,
    start_y: u32,
    end_y: u32,
    alpha: u8,
) {
    let end_y = end_y.min(dest_h);
    for y in start_y..end_y {
        let src_y = y % wp.height;
        let actual_src_y = if wp.bottom_up {
            wp.height - 1 - src_y
        } else {
            src_y
        };
        let src_row_ptr = wp.data_ptr.add(actual_src_y as usize * wp.row_stride);
        for x in 0..dest_w {
            let src_x = x % wp.width;
            let src_px_ptr = src_row_ptr.add(src_x as usize * wp.bytes_per_pixel);
            let b = *src_px_ptr;
            let g = *src_px_ptr.add(1);
            let r = *src_px_ptr.add(2);
            let src_pixel = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            
            let dest_idx = (y * dest_w + x) as usize;
            let dst_pixel = *dest.add(dest_idx);
            
            if alpha == 255 {
               *dest.add(dest_idx) = src_pixel;
            } else {
               *dest.add(dest_idx) = blend_pixel_with_alpha(src_pixel, dst_pixel, alpha);
            }
        }
    }
}

#[inline]
pub fn blend_pixel_with_alpha(src: u32, dst: u32, alpha: u8) -> u32 {
    let sa = alpha as u32;
    let inv_a = 255 - sa;
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;
    let or = (sr * sa + dr * inv_a + 127) / 255;
    let og = (sg * sa + dg * inv_a + 127) / 255;
    let ob = (sb * sa + db * inv_a + 127) / 255;
    0xFF000000 | (or << 16) | (og << 8) | ob
}
