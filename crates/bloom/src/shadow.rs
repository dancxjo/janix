use crate::painter::cpu::blend_pixel;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub struct ShadowParams {
    pub offset_x: i32,
    pub offset_y: i32,
    pub blur_radius: u32,
    pub color: u32, // ARGB, non-premultiplied
}

pub enum ShadowMask<'a> {
    SpriteAlpha { pixels: &'a [u32], width: u32, height: u32 },
    RoundedRect { width: u32, height: u32, radius: u16 },
    RoundedRectTop { width: u32, height: u32, radius: u16 },
}

impl<'a> ShadowMask<'a> {
    /// Get alpha value at a point in the mask.
    pub fn alpha_at(&self, x: i32, y: i32) -> u8 {
        match self {
            ShadowMask::SpriteAlpha { pixels, width, height } => {
                if x < 0 || y < 0 {
                    return 0;
                }
                let x = x as u32;
                let y = y as u32;
                if x >= *width || y >= *height {
                    return 0;
                }
                let idx = (y * *width + x) as usize;
                let px = pixels.get(idx).copied().unwrap_or(0);
                ((px >> 24) & 0xFF) as u8
            }
            ShadowMask::RoundedRect { width, height, radius } => {
                if x < 0 || y < 0 {
                    return 0;
                }
                let w = *width as i32;
                let h = *height as i32;
                if x >= w || y >= h {
                    return 0;
                }
                if *radius == 0 {
                    return 255;
                }
                let r = *radius as i32;
                let cx = if x < r {
                    r - 1
                } else if x >= w - r {
                    w - r
                } else {
                    x
                };
                let cy = if y < r {
                    r - 1
                } else if y >= h - r {
                    h - r
                } else {
                    y
                };
                let dx = x - cx;
                let dy = y - cy;
                let dist2 = dx * dx + dy * dy;
                if dist2 <= (r * r) {
                    255
                } else {
                    0
                }
            }
            ShadowMask::RoundedRectTop { width, height, radius } => {
                if x < 0 || y < 0 {
                    return 0;
                }
                let w = *width as i32;
                let h = *height as i32;
                if x >= w || y >= h {
                    return 0;
                }
                if *radius == 0 {
                    return 255;
                }
                let r = *radius as i32;
                
                // Only round the top corners
                if y >= r {
                    // Below top radius area: simple rect logic (fully opaque inside)
                    return 255;
                }

                // We are in the top strip (y < r). Check x corners.
                let cx = if x < r {
                    r - 1
                } else if x >= w - r {
                    w - r
                } else {
                     // Middle x, top y < r. Inside.
                    return 255;
                };

                let cy = r - 1; // Top circle center y
                
                let dx = x - cx;
                let dy = y - cy;
                let dist2 = dx * dx + dy * dy;
                if dist2 <= (r * r) {
                    255
                } else {
                    0
                }
            }
        }
    }

    /// Get the dimensions of the mask.
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ShadowMask::SpriteAlpha { width, height, .. } => (*width, *height),
            ShadowMask::RoundedRect { width, height, .. } => (*width, *height),
            ShadowMask::RoundedRectTop { width, height, .. } => (*width, *height),
        }
    }
}

/// Draw a soft shadow using a fast separable box blur.
///
/// Instead of O(r²) per pixel, we use two O(r) passes (horizontal then vertical).
/// 
/// DEPRECATED: Use Painter::draw_shadow_mask instead.
pub unsafe fn draw_shadow_from_mask(
    dest: *mut u32,
    screen_w: u32,
    _screen_h: u32,
    origin_x: i32,
    origin_y: i32,
    mask: ShadowMask<'_>,
    params: ShadowParams,
) {
    let (mask_w, mask_h) = mask.dimensions();
    let blur = params.blur_radius as i32;
    
    if mask_w == 0 || mask_h == 0 {
        return;
    }
    
    // Compute the blurred alpha buffer dimensions (mask + blur margins)
    let buf_w = mask_w as i32 + blur * 2;
    let buf_h = mask_h as i32 + blur * 2;
    
    if buf_w <= 0 || buf_h <= 0 {
        return;
    }
    
    let buf_w = buf_w as usize;
    let buf_h = buf_h as usize;
    
    // Step 1: Rasterize the mask into a buffer (with blur margins)
    let mut alpha_buf: Vec<u16> = vec![0u16; buf_w * buf_h];
    for my in 0..buf_h {
        let mask_y = my as i32 - blur;
        for mx in 0..buf_w {
            let mask_x = mx as i32 - blur;
            let a = mask.alpha_at(mask_x, mask_y) as u16;
            alpha_buf[my * buf_w + mx] = a;
        }
    }
    
    // Step 2: Horizontal box blur
    if blur > 0 {
        let mut row_tmp: Vec<u16> = vec![0u16; buf_w];
        
        for y in 0..buf_h {
            let row_start = y * buf_w;
            
            for x in 0..buf_w {
                let left_clamped = (x as i32 - blur).max(0) as usize;
                let right_clamped = (x as i32 + blur).min(buf_w as i32 - 1) as usize;
                let count = (right_clamped - left_clamped + 1) as u32;
                
                let mut acc: u32 = 0;
                for bx in left_clamped..=right_clamped {
                    acc += alpha_buf[row_start + bx] as u32;
                }
                
                row_tmp[x] = (acc / count.max(1)) as u16;
            }
            
            // Copy back
            for x in 0..buf_w {
                alpha_buf[row_start + x] = row_tmp[x];
            }
        }
        
        // Step 3: Vertical box blur
        let mut col_tmp: Vec<u16> = vec![0u16; buf_h];
        
        for x in 0..buf_w {
            for y in 0..buf_h {
                let top = (y as i32 - blur).max(0) as usize;
                let bottom = (y as i32 + blur).min(buf_h as i32 - 1) as usize;
                let count = (bottom - top + 1) as u32;
                
                let mut acc: u32 = 0;
                for by in top..=bottom {
                    acc += alpha_buf[by * buf_w + x] as u32;
                }
                col_tmp[y] = (acc / count.max(1)) as u16;
            }
            
            // Copy back
            for y in 0..buf_h {
                alpha_buf[y * buf_w + x] = col_tmp[y];
            }
        }
    }
    
    // Step 4: Blit the blurred alpha to screen with shadow color
    let shadow_x = origin_x + params.offset_x - blur;
    let shadow_y = origin_y + params.offset_y - blur;
    
    let base_a = ((params.color >> 24) & 0xFF) as u32;
    let tint = params.color & 0x00FF_FFFF;
    
    // Clip to screen
    let x0 = shadow_x.max(0);
    let y0 = shadow_y.max(0);
    let x1 = (shadow_x + buf_w as i32).min(screen_w as i32);
    let y1 = (shadow_y + buf_h as i32).min(screen_w as i32); // using screen_w as proxy for height bound
    
    for screen_y in y0..y1 {
        let buf_y = (screen_y - shadow_y) as usize;
        for screen_x in x0..x1 {
            let buf_x = (screen_x - shadow_x) as usize;
            let avg_alpha = alpha_buf[buf_y * buf_w + buf_x].min(255) as u8;
            
            if avg_alpha == 0 {
                continue;
            }
            
            let final_a = ((avg_alpha as u32) * base_a / 255).min(255) as u8;
            if final_a == 0 {
                continue;
            }
            
            let src = (final_a as u32) << 24 | tint;
            let idx = (screen_y as u32 * screen_w + screen_x as u32) as usize;
            let dst = *dest.add(idx);
            *dest.add(idx) = blend_pixel(src, dst);
        }
    }
}
