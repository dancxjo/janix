#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub fn main() {
    log_info("SPROUT: I am alive");

    // Spawn essential services
    log_info("SPROUT: spawning services...");
    spawn("bloom");
    spawn("clock");
    spawn("inputd");

    // Spawn validation tools
    spawn("thingcheck");
    spawn("bouncer_test");

    log_info("SPROUT: boot sequence complete.");
}
