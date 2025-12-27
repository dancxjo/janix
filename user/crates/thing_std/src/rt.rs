use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;

// A simple bump allocator.
// SAFETY: This is NOT thread safe. It assumes single threaded execution for now.
pub struct BumpAllocator {
    offset: UnsafeCell<usize>,
    start: UnsafeCell<usize>,
    end: UnsafeCell<usize>,
}

unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            offset: UnsafeCell::new(0),
            start: UnsafeCell::new(0),
            end: UnsafeCell::new(0),
        }
    }
}

pub unsafe fn init_heap(start: usize, size: usize) {
    let alloc = &ALLOCATOR;
    unsafe {
        *alloc.start.get() = start;
        *alloc.end.get() = start + size;
        *alloc.offset.get() = start;
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        use core::fmt::Write;
        use crate::debug::PortWrites;

        let current_ptr = self.offset.get();
        let current = *current_ptr;
        let end_limit = *self.end.get();
        
        let align_mask = layout.align() - 1;
        let aligned_start = (current + align_mask) & !align_mask;
        let new_end = aligned_start + layout.size();

        if new_end <= end_limit {
            *current_ptr = new_end;
            aligned_start as *mut u8
        } else {
            let _ = PortWrites.write_fmt(format_args!("Alloc FAILED: size={} align={} current={:x} limit={:x}\n", layout.size(), layout.align(), current, end_limit));
            null_mut()
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Leak memory
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    use crate::debug::PortWrites;
    let _ = PortWrites.write_str("\n!!! USER PANIC !!!\n");
    if let Some(loc) = info.location() {
        let _ = PortWrites.write_fmt(format_args!("at {}:{}\n", loc.file(), loc.line()));
    }
    let _ = PortWrites.write_fmt(format_args!("{}\n", info.message()));
    loop {}
}
