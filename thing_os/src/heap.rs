#![cfg(target_os = "none")]

use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use linked_list_allocator::LockedHeap;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

// First bad allocation recorded as (align << 32) | (size_low32)
static FIRST_BAD_LAYOUT: AtomicU64 = AtomicU64::new(0);

struct CheckedHeap(LockedHeap);

unsafe impl GlobalAlloc for CheckedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let a = layout.align();
        let s = layout.size();

        // If alignment is ever not a power-of-two, that is a *hard invariant break*.
        if a == 0 || (a & (a - 1)) != 0 {
            let packed = ((a as u64) << 32) | ((s as u64) & 0xFFFF_FFFF);
            FIRST_BAD_LAYOUT.compare_exchange(0, packed, Ordering::AcqRel, Ordering::Relaxed).ok();

            // Log without allocating
            let req = abi::KernelRequest::Log { message: "ALLOC ERROR: layout.align is not power-of-two (halting)" };
            crate::syscalls::syscall(req);

            // Halt: we want the earliest failure point, not cascading corruption.
            loop {}
        }

        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CheckedHeap = CheckedHeap(LockedHeap::empty());

pub fn init_user_heap() {
    // Manually log to avoid allocation/formatting issues during heap init
    let req = abi::KernelRequest::Log { message: "init_user_heap: entered" };
    unsafe { crate::syscalls::syscall(req); }

    if INITIALIZED.swap(true, Ordering::AcqRel) {
        let req = abi::KernelRequest::Log { message: "init_user_heap: already initialized" };
        unsafe { crate::syscalls::syscall(req); }
        return;
    }
    unsafe {
        let heap_start = USER_HEAP_START as *mut u8;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);
        
        let req = abi::KernelRequest::Log { message: "init_user_heap: locking global allocator" };
        crate::syscalls::syscall(req);

        GLOBAL_ALLOCATOR.0.lock().init(heap_start, heap_size);
        
        let req = abi::KernelRequest::Log { message: "init_user_heap: allocator initialized" };
        crate::syscalls::syscall(req);

        let packed = FIRST_BAD_LAYOUT.load(Ordering::Acquire);
        if packed != 0 {
            // Don’t format! Just log a fixed string.
            let req = abi::KernelRequest::Log { message: "init_user_heap: WARNING: FIRST_BAD_LAYOUT was set" };
            crate::syscalls::syscall(req);
        }
    }
}

#[cfg(target_os = "none")]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Allocation failed");
}
