#![allow(dead_code)]

use core::alloc::Layout;
use linked_list_allocator::LockedHeap;

pub const KERNEL_HEAP_SIZE_BYTES: usize = 32 * 1024 * 1024;

#[global_allocator]
static KERNEL_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn init_kernel_heap(heap_start: usize, heap_size: usize) {
    unsafe {
        KERNEL_ALLOCATOR
            .lock()
            .init(heap_start as *mut u8, heap_size);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}
