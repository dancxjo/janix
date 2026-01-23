
use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr;

unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Phase C2
        ptr::null_mut()
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
         // Phase C2
         ptr::null_mut()
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
         // Phase C2
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // use default fallback? 
        // crate::sys::alloc::realloc_fallback(self, ptr, layout, new_size)
        // realloc_fallback is not pub.
        // Copy realloc logic or use alloc+copy+dealloc
        let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
        let new_ptr = self.alloc(new_layout);
        if !new_ptr.is_null() {
            let size = usize::min(layout.size(), new_size);
            ptr::copy_nonoverlapping(ptr, new_ptr, size);
            self.dealloc(ptr, layout);
        }
        new_ptr
    }
}
