use crate::BootRuntime;
use crate::memory::kheap::kernel_heap;
use core::alloc::{GlobalAlloc, Layout};

use linked_list_allocator::LockedHeap;

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub static TRACE_ALLOC: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

pub fn init<R: BootRuntime>(_rt: &R) {
    // Pre-expand heap so we don't need R during alloc
    let mut heap = kernel_heap().lock();
    // Expand by 32MB (8192 pages) for safe early boot
    // NOTE: If we still hit OOM, we might need to increase this, but
    // identifying leaks with a freeing allocator is the first step.
    heap.expand::<R>(8192)
        .expect("Failed to pre-expand kernel heap");

    unsafe {
        ALLOCATOR.lock().init(heap.base as *mut u8, heap.size);
    }

    crate::kinfo!("Global allocator initialized (base={:x} size={})", heap.base, heap.size);
}
