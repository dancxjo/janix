#![cfg(target_os = "none")]

use abi::{USER_HEAP_END, USER_HEAP_START};
use core::alloc::Layout;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_user_heap() {
    unsafe {
        let heap_start = USER_HEAP_START as usize;
        let heap_size = USER_HEAP_END.saturating_sub(USER_HEAP_START);

        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

fn log_raw(s: &str) {
    let ptr = s.as_ptr() as u64;
    let len = s.len() as u64;
    unsafe {
        crate::sys::raw_syscall(abi::syscalls::SYSCALL_LOG, ptr, len, 0, 0, 0, 0);
    }
}

fn log_hex_val(val: u64) {
    let mut buf = [0u8; 16];
    for i in 0..16 {
        let shift = (15 - i) * 4;
        let nibble = ((val >> shift) & 0xF) as u8;
        buf[i] = if nibble < 10 {
            b'0' + nibble
        } else {
            b'A' + (nibble - 10)
        };
    }
    let s = unsafe { core::str::from_utf8_unchecked(&buf) };
    log_raw(s);
}

#[cfg(target_os = "none")]
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    unsafe {
        log_raw("ALLOCATION FAILED: size=");
        log_hex_val(layout.size() as u64);
        log_raw(" align=");
        log_hex_val(layout.align() as u64);
        log_raw("\n");
        crate::sys::raw_syscall(abi::syscalls::SYSCALL_EXIT_THREAD, 1, 0, 0, 0, 0, 0);
    }
    loop {}
}
