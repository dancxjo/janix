use crate::BootTasking;
pub mod boot_frame_alloc;
pub mod boot_heap;
pub mod frame_alloc;
pub mod global_alloc;
pub mod kheap;
pub mod layout;
pub mod paging;

use crate::kinfo;
pub use frame_alloc::FRAME_ALLOCATOR;

pub fn init<R: crate::BootRuntime>(rt: &R) {
    let map = rt.phys_memory_map();
    let _modules = rt.modules();
    let offset = rt.phys_to_virt_offset();

    kinfo!("Memory map has {} entries", map.len());
    for (i, range) in map.iter().enumerate() {
        kinfo!(
            "  [{}] 0x{:x} - 0x{:x} ({:?})",
            i,
            range.start,
            range.end,
            range.kind
        );
    }
    kinfo!("HHDM Offset: 0x{:x}", offset);

    // 1. Setup early frame allocator
    let bitmap = boot_frame_alloc::init(map, offset);
    let alloc = frame_alloc::FrameAllocator::new_from_boot(map, _modules, bitmap, offset);

    kinfo!(
        "Frame allocator initialized with {} free frames",
        alloc.free_count()
    );

    unsafe { FRAME_ALLOCATOR.init(alloc) };

    rt.tasking().init(offset);
}

pub fn alloc_frame() -> Option<u64> {
    FRAME_ALLOCATOR.with_lock(|a| a.alloc().map(|f| f.0))
}
