use crate::{BootTasking, MapPerms};
pub mod arena;
pub mod boot_frame_alloc;
pub mod boot_heap;
pub mod frame_alloc;
pub mod global_alloc;
pub mod handle;
pub mod kheap;
pub mod layout;
pub mod map;
pub mod paging;

use crate::kinfo;
pub use frame_alloc::FRAME_ALLOCATOR;
use spin::Mutex;

/// Next user VA for mappings (starts at 0x1000_0000, grows up)
static NEXT_MAP_VA: Mutex<u64> = Mutex::new(0x1000_0000);

/// Allocate a user VA range. Simple bump allocator for v0.
pub fn alloc_user_va(size: usize) -> u64 {
    let mut next = NEXT_MAP_VA.lock();
    let va = *next;
    // Align to page boundary and bump
    *next = (*next + size as u64 + 4095) & !4095;
    va
}

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

/// Allocate `count` physically contiguous 4K frames.
/// Returns the physical base address if successful.
pub fn alloc_contiguous_frames(count: usize) -> Option<u64> {
    FRAME_ALLOCATOR.with_lock(|a| a.alloc_contiguous(count))
}

/// Global hook for mapping user pages. Set by scheduler init.
static mut MAP_USER_PAGE_HOOK: Option<unsafe fn(u64, u64) -> Result<(), ()>> = None;
static mut MAP_USER_PAGE_PERMS_HOOK: Option<unsafe fn(u64, u64, MapPerms) -> Result<(), ()>> = None;

/// Initialize the user page mapping hook
pub unsafe fn set_map_user_page_hook(hook: unsafe fn(u64, u64) -> Result<(), ()>) {
    unsafe { MAP_USER_PAGE_HOOK = Some(hook) };
}

/// Initialize the user page mapping hook with custom permissions.
pub unsafe fn set_map_user_page_perms_hook(hook: unsafe fn(u64, u64, MapPerms) -> Result<(), ()>) {
    unsafe { MAP_USER_PAGE_PERMS_HOOK = Some(hook) };
}

/// Map a physical page into the current process's userspace at the given virtual address.
/// This uses the global hook set during scheduler initialization.
pub unsafe fn map_user_page(virt: u64, phys: u64) -> Result<(), abi::errors::Errno> {
    if let Some(hook) = unsafe { MAP_USER_PAGE_HOOK } {
        unsafe { hook(virt, phys) }.map_err(|_| abi::errors::Errno::ENOMEM)
    } else {
        Err(abi::errors::Errno::EIO)
    }
}

/// Map a physical page into the current process's userspace with explicit permissions.
pub unsafe fn map_user_page_with_perms(
    virt: u64,
    phys: u64,
    perms: MapPerms,
) -> Result<(), abi::errors::Errno> {
    if let Some(hook) = unsafe { MAP_USER_PAGE_PERMS_HOOK } {
        unsafe { hook(virt, phys, perms) }.map_err(|_| abi::errors::Errno::ENOMEM)
    } else {
        Err(abi::errors::Errno::EIO)
    }
}
