extern crate alloc;
use alloc::vec::Vec;
use micromath::F32Ext; // Added F32Ext for sqrt

/// A NineSlice asset defines a texture and how it should be sliced.
///
/// The texture is divided into 9 regions:
/// - 4 corners (fixed size)
/// - 4 edges (stretched or tiled)
/// - 1 center (stretched or tiled)
pub struct NineSlice {
    /// Pixels in premultiplied ARGB format (0xAARRGGBB)
    pub pixels: Vec<u32>,
    pub width: u32,
    pub height: u32,
    /// Inset from left edge
    pub left: u32,
    /// Inset from right edge
    pub right: u32,
    /// Inset from top edge
    pub top: u32,
    /// Inset from bottom edge
    pub bottom: u32,
}

impl NineSlice {
    pub fn new(pixels: Vec<u32>, width: u32, height: u32, left: u32, right: u32, top: u32, bottom: u32) -> Self {
        Self {
            pixels,
            width,
            height,
            left,
            right,
            top,
            bottom,
        }
    }
}

pub fn default_shadow() -> NineSlice {
    let size = 32;
    let center = size / 2;
    let radius = (size / 2 - 4) as f32; // leave some border
    let mut pixels = alloc::vec![0u32; (size * size) as usize];

    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - center as i32;
            let dy = y as i32 - center as i32;
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            let alpha = if dist < radius {
                let norm = dist / radius;
                // Cubic ease-out for softness
                let t = norm;
                let val = (1.0 - t) * (1.0 - t); 
                (val * 160.0) as u32 
            } else {
                0
            };
            // Fix parens warning
            pixels[(y * size + x) as usize] = alpha.min(255) << 24;
        }
    }

    NineSlice::new(
        pixels,
        size,
        size,
        center-1, 
        size-(center+1),
        center-1,
        size-(center+1)
    )
}

/// Renders a NineSlice asset into a destination buffer.
pub unsafe fn draw_nine_slice(
    dest: *mut u32,
    screen_w: u32,
    screen_h: u32,
    slice: &NineSlice,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
) {
    let x0 = x;
    let x1 = x + slice.left as i32;
    let x2 = x + w as i32 - slice.right as i32;
    let x3 = x + w as i32;

    let y0 = y;
    let y1 = y + slice.top as i32;
    let y2 = y + h as i32 - slice.bottom as i32;
    let y3 = y + h as i32;

    // Top-Left
    draw_rect(dest, screen_w, screen_h, slice, 
              0, 0, slice.left, slice.top, 
              x0, y0, (x1-x0) as u32, (y1-y0) as u32);

    // Top
    draw_rect(dest, screen_w, screen_h, slice, 
              slice.left, 0, slice.width - slice.left - slice.right, slice.top,
              x1, y0, (x2-x1) as u32, (y1-y0) as u32);

    // Top-Right
    draw_rect(dest, screen_w, screen_h, slice, 
              slice.width - slice.right, 0, slice.right, slice.top,
              x2, y0, (x3-x2) as u32, (y1-y0) as u32);

    // Left
    draw_rect(dest, screen_w, screen_h, slice, 
              0, slice.top, slice.left, slice.height - slice.top - slice.bottom,
              x0, y1, (x1-x0) as u32, (y2-y1) as u32);

    // Center
    draw_rect(dest, screen_w, screen_h, slice, 
              slice.left, slice.top, slice.width - slice.left - slice.right, slice.height - slice.top - slice.bottom,
              x1, y1, (x2-x1) as u32, (y2-y1) as u32);

    // Right
    draw_rect(dest, screen_w, screen_h, slice, 
              slice.width - slice.right, slice.top, slice.right, slice.height - slice.top - slice.bottom,
              x2, y1, (x3-x2) as u32, (y2-y1) as u32);

    // Bottom-Left
    draw_rect(dest, screen_w, screen_h, slice, 
              0, slice.height - slice.bottom, slice.left, slice.bottom,
              x0, y2, (x1-x0) as u32, (y3-y2) as u32);

    // Bottom
    draw_rect(dest, screen_w, screen_h, slice, 
              slice.left, slice.height - slice.bottom, slice.width - slice.left - slice.right, slice.bottom,
              x1, y2, (x2-x1) as u32, (y3-y2) as u32);

    // Bottom-Right
    draw_rect(dest, screen_w, screen_h, slice, 
              slice.width - slice.right, slice.height - slice.bottom, slice.right, slice.bottom,
              x2, y2, (x3-x2) as u32, (y3-y2) as u32);
}

/// Helper to draw a portion of the source texture into the destination
unsafe fn draw_rect(
    dest: *mut u32,
    screen_w: u32,
    screen_h: u32,
    slice: &NineSlice,
    src_x: u32, src_y: u32, src_w: u32, src_h: u32,
    dst_x: i32, dst_y: i32, dst_w: u32, dst_h: u32,
) {
    if dst_w == 0 || dst_h == 0 { return; }

    // Manual clipping/intersection
    // Target rect: dst_x, dst_y, dst_w, dst_h
    // Screen rect: 0, 0, screen_w, screen_h
    
    let clip_x = dst_x.max(0);
    let clip_y = dst_y.max(0);
    let clip_r = (dst_x + dst_w as i32).min(screen_w as i32);
    let clip_b = (dst_y + dst_h as i32).min(screen_h as i32);

    if clip_x >= clip_r || clip_y >= clip_b {
        return;
    }

    // We only iterate over the clipped region
    for screen_y in clip_y..clip_b {
        let dy = screen_y - dst_y; // relative to dest rect
        if dy < 0 { continue; } // should be covered by clip_y
        
        // Map dy to sy (nearest neighbor for now)
        let sy_rel = (dy * src_h as i32) / dst_h as i32;
        let sy = src_y as i32 + sy_rel;
        if sy >= (src_y + src_h) as i32 { continue; }

        let src_row_start = (sy * slice.width as i32) as usize;

        for screen_x in clip_x..clip_r {
            let dx = screen_x - dst_x;
            
            let sx_rel = (dx * src_w as i32) / dst_w as i32;
            let sx = src_x as i32 + sx_rel;
            
            let src_idx = src_row_start + sx as usize;
            let src_pixel = slice.pixels[src_idx];
            
            if (src_pixel >> 24) == 0 { continue; }

            let dest_idx = (screen_y as u32 * screen_w + screen_x as u32) as usize;
            let dst_pixel = *dest.add(dest_idx);
            
            *dest.add(dest_idx) = crate::pixels::blend_pixel(src_pixel, dst_pixel);
        }
    }
}
