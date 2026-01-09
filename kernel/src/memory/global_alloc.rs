use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use crate::memory::boot_heap::BootHeap;
use crate::memory::boot_frame_alloc::BootFrameAllocator;

pub struct BootGlobalAlloc {
    inner: UnsafeCell<BootHeap>,
}

unsafe impl Sync for BootGlobalAlloc {}

impl BootGlobalAlloc {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(BootHeap::empty()),
        }
    }

    /// SAFETY: Must be called only once and disjoint from any allocation.
    pub unsafe fn init(&self, allocator: BootFrameAllocator) {
        unsafe {
            let heap = &mut *self.inner.get();
            heap.init(allocator);
        }
    }
    
    pub fn stats(&self) {
        unsafe {
            (*self.inner.get()).stats();
        }
    }
}

unsafe impl GlobalAlloc for BootGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // In a real kernel we need critical sections.
        // For strictly boot single-core, this is "okay" but we should disable IRQs preferably.
        // Assuming IRQs are disabled during this early boot phase.
        
        unsafe {
            let heap = &mut *self.inner.get();
            heap.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Leak-only allocator
    }
}

#[global_allocator]
static GLOBAL: BootGlobalAlloc = BootGlobalAlloc::new();

pub fn init(allocator: BootFrameAllocator) {
    unsafe {
        GLOBAL.init(allocator);
    }
}
