#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    log_info("CLOCK: ALIVE");
    loop {
        let mono = monotonic_now();
        let sys = system_now();

        // We don't have a good way to format strings yet without alloc/format
        // but we can at least log that we got something.
        if mono > 0 {
            log_info("CLOCK: tick");
        }

        // Sleep for a bit (approx 1 second if 1 tick = 10ms, so 100 ticks)
        for _ in 0..100 {
            sched_yield();
        }
    }
}
