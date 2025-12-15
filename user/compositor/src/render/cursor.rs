use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};

use crate::config::{CURSOR_COLOR as COLOR_CURSOR_PRIMARY, CURSOR_SHADOW as COLOR_CURSOR_SHADOW};
use crate::render::bitmap::Bitmap;
use crate::render::primitives::fill_rect;

pub const CURSOR_SIZE: usize = 98;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorKind {
    Arrow,
    Move,
    ResizeN,
    ResizeS,
    ResizeE,
    ResizeW,
    ResizeNE,
    ResizeNW,
    ResizeSE,
    ResizeSW,
}

#[derive(Clone, Debug)]
pub struct CursorIcon {
    pub bitmap: Arc<Bitmap>,
    pub hotspot: (i32, i32),
}

#[derive(Debug)]
pub struct CursorSprites {
    pub arrow: CursorIcon,
    pub move_icon: CursorIcon,
    pub resize_ns: CursorIcon,
    pub resize_ew: CursorIcon,
    pub resize_ne_sw: CursorIcon,
    pub resize_nw_se: CursorIcon,
}

impl CursorSprites {
    pub fn for_kind(&self, kind: CursorKind) -> &CursorIcon {
        match kind {
            CursorKind::Arrow => &self.arrow,
            CursorKind::Move => &self.move_icon,
            CursorKind::ResizeN | CursorKind::ResizeS => &self.resize_ns,
            CursorKind::ResizeE | CursorKind::ResizeW => &self.resize_ew,
            CursorKind::ResizeNE | CursorKind::ResizeSW => &self.resize_ne_sw,
            CursorKind::ResizeNW | CursorKind::ResizeSE => &self.resize_nw_se,
        }
    }
}

pub fn draw_cursor(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    cx: i32,
    cy: i32,
) {
    let size = 12;
    let x = cx.clamp(0, fb_width as i32 - 1);
    let y = cy.clamp(0, fb_height as i32 - 1);
    fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x - 1,
        y,
        size,
        2,
        COLOR_CURSOR_PRIMARY,
    );
     fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x,
        y - 1,
        2,
        size,
        COLOR_CURSOR_PRIMARY,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::render::primitives::raster_draw_cursor; 
    // use crate::render::bitmap::Bitmap; 
    // use crate::cursor::build_c    use super::*;

    #[test]
    fn build_cursor_sprites_generates_bitmaps() {
        let sprites = build_cursor_sprites();
        
        let arrow = &sprites.arrow.bitmap;
        assert_eq!(arrow.width, CURSOR_SIZE);
        assert_eq!(arrow.height, CURSOR_SIZE);
        
        // Check finding a filled pixel (arrow body).
        // (5, 5) should be inside the body.
        let target_idx = 5 + 5 * arrow.width;
        let p_target = arrow.data[target_idx];
        assert_ne!(p_target, 0, "Body of arrow should be visible at (5,5)");
    }

    #[test]
    fn raster_draw_cursor_blends_correctly() {
        // White background
        let mut buffer = vec![0xFFFFFFFF; 100]; 
        let stride = 10;
        let w = 10;
        let h = 10;
        
        // Simple 1x1 red sprite with 50% alpha
        // Alpha = 0x80 (128)
        // Color = Red (0xFF0000)
        // Premultiplied: R=128, G=0, B=0
        // Pixel = 0x80800000
        let sprite_data = vec![0x80800000];
        let sprite = Bitmap { width: 1, height: 1, data: sprite_data };
        
        raster_draw_cursor(buffer.as_mut_ptr(), stride, w, h, (5, 5), &sprite, (0, 0));
        
        // Dest was White (0xFF, 0xFF, 0xFF)
        // Src alpha = 128 (~0.5)
        // Inv alpha = 127
        // Out R = 128 + (255 * 127) / 255 = 128 + 127 = 255
        // Out G = 0 + (255 * 127) / 255 = 127
        // Out B = 0 + (255 * 127) / 255 = 127
        // Result should be roughly Pink/Light Red (0xFF, 0x7F, 0x7F) -> 0xFFFF7F7F or 0xFF807F7F depending on rounding.
        
        let idx = 5 * stride + 5;
        let px = buffer[idx as usize];
        let r = (px >> 16) & 0xFF;
        let g = (px >> 8) & 0xFF;
        let b = px & 0xFF;
        
        assert_eq!(r, 255);
        assert!(g >= 126 && g <= 128);  
        assert!(b >= 126 && b <= 128);
    }
}

// --- Procedural GenerationHelpers ---

struct Mask {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Mask {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![false; width * height],
        }
    }

    fn filled(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }
        self.data[(y as usize) * self.width + (x as usize)]
    }

    fn set(&mut self, x: i32, y: i32, val: bool) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        self.data[(y as usize) * self.width + (x as usize)] = val;
    }
}

fn fill_triangle(mask: &mut Mask, p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) {
    // Simple scanline algorithm or bounding box check
    let min_x = min(p1.0, min(p2.0, p3.0));
    let max_x = max(p1.0, max(p2.0, p3.0));
    let min_y = min(p1.1, min(p2.1, p3.1));
    let max_y = max(p1.1, max(p2.1, p3.1));

    let edge = |a: (i32, i32), b: (i32, i32), c: (i32, i32)| -> i32 {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    };

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = (x, y);
            let w0 = edge(p1, p2, p);
            let w1 = edge(p2, p3, p);
            let w2 = edge(p3, p1, p);

            if (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0) {
                mask.set(x, y, true);
            }
        }
    }
}

fn isqrt(n: u32) -> u32 {
    if n < 2 { return n; }
    let mut x = n / 2;
    let mut y = (x + n / x) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

fn draw_line_thick(mask: &mut Mask, p1: (i32, i32), p2: (i32, i32), thickness: i32) {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let len_sq = (dx * dx + dy * dy) as u32;
    let len = isqrt(len_sq) as i32;

    if len == 0 { return; }

    // Perpendicular vector normalized (scaled by 1024 for precision)
    // perp = (-dy, dx)
    // normalized: (-dy * 1024 / len, dx * 1024 / len)
    let scale = 1024;
    let half_w = (thickness * scale) / 2;

    let perp_x = (-dy * scale) / len;
    let perp_y = (dx * scale) / len;

    // Offsets
    let off_x = (perp_x * half_w / scale) / scale; // Note: half_w already scaled? No, half_w is scaled thickness.
    // Wait, half_w is (thickness/2)*1024.
    // perp_x is (dir)*1024.
    // offset = perp_x * (thickness/2) / 1024.
    // offset = ((-dy * 1024 / len) * (thickness * 1024 / 2)) / 1024 isn't right div logic.

    // Let's simplify:
    // offset_x = -dy * (thickness/2) / len
    // offset_y = dx * (thickness/2) / len
    
    let hw = thickness / 2;
    let off_x = (-dy * hw) / len;
    let off_y = (dx * hw) / len;

    // Polygon corners
    let c1 = (p1.0 + off_x, p1.1 + off_y);
    let c2 = (p1.0 - off_x, p1.1 - off_y);
    let c3 = (p2.0 - off_x, p2.1 - off_y);
    let c4 = (p2.0 + off_x, p2.1 + off_y);

    // Split into 2 triangles
    fill_triangle(mask, c1, c2, c3);
    fill_triangle(mask, c1, c3, c4);
}

fn make_arrow_mask() -> (Mask, (i32, i32)) {
    let mut mask = Mask::new(CURSOR_SIZE, CURSOR_SIZE);
    
    // Main arrow shape
    let _p1 = (0, 0);
    let _p2 = (0, 26);
    let _p3 = (17, 17); // Inner corner
    let _p4 = (26, 26); // Tail bottom
    
    // We can composite it from triangles
    // Standard pointer geometry
    // (0,0) Tip
    // (0, 22) Base Left
    // (4, 18) Notch
    // (9, 28) Tail Tip (Extended)
    // (13, 26) Tail End
    // (8, 16) Notch Return
    // (16, 16) Wing
     let poly = [
        (0, 0),    // Tip
        (0, 22),   // Left base
        (5, 18),   // Notch start (inner)
        (10, 30),  // Tail tip
        (14, 28),  // Tail right
        (9, 16),   // Notch end (outer)
        (18, 16),  // Right base
    ];
    
    // Fan triangulation from (0,0)
    let center = poly[0];
    for i in 1..poly.len()-1 {
        fill_triangle(&mut mask, center, poly[i], poly[i+1]);
    }
    
    (mask, (0, 0))
}

fn make_move_mask() -> (Mask, (i32, i32)) {
    let mut mask = Mask::new(CURSOR_SIZE, CURSOR_SIZE);
    let c = (32, 32);
    let s = 4; // shaft thickness half
    let len = 18;
    
    // Cross
    draw_line_thick(&mut mask, (c.0 - len, c.1), (c.0 + len, c.1), s * 2);
    draw_line_thick(&mut mask, (c.0, c.1 - len), (c.0, c.1 + len), s * 2);
    
    // Arrowheads
    // Top
    fill_triangle(&mut mask, (c.0, c.1 - len - 8), (c.0 - 8, c.1 - len), (c.0 + 8, c.1 - len));
    // Bottom
    fill_triangle(&mut mask, (c.0, c.1 + len + 8), (c.0 - 8, c.1 + len), (c.0 + 8, c.1 + len));
    // Left
    fill_triangle(&mut mask, (c.0 - len - 8, c.1), (c.0 - len, c.1 - 8), (c.0 - len, c.1 + 8));
    // Right
    fill_triangle(&mut mask, (c.0 + len + 8, c.1), (c.0 + len, c.1 - 8), (c.0 + len, c.1 + 8));
    
    (mask, c)
}

fn make_resize_ns_mask() -> (Mask, (i32, i32)) {
    let mut mask = Mask::new(CURSOR_SIZE, CURSOR_SIZE);
    let c = (32, 32);
    let s = 4;
    let len = 18;
    
    draw_line_thick(&mut mask, (c.0, c.1 - len), (c.0, c.1 + len), s * 2);
    
    fill_triangle(&mut mask, (c.0, c.1 - len - 8), (c.0 - 8, c.1 - len), (c.0 + 8, c.1 - len));
    fill_triangle(&mut mask, (c.0, c.1 + len + 8), (c.0 - 8, c.1 + len), (c.0 + 8, c.1 + len));
    
    (mask, c)
}

fn make_resize_ew_mask() -> (Mask, (i32, i32)) {
    let mut mask = Mask::new(CURSOR_SIZE, CURSOR_SIZE);
    let c = (32, 32);
    let s = 4;
    let len = 18;
    
    draw_line_thick(&mut mask, (c.0 - len, c.1), (c.0 + len, c.1), s * 2);
    
    fill_triangle(&mut mask, (c.0 - len - 8, c.1), (c.0 - len, c.1 - 8), (c.0 - len, c.1 + 8));
    fill_triangle(&mut mask, (c.0 + len + 8, c.1), (c.0 + len, c.1 - 8), (c.0 + len, c.1 + 8));
    
    (mask, c)
}

fn make_resize_nw_se_mask() -> (Mask, (i32, i32)) {
    let mut mask = Mask::new(CURSOR_SIZE, CURSOR_SIZE);
    let c = (32, 32);
    let s = 4;
    let _len = 14; 
    let d = (10.0 * 1.414) as i32; // ~14
    
    draw_line_thick(&mut mask, (c.0 - d, c.1 - d), (c.0 + d, c.1 + d), s * 2);
    
    // Rotate points 45deg... or just use fixed coords
    // Top-Left arrow
    // p1 = tip, p2, p3 base
    let tip = (c.0 - d - 6, c.1 - d - 6);
    fill_triangle(&mut mask, tip, (c.0 - d, c.1 - d - 8), (c.0 - d - 8, c.1 - d));
    
    // Bottom-Right
    let tip = (c.0 + d + 6, c.1 + d + 6);
    fill_triangle(&mut mask, tip, (c.0 + d, c.1 + d + 8), (c.0 + d + 8, c.1 + d));

    (mask, c)
}

fn make_resize_ne_sw_mask() -> (Mask, (i32, i32)) {
    let mut mask = Mask::new(CURSOR_SIZE, CURSOR_SIZE);
    let c = (32, 32);
    let s = 4;
    let d = 14;
    
    draw_line_thick(&mut mask, (c.0 + d, c.1 - d), (c.0 - d, c.1 + d), s * 2);

    // Top-Right
    let tip = (c.0 + d + 6, c.1 - d - 6);
    fill_triangle(&mut mask, tip, (c.0 + d, c.1 - d - 8), (c.0 + d + 8, c.1 - d));
    
    // Bottom-Left
    let tip = (c.0 - d - 6, c.1 + d + 6);
    fill_triangle(&mut mask, tip, (c.0 - d, c.1 + d + 8), (c.0 - d - 8, c.1 + d));
    
    (mask, c)
}

fn cursor_icon_from_mask(
    input: (Mask, (i32, i32)),
    fill_color: u32,
    outline_color: u32,
    shadow_color: u32,
) -> CursorIcon {
    let (mask, hotspot) = input;
    let w = mask.width;
    let h = mask.height;
    let mut pixels = vec![0u32; w * h];
    
    // 1. Generate shadow map
    let mut shadow_alpha = vec![0u8; w * h];
    let offset_x = 4;
    let offset_y = 4;
    
    for y in 0..h {
        for x in 0..w {
            if mask.filled(x as i32, y as i32) {
                let sx = x + offset_x;
                let sy = y + offset_y;
                if sx < w && sy < h {
                    shadow_alpha[sy * w + sx] = 0xA0;
                }
            }
        }
    }
    
    // 2. Blur shadow
    for _ in 0..3 {
        let src = shadow_alpha.clone();
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let mut sum: u32 = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let val = src[((y as isize + dy) as usize) * w + ((x as isize + dx) as usize)];
                        sum += val as u32;
                    }
                }
                shadow_alpha[y * w + x] = (sum / 9) as u8;
            }
        }
    }
    
    // 3. Composite shadow
    for i in 0..pixels.len() {
        let a = shadow_alpha[i];
        if a > 0 {
             // Shadow color is usually black, apply alpha
            let sa = ((shadow_color >> 24) & 0xFF) * a as u32 / 255;
            let sr = (shadow_color >> 16) & 0xFF;
            let sg = (shadow_color >> 8) & 0xFF;
            let sb = (shadow_color) & 0xFF;
            
            // Premultiply
            let r = (sr * sa) / 255;
            let g = (sg * sa) / 255;
            let b = (sb * sa) / 255;
            
            pixels[i] = (sa << 24) | (r << 16) | (g << 8) | b;
        }
    }
    
    // 4. Composite shape
    for y in 0..h {
        for x in 0..w {
            if mask.filled(x as i32, y as i32) {
                 // Check neighbors for outline
                 let neighbors = [
                    (x as i32 - 1, y as i32),
                    (x as i32 + 1, y as i32),
                    (x as i32, y as i32 - 1),
                    (x as i32, y as i32 + 1),
                ];
                let is_edge = neighbors.iter().any(|(nx, ny)| !mask.filled(*nx, *ny));
                let color = if is_edge { outline_color } else { fill_color };
                
                let a = (color >> 24) & 0xFF;
                if a > 0 {
                    let r = ((color >> 16) & 0xFF) * a / 255;
                    let g = ((color >> 8) & 0xFF) * a / 255;
                    let b = (color & 0xFF) * a / 255;
                    pixels[y * w + x] = (a << 24) | (r << 16) | (g << 8) | b;
                }
            }
        }
    }
    
    CursorIcon {
        bitmap: Arc::new(Bitmap {
            width: w,
            height: h,
            data: pixels,
        }),
        hotspot,
    }
}

pub fn build_cursor_sprites() -> CursorSprites {
    let outline = crate::config::FRAME_BORDER; // Dark
    let fill = COLOR_CURSOR_PRIMARY;
    let shadow = COLOR_CURSOR_SHADOW;

    let arrow = cursor_icon_from_mask(make_arrow_mask(), fill, outline, shadow);
    let move_icon = cursor_icon_from_mask(make_move_mask(), fill, outline, shadow);
    let resize_ns = cursor_icon_from_mask(make_resize_ns_mask(), fill, outline, shadow);
    let resize_ew = cursor_icon_from_mask(make_resize_ew_mask(), fill, outline, shadow);
    let resize_nw_se = cursor_icon_from_mask(make_resize_nw_se_mask(), fill, outline, shadow);
    let resize_ne_sw = cursor_icon_from_mask(make_resize_ne_sw_mask(), fill, outline, shadow);

    CursorSprites { arrow, move_icon, resize_ns, resize_ew, resize_ne_sw, resize_nw_se }
}

pub fn raster_draw_cursor(
    buffer: *mut u32,
    stride: u32,
    fb_width: u32,
    fb_height: u32,
    origin: (i32, i32),
    sprite: &Bitmap,
    hotspot: (i32, i32),
) {
    /*
    if origin.0 > 100 { // Reduced spam
         let msg = alloc::format!("draw_cursor sat {:?} fb={}x{}", origin, fb_width, fb_height);
         let leaked = alloc::boxed::Box::leak(msg.into_boxed_str());
         unsafe {
             // We don't have sys here easily! 
             // We can't print easily without Sys.
             // But we can verify safety logic.
         }
    }
    */
    let top_left_x = origin.0 - hotspot.0;
    let top_left_y = origin.1 - hotspot.1;
    
    let start_x = max(0, top_left_x);
    let start_y = max(0, top_left_y);
    let end_x = min(fb_width as i32, top_left_x + sprite.width as i32);
    let end_y = min(fb_height as i32, top_left_y + sprite.height as i32);
    
    if start_x >= end_x || start_y >= end_y {
        return;
    }
    
    let sprite_width = sprite.width;
    let sprite_data = &sprite.data;

    for y in start_y..end_y {
        let sy = (y - top_left_y) as usize;
        let sx_start = (start_x - top_left_x) as usize;
        let sx_end = (end_x - top_left_x) as usize;

        let sprite_row = &sprite_data[sy * sprite_width + sx_start..sy * sprite_width + sx_end];
        let dst_row_start = (y as usize) * (stride as usize) + (start_x as usize);
        
        unsafe {
            let dst_ptr = buffer.add(dst_row_start);
            
            for (i, px) in sprite_row.iter().enumerate() {
                let px = *px;
                let alpha = (px >> 24) & 0xFF;
                if alpha == 0 { continue; }

                let dst = dst_ptr.add(i);
                
                if alpha == 0xFF {
                    *dst = px;
                } else {
                    let inv_a = 255 - alpha;
                    let dst_val = *dst;

                    let dst_r = (dst_val >> 16) & 0xFF;
                    let dst_g = (dst_val >> 8) & 0xFF;
                    let dst_b = dst_val & 0xFF;

                    // NOTE: src is premultiplied already
                    let src_r = (px >> 16) & 0xFF;
                    let src_g = (px >> 8) & 0xFF;
                    let src_b = px & 0xFF;

                    let r = src_r + (dst_r * inv_a) / 255;
                    let g = src_g + (dst_g * inv_a) / 255;
                    let b = src_b + (dst_b * inv_a) / 255;

                    *dst = 0xFF000000 | (r << 16) | (g << 8) | b;
                }
            }
        }
    }
}
