#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("HEAP SMOKE: Starting...");

    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);
        if i % 100 == 0 {
            log_info("HEAP SMOKE: Pushed 100 items...");
        }
    }

    log_info("HEAP SMOKE: Vector push success!");
    loop {
        sched_yield();
    }
}
