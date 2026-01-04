#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("SPROUT: I am alive!");
    
    // Spawn Bloom (desktop service)
    log_info("SPROUT: spawning bloom");
    process::spawn("bloom");
    log_info("SPROUT: bloom spawned");
    
    // Orchestrator loop - keep running and yield to scheduler
    loop {
        sched_yield();
    }
}
