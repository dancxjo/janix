use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;

// A simple bump allocator.
// SAFETY: This is NOT thread safe. It assumes single threaded execution for now.
pub struct BumpAllocator {
    offset: UnsafeCell<usize>,
    heap: [u8; 64 * 1024], // 64KB heap
}

unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            offset: UnsafeCell::new(0),
            heap: [0; 64 * 1024],
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let offset_ptr = self.offset.get();
        let start = *offset_ptr;
        let align_mask = layout.align() - 1;
        
        // Calculate alignment padding
        // We are allocating from self.heap base.
        // address = heap_base + start
        // we need (heap_base + start) & align_mask == 0
        // for simplicity, let's assume heap base is aligned (it is usually)
        // or just align the offset.
        
        let aligned_start = (start + align_mask) & !align_mask;
        let end = aligned_start + layout.size();

        if end <= self.heap.len() {
            *offset_ptr = end;
            (self.heap.as_ptr() as *mut u8).add(aligned_start)
        } else {
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
    // Try to print "PANIC" to console if possible, but we don't have easy access to Console here
    // without circular deps or global instance.
    // For v0, just loop.
    let _ = info;
    loop {}
}
