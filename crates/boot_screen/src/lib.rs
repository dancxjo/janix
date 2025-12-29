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

static BOOT_ALLOC: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());

pub fn set_boot_alloc(alloc: BootAlloc) {
    BOOT_ALLOC.store(alloc as *mut (), Ordering::SeqCst);
}

pub struct BootScreen<'a> {
    fb: FramebufferInfo,
    shadow: &'a mut [u8], // Stored as BGRA bytes
    current_msg: &'a str,
    last_text_rect: Option<Rect>,
    fade_alpha: u8,
    damage: Option<Rect>,
    width_center: u32,
    height_center: u32,
}

impl<'a> BootScreen<'a> {
    /// Create a new BootScreen using the global boot allocator for the shadow buffer.
    /// Safety: fb.addr must be valid.
    pub unsafe fn new(fb: FramebufferInfo) -> Self {
        let alloc_ptr = BOOT_ALLOC.load(Ordering::SeqCst);
        if alloc_ptr.is_null() {
            panic!("BootScreen: No allocator set via set_boot_alloc");
        }
        let alloc_fn: BootAlloc = core::mem::transmute(alloc_ptr);

        let ptr = alloc_fn(fb.size_bytes, 4096);
        if ptr.is_null() {
            panic!("BootScreen: Alloc failed");
        }

        // Zero the shadow buffer initially
        core::ptr::write_bytes(ptr, 0, fb.size_bytes);

        let shadow = core::slice::from_raw_parts_mut(ptr, fb.size_bytes);
        Self::new_with_shadow(fb, shadow)
    }

    /// Create a new BootScreen with a provided shadow buffer.
    pub unsafe fn new_with_shadow(fb: FramebufferInfo, shadow: &'a mut [u8]) -> Self {
        if shadow.len() != fb.size_bytes {
            panic!("BootScreen: Shadow buffer size mismatch");
        }

        let width_center = fb.width / 2;
        let height_center = fb.height / 2;

        BootScreen {
            fb,
            shadow,
            current_msg: "",
            last_text_rect: None,
            fade_alpha: 0,
            damage: Some(Rect::new(0, 0, fb.width, fb.height)),
            width_center,
            height_center,
        }
    }

    pub fn milestone(&mut self, msg: &'a str) {
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

    pub fn show(&mut self, msg: &'a str) {
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

            // Draw text into shadow (if visible)
            self.draw_text_shadow();

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

    fn draw_text_shadow(&mut self) {
        if self.current_msg.is_empty() {
            return;
        }

        let msg_len = self.current_msg.bytes().count() as u32;
        let text_w = msg_len * (font8x16::FONT_WIDTH + 1) - 1;
        let text_h = font8x16::FONT_HEIGHT;
        let start_x = self.width_center.saturating_sub(text_w / 2);
        let start_y = self.height_center.saturating_sub(text_h / 2);

        let mut cx = start_x;
        for byte in self.current_msg.bytes() {
             self.draw_char_shadow(cx, start_y, byte, 0xFFFFFFFF); // White
             cx += font8x16::FONT_WIDTH + 1;
        }
    }

    fn draw_char_shadow(&mut self, x: u32, y: u32, ch: u8, color: u32) {
        if ch < 0x20 || ch > 0x7E {
            // draw ?
            self.draw_char_shadow(x, y, b'?', color);
            return;
        }

        let idx = (ch - 0x20) as usize;
        let glyph = font8x16::FONT_DATA[idx];

        for row in 0..16 {
            let row_data = glyph[row];
            let py = y + row as u32;
            if py >= self.fb.height { continue; }

            for col in 0..8 {
                if (row_data & (0x80 >> col)) != 0 {
                    let px = x + col;
                    if px >= self.fb.width { continue; }

                    // Draw Shadow (1px offset)
                    let sx = px + 1;
                    let sy = py + 1;
                    if sx < self.fb.width && sy < self.fb.height {
                        // Shadow color is black 0xFF000000
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
