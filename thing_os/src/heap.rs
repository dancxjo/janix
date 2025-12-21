#![cfg(target_os = "none")]

use abi::wire::common::UserSlice;
use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicU64, Ordering};
use linked_list_allocator::LockedHeap;

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
            log_raw("ALLOC ERROR: layout.align is not power-of-two (halting)\n");

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

fn log_raw(s: &str) {
    let ptr = s.as_ptr() as u64;
    let len = s.len() as u64;
    unsafe {
        crate::sys::raw_syscall(abi::syscalls::SYSCALL_LOG, ptr, len, 0, 0, 0, 0);
    }
}

fn log_hex(label: &str, val: usize) {
    log_raw(label);
    log_raw("0x");

    let mut buf = [0u8; 18]; // 16 digits + newline + null
    let mut idx = 0;

    // Convert to hex
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        let c = if digit < 10 {
            b'0' + digit as u8
        } else {
            b'a' + (digit - 10) as u8
        };
        // Skip leading zeros? No, print full 64-bit for clarity
        buf[idx] = c;
        idx += 1;
    }
    buf[idx] = b'\n';

    let s = unsafe { core::str::from_utf8_unchecked(&buf[..idx+1]) };
    log_raw(s);
}

pub fn init_user_heap() {
    log_raw("heap::init_heap: starting\n");
    log_hex("heap::init_heap: start=", USER_HEAP_START);

    unsafe {
        let heap_start = USER_HEAP_START as *mut u8;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);

        log_hex("heap::init_heap: size=", heap_size);

        GLOBAL_ALLOCATOR.0.lock().init(heap_start, heap_size);

        log_raw("heap::init_heap: initialized\n");
    }
}

#[cfg(target_os = "none")]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    log_raw("ALLOCATION FAILED\n");

    // Exit thread gracefully (code 1 for error)
    unsafe {
        crate::sys::raw_syscall(abi::syscalls::SYSCALL_EXIT_THREAD, 1, 0, 0, 0, 0, 0);
    }
    loop {}
}
