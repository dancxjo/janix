#![allow(dead_code)]

use core::alloc::Layout;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use linked_list_allocator::LockedHeap;

// ===== CONFIG =====
// Keep this modest for bring-up. Increase later when paging/mem map is real.
pub const KERNEL_HEAP_SIZE_BYTES: usize = 256 * 1024 * 1024; // 256 MiB

#[global_allocator]
static KERNEL_ALLOCATOR: SafeLockedHeap = SafeLockedHeap(LockedHeap::empty());

pub struct SafeLockedHeap(LockedHeap);

unsafe impl core::alloc::GlobalAlloc for SafeLockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        x86_64::instructions::interrupts::without_interrupts(|| {
            self.0.alloc(layout)
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        x86_64::instructions::interrupts::without_interrupts(|| {
            self.0.dealloc(ptr, layout)
        })
    }
}

// Track intended heap region so alloc_error_handler can still report if allocator is clobbered.
static HEAP_INITIALIZED: AtomicBool = AtomicBool::new(false);
static HEAP_START: AtomicUsize = AtomicUsize::new(0);
static HEAP_SIZE: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn init_kernel_heap(heap_start: usize, heap_size: usize) {
    // Record intended region first.
    HEAP_START.store(heap_start, Ordering::Release);
    HEAP_SIZE.store(heap_size, Ordering::Release);

    // Initialize allocator.
    // Init happens before interrupts are enabled, so direct lock is fine, but we use the inner lock.
    KERNEL_ALLOCATOR
        .0
        .lock()
        .init(heap_start as *mut u8, heap_size);
    HEAP_INITIALIZED.store(true, Ordering::Release);
}

pub fn stats() -> (usize, usize) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let heap = KERNEL_ALLOCATOR.0.lock();
        (heap.used(), heap.size())
    })
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
