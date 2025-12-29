#![no_std]

mod damage;
mod unifont;
mod pixel;
pub mod milestones;

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

pub struct BootScreenOwned {
    inner: BootScreen<'static>,
    _shadow_ptr: *mut u8,
    _shadow_len: usize,
}

impl BootScreenOwned {
    /// Create a new BootScreenOwned using the global boot allocator.
    /// Returns None if allocator not set or allocation fails.
    /// Safety: fb.addr must be valid.
    pub unsafe fn new(fb: FramebufferInfo) -> Option<Self> {
        let alloc_ptr = BOOT_ALLOC.load(Ordering::SeqCst);
        if alloc_ptr.is_null() {
            return None;
        }
        let alloc_fn: BootAlloc = core::mem::transmute(alloc_ptr);

        let min_size = (fb.pitch_bytes as usize).checked_mul(fb.height as usize)?;
        if fb.size_bytes < min_size {
            return None;
        }

        let ptr = alloc_fn(fb.size_bytes, 4096);
        if ptr.is_null() {
            return None;
        }

        core::ptr::write_bytes(ptr, 0, fb.size_bytes);

        // Create 'static slice because we leak the memory (it persists for boot)
        let shadow = core::slice::from_raw_parts_mut(ptr, fb.size_bytes);
        let shadow_static = core::mem::transmute::<&mut [u8], &'static mut [u8]>(shadow);

        let width_center = fb.width / 2;
        let height_center = fb.height / 2;

        let inner = BootScreen {
            fb,
            shadow: shadow_static,
            current_msg: "",
            last_text_rect: None,
            fade_alpha: 0,
            damage: Some(Rect::new(0, 0, fb.width, fb.height)),
            width_center,
            height_center,
        };

        Some(Self {
            inner,
            _shadow_ptr: ptr,
            _shadow_len: fb.size_bytes,
        })
    }

    pub fn milestone(&mut self, msg: &'static str) {
        self.inner.milestone(msg);
    }

    pub fn show(&mut self, msg: &'static str) {
        self.inner.show(msg);
    }

    pub fn set_fade(&mut self, alpha: u8) {
        self.inner.set_fade(alpha);
    }

    pub fn draw(&mut self) {
        self.inner.draw();
    }
}

// Fade helper
pub fn fade_in<F: FnMut(u64)>(bs: &mut BootScreenOwned, mut delay: F) {
    bs.show(milestones::BOOTING);
    for a in (0..=255).step_by(8) {
        bs.set_fade(a as u8);
        bs.draw();
        delay(100_000);
    }
}

impl<'a> BootScreen<'a> {
    fn milestone(&mut self, msg: &'static str) {
        self.current_msg = msg;

        if msg.is_empty() {
             if let Some(old) = self.last_text_rect {
                  let d = old.inflate(2);
                  self.add_damage(d);
             }
             self.last_text_rect = None;
             return;
        }

        let msg_len = msg.bytes().count() as u32;
        let text_w = msg_len * (8 + 1) - 1;
        let text_h = 16;
        let x = self.width_center.saturating_sub(text_w / 2);
        let y = self.height_center.saturating_sub(text_h / 2);

        let new_rect = Rect::new(x, y, text_w, text_h);

        let mut d = new_rect;
        if let Some(old) = self.last_text_rect {
            d = d.union(&old);
        }
        d = d.inflate(2);

        self.add_damage(d);
        self.last_text_rect = Some(new_rect);
    }

    fn set_fade(&mut self, alpha: u8) {
        if self.fade_alpha != alpha {
            self.fade_alpha = alpha;
            self.add_damage(Rect::new(0, 0, self.fb.width, self.fb.height));
        }
    }

    fn show(&mut self, msg: &'static str) {
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

    fn draw(&mut self) {
        if self.fb.bpp != 32 {
            return;
        }

        if let Some(damage_rect) = self.damage.take() {
            let screen_rect = Rect::new(0, 0, self.fb.width, self.fb.height);
            let d = match damage_rect.intersection(&screen_rect) {
                Some(r) => r,
                None => return,
            };

            self.clear_shadow_rect(&d);
            self.draw_text_shadow(&d);
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
        let text_w = msg_len * (8 + 1) - 1;
        let text_h = 16;
        let start_x = self.width_center.saturating_sub(text_w / 2);
        let start_y = self.height_center.saturating_sub(text_h / 2);

        let text_rect = Rect::new(start_x, start_y, text_w, text_h);
        if text_rect.intersection(clip).is_none() {
            return;
        }

        let mut cx = start_x;
        for byte in self.current_msg.bytes() {
             let char_rect = Rect::new(cx, start_y, 8, 16);
             if char_rect.intersection(clip).is_some() {
                 self.draw_char_shadow(cx, start_y, byte, 0xFFFFFFFF, clip);
             }
             cx += 8 + 1;
        }
    }

    fn draw_char_shadow(&mut self, x: u32, y: u32, ch: u8, color: u32, clip: &Rect) {
        let glyph = if let Some(g) = unifont::get_glyph(ch as char) {
            g
        } else if let Some(g) = unifont::get_glyph('?') {
            g
        } else {
            return;
        };

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

                    let sx = px + 1;
                    let sy = py + 1;

                    if sx < self.fb.width && sy < self.fb.height {
                         self.put_pixel_shadow_bgra(sx, sy, 0xFF000000);
                    }

                    self.put_pixel_shadow_bgra(px, py, color);
                }
            }
        }
    }

    fn put_pixel_shadow_bgra(&mut self, x: u32, y: u32, argb: u32) {
        let offset = (y as usize * self.fb.pitch_bytes as usize) + (x as usize * 4);
        if offset + 4 > self.shadow.len() { return; }

        let a = ((argb >> 24) & 0xFF) as u8;
        let r = ((argb >> 16) & 0xFF) as u8;
        let g = ((argb >> 8) & 0xFF) as u8;
        let b = (argb & 0xFF) as u8;

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
            let dst_ptr = unsafe { self.fb.addr.add(start) };

            for i in (0..width_bytes).step_by(4) {
                 let b = src_slice[i];
                 let g = src_slice[i+1];
                 let r = src_slice[i+2];
                 let a = src_slice[i+3];

                 let alpha_scale = self.fade_alpha as u32;
                 let r_out = ((r as u32 * alpha_scale) / 255) as u8;
                 let g_out = ((g as u32 * alpha_scale) / 255) as u8;
                 let b_out = ((b as u32 * alpha_scale) / 255) as u8;
                 let a_out = a;

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
