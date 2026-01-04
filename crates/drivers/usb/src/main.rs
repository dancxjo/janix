#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("USB: Driver starting");

    loop {
        sched_yield();
    }
}
