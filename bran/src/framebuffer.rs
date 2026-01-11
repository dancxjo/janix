use kernel::{FramebufferInfo, PixelFormat};
use crate::requests::FRAMEBUFFER_REQUEST;

pub struct Framebuffer {
    pub addr: *mut u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
}

impl Framebuffer {
    pub fn new(fb: &limine::framebuffer::Framebuffer) -> Self {
        Self {
            addr: fb.addr() as *mut u32,
            width: fb.width() as u32,
            height: fb.height() as u32,
            pitch: fb.pitch() as u32,
        }
    }

    pub fn clear(&mut self, color: u32) {
        // Convert raw pointer to a slice for safe(r) manipulation
        // Safety: We assume the framebuffer memory is valid for the byte length reported by Limine.
        // We only access up to pitch * height.
        let buffer = unsafe {
            core::slice::from_raw_parts_mut(
                self.addr,
                (self.pitch as usize * self.height as usize) / 4,
            )
        };

        for y in 0..self.height as usize {
            let row_start = (y * self.pitch as usize) / 4;
            let row_end = row_start + self.width as usize;

            if row_end <= buffer.len() {
                buffer[row_start..row_end].fill(color);
            }
        }
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
                    limine::framebuffer::MemoryModel::RGB => PixelFormat::Xrgb8888,
                    _ => PixelFormat::Unknown,
                },
            });
        }
    }
    None
}
