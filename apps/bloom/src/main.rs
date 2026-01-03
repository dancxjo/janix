#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use thing_std::graph::*;
// use thing_std::graphics::*; // Not strictly needed for discovery check

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    loop {
         if let Some(display_dev) = thing_find("device.display0") {
             log_info("BLOOM: found display0");
             log_info("BLOOM: discovery success!");
             break;
         }
         sched_yield();
    }
    
    loop { sched_yield(); }
}
