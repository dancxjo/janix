use crate::pixels::blend_pixel;
use crate::scene::Rect;

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
}

impl<'a> ShadowMask<'a> {
    fn alpha_at(&self, x: i32, y: i32) -> u8 {
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
                // Corner check: use circle of radius r at each corner
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
        }
    }

    fn bounds_with_blur(&self, origin_x: i32, origin_y: i32, blur: u32) -> Rect {
        let (w, h) = match self {
            ShadowMask::SpriteAlpha { width, height, .. } => (*width, *height),
            ShadowMask::RoundedRect { width, height, .. } => (*width, *height),
        };
        let inflate = blur as i32;
        Rect {
            x: origin_x - inflate,
            y: origin_y - inflate,
            w: w + (blur * 2),
            h: h + (blur * 2),
        }
    }
}

/// Draw a soft shadow given an analytic mask.
///
/// The same kernel is used for cursor sprites (SpriteAlpha) and analytic shapes (RoundedRect).
pub unsafe fn draw_shadow_from_mask(
    dest: *mut u32,
    screen_w: u32,
    screen_h: u32,
    origin_x: i32,
    origin_y: i32,
    mask: ShadowMask<'_>,
    params: ShadowParams,
) {
    let bounds = mask.bounds_with_blur(origin_x + params.offset_x, origin_y + params.offset_y, params.blur_radius);
    let blur = params.blur_radius as i32;
    if bounds.w == 0 || bounds.h == 0 {
        return;
    }
    for y in bounds.y.max(0) as u32..((bounds.y + bounds.h as i32).min(screen_h as i32).max(0) as u32) {
        for x in bounds.x.max(0) as u32..((bounds.x + bounds.w as i32).min(screen_w as i32).max(0) as u32) {
            // Map back to mask space (remove offset but keep blur margin)
            let mx = x as i32 - params.offset_x - origin_x;
            let my = y as i32 - params.offset_y - origin_y;

            // Box blur in mask space
            let mut acc = 0u32;
            let mut samples = 0u32;
            for by in -blur..=blur {
                for bx in -blur..=blur {
                    let sx = mx + bx;
                    let sy = my + by;
                    let a = mask.alpha_at(sx, sy) as u32;
                    acc += a;
                    samples += 1;
                }
            }
            if samples == 0 {
                continue;
            }
            let avg_alpha = (acc / samples).min(255) as u8;
            if avg_alpha == 0 {
                continue;
            }

            // Apply global color/opacity
            let base_a = ((params.color >> 24) & 0xFF) as u32;
            let tint = params.color & 0x00FF_FFFF;
            let final_a = ((avg_alpha as u32) * base_a / 255).min(255) as u8;
            if final_a == 0 {
                continue;
            }
            let src = (final_a as u32) << 24 | tint;
            let idx = (y * screen_w + x) as usize;
            let dst = *dest.add(idx);
            *dest.add(idx) = blend_pixel(src, dst);
        }
    }
}
