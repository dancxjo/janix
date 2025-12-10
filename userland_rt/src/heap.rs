#![cfg(all(target_os = "none", not(feature = "kernel")))]

use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

struct HeapState {
    start: usize,
    end: usize,
    next: usize,
}

impl HeapState {
    const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            next: 0,
        }
    }
}

static HEAP_STATE: Mutex<HeapState> = Mutex::new(HeapState::new());
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn init_user_heap() {
    if INITIALIZED.swap(true, Ordering::AcqRel) {
        return;
    }
    let mut state = HEAP_STATE.lock();
    state.start = USER_HEAP_START;
    state.end = USER_HEAP_END;
    state.next = USER_HEAP_START;
}

struct UserHeapAllocator;

unsafe impl GlobalAlloc for UserHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut state = HEAP_STATE.lock();
        if state.start == 0 || layout.size() == 0 {
            return null_mut();
        }
        let align_mask = layout.align().saturating_sub(1);
        let aligned = (state.next + align_mask) & !align_mask;
        let new_next = match aligned.checked_add(layout.size()) {
            Some(val) => val,
            None => return null_mut(),
        };
        if new_next > state.end {
            return null_mut();
        }
        state.next = new_next;
        aligned as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // leak for now
    }
}

#[global_allocator]
static USER_HEAP_ALLOCATOR: UserHeapAllocator = UserHeapAllocator;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}
