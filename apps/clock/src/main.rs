#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    log_info("CLOCK: ALIVE");
    loop {
        sched_yield();
    }
}
