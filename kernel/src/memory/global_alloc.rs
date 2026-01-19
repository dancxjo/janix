use crate::BootRuntime;
use crate::memory::kheap::kernel_heap;
use core::alloc::{GlobalAlloc, Layout};

pub struct ArenaAllocator;

unsafe impl GlobalAlloc for ArenaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // We must lock the kernel heap and try to allocate from the pinned arena.
        let mut heap = kernel_heap().lock();
        match heap.alloc_pinned(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => core::ptr::null_mut(), // OOM
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op as per design. Pinned memory is never freed individually.
        // It is reclaimed only if the entire arena is dropped (which pinned ones aren't).
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // Default realloc is alloc + memcpy + dealloc.
        // Since dealloc is no-op, we just implement alloc + memcpy.
        // SAFETY: Layout::from_size_align_unchecked is unsafe.
        let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
        // SAFETY: self.alloc is unsafe, but we are in an unsafe fn.
        // Rust 2024 requires explicit unsafe block even inside unsafe fn.
        let new_ptr = unsafe { self.alloc(new_layout) };
        if !new_ptr.is_null() {
            // SAFETY: copy_nonoverlapping is unsafe.
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ptr,
                    new_ptr,
                    core::cmp::min(layout.size(), new_size),
                );
            }
        }
        new_ptr
    }
}

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: ArenaAllocator = ArenaAllocator;

pub static TRACE_ALLOC: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

pub fn init<R: BootRuntime>(_rt: &R) {
    // Expand the "pinned" heap so we have space to start with.
    let mut heap = kernel_heap().lock();
    // Expand by 32MB (8192 pages) for implicit pinned arena
    heap.expand_pinned::<R>(8192)
        .expect("Failed to pre-expand kernel pinned heap");

    crate::kinfo!("Arena allocator initialized (pinned pre-expanded)");
}
