#![cfg(all(target_os = "none", not(feature = "kernel")))]

use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::Layout;
use core::sync::atomic::{AtomicBool, Ordering};
use linked_list_allocator::LockedHeap;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[global_allocator]
static GLOBAL_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_user_heap() {
    if INITIALIZED.swap(true, Ordering::AcqRel) {
        return;
    }
    unsafe {
        let heap_start = USER_HEAP_START as *mut u8;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);
        GLOBAL_ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}
