#![no_std]
#![no_main]

use core::time::Duration;
use stem::info;

/// Print a single tick with both wall clock (if anchored) and monotonic time.
fn print_tick(unix: u64, mono_ns: u64) {
    info!("CLOCK: unix={} mono_ns={}", unix, mono_ns);
}

#[stem::main]
fn main() -> ! {
    info!("[clock] starting");

    loop {
        let unix = stem::time::now_unix_seconds();
        let mono_ns = stem::monotonic_ns();
        print_tick(unix, mono_ns);

        stem::sleep(Duration::from_secs(1));
    }
}
