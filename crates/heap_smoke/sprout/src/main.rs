#![no_std]
#![no_main]

extern crate alloc;

use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("SPROUT: I am alive!");

    loop {
        sched_yield();
    }
}

