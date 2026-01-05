use alloc::vec::Vec;
use thing_std::graph::*;
use thing_std::*;

#[derive(Clone, Copy, Debug)]
pub struct SurfaceDesc {
    pub width: u32,
    pub height: u32,
    pub stride_pixels: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct DirtyRect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

pub trait Backend {
    fn configure(&mut self, desc: SurfaceDesc);
    fn present(&mut self, backbuffer: &[u32], dirty: DirtyRect);
}

pub struct CpuBytespaceBackend {
    display_bs_id: ThingId,
    fb_base: u64,
    width: u32,
    height: u32,
    stride: u32,
    fb_ptr: *mut u32,
}

impl CpuBytespaceBackend {
    pub fn new(display_bs_id: ThingId, fb_base: u64) -> Self {
        Self {
            display_bs_id,
            fb_base,
            width: 0,
            height: 0,
            stride: 0,
            fb_ptr: core::ptr::null_mut(),
        }
    }
}

impl Backend for CpuBytespaceBackend {
    fn configure(&mut self, desc: SurfaceDesc) {
        self.width = desc.width;
        self.height = desc.height;
        self.stride = desc.stride_pixels;
        
        let fb_size: u64 = (desc.width as u64) * (desc.height as u64) * 4;
        let _mapped = thing_std::memory::space_map(self.display_bs_id, self.fb_base, 0, fb_size);
        self.fb_ptr = self.fb_base as *mut u32;
    }

    fn present(&mut self, backbuffer: &[u32], dirty: DirtyRect) {
        if self.fb_ptr.is_null() { return; }
        
        unsafe {
            let x1 = dirty.x.max(0) as u32;
            let y1 = dirty.y.max(0) as u32;
            let x2 = ((dirty.x + (dirty.w as i32)) as u32).min(self.width);
            let y2 = ((dirty.y + (dirty.h as i32)) as u32).min(self.height);

            for y in y1..y2 {
                let row_start = (y * self.stride + x1) as usize;
                let row_len = (x2 - x1) as usize;
                
                // Backbuffer is assumed to be packed (stride = width)
                // But wait, backbuffer slice is just data.
                // We need to know backbuffer stride if it differs from screen stride.
                // For now, assume backbuffer stride == width == screen stride.
                // The `app.rs` logic allocates backbuffer as `width * height`.
                
                core::ptr::copy_nonoverlapping(
                    backbuffer.as_ptr().add(row_start),
                    self.fb_ptr.add(row_start),
                    row_len
                );
            }
        }
    }
}
