#![no_std]
#![no_main]
extern crate alloc;
use thing_std::*;
#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init(0);
    log_info("INPUTD: Alive");
    loop { sched_yield(); }
}
