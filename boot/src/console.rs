use limine::framebuffer::Framebuffer;

// Will be provided by build.rs:
include!(concat!(env!("OUT_DIR"), "/unifont.rs"));

static mut CONSOLE: Option<Console> = None;

pub struct Console {
    fb_ptr: *mut u8,
    width: u64,
    height: u64,
    pitch: u64,
    bpp: u16, // assume 32 for now
    cursor_x: u32,
    cursor_y: u32,
    cols: u32,
    rows: u32,
}

impl Console {
    pub unsafe fn from_framebuffer(fb: &Framebuffer) -> Self {
        let width = fb.width() as u64;
        let height = fb.height() as u64;
        let pitch = fb.pitch() as u64;
        let bpp = fb.bpp();
        let fb_ptr = fb.addr() as *mut u8;

        let cols = (width as u32 / GLYPH_WIDTH).max(1);
        let rows = (height as u32 / GLYPH_HEIGHT).max(1);

        Console {
            fb_ptr,
            width,
            height,
            pitch,
            bpp,
            cursor_x: 0,
            cursor_y: 0,
            cols,
            rows,
        }
    }

    pub fn clear(&mut self) {
        // Zero the framebuffer; assume 32 bpp.
        let bytes = (self.pitch * self.height) as usize;
        unsafe {
            core::ptr::write_bytes(self.fb_ptr, 0, bytes);
        }
    }

    pub fn put_char(&mut self, ch: char) {
        match ch {
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
                if self.cursor_y >= self.rows {
                    self.scroll();
                    self.cursor_y = self.rows - 1;
                }
            }
            _ => {
                if let Some(bitmap) = lookup_glyph(ch) {
                    self.draw_glyph(self.cursor_x, self.cursor_y, bitmap);
                }
                self.cursor_x += 1;
                if self.cursor_x >= self.cols {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                    if self.cursor_y >= self.rows {
                        self.scroll();
                        self.cursor_y = self.rows - 1;
                    }
                }
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.put_char(ch);
        }
    }

    fn scroll(&mut self) {
        // Simple “scroll up one row”: memmove framebuffer up by one glyph row,
        // then clear the bottom row.
        let row_bytes = (self.pitch * GLYPH_HEIGHT as u64) as usize;
        let total_bytes = (self.pitch * self.height) as usize;
        let copy_bytes = total_bytes - row_bytes;

        unsafe {
            // Move everything up by one row height
            core::ptr::copy(self.fb_ptr.add(row_bytes), self.fb_ptr, copy_bytes);

            // Clear the last row
            core::ptr::write_bytes(self.fb_ptr.add(copy_bytes), 0, row_bytes);
        }
    }

    fn draw_glyph(&mut self, col: u32, row: u32, bitmap: &[u8; GLYPH_HEIGHT as usize]) {
        // Convert (col,row) to pixel origin:
        let x0 = col * GLYPH_WIDTH;
        let y0 = row * GLYPH_HEIGHT;

        // For each row of glyph:
        for (dy, byte) in bitmap.iter().enumerate() {
            let y = y0 + dy as u32;
            if y >= self.height as u32 {
                break;
            }
            // Each bit in `byte` is a pixel.
            for bit in 0..GLYPH_WIDTH {
                let x = x0 + bit;
                if x >= self.width as u32 {
                    break;
                }
                let on = (byte >> (7 - bit)) & 1 != 0;
                if on {
                    self.set_pixel(x, y, 0xFFFFFFFF);
                } else {
                    // Optional: draw background color (black)
                    self.set_pixel(x, y, 0x00000000);
                }
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, argb: u32) {
        if self.bpp != 32 {
            // For now, ignore non-32bpp modes.
            return;
        }
        let offset = y as u64 * self.pitch + x as u64 * 4;
        unsafe {
            let ptr = self.fb_ptr.add(offset as usize).cast::<u32>();
            ptr.write_volatile(argb);
        }
    }
}

pub unsafe fn init_global(fb: &Framebuffer) {
    // SAFETY: We are in single-threaded boot context.
    let console = unsafe { Console::from_framebuffer(fb) };
    unsafe { CONSOLE = Some(console) };
}

pub fn print(s: &str) {
    unsafe {
        // SAFETY: We are in single-threaded boot context.
        let console_ptr = &raw mut CONSOLE;
        if let Some(console) = &mut *console_ptr {
            console.write_str(s);
        }
    }
}

pub fn clear_screen() {
    unsafe {
        // SAFETY: We are in single-threaded boot context.
        let console_ptr = &raw mut CONSOLE;
        if let Some(console) = &mut *console_ptr {
            console.clear();
        }
    }
}
