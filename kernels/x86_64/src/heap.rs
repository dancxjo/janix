#![allow(dead_code)]

use core::alloc::Layout;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use linked_list_allocator::LockedHeap;

// ===== CONFIG =====
// Keep this modest for bring-up. Increase later when paging/mem map is real.
pub const KERNEL_HEAP_SIZE_BYTES: usize = 256 * 1024 * 1024; // 256 MiB

#[global_allocator]
static KERNEL_ALLOCATOR: LockedHeap = LockedHeap::empty();

// Track intended heap region so alloc_error_handler can still report if allocator is clobbered.
static HEAP_INITIALIZED: AtomicBool = AtomicBool::new(false);
static HEAP_START: AtomicUsize = AtomicUsize::new(0);
static HEAP_SIZE: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn init_kernel_heap(heap_start: usize, heap_size: usize) {
    // Record intended region first.
    HEAP_START.store(heap_start, Ordering::Release);
    HEAP_SIZE.store(heap_size, Ordering::Release);

    // Initialize allocator.
    KERNEL_ALLOCATOR
        .lock()
        .init(heap_start as *mut u8, heap_size);
    HEAP_INITIALIZED.store(true, Ordering::Release);
}

pub fn stats() -> (usize, usize) {
    let heap = KERNEL_ALLOCATOR.lock();
    (heap.used(), heap.size())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    // We keep this intentionally dependency-light. During bring-up we may not have printing.
    // If you have a serial logger available, wire it here later.
    let _init = HEAP_INITIALIZED.load(Ordering::Acquire);
    let _start = HEAP_START.load(Ordering::Acquire);
    let _size_expected = HEAP_SIZE.load(Ordering::Acquire);
    let _ = layout;

    loop {
        core::hint::spin_loop();
    }
}
