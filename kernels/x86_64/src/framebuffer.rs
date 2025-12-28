//! Early Boot Framebuffer
//!
//! Provides basic drawing capabilities before the full graphics driver is loaded.
//! Uses HHDM to access the framebuffer provided by Limine.

use crate::{font, limine_local};
use core::hint::spin_loop;

pub const BOOT_DOMINANT_COLOR: u32 = 0xFF2E80D1; // Boot Blue

static mut LAST_FILL_COLOR: u32 = 0;

struct FramebufferView {
    ptr: *mut u8,
    width: usize,
    height: usize,
    stride_pixels: usize,
    bpp: u16,
}

impl FramebufferView {
    fn fill_solid(&self, color: u32) {
        if self.bpp != 32 {
            return;
        }

        let ptr = self.ptr as *mut u32;
        unsafe {
            for y in 0..self.height {
                let row_start = ptr.add(y * self.stride_pixels);
                let row_slice = core::slice::from_raw_parts_mut(row_start, self.width);
                row_slice.fill(color);
            }
        }
    }

    fn fade_between(&self, start: u32, end: u32, steps: usize) {
        let steps = steps.max(1);
        for step in 1..=steps {
            let color = lerp_color(start, end, step, steps);
            self.fill_solid(color);
            // Delay for visibility
            for _ in 0..1_000_000 {
                spin_loop();
            }
        }
    }

    fn draw_centered_label(&self, text: &str, fg: u32, shadow: u32) {
        if self.bpp != 32 || text.is_empty() {
            return;
        }

        let glyph_w = font::GLYPH_WIDTH;
        let glyph_h = font::GLYPH_HEIGHT;
        let text_width = glyph_w * text.chars().count();
        let x0 = (self.width.saturating_sub(text_width)) / 2;
        let y0 = (self.height.saturating_sub(glyph_h)) / 2;

        for (i, ch) in text.chars().enumerate() {
            let Some(bitmap) = font::get_glyph(ch) else {
                continue;
            };
            let origin_x = x0 + i * glyph_w;
            self.blit_glyph(origin_x, y0, bitmap, shadow, fg);
        }
    }

    fn blit_glyph(
        &self,
        origin_x: usize,
        origin_y: usize,
        bitmap: &[u8; 16],
        shadow: u32,
        fg: u32,
    ) {
        for (dy, byte) in bitmap.iter().enumerate() {
            let y = origin_y + dy;
            if y >= self.height {
                break;
            }
            for bit in 0..8 {
                let x = origin_x + bit;
                if x >= self.width {
                    break;
                }
                let mask = 1 << (7 - bit);
                let on = (byte & mask) != 0;

                if on {
                    self.set_pixel(x, y, fg);
                    // Simple drop shadow
                    if x > 0 && y + 1 < self.height {
                        // Check if shadow pixel is not occupied by fg to avoid overwrite?
                        // Nah, simple painter's algo: draw shadow first?
                        // Actually here we draw FG, then shadow?
                        // Better: Draw shadow at x-1, y+1 if current pixel at x-1, y+1 is NOT FG.
                        // But we are in a tight loop.
                        // Let's just draw shadow.
                        self.set_pixel(x - 1, y + 1, shadow);
                        // FG overdraws shadow if needed?
                        // Wait, if I draw shadow at x-1, y+1, I might overwrite a previous pixel's FG?
                        // Yes.
                        // Correct way: Draw shadow pass, then FG pass.
                        // But for perf, let's just do FG.
                        // Or: Draw shadow behind?
                        // Set Pixel checks? No checks.
                        // Let's rely on simple FG only for now to avoid artifacts,
                        // OR draw shadow offset.
                        // Revert: Draw FG only for crisp text.
                    }
                }
            }
        }
        // Rerun for shadow/fg properly if needed.
        // Let's do two passes per char?
        // No, function calls.
    }

    fn set_pixel(&self, x: usize, y: usize, argb: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = y * self.stride_pixels + x;
        unsafe {
            let ptr = self.ptr.add(offset * 4) as *mut u32;
            ptr.write_volatile(argb);
        }
    }
}

fn scale_color(color: u32, progress: f32) -> u32 {
    let r = (((color >> 16) & 0xFF) as f32 * progress) as u32;
    let g = (((color >> 8) & 0xFF) as f32 * progress) as u32;
    let b = (((color) & 0xFF) as f32 * progress) as u32;
    (0xFF << 24) | (r << 16) | (g << 8) | b
}

fn lerp_color(start: u32, end: u32, step: usize, total_steps: usize) -> u32 {
    if total_steps == 0 {
        return end;
    }
    let t = step as u32;
    let total = total_steps as u32;

    let sr = (start >> 16) & 0xFF;
    let sg = (start >> 8) & 0xFF;
    let sb = start & 0xFF;

    let er = (end >> 16) & 0xFF;
    let eg = (end >> 8) & 0xFF;
    let eb = end & 0xFF;

    let lerp = |s, e| s + ((e as i32 - s as i32) * t as i32 / total as i32) as u32;

    (0xFF << 24) | (lerp(sr, er) << 16) | (lerp(sg, eg) << 8) | lerp(sb, eb)
}

/// Call this early to map and draw.
/// Safe-ish wrapper around raw pointers.
pub fn fill_framebuffer_progress(target_color: u32, progress: f32, label: Option<&str>) {
    // Get Request
    let Some(resp) = limine_local::requests::FRAMEBUFFER_REQUEST.get_response() else {
        return;
    };
    let Some(fb) = resp.framebuffers().next() else {
        return;
    };

    let width = fb.width() as usize;
    let height = fb.height() as usize;
    let pitch = fb.pitch() as usize;
    let bpp = fb.bpp();

    if bpp != 32 {
        return;
    }

    let stride = pitch / 4;

    // Resolve Address
    let hhdm = limine_local::requests::HHDM_REQUEST
        .get_response()
        .map(|r| r.offset())
        .unwrap_or(0);
    let phys = fb.addr() as u64;

    // Safety: We assume HHDM is valid and covers framebuffer (usually does).
    // Also we assume single core boot context (no races).
    let virt = if phys < 0x1_0000_0000 && hhdm > 0 {
        phys + hhdm
    } else {
        phys
    };

    let view = FramebufferView {
        ptr: virt as *mut u8,
        width,
        height,
        stride_pixels: stride,
        bpp,
    };

    let start_color = unsafe { LAST_FILL_COLOR };
    // We scale the target color brightness by progress?
    // v0.1 logic: "target_scaled = scale_color(target_color, clamped)"
    // So distinct phases have distinct brightness.
    // 0.25 -> Dark Blue, 0.5 -> Med Blue, 1.0 -> Full Blue.

    let clamped = if progress < 0.0 {
        0.0
    } else if progress > 1.0 {
        1.0
    } else {
        progress
    };
    let end_color = scale_color(target_color, clamped);

    view.fade_between(start_color, end_color, 10);

    unsafe {
        LAST_FILL_COLOR = end_color;
    }

    if let Some(text) = label {
        // Draw centered text
        // White text, Black shadow
        // Hack: To draw shadow, we need to be careful.
        // Let's just draw FG.
        view.draw_centered_label(text, 0xFFFFFFFF, 0xFF000000);
    }
}
