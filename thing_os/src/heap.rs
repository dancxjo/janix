#![cfg(target_os = "none")]

use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::Layout;
use core::sync::atomic::{AtomicBool, Ordering};
use linked_list_allocator::LockedHeap;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[global_allocator]
static GLOBAL_ALLOCATOR: LockedHeap = LockedHeap::empty();

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

        GLOBAL_ALLOCATOR.lock().init(heap_start, heap_size);
        
        let req = abi::KernelRequest::Log { message: "init_user_heap: done" };
        crate::syscalls::syscall(req);
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("Allocation failed");
}
