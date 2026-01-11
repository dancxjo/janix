use core::alloc::{GlobalAlloc, Layout};
use crate::memory::kheap::kernel_heap;
use crate::BootRuntime;

#[global_allocator]
static ALLOCATOR: GlobalHeap = GlobalHeap;

struct GlobalHeap;

unsafe impl GlobalAlloc for GlobalHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut heap = kernel_heap().lock();
        
        // Align 'used' to the requested alignment
        let align = layout.align();
        let mask = align - 1;
        let aligned_used = (heap.used + mask) & !mask;
        
        if aligned_used + layout.size() > heap.size {
            // No auto-expansion for now, pre-expand in init
            return core::ptr::null_mut();
        }
        
        let ptr = (heap.base + aligned_used as u64) as *mut u8;
        heap.used = aligned_used + layout.size();
        crate::kprintln!("GlobalAlloc: Alloc {:p} (base={:x} used={:x} layout={:?})", ptr, heap.base, aligned_used, layout);
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op for bump allocator
    }
}

pub fn init<R: BootRuntime>(rt: &R) {
     // Pre-expand heap so we don't need R during alloc
     let mut heap = kernel_heap().lock();
     // Expand by 2MB for safe early boot
     heap.expand::<R>(512).expect("Failed to pre-expand kernel heap");
}
