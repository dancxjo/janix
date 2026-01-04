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

    // Spawn ontology tools if present (for testing)
    // In a real system, init would scan /boot/modules
    process::spawn("ontology_dump");
    process::spawn("ontology_check");
    process::spawn("thingcheck");

    // Orchestrator loop - keep running and yield to scheduler
    loop {
        sched_yield();
    }
}
