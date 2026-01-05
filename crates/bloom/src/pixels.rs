use crate::scene::Rect;
use crate::assets::cursor::CursorFrame;

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

pub unsafe fn draw_cursor_shadow(
    dest: *mut u32,
    screen_w: u32,
    screen_h: u32,
    frame: &CursorFrame,
    px: i32,
    py: i32,
) {
    let cx = px - frame.hotspot_x + frame.shadow_offset_x;
    let cy = py - frame.hotspot_y + frame.shadow_offset_y;

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

            let shadow_idx = (row * frame.width + col) as usize;
            let shadow_pixel = frame.shadow_pixels[shadow_idx];
            let alpha = (shadow_pixel >> 24) & 0xFF;

            if alpha == 0 {
                continue;
            }

            let dest_idx = (screen_y as u32 * screen_w + screen_x as u32) as usize;
            let dst_pixel = *dest.add(dest_idx);
            *dest.add(dest_idx) = blend_pixel(shadow_pixel, dst_pixel);
        }
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
