//! Precomputed shadow cache - eliminates per-frame Vec allocation and blur.
//!
//! Instead of computing box blur at runtime, we precompute a single
//! corner tile at startup and use 9-slice rendering for all shadows.

use alloc::vec;
use alloc::vec::Vec;

/// Precomputed shadow corner tile.
/// 
/// A shadow with blur radius R can be rendered using a single corner tile
/// of size (R+1) x (R+1) that is mirrored/replicated for all 4 corners,
/// with solid fill for edges and center.
pub struct ShadowCache {
    /// Corner tile alpha values (top-left corner, flipped for others).
    /// Size is (blur_radius + 1) x (blur_radius + 1).
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
        
        // The corner tile captures the falloff from edge to transparent
        let tile_size = blur_radius + 1;
        let ts = tile_size as usize;
        let mut corner = vec![0u8; ts * ts];
        
        // Build a simple radial falloff for the corner
        // The corner represents what happens at distance from the shadow edge
        for y in 0..ts {
            for x in 0..ts {
                // Distance from the inner corner (blur_radius, blur_radius)
                // which is the opaque part
                let dx = blur_radius as f32 - x as f32;
                let dy = blur_radius as f32 - y as f32;
                
                if dx <= 0.0 && dy <= 0.0 {
                    // Inside the solid region
                    corner[y * ts + x] = 255;
                } else {
                    // Distance from edge
                    let dist = (dx.max(0.0) * dx.max(0.0) + dy.max(0.0) * dy.max(0.0)).sqrt();
                    
                    // Linear falloff within blur radius
                    let alpha = if dist >= blur_radius as f32 {
                        0.0
                    } else {
                        1.0 - dist / blur_radius as f32
                    };
                    
                    // Apply simple gaussian-like curve for smoother falloff
                    let alpha = alpha * alpha * (3.0 - 2.0 * alpha); // smoothstep
                    corner[y * ts + x] = (alpha * 255.0) as u8;
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
    /// 
    /// (0, 0) is top-left of the shadow (including blur margin).
    /// The shadow has dimensions: (width + 2*blur, height + 2*blur).
    #[inline]
    pub fn alpha_at(&self, x: i32, y: i32, inner_w: u32, inner_h: u32) -> u8 {
        if self.blur_radius == 0 {
            // No blur - binary alpha
            if x >= 0 && y >= 0 && x < inner_w as i32 && y < inner_h as i32 {
                return 255;
            }
            return 0;
        }
        
        let blur = self.blur_radius as i32;
        let ts = self.tile_size as i32;
        
        // Total shadow dimensions
        let total_w = inner_w as i32 + 2 * blur;
        let total_h = inner_h as i32 + 2 * blur;
        
        // Bounds check
        if x < 0 || y < 0 || x >= total_w || y >= total_h {
            return 0;
        }
        
        // Determine which region we're in (9-slice)
        let in_left = x < ts;
        let in_right = x >= total_w - ts;
        let in_top = y < ts;
        let in_bottom = y >= total_h - ts;
        
        // Center region - fully opaque
        if !in_left && !in_right && !in_top && !in_bottom {
            return 255;
        }
        
        // Edge regions - use 1D falloff
        if !in_top && !in_bottom {
            // Horizontal edge
            if in_left {
                return self.corner[(ts - 1) as usize * ts as usize + x as usize];
            } else {
                let rx = total_w - 1 - x;
                return self.corner[(ts - 1) as usize * ts as usize + rx as usize];
            }
        }
        
        if !in_left && !in_right {
            // Vertical edge
            if in_top {
                return self.corner[y as usize * ts as usize + (ts - 1) as usize];
            } else {
                let ry = total_h - 1 - y;
                return self.corner[ry as usize * ts as usize + (ts - 1) as usize];
            }
        }
        
        // Corner regions - sample from precomputed tile
        let (tx, ty) = if in_left && in_top {
            // Top-left corner - direct
            (x, y)
        } else if in_right && in_top {
            // Top-right corner - flip x
            (total_w - 1 - x, y)
        } else if in_left && in_bottom {
            // Bottom-left corner - flip y
            (x, total_h - 1 - y)
        } else {
            // Bottom-right corner - flip both
            (total_w - 1 - x, total_h - 1 - y)
        };
        
        let tx = tx.min(ts - 1).max(0) as usize;
        let ty = ty.min(ts - 1).max(0) as usize;
        
        self.corner[ty * ts as usize + tx]
    }
}

/// Global shadow cache - computed once at startup.
static mut SHADOW_CACHE: Option<ShadowCache> = None;

/// Initialize the global shadow cache.
pub fn init_shadow_cache(blur_radius: u32) {
    unsafe {
        SHADOW_CACHE = Some(ShadowCache::new(blur_radius));
    }
}

/// Get the global shadow cache.
pub fn get_shadow_cache() -> Option<&'static ShadowCache> {
    unsafe { SHADOW_CACHE.as_ref() }
}
