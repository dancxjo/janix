use kernel::memory::boot_frame_alloc::BootFrameAllocator;
use kernel::memory::paging::PageFlags;
use kernel::memory::frame_alloc::PhysFrame;

pub fn phys_to_virt_offset() -> u64 { 0 }

pub fn map_page(_virt: u64, _phys: PhysFrame, _flags: PageFlags) -> Result<(), ()> { Err(()) }
pub fn unmap_page(_virt: u64) -> Result<Option<PhysFrame>, ()> { Ok(None) }
pub fn translate(_virt: u64) -> Option<PhysFrame> { None }

pub fn tlb_flush_page(_virt: u64) {}
pub fn tlb_flush_all() {}

pub fn map_bootheap_page(_virt: u64, _phys: u64, _allocator: &mut BootFrameAllocator) {}
