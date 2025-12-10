#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", feature(alloc_error_handler))]

#[cfg(target_os = "none")]
use core::alloc::Layout;
#[cfg(target_os = "none")]
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_os = "none")]
use linked_list_allocator::LockedHeap;

#[cfg(target_os = "none")]
const USER_HEAP_SIZE: usize = 512 * 1024; // 512 KiB per process for now

#[cfg(target_os = "none")]
static mut USER_HEAP_MEMORY: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[cfg(target_os = "none")]
static HEAP_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "none")]
#[global_allocator]
static USER_HEAP: LockedHeap = LockedHeap::empty();

/// Initialize the simple per-process heap region backing `alloc`.
#[cfg(target_os = "none")]
pub fn init_user_heap() {
    if !HEAP_INITIALIZED.load(Ordering::Acquire) {
        unsafe {
            let heap_ptr = core::ptr::addr_of_mut!(USER_HEAP_MEMORY) as *mut u8;
            USER_HEAP.lock().init(heap_ptr, USER_HEAP_SIZE);
        }
        HEAP_INITIALIZED.store(true, Ordering::Release);
    }
}

/// Host builds already have a global allocator.
#[cfg(not(target_os = "none"))]
pub fn init_user_heap() {}

#[cfg(target_os = "none")]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}
