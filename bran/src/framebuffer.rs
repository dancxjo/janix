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
        let pixels = self.width * self.height;
        // Simple but slow clear for early boot
        for i in 0..pixels as usize {
            unsafe {
                // Correct for pitch
                let x = i % self.width as usize;
                let y = i / self.width as usize;
                let offset = (y * (self.pitch as usize / 4)) + x;
                *self.addr.add(offset) = color;
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
