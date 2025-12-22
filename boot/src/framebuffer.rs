#[cfg(feature = "fill-framebuffer")]
use core::hint::spin_loop;

#[cfg(feature = "fill-framebuffer")]
use crate::console::{GLYPH_HEIGHT, GLYPH_WIDTH, lookup_glyph};

#[cfg(feature = "fill-framebuffer")]
pub const BOOT_DOMINANT_COLOR: u32 = 0xFF4C89AA; // Dominant sky blue from clouds.bmp

#[cfg(feature = "fill-framebuffer")]
static mut LAST_FILL_COLOR: u32 = 0;

#[cfg(feature = "fill-framebuffer")]
struct FramebufferView {
    ptr: *mut u8,
    width: usize,
    height: usize,
    stride_pixels: usize,
    bpp: u16,
}

#[cfg(feature = "fill-framebuffer")]
impl FramebufferView {
    fn fill_solid(&self, color: u32) {
        if self.bpp != 32 {
            kernel::println!("Framebuffer fill skipped: unsupported bpp {}", self.bpp);
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
            // Tiny pause so the transition is visible on real hardware
            for _ in 0..40_000 {
                spin_loop();
            }
        }
    }

    fn draw_centered_label(&self, text: &str, fg: u32, shadow: u32) {
        if self.bpp != 32 || text.is_empty() {
            return;
        }

        let glyph_w = GLYPH_WIDTH as usize;
        let glyph_h = GLYPH_HEIGHT as usize;
        let text_width = glyph_w.saturating_mul(text.chars().count());
        let x0 = self.width.saturating_sub(text_width).saturating_div(2);
        let y0 = self.height.saturating_sub(glyph_h).saturating_div(2);

        for (i, ch) in text.chars().enumerate() {
            let Some(bitmap) = lookup_glyph(ch) else {
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
        bitmap: &[u8; GLYPH_HEIGHT as usize],
        shadow: u32,
        fg: u32,
    ) {
        for (dy, byte) in bitmap.iter().enumerate() {
            let y = origin_y + dy;
            if y >= self.height {
                break;
            }
            for bit in 0..GLYPH_WIDTH as usize {
                let x = origin_x + bit;
                if x >= self.width {
                    break;
                }
                let on = (byte >> (7 - bit)) & 1 != 0;
                if on {
                    self.set_pixel(x, y, fg);
                    if x > 0 && y + 1 < self.height {
                        self.set_pixel(x - 1, y + 1, shadow);
                    }
                } else {
                    // Leave background untouched
                }
            }
        }
    }

    fn set_pixel(&self, x: usize, y: usize, argb: u32) {
        if x >= self.width || y >= self.height || self.bpp != 32 {
            return;
        }
        let offset = y * self.stride_pixels + x;
        unsafe {
            let ptr = self.ptr.add(offset * 4).cast::<u32>();
            ptr.write_volatile(argb);
        }
    }
}

#[cfg(feature = "fill-framebuffer")]
pub fn fill_framebuffer_progress(target_color: u32, progress: f32, label: Option<&str>) {
    let Some(view) = map_framebuffer() else {
        kernel::println!("Framebuffer fill skipped: no accessible framebuffer");
        return;
    };

    let clamped = progress.clamp(0.0, 1.0);
    let target_scaled = scale_color(target_color, clamped);
    let start_color = unsafe { LAST_FILL_COLOR };

    view.fade_between(start_color, target_scaled, 18);
    if let Some(text) = label {
        view.draw_centered_label(text, 0xFFFFFFFF, 0x7F000000);
    }

    unsafe {
        LAST_FILL_COLOR = target_scaled;
    }
    kernel::println!(
        "Framebuffer filled to {:.0}% toward target color {:#010x}",
        clamped * 100.0,
        target_color
    );
}

#[cfg(feature = "fill-framebuffer")]
pub fn fill_framebuffer_with_color(color: u32) {
    fill_framebuffer_progress(color, 1.0, None);
}

#[cfg(feature = "fill-framebuffer")]
fn map_framebuffer() -> Option<FramebufferView> {
    let response = crate::FRAMEBUFFER_REQUEST.get_response()?;
    let framebuffer = response.framebuffers().next()?;

    let width = framebuffer.width() as usize;
    let height = framebuffer.height() as usize;
    let pitch = framebuffer.pitch() as usize;
    let bpp = framebuffer.bpp();
    if width == 0 || height == 0 || pitch == 0 {
        kernel::println!("Framebuffer fill skipped: invalid dimensions");
        return None;
    }

    let stride = match bpp {
        32 => pitch / 4,
        _ => {
            kernel::println!("Framebuffer fill skipped: unsupported bpp {}", bpp);
            return None;
        }
    };

    if stride < width {
        kernel::println!("Framebuffer fill skipped: pitch less than width");
        return None;
    }

    let hhdm_offset = crate::boot_model::HHDM_REQUEST
        .get_response()
        .map(|resp| resp.offset())
        .unwrap_or(0);
    let fb_phys = framebuffer.addr() as u64;
    let fb_virt = match virtual_framebuffer_address(fb_phys, hhdm_offset) {
        Some(addr) => addr,
        None => {
            kernel::println!("Framebuffer fill skipped: could not determine virtual pointer");
            return None;
        }
    };

    kernel::println!(
        "Framebuffer fill: phys={:#x} hhdm={:#x} virt={:#x} pitch={} width={} height={}",
        fb_phys,
        hhdm_offset,
        fb_virt,
        pitch,
        width,
        height
    );

    Some(FramebufferView {
        ptr: fb_virt as *mut u8,
        width,
        height,
        stride_pixels: stride,
        bpp,
    })
}

#[cfg(feature = "fill-framebuffer")]
fn scale_color(color: u32, progress: f32) -> u32 {
    // Avoid f32::round/floor (not available in core); emulate with (x + 0.5) truncation.
    let r = (((color >> 16) & 0xFF) as f32 * progress + 0.5) as u32;
    let g = (((color >> 8) & 0xFF) as f32 * progress + 0.5) as u32;
    let b = (((color) & 0xFF) as f32 * progress + 0.5) as u32;
    (0xFF << 24) | (r << 16) | (g << 8) | b
}

#[cfg(feature = "fill-framebuffer")]
fn lerp_color(start: u32, end: u32, step: usize, total_steps: usize) -> u32 {
    if total_steps == 0 {
        return end;
    }
    let t = step as u32;
    let total = total_steps as u32;
    let lerp_channel = |s: u32, e: u32| -> u32 { s + ((e.saturating_sub(s)) * t) / total };

    let sr = (start >> 16) & 0xFF;
    let sg = (start >> 8) & 0xFF;
    let sb = start & 0xFF;

    let er = (end >> 16) & 0xFF;
    let eg = (end >> 8) & 0xFF;
    let eb = end & 0xFF;

    (0xFF << 24) | (lerp_channel(sr, er) << 16) | (lerp_channel(sg, eg) << 8) | lerp_channel(sb, eb)
}

pub fn virtual_framebuffer_address(guest_addr: u64, hhdm_offset: u64) -> Option<u64> {
    if is_canonical_address(guest_addr) {
        return Some(guest_addr);
    }

    if hhdm_offset == 0 {
        return None;
    }

    let translated = guest_addr.wrapping_add(hhdm_offset);
    if is_canonical_address(translated) {
        Some(translated)
    } else {
        None
    }
}

pub fn is_canonical_address(addr: u64) -> bool {
    addr <= 0x0000_7fff_ffff_ffff || addr >= 0xffff_8000_0000_0000
}
