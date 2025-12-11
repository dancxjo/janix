#![allow(dead_code)]

use core::alloc::Layout;
use linked_list_allocator::LockedHeap;

pub const KERNEL_HEAP_SIZE_BYTES: usize = 64 * 1024 * 1024;

/*
static GUARD: [u8; 0x10000] = [0; 0x10000];

#[global_allocator]
static KERNEL_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn init_kernel_heap(heap_start: usize, heap_size: usize) {
    kernel::println!(
        "heap::init_heap: start=0x{:x}, size=0x{:x} ({} bytes)",
        heap_start,
        heap_size,
        heap_size
    );
    kernel::println!("GUARD address: {:p}", &GUARD);
    kernel::println!("KERNEL_ALLOCATOR address: {:p}", &KERNEL_ALLOCATOR);
    unsafe {
        KERNEL_ALLOCATOR
            .lock()
            .init(heap_start as *mut u8, heap_size);
    }
}

pub fn get_heap_stats() -> (usize, usize) {
    let heap = KERNEL_ALLOCATOR.lock();
    (heap.used(), heap.size())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    kernel::println!("alloc_error_handler: KERNEL_ALLOCATOR address: {:p}", &KERNEL_ALLOCATOR);
    let (used, size) = get_heap_stats();
    kernel::println!("Heap stats: used={} size={}", used, size);
    panic!("allocation error: {:?}", layout);
}
*/
