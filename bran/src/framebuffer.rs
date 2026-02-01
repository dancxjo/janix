use crate::requests::FRAMEBUFFER_REQUEST;
use kernel::{FramebufferInfo, PixelFormat};
use bud::framebuffer::PixelFormat as BudPixelFormat;

#[derive(Clone, Copy)]
pub struct Framebuffer {
    pub addr: *mut u8,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u16,
    pub format: BudPixelFormat,
}

impl Framebuffer {
    pub fn new(fb: &limine::framebuffer::Framebuffer) -> Self {
        let bpp = fb.bpp() as u16;
        let format = match (fb.memory_model(), bpp) {
            (limine::framebuffer::MemoryModel::RGB, 32) => BudPixelFormat::Bgrx8888,
            (limine::framebuffer::MemoryModel::RGB, 24) => BudPixelFormat::Bgr888,
            _ => BudPixelFormat::Unknown,
        };

        Self {
            addr: fb.addr() as *mut u8,
            width: fb.width() as u32,
            height: fb.height() as u32,
            pitch: fb.pitch() as u32,
            bpp,
            format,
        }
    }

    pub fn clear(&mut self, color: u32) {
        // color is 0xAARRGGBB
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        // let a = ((color >> 24) & 0xFF) as u8;

        // For clear, we fill the buffer.
        // We only support 32bpp and 24bpp BGR(A/X) for now.

        for y in 0..self.height {
            let row_offset = (y as usize) * (self.pitch as usize);

            // Calculate row length in bytes based on width * bytes_per_pixel
            // Note: pitch might be larger than width * bpp (padding)
            let row_bytes = self.width as usize * (self.bpp as usize / 8);

            let row_slice = unsafe {
                core::slice::from_raw_parts_mut(
                    self.addr.add(row_offset),
                    row_bytes
                )
            };

            if self.bpp == 32 {
                for chunk in row_slice.chunks_exact_mut(4) {
                    chunk[0] = b;
                    chunk[1] = g;
                    chunk[2] = r;
                    chunk[3] = 0xFF; // Fill alpha/reserved with opaque
                }
            } else if self.bpp == 24 {
                for chunk in row_slice.chunks_exact_mut(3) {
                    chunk[0] = b;
                    chunk[1] = g;
                    chunk[2] = r;
                }
            }
        }
    }
}

impl bud::framebuffer::FramebufferTarget for Framebuffer {
    fn info(&self) -> bud::framebuffer::FramebufferInfo {
        bud::framebuffer::FramebufferInfo {
            width: self.width,
            height: self.height,
            stride: self.pitch,
            format: self.format,
        }
    }

    fn buffer_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self.addr,
                self.pitch as usize * self.height as usize,
            )
        }
    }

    fn clear(&mut self, color: u32) {
        self.clear(color);
    }
}

pub fn get_info() -> Option<FramebufferInfo> {
    if let Some(resp) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = resp.framebuffers().into_iter().next() {
            return Some(FramebufferInfo {
                addr: fb.addr() as u64,
                byte_len: (fb.pitch() * fb.height()) as usize,
                width: fb.width() as u32,
                height: fb.height() as u32,
                pitch: fb.pitch() as u32,
                bpp: fb.bpp() as u16,
                format: match fb.memory_model() {
                    // Limine "RGB" memory model is actually BGRX in memory layout
                    limine::framebuffer::MemoryModel::RGB => PixelFormat::Bgrx8888,
                    _ => PixelFormat::Unknown,
                },
            });
        }
    }
    None
}
