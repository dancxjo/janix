#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate alloc;

use thing_std::*;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log_info("SPROUT: PANIC!");
    if let Some(_location) = info.location() {
        // No format! in no_std easily without alloc formatted string, but we have alloc now!
        // or just log hardcoded strings
        log_info("SPROUT: panic at location");
    }
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("SPROUT: I am alive!");

    loop {
        sched_yield();
    }
}

