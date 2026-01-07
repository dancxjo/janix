#![no_std]
//! Shared boot progress logic for kernel and userspace
//! 
//! Provides color calculation based on theme for boot animation.

/// Total steps in kernel boot sequence (for scaling)
pub const KERNEL_BOOT_STEPS: u32 = 6;

/// Calculate progress color: lerp from half to full theme dominant color.
/// Returns (R, G, B) tuple.
/// 
/// - step 0 = half brightness (where kernel ends)
/// - step == max_step = full brightness
pub fn progress_color(step: u32, max_step: u32) -> (u8, u8, u8) {
    let half = theme::current::DOMINANT_HALF_ARGB;
    let full = theme::current::DOMINANT_COLOR_ARGB;
    
    if max_step == 0 {
        return extract_rgb(full);
    }
    
    let t = if step >= max_step {
        255u8
    } else {
        ((step * 255) / max_step) as u8
    };
    
    lerp_rgb(half, full, t)
}

/// Calculate boot fade color: kernel uses this to fade from black to half
pub fn kernel_boot_color(step: u32, max_step: u32) -> (u8, u8, u8) {
    let r = theme::current::DOMINANT_R;
    let g = theme::current::DOMINANT_G;
    let b = theme::current::DOMINANT_B;
    
    if max_step == 0 {
        return (0, 0, 0);
    }
    
    // Scale to 50% max brightness (half of dominant color)
    let clamped = step.min(max_step);
    let scale = |c: u8| ((c as u32 * clamped) / (max_step * 2)) as u8;
    
    (scale(r), scale(g), scale(b))
}

fn extract_rgb(argb: u32) -> (u8, u8, u8) {
    (
        ((argb >> 16) & 0xFF) as u8,
        ((argb >> 8) & 0xFF) as u8,
        (argb & 0xFF) as u8,
    )
}

fn lerp_rgb(a: u32, b: u32, t: u8) -> (u8, u8, u8) {
    let lerp = |a: u8, b: u8| -> u8 {
        let a32 = a as u32;
        let b32 = b as u32;
        let t32 = t as u32;
        ((a32 * (255 - t32) + b32 * t32) / 255) as u8
    };
    
    let (ar, ag, ab) = extract_rgb(a);
    let (br, bg, bb) = extract_rgb(b);
    
    (lerp(ar, br), lerp(ag, bg), lerp(ab, bb))
}
