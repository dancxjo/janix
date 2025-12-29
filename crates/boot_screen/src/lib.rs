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

pub type BlitFn = unsafe fn(dst: *mut u8, src: *const u8, len: usize, scale: u32);
static BLIT_HOOK: AtomicPtr<core::ffi::c_void> = AtomicPtr::new(core::ptr::null_mut());

pub fn set_blit_hook(f: BlitFn) {
    BLIT_HOOK.store(f as *mut core::ffi::c_void, Ordering::SeqCst);
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
    bg_color: u32, // ARGB
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

        if fb.addr.is_null() {
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
            bg_color: 0, // Default to Black
        };

        Some(Self {
            inner,
            _shadow_ptr: ptr,
            _shadow_len: fb.size_bytes,
        })
    }

    pub fn set_background_color(&mut self, argb: u32) {
        self.inner.set_background_color(argb);
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
    for a in (0..=255).step_by(32) {
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
        // Inflate to cover shadow (1px offset)
        d = d.inflate(4);

        self.add_damage(d);
        self.last_text_rect = Some(new_rect);
    }

    fn set_fade(&mut self, alpha: u8) {
        if self.fade_alpha != alpha {
            self.fade_alpha = alpha;
            self.add_damage(Rect::new(0, 0, self.fb.width, self.fb.height));
        }
    }

    fn set_background_color(&mut self, argb: u32) {
        if self.bg_color != argb {
            self.bg_color = argb;
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
            
            // Pass 1: Shadow (Offset 1,1, Black)
            self.draw_text(1, 1, 0xFF000000, &d);
            
            // Pass 2: Main Text (Offset 0,0, White)
            self.draw_text(0, 0, 0xFFFFFFFF, &d);

            self.blit_shadow_to_fb(&d);
        }
    }

    fn clear_shadow_rect(&mut self, rect: &Rect) {
        let b = (self.bg_color & 0xFF) as u8;
        let g = ((self.bg_color >> 8) & 0xFF) as u8;
        let r = ((self.bg_color >> 16) & 0xFF) as u8;
        let a = ((self.bg_color >> 24) & 0xFF) as u8;

        for y in rect.y .. (rect.y + rect.h) {
            let start = (y as usize * self.fb.pitch_bytes as usize) + (rect.x as usize * 4);
            let width_bytes = rect.w as usize * 4;
            
            if start + width_bytes <= self.shadow.len() {
                // Optimization: If all bytes are the same, use fill
                if b == g && g == r && r == a {
                    self.shadow[start..start+width_bytes].fill(b);
                } else {
                    // Manual fill for complex colors
                    for i in 0..rect.w as usize {
                        let base = start + i * 4;
                        self.shadow[base] = b;
                        self.shadow[base+1] = g;
                        self.shadow[base+2] = r;
                        self.shadow[base+3] = a;
                    }
                }
            }
        }
    }

    fn draw_text(&mut self, offset_x: i32, offset_y: i32, color: u32, clip: &Rect) {
        if self.current_msg.is_empty() {
            return;
        }

        let msg_len = self.current_msg.bytes().count() as u32;
        let text_w = msg_len * (8 + 1) - 1;
        let text_h = 16;
        let base_x = self.width_center.saturating_sub(text_w / 2);
        let base_y = self.height_center.saturating_sub(text_h / 2);

        let start_x = (base_x as i32 + offset_x) as u32;
        let start_y = (base_y as i32 + offset_y) as u32;

        let mut cx = start_x;
        for byte in self.current_msg.bytes() {
            let char_rect = Rect::new(cx, start_y, 8, 16);
            if char_rect.intersection(clip).is_some() {
                self.draw_char(cx, start_y, byte, color, clip);
            }
            cx += 8 + 1;
        }
    }

    fn draw_char(&mut self, x: u32, y: u32, ch: u8, color: u32, clip: &Rect) {
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
            if py < clip_y0 || py >= clip_y1 || py >= self.fb.height {
                continue;
            }

            let row_data = glyph[row];
            for col in 0..8 {
                if (row_data & (0x80 >> col)) != 0 {
                    let px = x + col;
                    if px < clip_x0 || px >= clip_x1 || px >= self.fb.width {
                        continue;
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
        // Optimization: Pre-calculate alpha scale
        let alpha = self.fade_alpha as u32;
        let scale = alpha + 1; // Used for fast shift approximation

        for y in rect.y .. (rect.y + rect.h) {
            let row_offset = y as usize * self.fb.pitch_bytes as usize;
            let start = row_offset + (rect.x as usize * 4);
            let width_bytes = rect.w as usize * 4;

            if start + width_bytes > self.shadow.len() { continue; }

            let dst_ptr = unsafe { self.fb.addr.add(start) };
            let src_ptr = self.shadow.as_ptr().wrapping_add(start);

            let blit_ptr = BLIT_HOOK.load(Ordering::Relaxed);
            if !blit_ptr.is_null() {
                 unsafe {
                     let blit_fn: BlitFn = core::mem::transmute(blit_ptr);
                     blit_fn(dst_ptr, src_ptr, width_bytes, scale);
                 }
                 continue;
            }

            // Fallback: Use pointer arithmetic for critical boot performance (shadow-to-fb)
            // Safety: We verified bounds in `blit_shadow_to_fb` entry check and `min_size` check.
            let len = width_bytes / 4;
            unsafe {
                let mut s_ptr = src_ptr;
                let mut d_ptr = dst_ptr;
                
                for _ in 0..len {
                     let b = *s_ptr;
                     let g = *s_ptr.wrapping_add(1);
                     let r = *s_ptr.wrapping_add(2);
                     let a = *s_ptr.wrapping_add(3);

                     // Fast alpha blending: (color * (alpha + 1)) >> 8
                     // eliminates expensive division
                     // We use u32 casts for the multiply
                     let r_out = ((r as u32 * scale) >> 8) as u8;
                     let g_out = ((g as u32 * scale) >> 8) as u8;
                     let b_out = ((b as u32 * scale) >> 8) as u8;
                     // For packed output, we need to respect format. 
                     // Assuming Xrgb8888/Abgr8888 as per boot_screen init.
                     
                     // Optimization: Use direct u32 write if possible, but packing is safer.
                     let argb_out = ((a as u32) << 24) | ((r_out as u32) << 16) | ((g_out as u32) << 8) | (b_out as u32);
                     let final_val = self.fb.pixel_format.pack_le_bytes(argb_out).to_le_bytes();

                     *d_ptr = final_val[0];
                     *d_ptr.wrapping_add(1) = final_val[1];
                     *d_ptr.wrapping_add(2) = final_val[2];
                     *d_ptr.wrapping_add(3) = final_val[3];

                     s_ptr = s_ptr.wrapping_add(4);
                     d_ptr = d_ptr.wrapping_add(4);
                }
            }
        }
    }
}
