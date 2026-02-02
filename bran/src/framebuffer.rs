use crate::requests::FRAMEBUFFER_REQUEST;
use kernel::{FramebufferInfo, PixelFormat};

#[derive(Clone, Copy)]
pub struct Framebuffer {
    pub addr: *mut u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u32, // bytes per pixel
}

impl Framebuffer {
    pub fn new(fb: &limine::framebuffer::Framebuffer) -> Self {
        // Limine reports `bpp` in bits; clamp to supported 24/32-bit formats.
        let bits_per_pixel = fb.bpp() as u32;
        let bpp = match bits_per_pixel {
            16 => 2,
            24 => 3,
            32 => 4,
            _ => 4, // default to 32-bit to avoid divide-by-zero later
        };

        Self {
            addr: fb.addr() as *mut u32,
            width: fb.width() as u32,
            height: fb.height() as u32,
            pitch: fb.pitch() as u32,
            bpp,
        }
    }

    pub fn clear(&mut self, color: u32) {
        // Safety: We assume the framebuffer memory is valid for the byte length reported by Limine.
        // We only access up to pitch * height bytes.
        let buf_len = (self.pitch as usize).saturating_mul(self.height as usize);
        let buffer = unsafe {
            core::slice::from_raw_parts_mut(self.addr as *mut u8, buf_len)
        };

        let bpp = self.bpp.max(1);
        let color_bytes = color.to_le_bytes();
        let row_payload_bytes = (self.width as usize).saturating_mul(bpp as usize);

        for y in 0..self.height as usize {
            let row_start = y.saturating_mul(self.pitch as usize);
            let row_end = row_start.saturating_add(row_payload_bytes.min(self.pitch as usize));
            if row_end > buffer.len() {
                break;
            }

            let row = &mut buffer[row_start..row_end];
            for chunk in row.chunks_mut(bpp as usize) {
                chunk[0] = color_bytes[0];
                if bpp > 1 {
                    chunk[1] = color_bytes[1];
                }
                if bpp > 2 {
                    chunk[2] = color_bytes[2];
                }
                if bpp > 3 {
                    chunk[3] = color_bytes[3];
                }
            }
        }
    }
}

impl bud::framebuffer::FramebufferTarget for Framebuffer {
    fn info(&self) -> bud::framebuffer::FramebufferInfo {
        // Ensure stride is at least width * bpp
        // This handles cases where the bootloader might report 0 or invalid pitch
        let min_stride = self.width.saturating_mul(self.bpp);
        let stride = if self.pitch > 0 { self.pitch } else { min_stride };

        let format = match self.bpp {
            2 => bud::framebuffer::PixelFormat::Rgb565,
            3 => bud::framebuffer::PixelFormat::Bgr888,
            4 => bud::framebuffer::PixelFormat::Bgrx8888,
            _ => bud::framebuffer::PixelFormat::Unknown,
        };

        bud::framebuffer::FramebufferInfo {
            width: self.width,
            height: self.height,
            stride,
            format,
        }
    }

    fn buffer_mut(&mut self) -> &mut [u8] {
        let stride = if self.pitch > 0 {
            self.pitch
        } else {
            self.width.saturating_mul(self.bpp)
        };
        unsafe {
            core::slice::from_raw_parts_mut(
                self.addr as *mut u8,
                stride as usize * self.height as usize,
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
