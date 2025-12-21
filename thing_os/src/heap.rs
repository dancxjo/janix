#![cfg(target_os = "none")]

use abi::wire::common::UserSlice;
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
            let req = abi::KernelRequest::Log { message: UserSlice::from_slice("ALLOC ERROR: layout.align is not power-of-two (halting)".as_bytes()) };
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
    crate::println!("heap::init_heap: start={:#x}", USER_HEAP_START);

    if INITIALIZED.swap(true, Ordering::AcqRel) {
        return;
    }

    // Print address of GLOBAL_ALLOCATOR to verify it's not NULL
    let alloc_addr = &GLOBAL_ALLOCATOR as *const _ as usize;
    crate::println!("KERNEL_ALLOCATOR address: {:#x}", alloc_addr);

    unsafe {
        let heap_start = USER_HEAP_START as *mut u8;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);
        crate::println!("heap::init_heap: initializing size={}", heap_size);

        GLOBAL_ALLOCATOR.0.lock().init(heap_start, heap_size);
        crate::println!("heap::init_heap: initialized");

    }
}

#[cfg(target_os = "none")]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    // Log without allocating to avoid recursive panic
    let msg = "ALLOCATION FAILED\n";
    let req = abi::KernelRequest::Log { message: UserSlice::from_slice(msg.as_bytes()) };
    crate::syscalls::syscall(req);

    // Exit thread gracefully (code 1 for error)
    unsafe {
        crate::sys::raw_syscall(abi::syscalls::SYSCALL_EXIT_THREAD, 1, 0, 0, 0, 0, 0);
    }
    loop {}
}
