// library/std/src/sys/alloc/thingos.rs
use crate::alloc::{GlobalAlloc, Layout, System};

unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.alloc(Layout::from_size_align_unchecked(new_size, layout.align()))
    }
}
