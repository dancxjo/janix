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
    let cpu = stem::arch::whoami();
    info!(
        "[clock] whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    );

    info!("[clock] starting");

    loop {
        let unix = stem::time::now_unix_seconds();
        let mono_ns = stem::monotonic_ns();
        print_tick(unix, mono_ns);

        stem::sleep(Duration::from_secs(1));
    }
}
