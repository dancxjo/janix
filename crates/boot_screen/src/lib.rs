#![no_std]

mod damage;
mod font8x16;
mod pixel;

pub use damage::Rect;
pub use pixel::PixelFormat;

use core::sync::atomic::{AtomicPtr, Ordering};

#[derive(Clone, Copy, Debug)]
pub struct FramebufferInfo {
    pub addr: *mut u8,
    pub size_bytes: usize,
    pub width: u32,
    pub height: u32,
    pub pitch_bytes: u32,
    pub bpp: u16,
    pub pixel_format: PixelFormat,
}

unsafe impl Send for FramebufferInfo {}

pub type BootAlloc = unsafe fn(size: usize, align: usize) -> *mut u8;

static BOOT_ALLOC: AtomicPtr<core::ffi::c_void> = AtomicPtr::new(core::ptr::null_mut());

pub fn set_boot_alloc(alloc: BootAlloc) {
    BOOT_ALLOC.store(alloc as *mut core::ffi::c_void, Ordering::SeqCst);
}

pub struct BootScreen<'a> {
    fb: FramebufferInfo,
    shadow: &'a mut [u8], // Stored as BGRA bytes
    current_msg: &'static str,
    last_text_rect: Option<Rect>,
    fade_alpha: u8,
    damage: Option<Rect>,
    width_center: u32,
    height_center: u32,
}

impl<'a> BootScreen<'a> {
    /// Create a new BootScreen using the global boot allocator for the shadow buffer.
    /// Returns None if allocator not set or allocation fails.
    /// Safety: fb.addr must be valid.
    pub unsafe fn new(fb: FramebufferInfo) -> Option<Self> {
        let alloc_ptr = BOOT_ALLOC.load(Ordering::SeqCst);
        if alloc_ptr.is_null() {
            return None;
        }
        let alloc_fn: BootAlloc = core::mem::transmute(alloc_ptr);

        // Ensure size covers at least pitch * height
        let min_size = (fb.pitch_bytes as usize).checked_mul(fb.height as usize)?;
        if fb.size_bytes < min_size {
            return None;
        }

        let ptr = alloc_fn(fb.size_bytes, 4096);
        if ptr.is_null() {
            return None;
        }

        // Zero the shadow buffer initially
        core::ptr::write_bytes(ptr, 0, fb.size_bytes);

        let shadow = core::slice::from_raw_parts_mut(ptr, fb.size_bytes);
        Self::new_with_shadow(fb, shadow)
    }

    /// Create a new BootScreen with a provided shadow buffer.
    pub unsafe fn new_with_shadow(fb: FramebufferInfo, shadow: &'a mut [u8]) -> Option<Self> {
        let min_size = (fb.pitch_bytes as usize).checked_mul(fb.height as usize)?;
        if shadow.len() < min_size {
            return None;
        }
        // Also clamp usage to min(shadow.len(), fb.size_bytes) if needed,
        // but we assume fb.size_bytes is authoritative for the FB mapping.
        // We just ensure shadow is big enough.

        let width_center = fb.width / 2;
        let height_center = fb.height / 2;

        Some(BootScreen {
            fb,
            shadow,
            current_msg: "",
            last_text_rect: None,
            fade_alpha: 0,
            damage: Some(Rect::new(0, 0, fb.width, fb.height)),
            width_center,
            height_center,
        })
    }

    pub fn milestone(&mut self, msg: &'static str) {
        self.current_msg = msg;

        if msg.is_empty() {
             if let Some(old) = self.last_text_rect {
                  let d = old.inflate(2);
                  self.add_damage(d);
             }
             self.last_text_rect = None;
             return;
        }

        // Calculate new text rect
        let msg_len = msg.bytes().count() as u32;
        let text_w = msg_len * (font8x16::FONT_WIDTH + 1) - 1;
        let text_h = font8x16::FONT_HEIGHT;
        let x = self.width_center.saturating_sub(text_w / 2);
        let y = self.height_center.saturating_sub(text_h / 2);

        let new_rect = Rect::new(x, y, text_w, text_h);

        // Union with old rect and inflate
        let mut d = new_rect;
        if let Some(old) = self.last_text_rect {
            d = d.union(&old);
        }
        d = d.inflate(2);

        self.add_damage(d);
        self.last_text_rect = Some(new_rect);
    }

    pub fn set_fade(&mut self, alpha: u8) {
        if self.fade_alpha != alpha {
            self.fade_alpha = alpha;
            // Full screen damage on fade change
            self.add_damage(Rect::new(0, 0, self.fb.width, self.fb.height));
        }
    }

    pub fn show(&mut self, msg: &'static str) {
        self.milestone(msg);
        self.draw();
    }

    fn add_damage(&mut self, rect: Rect) {
        if let Some(d) = self.damage {
            self.damage = Some(d.union(&rect));
        } else {
            self.damage = Some(rect);
        }
    }

    pub fn draw(&mut self) {
        if self.fb.bpp != 32 {
            return;
        }

        if let Some(damage_rect) = self.damage.take() {
            // Clip damage to screen
            let screen_rect = Rect::new(0, 0, self.fb.width, self.fb.height);
            let d = match damage_rect.intersection(&screen_rect) {
                Some(r) => r,
                None => return,
            };

            // 1. Render to shadow buffer (Ideal, Alpha=255) within damage rect
            // Clear damaged area to black
            self.clear_shadow_rect(&d);

            // Draw text into shadow (if visible), clipped to damage
            self.draw_text_shadow(&d);

            // 2. Copy shadow to FB with Fade
            self.blit_shadow_to_fb(&d);
        }
    }

    fn clear_shadow_rect(&mut self, rect: &Rect) {
        for y in rect.y .. (rect.y + rect.h) {
            let start = (y as usize * self.fb.pitch_bytes as usize) + (rect.x as usize * 4);
            let end = start + (rect.w as usize * 4);
            if end <= self.shadow.len() {
                self.shadow[start..end].fill(0);
            }
        }
    }

    fn draw_text_shadow(&mut self, clip: &Rect) {
        if self.current_msg.is_empty() {
            return;
        }

        let msg_len = self.current_msg.bytes().count() as u32;
        let text_w = msg_len * (font8x16::FONT_WIDTH + 1) - 1;
        let text_h = font8x16::FONT_HEIGHT;
        let start_x = self.width_center.saturating_sub(text_w / 2);
        let start_y = self.height_center.saturating_sub(text_h / 2);

        // Quick intersection test of text rect vs clip
        let text_rect = Rect::new(start_x, start_y, text_w, text_h);
        if text_rect.intersection(clip).is_none() {
            return;
        }

        let mut cx = start_x;
        for byte in self.current_msg.bytes() {
             // Only draw if char intersects clip
             let char_rect = Rect::new(cx, start_y, font8x16::FONT_WIDTH, font8x16::FONT_HEIGHT);
             if char_rect.intersection(clip).is_some() {
                 self.draw_char_shadow(cx, start_y, byte, 0xFFFFFFFF, clip);
             }
             cx += font8x16::FONT_WIDTH + 1;
        }
    }

    fn draw_char_shadow(&mut self, x: u32, y: u32, ch: u8, color: u32, clip: &Rect) {
        if ch < 0x20 || ch > 0x7E {
            // draw ?
            self.draw_char_shadow(x, y, b'?', color, clip);
            return;
        }

        let idx = (ch - 0x20) as usize;
        let glyph = font8x16::FONT_DATA[idx];

        // Clip bounds
        let clip_x0 = clip.x;
        let clip_x1 = clip.x + clip.w;
        let clip_y0 = clip.y;
        let clip_y1 = clip.y + clip.h;

        for row in 0..16 {
            let py = y + row as u32;
            if py < clip_y0 || py >= clip_y1 || py >= self.fb.height { continue; }

            let row_data = glyph[row];
            for col in 0..8 {
                if (row_data & (0x80 >> col)) != 0 {
                    let px = x + col;
                    if px < clip_x0 || px >= clip_x1 || px >= self.fb.width { continue; }

                    // Draw Shadow (1px offset)
                    let sx = px + 1;
                    let sy = py + 1;
                    // Check shadow clip separately?
                    // It's fine if shadow is outside clip, we just won't update it (it won't be blitted).
                    // But we should only write to shadow buffer if it's safe.
                    // put_pixel_shadow_bgra checks bounds of buffer.

                    if sx < self.fb.width && sy < self.fb.height {
                         self.put_pixel_shadow_bgra(sx, sy, 0xFF000000);
                    }

                    // Draw Foreground
                    self.put_pixel_shadow_bgra(px, py, color);
                }
            }
        }
    }

    fn put_pixel_shadow_bgra(&mut self, x: u32, y: u32, argb: u32) {
        let offset = (y as usize * self.fb.pitch_bytes as usize) + (x as usize * 4);
        if offset + 4 > self.shadow.len() { return; }

        // Convert ARGB to BGRA bytes for shadow storage
        let a = ((argb >> 24) & 0xFF) as u8;
        let r = ((argb >> 16) & 0xFF) as u8;
        let g = ((argb >> 8) & 0xFF) as u8;
        let b = (argb & 0xFF) as u8;

        // Store as BGRA
        self.shadow[offset] = b;
        self.shadow[offset+1] = g;
        self.shadow[offset+2] = r;
        self.shadow[offset+3] = a;
    }

    fn blit_shadow_to_fb(&mut self, rect: &Rect) {
        for y in rect.y .. (rect.y + rect.h) {
            let row_offset = y as usize * self.fb.pitch_bytes as usize;
            let start = row_offset + (rect.x as usize * 4);
            let width_bytes = rect.w as usize * 4;

            if start + width_bytes > self.shadow.len() { continue; }

            let src_slice = &self.shadow[start .. start + width_bytes];

            // We need to apply fade and format conversion
            // This is unsafe because we write to raw ptr fb.addr
            let dst_ptr = unsafe { self.fb.addr.add(start) };

            for i in (0..width_bytes).step_by(4) {
                 // Read BGRA from shadow
                 let b = src_slice[i];
                 let g = src_slice[i+1];
                 let r = src_slice[i+2];
                 let a = src_slice[i+3];

                 // Apply fade to RGB channels only
                 let alpha_scale = self.fade_alpha as u32;
                 // out = (in * alpha) / 255
                 let r_out = ((r as u32 * alpha_scale) / 255) as u8;
                 let g_out = ((g as u32 * alpha_scale) / 255) as u8;
                 let b_out = ((b as u32 * alpha_scale) / 255) as u8;
                 let a_out = a; // Do NOT scale alpha

                 let argb_out = ((a_out as u32) << 24) | ((r_out as u32) << 16) | ((g_out as u32) << 8) | (b_out as u32);

                 let final_val = self.fb.pixel_format.pack_le_bytes(argb_out);
                 let final_bytes = final_val.to_le_bytes();

                 unsafe {
                     *dst_ptr.add(i) = final_bytes[0];
                     *dst_ptr.add(i+1) = final_bytes[1];
                     *dst_ptr.add(i+2) = final_bytes[2];
                     *dst_ptr.add(i+3) = final_bytes[3];
                 }
            }
        }
    }
}
