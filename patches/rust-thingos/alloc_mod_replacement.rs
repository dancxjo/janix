#![forbid(unsafe_op_in_unsafe_fn)]

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr;

// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values.
#[allow(dead_code)]
const MIN_ALIGN: usize = 16;

#[allow(dead_code)]
unsafe fn realloc_fallback(
    alloc: &System,
    ptr: *mut u8,
    old_layout: Layout,
    new_size: usize,
) -> *mut u8 {
    // SAFETY: Docs for GlobalAlloc::realloc require this to be valid
    unsafe {
        let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());

        let new_ptr = GlobalAlloc::alloc(alloc, new_layout);
        if !new_ptr.is_null() {
            let size = usize::min(old_layout.size(), new_size);
            ptr::copy_nonoverlapping(ptr, new_ptr, size);
            GlobalAlloc::dealloc(alloc, ptr, old_layout);
        }

        new_ptr
    }
}

// Force thingos
mod thingos;
