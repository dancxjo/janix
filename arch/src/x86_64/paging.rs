use core::arch::asm;
use kernel::memory::{PhysFrame, allocate_frame, phys_to_virt};
use x86_64::structures::paging::{
    Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame as X86PhysFrame, Size4KiB,
    FrameAllocator,
};
use x86_64::{PhysAddr, VirtAddr};

/// A FrameAllocator that wraps the kernel's global frame allocator.
pub struct KernelFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for KernelFrameAllocator {
    fn allocate_frame(&mut self) -> Option<X86PhysFrame<Size4KiB>> {
        let frame = allocate_frame()?;
        Some(X86PhysFrame::containing_address(PhysAddr::new(
            frame.start_address,
        )))
    }
}

pub unsafe fn map_device_region(phys: u64, len: u64) {
    let hhdm_offset = kernel::memory::get_hhdm_offset();

    let level_4_table_ptr = x86_64::registers::control::Cr3::read()
        .0
        .start_address()
        .as_u64();
    let level_4_table_ptr = VirtAddr::new(level_4_table_ptr + hhdm_offset);
    let level_4_table: &mut PageTable = unsafe { &mut *level_4_table_ptr.as_mut_ptr() };

    let mut mapper = unsafe { OffsetPageTable::new(level_4_table, VirtAddr::new(hhdm_offset)) };
    let mut allocator = KernelFrameAllocator;

    let start = PhysAddr::new(phys);
    let end = PhysAddr::new(phys + len - 1);

    let start_frame = X86PhysFrame::<Size4KiB>::containing_address(start);
    let end_frame = X86PhysFrame::<Size4KiB>::containing_address(end);

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE;
        // Note: x86_64 crate doesn't have a direct "DEVICE" flag, usually we want NO_CACHE (PCD) or WRITE_THROUGH (PWT).
        // For MMIO, NO_CACHE is generally safer.

    for frame in X86PhysFrame::range_inclusive(start_frame, end_frame) {
        let page = Page::containing_address(VirtAddr::new(frame.start_address().as_u64() + hhdm_offset));
        
        // Check if already mapped
        if mapper.translate_page(page).is_ok() {
             kernel::println!("map_device_region: page {:?} already mapped, skipping/updating flags", page);
             // Verify/Update flags? For now just log.
             continue;
        }

        unsafe {
            match mapper.map_to(page, frame, flags, &mut allocator) {
                Ok(flush) => flush.flush(),
                Err(e) => {
                     kernel::println!("map_device_region: failed to map {:?} to {:?}: {:?}", page, frame, e);
                     panic!("Failed to map device region");
                }
            }
        }
    }
    
    kernel::println!("Mapped device region {:#x} len {:#x}", phys, len);
}
