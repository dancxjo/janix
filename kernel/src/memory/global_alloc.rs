use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, Ordering};
use crate::memory::boot_heap::BootHeap;
use crate::memory::boot_frame_alloc::BootFrameAllocator;
use crate::memory::kheap::KernelHeap;
use crate::memory::layout::KHEAP_GROW_PAGES;
use spin::Mutex;

static BOOT_HEAP: Mutex<BootHeap> = Mutex::new(BootHeap::empty());
static KERNEL_HEAP: KernelHeap = KernelHeap::empty();
static USE_KERNEL_HEAP: AtomicBool = AtomicBool::new(false);

pub struct GlobalAllocator;

unsafe impl Sync for GlobalAllocator {}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if USE_KERNEL_HEAP.load(Ordering::Acquire) {
            // Kernel Heap Strategy
            let ptr = KERNEL_HEAP.alloc(layout);
            if !ptr.is_null() {
                return ptr;
            }
            
            // OOM? Try growing.
            if let Ok(_) = KERNEL_HEAP.grow(KHEAP_GROW_PAGES) {
                let ptr = KERNEL_HEAP.alloc(layout);
                if !ptr.is_null() {
                    return ptr;
                }
            }
            
            crate::kinfo!("Kernel Heap OOM! Request: {:?} Stats:", layout);
            KERNEL_HEAP.stats();
            panic!("Kernel Heap OOM");
        } else {
            // Boot Heap Strategy
            // Safety: We assume single-threaded execution during boot phase.
            // But we use a lock now for static safety.
            BOOT_HEAP.lock().alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if USE_KERNEL_HEAP.load(Ordering::Acquire) {
            KERNEL_HEAP.dealloc(ptr, layout);
        } else {
             // Boot heap leaks.
        }
    }
}

#[global_allocator]
static GLOBAL: GlobalAllocator = GlobalAllocator;

/// Initialize the boot heap.
pub fn init_boot(allocator: BootFrameAllocator) {
    BOOT_HEAP.lock().init(allocator);
}

pub fn get_global() -> &'static GlobalAllocator {
    &GLOBAL
}

/// Helper to transfer boot frames for the frame allocator transition.
pub unsafe fn transfer_boot_frames(target: &mut crate::memory::frame_alloc::FrameAllocator) {
    let heap = BOOT_HEAP.lock();
    if let Some(boot_alloc) = &heap.allocator {
            boot_alloc.transfer_state_to(target);
    }
}

pub fn switch_to_kernel_heap() {
    USE_KERNEL_HEAP.store(true, Ordering::Release);
    crate::kinfo!("global_alloc: switched to kernel heap");
}

pub fn kernel_heap() -> &'static KernelHeap {
    &KERNEL_HEAP
}
