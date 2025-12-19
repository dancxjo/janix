use abi::resident::ResidentMapPerms;
use abi::{MapFlags, ProcessId};
use crate::memory::PhysFrame;
use alloc::vec::Vec;


#[derive(Debug, Clone)]
pub struct ResidentPage {
    pub frame: PhysFrame,
}

impl Drop for ResidentPage {
    fn drop(&mut self) {
        crate::memory::free_frame(self.frame);
    }
}

pub fn allocate_pages(byte_len: usize) -> Option<Vec<ResidentPage>> {
    let page_count = (byte_len + 4095) / 4096;
    let mut pages = Vec::with_capacity(page_count);

    for _ in 0..page_count {
        if let Some(frame) = crate::memory::allocate_frame() {
            pages.push(ResidentPage { frame });
        } else {
            // Vec will drop existing pages, triggering Drop to free frames
            return None;
        }
    }
    Some(pages)
}

// Actual implementation will depend on `kernel::memory`
