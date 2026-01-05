#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    log_info("CLOCK: ALIVE");
    let mut ticks = 0;
    loop {
        let _mono = monotonic_now();
        ticks += 1;

        let msg = format!("CLOCK: tick {}", ticks);
        log_info(&msg);

        // Sleep for approx 10 seconds
        sleep_ms(10000);
    }
}
