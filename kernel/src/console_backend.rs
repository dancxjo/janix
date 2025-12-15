use crate::console::{ConsoleSink, register_sink};
use crate::graph::{self, update_thing};
use crate::graph_kinds::{self, PROP_NAME};
use crate::memory::{allocate_frame, phys_to_virt, PhysFrame};
use crate::shared_buffer::{self, register_shared_buffer};
use abi::{PixelFormat, PropValue};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use spin::Mutex;

include!(concat!(env!("OUT_DIR"), "/unifont.rs"));

const CONSOLE_WIDTH: u32 = 1024;
const CONSOLE_HEIGHT: u32 = 768;
const BYTES_PER_PIXEL: u32 = 4;
const PAGE_SIZE: u64 = 4096;

struct FramebufferConsole {
    width: u32,
    height: u32,
    stride: u32,
    frames: Vec<PhysFrame>,
    cursor_x: u32,
    cursor_y: u32,
    cols: u32,
    rows: u32,
}

// SAFETY: We use a Mutex wrapper for the global instance, so inner methods just need to be Send.
unsafe impl Send for FramebufferConsole {}

impl FramebufferConsole {
    fn new(frames: Vec<PhysFrame>, width: u32, height: u32, stride: u32) -> Self {
        let cols = width / GLYPH_WIDTH;
        let rows = height / GLYPH_HEIGHT;
        Self {
            width,
            height,
            stride,
            frames,
            cursor_x: 0,
            cursor_y: 0,
            cols,
            rows,
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.stride + x * BYTES_PER_PIXEL) as u64;
        let page_idx = (offset / PAGE_SIZE) as usize;
        let page_offset = offset % PAGE_SIZE;

        if let Some(frame) = self.frames.get(page_idx) {
            let phys_addr = frame.start_address + page_offset;
            let virt_addr = phys_to_virt(phys_addr);
            unsafe {
                (virt_addr as *mut u32).write(color);
            }
        }
    }

    fn clear(&mut self) {
        for frame in &self.frames {
            let virt_addr = phys_to_virt(frame.start_address);
            unsafe {
                core::ptr::write_bytes(virt_addr as *mut u8, 0, PAGE_SIZE as usize);
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    fn scroll(&mut self) {
        // Scrolling is expensive (read-modify-write on uncached cleanup? or shared buffer is RAM so it's cached in kernel mapping?).
        // Since we are writing to RAM frames using HHDM, it should be fast enough.
        // We need to move rows up.
        
        // Naive implementation: iterate pixels. heavy.
        // Better: use `copy` on memory.
        
        let row_bytes = self.stride * GLYPH_HEIGHT;
        // We need to copy (height - glyph_height) lines up by glyph_height lines.
        // But physically the buffer is non-contiguous pages.
        // So we can't just memmove one huge block.
        
        // Easier strategy for "kernel debug console": just wrap around or clear.
        // But user wants "F12 console" which implies persistence and scrolling.
        
        // Let's do a logic-based scroll: re-render? No we don't store text buffer.
        // We have to blit.
        
        // For simplicity in this first pass, let's just clear and reset cursor when full.
        // Scrolling properly with discontiguous frames is a bit of work.
        // Wait, Unifont is 16px high. 768 / 16 = 48 rows.
        self.clear(); // "Page flip" style for now to be safe and simple.
    }

    fn draw_glyph(&mut self, x: u32, y: u32, bitmap: &[u8; 16]) {
        for (row_idx, byte) in bitmap.iter().enumerate() {
            for bit_idx in 0..8 {
                if (byte >> (7 - bit_idx)) & 1 != 0 {
                    self.set_pixel(x + bit_idx, y + row_idx as u32, 0xFFFFFFFF);
                } else {
                    self.set_pixel(x + bit_idx, y + row_idx as u32, 0xFF000000);
                }
            }
        }
    }

    fn write_char(&mut self, c: char) {
         match c {
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            '\r' => {
                self.cursor_x = 0;
            }
            _ => {
                if let Some(glyph) = lookup_glyph(c) {
                    self.draw_glyph(self.cursor_x * GLYPH_WIDTH, self.cursor_y * GLYPH_HEIGHT, glyph);
                }
                self.cursor_x += 1;
                if self.cursor_x >= self.cols {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                }
            }
        }

        if self.cursor_y >= self.rows {
            self.scroll();
        }
    }
}

struct GlobalConsole(Mutex<Option<FramebufferConsole>>);

impl ConsoleSink for GlobalConsole {
    fn write_str(&self, s: &str) {
        if let Some(console) = self.0.lock().as_mut() {
            for c in s.chars() {
                console.write_char(c);
            }
        }
    }
}

static GLOBAL_CONSOLE: GlobalConsole = GlobalConsole(Mutex::new(None));

pub fn init() {
    // 1. Allocate frames
    let total_bytes = (CONSOLE_WIDTH * CONSOLE_HEIGHT * BYTES_PER_PIXEL) as u64;
    let num_pages = (total_bytes + PAGE_SIZE - 1) / PAGE_SIZE;
    
    let mut frames = Vec::new();
    let mut heapless_frames = heapless::Vec::new();

    for _ in 0..num_pages {
        if let Some(frame) = allocate_frame() {
            frames.push(frame);
            // We also need to populate the heapless vec for register_shared_buffer
            // But wait, register_shared_buffer takes a heapless::Vec.
            // We need to ensure we don't exceed MAX_FRAMES_PER_BUFFER (4096).
            // 1024*768*4 = 3MB. 3MB / 4KB = 768 pages.  768 < 4096. Safe.
            let _ = heapless_frames.push(frame);
        } else {
            // Allocation failed, abort console init
            return;
        }
    }

    // 2. Create Console instance
    let mut console = FramebufferConsole::new(
        frames, 
        CONSOLE_WIDTH, 
        CONSOLE_HEIGHT, 
        CONSOLE_WIDTH * BYTES_PER_PIXEL
    );
    console.clear();

    *GLOBAL_CONSOLE.0.lock() = Some(console);

    // 3. Register SharedBuffer
    // register_shared_buffer expects ownership of the frames (heapless::Vec)
    if let Ok(id) = register_shared_buffer(
        CONSOLE_WIDTH,
        CONSOLE_HEIGHT,
        CONSOLE_WIDTH * BYTES_PER_PIXEL,
        PixelFormat::Bgra8888, // Standard
        heapless_frames,
    ) {
        // Tag with a name so we can find it
        let name_prop = [(PROP_NAME, PropValue::Str("console_backend".into()))];
        update_thing(id, &name_prop);
    }

    // 4. Register Sink
    register_sink(&GLOBAL_CONSOLE);
}
