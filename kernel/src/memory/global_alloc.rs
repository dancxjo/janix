use crate::BootRuntime;
use crate::memory::kheap::kernel_heap;
use core::alloc::{GlobalAlloc, Layout};

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub static TRACE_ALLOC: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

pub fn init<R: BootRuntime>(_rt: &R) {
    let mut heap = kernel_heap().lock();
    // Reserve 32MB (8192 pages) for the global heap
    let (base, size) = heap.reserve_region::<R>(8192)
        .expect("Failed to reserve kernel heap region");

    unsafe {
        ALLOCATOR.lock().init(base as *mut u8, size);
    }

    crate::kinfo!("Global allocator initialized (LinkedHeap, 32MB)");
}

