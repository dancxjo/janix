use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;

// A simple bump allocator.
// SAFETY: This is NOT thread safe. It assumes single threaded execution for now.
pub struct BumpAllocator {
    offset: UnsafeCell<usize>,
    heap: [u8; 1024 * 1024], // 1MB heap
}

unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            offset: UnsafeCell::new(0),
            heap: [0; 1024 * 1024],
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        use core::fmt::Write;
        use crate::debug::PortWrites;

        let offset_ptr = self.offset.get();
        let start = *offset_ptr;
        let align_mask = layout.align() - 1;
        
        let aligned_start = (start + align_mask) & !align_mask;
        let end = aligned_start + layout.size();

        // Very noisy, enable only for debug
        // let _ = PortWrites.write_fmt(format_args!("Alloc: size={} align={} start={} end={}\n", layout.size(), layout.align(), start, end));

        if end <= self.heap.len() {
            *offset_ptr = end;
            (self.heap.as_ptr() as *mut u8).add(aligned_start)
        } else {
            let _ = PortWrites.write_fmt(format_args!("Alloc FAILED: size={} align={} start={} end={} heap_len={}\n", layout.size(), layout.align(), start, end, self.heap.len()));
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
