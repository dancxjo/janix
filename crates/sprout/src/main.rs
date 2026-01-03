#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    loop {}
    thing_std::init(0); // Dummy init
    log_info("KERNEL: init task alive");
    log_info("SPROUT: I am alive!");

    loop {
        sched_yield();
    }
}
