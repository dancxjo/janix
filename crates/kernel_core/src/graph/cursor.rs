use spin::Mutex;
use thing_models::builtins::core_kinds::DisplayFramebufferBody;
use thing_models::schema::bitmap::BitmapBody;

pub struct CursorRenderer {
    fb_ptr: Option<*mut u32>,
    fb_width: u32,
    fb_height: u32,
    fb_pitch: u32,

    backing_store: [u32; 32*32],
    cursor_pixels: [u32; 32*32], // Pre-converted ARGB
    cursor_w: u32,
    cursor_h: u32,

    last_x: i32,
    last_y: i32,
    initialized: bool,
}

static CURSOR: Mutex<CursorRenderer> = Mutex::new(CursorRenderer {
    fb_ptr: None,
    fb_width: 0,
    fb_height: 0,
    fb_pitch: 0,
    backing_store: [0; 1024],
    cursor_pixels: [0; 1024],
    cursor_w: 0,
    cursor_h: 0,
    last_x: -1,
    last_y: -1,
    initialized: false,
});

unsafe impl Send for CursorRenderer {}
unsafe impl Sync for CursorRenderer {}

pub fn init(fb: &DisplayFramebufferBody, bitmap: &BitmapBody) {
    let mut c = CURSOR.lock();
    c.fb_ptr = Some(fb.address as *mut u32);
    c.fb_width = fb.width as u32;
    c.fb_height = fb.height as u32;
    c.fb_pitch = fb.pitch as u32;

    c.cursor_w = bitmap.width.min(32);
    c.cursor_h = bitmap.height.min(32);

    for y in 0..c.cursor_h {
        for x in 0..c.cursor_w {
            let i = ((y * bitmap.width + x) * 4) as usize;
            if i + 3 < bitmap.pixels.len() {
                let r = bitmap.pixels[i];
                let g = bitmap.pixels[i+1];
                let b = bitmap.pixels[i+2];
                let a = bitmap.pixels[i+3];
                // 0xAARRGGBB
                let val = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                c.cursor_pixels[(y * 32 + x) as usize] = val;
            }
        }
    }

    c.initialized = true;

    // Set screen size in mouse
    crate::input::mouse::set_screen_size(fb.width as i32, fb.height as i32);
}

pub fn update_cursor(x: i32, y: i32) {
    let mut c = CURSOR.lock();
    if !c.initialized { return; }

    // Restore
    if c.last_x != -1 {
        c.restore_bg(c.last_x, c.last_y);
    }

    // Save
    c.save_bg(x, y);

    // Draw
    c.draw(x, y);

    c.last_x = x;
    c.last_y = y;
}

impl CursorRenderer {
    fn restore_bg(&self, x: i32, y: i32) {
        if let Some(ptr) = self.fb_ptr {
            unsafe {
                for dy in 0..self.cursor_h {
                     let sy = y + dy as i32;
                     if sy < 0 || sy >= self.fb_height as i32 { continue; }

                     for dx in 0..self.cursor_w {
                         let sx = x + dx as i32;
                         if sx < 0 || sx >= self.fb_width as i32 { continue; }

                         let offset = (sy as u32 * self.fb_pitch / 4 + sx as u32) as isize;
                         let bg = self.backing_store[(dy * 32 + dx) as usize];
                         *ptr.offset(offset) = bg;
                     }
                }
            }
        }
    }

    fn save_bg(&mut self, x: i32, y: i32) {
        if let Some(ptr) = self.fb_ptr {
            unsafe {
                for dy in 0..self.cursor_h {
                     let sy = y + dy as i32;
                     if sy < 0 || sy >= self.fb_height as i32 {
                         self.backing_store[(dy * 32) as usize] = 0; // Padding
                         continue;
                     }

                     for dx in 0..self.cursor_w {
                         let sx = x + dx as i32;
                         if sx < 0 || sx >= self.fb_width as i32 {
                             self.backing_store[(dy * 32 + dx) as usize] = 0;
                             continue;
                         }

                         let offset = (sy as u32 * self.fb_pitch / 4 + sx as u32) as isize;
                         self.backing_store[(dy * 32 + dx) as usize] = *ptr.offset(offset);
                     }
                }
            }
        }
    }

    fn draw(&self, x: i32, y: i32) {
        if let Some(ptr) = self.fb_ptr {
            unsafe {
                for dy in 0..self.cursor_h {
                     let sy = y + dy as i32;
                     if sy < 0 || sy >= self.fb_height as i32 { continue; }

                     for dx in 0..self.cursor_w {
                         let sx = x + dx as i32;
                         if sx < 0 || sx >= self.fb_width as i32 { continue; }

                         let src_val = self.cursor_pixels[(dy * 32 + dx) as usize];
                         let alpha = (src_val >> 24) as u32;

                         if alpha == 0 { continue; }

                         let offset = (sy as u32 * self.fb_pitch / 4 + sx as u32) as isize;
                         let dst_val = *ptr.offset(offset);

                         if alpha == 255 {
                             *ptr.offset(offset) = src_val;
                         } else {
                             // Blend
                             let sr = (src_val >> 16) & 0xFF;
                             let sg = (src_val >> 8) & 0xFF;
                             let sb = src_val & 0xFF;

                             let dr = (dst_val >> 16) & 0xFF;
                             let dg = (dst_val >> 8) & 0xFF;
                             let db = dst_val & 0xFF;

                             let inv_a = 255 - alpha;

                             let r = (sr * alpha + dr * inv_a) / 255;
                             let g = (sg * alpha + dg * inv_a) / 255;
                             let b = (sb * alpha + db * inv_a) / 255;

                             *ptr.offset(offset) = (0xFF << 24) | (r << 16) | (g << 8) | b;
                         }
                     }
                }
            }
        }
    }
}
