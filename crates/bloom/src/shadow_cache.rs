//! Precomputed shadow cache - eliminates per-frame Vec allocation and blur.
//!
//! Uses integer math only (no libm/sqrt) for no_std compatibility.

use alloc::vec;
use alloc::vec::Vec;

/// Precomputed shadow corner tile.
pub struct ShadowCache {
    /// Corner tile alpha values (top-left corner, flipped for others).
    corner: Vec<u8>,
    
    /// The blur radius this cache was computed for.
    pub blur_radius: u32,
    
    /// Corner tile size.
    pub tile_size: u32,
}

impl ShadowCache {
    /// Create a new shadow cache for the given blur radius.
    pub fn new(blur_radius: u32) -> Self {
        if blur_radius == 0 {
            return Self {
                corner: vec![255],
                blur_radius: 0,
                tile_size: 1,
            };
        }
        
        let tile_size = blur_radius + 1;
        let ts = tile_size as usize;
        let mut corner = vec![0u8; ts * ts];
        
        let br = blur_radius as i32;
        let br_sq = br * br;
        
        for y in 0..ts {
            for x in 0..ts {
                // Distance from the inner corner (blur_radius, blur_radius)
                let dx = br - x as i32;
                let dy = br - y as i32;
                
                if dx <= 0 && dy <= 0 {
                    // Inside the solid region
                    corner[y * ts + x] = 255;
                } else {
                    // Distance squared from edge
                    let dx_clamped = if dx > 0 { dx } else { 0 };
                    let dy_clamped = if dy > 0 { dy } else { 0 };
                    let dist_sq = dx_clamped * dx_clamped + dy_clamped * dy_clamped;
                    
                    // Linear falloff based on distance squared (approximation)
                    if dist_sq >= br_sq {
                        corner[y * ts + x] = 0;
                    } else {
                        // sqrt approximation using integer math
                        // alpha = 1 - sqrt(dist_sq) / blur_radius
                        // ≈ 1 - dist_sq / br_sq (for small blur, this underestimates falloff)
                        // Use lookup table or simple linear approximation
                        let ratio = (dist_sq * 255) / br_sq.max(1);
                        let alpha = 255i32 - ratio;
                        corner[y * ts + x] = alpha.max(0) as u8;
                    }
                }
            }
        }
        
        Self {
            corner,
            blur_radius,
            tile_size,
        }
    }
    
    /// Get alpha value for a point relative to shadow origin.
    #[inline]
    pub fn alpha_at(&self, x: i32, y: i32, inner_w: u32, inner_h: u32) -> u8 {
        if self.blur_radius == 0 {
            if x >= 0 && y >= 0 && x < inner_w as i32 && y < inner_h as i32 {
                return 255;
            }
            return 0;
        }
        
        let blur = self.blur_radius as i32;
        let ts = self.tile_size as i32;
        
        let total_w = inner_w as i32 + 2 * blur;
        let total_h = inner_h as i32 + 2 * blur;
        
        if x < 0 || y < 0 || x >= total_w || y >= total_h {
            return 0;
        }
        
        // 9-slice regions
        let in_left = x < ts;
        let in_right = x >= total_w - ts;
        let in_top = y < ts;
        let in_bottom = y >= total_h - ts;
        
        // Center - fully opaque
        if !in_left && !in_right && !in_top && !in_bottom {
            return 255;
        }
        
        // Horizontal edges
        if !in_top && !in_bottom {
            if in_left {
                return self.corner[(ts - 1) as usize * ts as usize + x as usize];
            } else {
                let rx = total_w - 1 - x;
                return self.corner[(ts - 1) as usize * ts as usize + rx as usize];
            }
        }
        
        // Vertical edges
        if !in_left && !in_right {
            if in_top {
                return self.corner[y as usize * ts as usize + (ts - 1) as usize];
            } else {
                let ry = total_h - 1 - y;
                return self.corner[ry as usize * ts as usize + (ts - 1) as usize];
            }
        }
        
        // Corner regions
        let (tx, ty) = if in_left && in_top {
            (x, y)
        } else if in_right && in_top {
            (total_w - 1 - x, y)
        } else if in_left && in_bottom {
            (x, total_h - 1 - y)
        } else {
            (total_w - 1 - x, total_h - 1 - y)
        };
        
        let tx = tx.min(ts - 1).max(0) as usize;
        let ty = ty.min(ts - 1).max(0) as usize;
        
        self.corner[ty * ts as usize + tx]
    }
}

static mut SHADOW_CACHE: Option<ShadowCache> = None;

pub fn init_shadow_cache(blur_radius: u32) {
    unsafe {
        SHADOW_CACHE = Some(ShadowCache::new(blur_radius));
    }
}

pub fn get_shadow_cache() -> Option<&'static ShadowCache> {
    unsafe { SHADOW_CACHE.as_ref() }
}
