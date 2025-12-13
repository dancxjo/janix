#![allow(dead_code)]

use core::alloc::Layout;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use linked_list_allocator::LockedHeap;

pub const KERNEL_HEAP_SIZE_BYTES: usize = 64 * 1024 * 1024;

// A little "tripwire" in .bss so we can see when nearby memory gets stomped.
static GUARD: [u8; 0x10000] = [0; 0x10000];

#[global_allocator]
static KERNEL_ALLOCATOR: LockedHeap = LockedHeap::empty();

// Track what we *think* the heap is, so alloc_error_handler can report useful info
// even if the allocator itself got clobbered.
static HEAP_INITIALIZED: AtomicBool = AtomicBool::new(false);
static HEAP_START: AtomicUsize = AtomicUsize::new(0);
static HEAP_SIZE: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn init_kernel_heap(heap_start: usize, heap_size: usize) {
    // Record intended heap region first, so a later crash can still print it.
    HEAP_START.store(heap_start, Ordering::Release);
    HEAP_SIZE.store(heap_size, Ordering::Release);

    kernel::println!(
        "heap::init_heap: start=0x{:x}, size=0x{:x} ({} bytes)",
        heap_start,
        heap_size,
        heap_size
    );
    kernel::println!("GUARD address: {:p}", &GUARD);
    kernel::println!("KERNEL_ALLOCATOR address: {:p}", &KERNEL_ALLOCATOR);

    KERNEL_ALLOCATOR
        .lock()
        .init(heap_start as *mut u8, heap_size);
    HEAP_INITIALIZED.store(true, Ordering::Release);

    // Confirm initialization immediately.
    let (used, size) = get_heap_stats();
    kernel::println!(
        "heap::init_heap: confirmed heap size={} used={}",
        size,
        used
    );
}

pub fn get_heap_stats() -> (usize, usize) {
    let heap = KERNEL_ALLOCATOR.lock();
    (heap.used(), heap.size())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    kernel::println!("alloc_error_handler: layout={:?}", layout);
    kernel::println!(
        "alloc_error_handler: KERNEL_ALLOCATOR address: {:p}",
        &KERNEL_ALLOCATOR
    );
    kernel::println!("alloc_error_handler: GUARD address: {:p}", &GUARD);

    let init = HEAP_INITIALIZED.load(Ordering::Acquire);
    let start = HEAP_START.load(Ordering::Acquire);
    let size_expected = HEAP_SIZE.load(Ordering::Acquire);

    kernel::println!(
        "alloc_error_handler: heap_init={} expected_start=0x{:x} expected_size=0x{:x}",
        init,
        start,
        size_expected
    );

    // If the allocator was stomped back to "empty", heap.size() will be 0.
    let (used, size_actual) = get_heap_stats();
    kernel::println!(
        "alloc_error_handler: heap stats used={} size={}",
        used,
        size_actual
    );

    if init && size_expected != 0 && size_actual == 0 {
        kernel::println!(
            "alloc_error_handler: HEAP LOOKS CLOBBERED (likely stack overflow / static overwrite)"
        );
    }

    panic!("allocation error: {:?}", layout);
}
