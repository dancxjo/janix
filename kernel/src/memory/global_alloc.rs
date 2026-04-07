use crate::BootRuntime;
#[cfg(not(test))]
use crate::memory::kheap::kernel_heap;
#[cfg(not(test))]
use core::alloc::{GlobalAlloc, Layout};

#[cfg(not(test))]
use linked_list_allocator::LockedHeap;

#[cfg(not(test))]
use core::sync::atomic::{AtomicU64, Ordering};

pub static TRACE_ALLOC: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Track the largest allocation seen
#[cfg(not(test))]
static LARGEST_ALLOC: AtomicU64 = AtomicU64::new(0);

/// Count of allocations over 1MB
#[cfg(not(test))]
static LARGE_ALLOC_COUNT: AtomicU64 = AtomicU64::new(0);

/// The inner heap allocator
#[cfg(not(test))]
static INNER_ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Wrapper allocator that logs large allocations
#[cfg(not(test))]
struct TracingAllocator;

#[cfg(not(test))]
unsafe impl GlobalAlloc for TracingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();

        // Track largest allocation
        let mut current = LARGEST_ALLOC.load(Ordering::Relaxed);
        while size as u64 > current {
            match LARGEST_ALLOC.compare_exchange_weak(
                current,
                size as u64,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(c) => current = c,
            }
        }



        let irq = crate::irq::irq_disable_erased();
        let ptr = unsafe { INNER_ALLOCATOR.alloc(layout) };
        crate::irq::irq_restore_erased(irq);
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let irq = crate::irq::irq_disable_erased();
        unsafe { INNER_ALLOCATOR.dealloc(ptr, layout) }
        crate::irq::irq_restore_erased(irq);
    }
}

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: TracingAllocator = TracingAllocator;

#[cfg(not(test))]
pub fn init<R: BootRuntime>(_rt: &R) {
    let mut heap = kernel_heap().lock();
    // Reserve 32MB (8192 pages) for the global heap
    let (base, size) = heap
        .reserve_region::<R>(8192)
        .expect("Failed to reserve kernel heap region");

    unsafe {
        INNER_ALLOCATOR.lock().init(base as *mut u8, size);
    }

    crate::kinfo!("Global allocator initialized (LinkedHeap, 32MB)");
}

/// Get diagnostics about large allocations
#[cfg(not(test))]
pub fn alloc_stats() -> (u64, u64) {
    (
        LARGEST_ALLOC.load(Ordering::Relaxed),
        LARGE_ALLOC_COUNT.load(Ordering::Relaxed),
    )
}

#[cfg(test)]
pub fn init<R: BootRuntime>(_rt: &R) {
    // In tests, we use the system allocator (std), so no manual init needed.
}

#[cfg(test)]
pub fn alloc_stats() -> (u64, u64) {
    (0, 0)
}
