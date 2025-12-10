#![allow(dead_code)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

pub trait AllocationObserver {
    fn on_alloc(&self, addr: usize, size: usize);
    fn on_dealloc(&self, addr: usize, size: usize);
}

pub struct NullObserver;

impl AllocationObserver for NullObserver {
    fn on_alloc(&self, _addr: usize, _size: usize) {}
    fn on_dealloc(&self, _addr: usize, _size: usize) {}
}

/// Simple bump allocator over a fixed virtual heap region.
/// For now: no deallocation, no reuse.
struct BumpAllocator<O: AllocationObserver> {
    heap_start: AtomicUsize,
    heap_size: AtomicUsize,
    next: AtomicUsize,
    observer: O,
}

impl<O: AllocationObserver> BumpAllocator<O> {
    const fn new(observer: O) -> Self {
        Self {
            heap_start: AtomicUsize::new(0),
            heap_size: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
            observer,
        }
    }

    /// Call this once after paging is up to point at a heap region.
    unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        self.heap_start.store(heap_start, Ordering::SeqCst);
        self.heap_size.store(heap_size, Ordering::SeqCst);
        self.next.store(heap_start, Ordering::SeqCst);
    }

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();

        let heap_start = self.heap_start.load(Ordering::SeqCst);
        let heap_size = self.heap_size.load(Ordering::SeqCst);

        // Align "next" up.
        let current = self.next.load(Ordering::SeqCst);
        if current == 0 {
            // Not initialized yet.
            return null_mut();
        }

        let aligned = (current + (align - 1)) & !(align - 1);
        let end = match aligned.checked_add(size) {
            Some(end) => end,
            None => return null_mut(),
        };

        if end > heap_start + heap_size {
            // Out of memory.
            return null_mut();
        }

        self.next.store(end, Ordering::SeqCst);
        self.observer.on_alloc(aligned, size);
        aligned as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.observer.on_dealloc(ptr as usize, layout.size());
        // No-op for now (leaking allocator).
        // Later we can add free list / buddy / graph-backed bookkeeping.
    }
}

pub struct GlobalBump(BumpAllocator<NullObserver>);

impl GlobalBump {
    pub const fn new() -> Self {
        Self(BumpAllocator::new(NullObserver))
    }

    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        unsafe { self.0.init(heap_start, heap_size) };
    }
}

unsafe impl GlobalAlloc for GlobalBump {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.0.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.0.dealloc(ptr, layout) }
    }
}

// This is the global allocator that satisfies the compiler.
#[global_allocator]
pub static KERNEL_ALLOCATOR: GlobalBump = GlobalBump::new();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    // You already have a panic handler; just call it.
    panic!("allocation error: {:?}", layout);
}
