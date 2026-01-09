use crate::memory::boot_frame_alloc::BootFrameAllocator;
use crate::memory::paging::PageFlags;
use crate::memory::frame_alloc::PhysFrame;

pub fn init(_offset: u64) {}

pub fn map_bootheap_page(_virt: u64, _phys: u64, _allocator: &mut BootFrameAllocator) {
     // Stub
}

pub fn test_paging() {
    // Stub
}

pub fn tlb_flush_page(_virt: u64) {}

pub struct AddressSpace;

impl AddressSpace {
    pub fn active() -> Self { Self }
    pub fn map_page(&mut self, _virt: u64, _phys: PhysFrame, _flags: PageFlags) -> Result<(), ()> { Ok(()) }
}
