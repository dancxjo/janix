//! CPU Rasterizer - executes LowLevelOps on a framebuffer Surface
//!
//! Supports both full-frame and damage-aware rendering.

use crate::damage::{Damage, Rect};
use crate::drawlist::DrawList;
use crate::lowered::{lower, LowLevelOp, LoweredDraw};
use crate::surface::Surface;
use crate::asset::Image;

/// Execute a DrawList on a CPU surface (convenience wrapper)
pub fn execute(surface: &mut Surface, list: &DrawList) {
    let lowered = lower(list);
    execute_lowered(surface, &lowered);
}

/// Execute a DrawList respecting damage regions
/// 
/// Only pixels within damaged rectangles are updated.
/// This can significantly reduce CPU work when only cursor moves.
pub fn execute_with_damage(surface: &mut Surface, list: &DrawList, damage: &Damage) {
    // If full-frame damage or no damage tracking, fall back to full render
    if damage.is_full {
        execute(surface, list);
        return;
    }

    let lowered = lower(list);
    execute_lowered_with_damage(surface, &lowered, damage);
}

/// Execute lowered ops directly (allows future targets to share this interface)
pub fn execute_lowered(surface: &mut Surface, lowered: &LoweredDraw) {
    for op in lowered.ops.iter() {
        match op {
            LowLevelOp::Clear { xrgb } => clear(surface, *xrgb),
            LowLevelOp::FillRect { x, y, w, h, xrgb } => fill_rect(surface, *x, *y, *w, *h, *xrgb),
            LowLevelOp::Line { x0, y0, x1, y1, xrgb } => line(surface, *x0, *y0, *x1, *y1, *xrgb),
            LowLevelOp::Blit { image, src, dst } => blit_scaled(surface, image, src, dst),
            LowLevelOp::BlitAlpha { image, src, dst, shadow_factor } => {
                blit_alpha_scaled(surface, image, src, dst, *shadow_factor);
            }
        }
    }
}

/// Execute lowered ops clipped to damage regions
pub fn execute_lowered_with_damage(surface: &mut Surface, lowered: &LoweredDraw, damage: &Damage) {
    // Collect damage rects into a local array for iteration
    let mut damage_rects = [Rect::default(); 8];
    let mut damage_count = 0;
    for rect in damage.iter() {
        if damage_count < 8 {
            damage_rects[damage_count] = rect;
            damage_count += 1;
        }
    }

    // For each operation, render only the portions that intersect with damage
    for op in lowered.ops.iter() {
        match op {
            LowLevelOp::Clear { xrgb } => {
                // Clear only damaged regions
                for i in 0..damage_count {
                    let r = damage_rects[i];
                    fill_rect(surface, r.x, r.y, r.w, r.h, *xrgb);
                }
            }
            LowLevelOp::FillRect { x, y, w, h, xrgb } => {
                let op_rect = Rect::new(*x, *y, *w, *h);
                for i in 0..damage_count {
                    let intersect = op_rect.intersect(damage_rects[i]);
                    if !intersect.is_empty() {
                        fill_rect(surface, intersect.x, intersect.y, intersect.w, intersect.h, *xrgb);
                    }
                }
            }
            LowLevelOp::Line { x0, y0, x1, y1, xrgb } => {
                // Lines are typically small; just draw them entirely if they intersect any damage
                let line_rect = line_bbox(*x0, *y0, *x1, *y1);
                for i in 0..damage_count {
                    if !line_rect.intersect(damage_rects[i]).is_empty() {
                        line(surface, *x0, *y0, *x1, *y1, *xrgb);
                        break; // Only draw once
                    }
                }
            }
            LowLevelOp::Blit { image, src, dst } => {
                for i in 0..damage_count {
                    blit_scaled_clipped(surface, image, src, dst, damage_rects[i]);
                }
            }
            LowLevelOp::BlitAlpha { image, src, dst, shadow_factor } => {
                for i in 0..damage_count {
                    blit_alpha_scaled_clipped(surface, image, src, dst, *shadow_factor, damage_rects[i]);
                }
            }
        }
    }
}

fn line_bbox(x0: i32, y0: i32, x1: i32, y1: i32) -> Rect {
    let min_x = x0.min(x1);
    let min_y = y0.min(y1);
    let max_x = x0.max(x1);
    let max_y = y0.max(y1);
    Rect::new(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
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

/// Blit with scaling support (for nine-slice stretching)
fn blit_scaled(surface: &mut Surface, image: &Image, src: &Rect, dst: &Rect) {
    // For 1:1 blits (most common case), use fast path
    if src.w == dst.w && src.h == dst.h {
        blit_image_region(surface, image, src, dst.x, dst.y);
        return;
    }

    // Scaled blit using nearest-neighbor sampling
    let dst_x0 = dst.x.max(0);
    let dst_y0 = dst.y.max(0);
    let dst_x1 = (dst.x + dst.w).min(surface.width());
    let dst_y1 = (dst.y + dst.h).min(surface.height());

    if dst_x0 >= dst_x1 || dst_y0 >= dst_y1 || dst.w <= 0 || dst.h <= 0 || src.w <= 0 || src.h <= 0 {
        return;
    }

    for dy in dst_y0..dst_y1 {
        for dx in dst_x0..dst_x1 {
            // Map destination coordinate to source
            let sx = src.x + ((dx - dst.x) * src.w / dst.w);
            let sy = src.y + ((dy - dst.y) * src.h / dst.h);

            if sx >= 0 && sy >= 0 && sx < image.width as i32 && sy < image.height as i32 {
                let idx = (sy as usize) * (image.width as usize) + (sx as usize);
                if idx < image.pixels.len() {
                    let px = image.pixels[idx];
                    // Skip fully transparent pixels
                    if (px >> 24) != 0 {
                        surface.put_px(dx, dy, px);
                    }
                }
            }
        }
    }
}

/// Blit clipped to a specific damage rectangle
fn blit_scaled_clipped(surface: &mut Surface, image: &Image, src: &Rect, dst: &Rect, clip: Rect) {
    // Compute the intersection of dst and clip
    let clipped_dst = dst.intersect(clip);
    if clipped_dst.is_empty() {
        return;
    }

    // Adjust src rect proportionally
    let scale_x = if dst.w > 0 { src.w as f32 / dst.w as f32 } else { 1.0 };
    let scale_y = if dst.h > 0 { src.h as f32 / dst.h as f32 } else { 1.0 };

    let src_x_offset = ((clipped_dst.x - dst.x) as f32 * scale_x) as i32;
    let src_y_offset = ((clipped_dst.y - dst.y) as f32 * scale_y) as i32;
    let src_w = (clipped_dst.w as f32 * scale_x) as i32;
    let src_h = (clipped_dst.h as f32 * scale_y) as i32;

    let adjusted_src = Rect::new(src.x + src_x_offset, src.y + src_y_offset, src_w, src_h);

    blit_scaled(surface, image, &adjusted_src, &clipped_dst);
}

/// Fast path for 1:1 region blit (no scaling)
fn blit_image_region(surface: &mut Surface, image: &Image, src: &Rect, dst_x: i32, dst_y: i32) {
    let img_w = image.width as i32;
    let img_h = image.height as i32;

    // Clamp source rect to image bounds
    let src_x0 = src.x.max(0);
    let src_y0 = src.y.max(0);
    let src_x1 = (src.x + src.w).min(img_w);
    let src_y1 = (src.y + src.h).min(img_h);

    if src_x0 >= src_x1 || src_y0 >= src_y1 {
        return;
    }

    let actual_w = src_x1 - src_x0;
    let actual_h = src_y1 - src_y0;

    // Destination clipping
    let mut draw_x = dst_x + (src_x0 - src.x);
    let mut draw_y = dst_y + (src_y0 - src.y);
    let mut draw_w = actual_w;
    let mut draw_h = actual_h;
    let mut src_off_x = 0;
    let mut src_off_y = 0;

    if draw_x < 0 {
        src_off_x = -draw_x;
        draw_w += draw_x;
        draw_x = 0;
    }
    if draw_y < 0 {
        src_off_y = -draw_y;
        draw_h += draw_y;
        draw_y = 0;
    }
    if draw_x + draw_w > surface.width() {
        draw_w = surface.width() - draw_x;
    }
    if draw_y + draw_h > surface.height() {
        draw_h = surface.height() - draw_y;
    }

    if draw_w <= 0 || draw_h <= 0 {
        return;
    }

    // Copy rows
    for y in 0..draw_h {
        let sy = (src_y0 + src_off_y + y) as usize;
        let dy = (draw_y + y) as usize;

        let src_row_start = sy * (image.width as usize) + (src_x0 + src_off_x) as usize;
        let dst_row_start = dy * (surface.stride_bytes / 4) + draw_x as usize;

        let src_slice = &image.pixels[src_row_start..src_row_start + draw_w as usize];
        let dst_ptr = unsafe { (surface.ptr as *mut u32).add(dst_row_start) };

        unsafe {
            core::ptr::copy_nonoverlapping(src_slice.as_ptr(), dst_ptr, draw_w as usize);
        }
    }
}

/// Alpha blit with optional shadow factor for darkening/fading
fn blit_alpha_scaled(surface: &mut Surface, image: &Image, src: &Rect, dst: &Rect, shadow_factor: Option<u8>) {
    let dst_x0 = dst.x.max(0);
    let dst_y0 = dst.y.max(0);
    let dst_x1 = (dst.x + dst.w).min(surface.width());
    let dst_y1 = (dst.y + dst.h).min(surface.height());

    if dst_x0 >= dst_x1 || dst_y0 >= dst_y1 || dst.w <= 0 || dst.h <= 0 || src.w <= 0 || src.h <= 0 {
        return;
    }

    for dy in dst_y0..dst_y1 {
        for dx in dst_x0..dst_x1 {
            blit_alpha_pixel(surface, image, src, dst, dx, dy, shadow_factor);
        }
    }
}

/// Alpha blit clipped to a specific damage rectangle  
fn blit_alpha_scaled_clipped(surface: &mut Surface, image: &Image, src: &Rect, dst: &Rect, shadow_factor: Option<u8>, clip: Rect) {
    let clipped_dst = dst.intersect(clip);
    if clipped_dst.is_empty() {
        return;
    }

    let dst_x0 = clipped_dst.x.max(0);
    let dst_y0 = clipped_dst.y.max(0);
    let dst_x1 = (clipped_dst.x + clipped_dst.w).min(surface.width());
    let dst_y1 = (clipped_dst.y + clipped_dst.h).min(surface.height());

    if dst_x0 >= dst_x1 || dst_y0 >= dst_y1 {
        return;
    }

    for dy in dst_y0..dst_y1 {
        for dx in dst_x0..dst_x1 {
            blit_alpha_pixel(surface, image, src, dst, dx, dy, shadow_factor);
        }
    }
}

/// Blit a single alpha-blended pixel
fn blit_alpha_pixel(surface: &mut Surface, image: &Image, src: &Rect, dst: &Rect, dx: i32, dy: i32, shadow_factor: Option<u8>) {
    // Map destination coordinate to source (handle scaling)
    let sx = if dst.w == src.w {
        src.x + (dx - dst.x)
    } else {
        src.x + ((dx - dst.x) * src.w / dst.w)
    };
    let sy = if dst.h == src.h {
        src.y + (dy - dst.y)
    } else {
        src.y + ((dy - dst.y) * src.h / dst.h)
    };

    if sx < 0 || sy < 0 || sx >= image.width as i32 || sy >= image.height as i32 {
        return;
    }

    let idx = (sy as usize) * (image.width as usize) + (sx as usize);
    if idx >= image.pixels.len() {
        return;
    }

    let src_px = image.pixels[idx];
    let mut alpha = (src_px >> 24) & 0xFF;

    if alpha == 0 {
        return;
    }

    // Apply shadow factor if present (darkens and reduces opacity)
    if let Some(factor) = shadow_factor {
        alpha = (alpha * factor as u32) >> 8;
        if alpha == 0 {
            return;
        }
        // For shadows, we darken the color by blending toward black
        let offset = (surface.stride_bytes / 4) * (dy as usize) + (dx as usize);
        let dst_ptr = unsafe { (surface.ptr as *mut u32).add(offset) };
        let dst_px = unsafe { *dst_ptr };

        // Shadow: blend black with shadow alpha
        let da = 255 - alpha;
        let dst_r = (dst_px >> 16) & 0xFF;
        let dst_g = (dst_px >> 8) & 0xFF;
        let dst_b = dst_px & 0xFF;

        let out_r = (dst_r * da) >> 8;
        let out_g = (dst_g * da) >> 8;
        let out_b = (dst_b * da) >> 8;

        unsafe { *dst_ptr = (out_r << 16) | (out_g << 8) | out_b; }
    } else {
        // Normal alpha blending
        if alpha == 255 {
            let offset = (surface.stride_bytes / 4) * (dy as usize) + (dx as usize);
            unsafe { *(surface.ptr as *mut u32).add(offset) = src_px; }
        } else {
            let offset = (surface.stride_bytes / 4) * (dy as usize) + (dx as usize);
            let dst_ptr = unsafe { (surface.ptr as *mut u32).add(offset) };
            let dst_px = unsafe { *dst_ptr };

            let sa = alpha;
            let da = 255 - sa;

            let src_r = (src_px >> 16) & 0xFF;
            let src_g = (src_px >> 8) & 0xFF;
            let src_b = src_px & 0xFF;

            let dst_r = (dst_px >> 16) & 0xFF;
            let dst_g = (dst_px >> 8) & 0xFF;
            let dst_b = dst_px & 0xFF;

            let out_r = (src_r * sa + dst_r * da) >> 8;
            let out_g = (src_g * sa + dst_g * da) >> 8;
            let out_b = (src_b * sa + dst_b * da) >> 8;

            unsafe { *dst_ptr = (out_r << 16) | (out_g << 8) | out_b; }
        }
    }
}
