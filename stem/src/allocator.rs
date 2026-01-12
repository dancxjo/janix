use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;

const HEAP_SIZE: usize = 128 * 1024;

#[repr(C, align(4096))]
struct Heap {
    buf: [u8; HEAP_SIZE],
}

pub struct BumpAllocator {
    heap: UnsafeCell<Heap>,
    next: UnsafeCell<usize>,
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let next = self.next.get();
        let heap = self.heap.get();
        let start = (*heap).buf.as_ptr() as usize;
        let current = start + *next;

        // Align
        let aligned = (current + align - 1) & !(align - 1);
        let updated = aligned + size;

        if updated > start + HEAP_SIZE {
            core::ptr::null_mut()
        } else {
            *next = updated - start;
            aligned as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator {
    heap: UnsafeCell::new(Heap {
        buf: [0; HEAP_SIZE],
    }),
    next: UnsafeCell::new(0),
};
