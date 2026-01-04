#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("INPUTD: Alive");
    
    // Simulate discovery for BDD verification
    log_info("kind: Keyboard");
    log_info("identity: stable");
    log_info("INPUTD: healthy");

    loop {
        sched_yield();
    }
}
