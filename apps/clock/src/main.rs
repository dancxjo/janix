#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    log_info("CLOCK: ALIVE");
    loop {
        let mono = monotonic_now();

        // Since we don't have easy formatting, we'll just log that we're alive
        // and the monotonic clock is increasing.
        log_info("CLOCK: tick");

        // In a real app we'd format 'mono' into a string and draw it.
        // For now, this verifies the syscall works.

        // Sleep for approx 1 second
        sleep_ms(1000);
    }
}
