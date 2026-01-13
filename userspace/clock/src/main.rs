#![no_std]
#![no_main]

use core::time::Duration;
use stem::info;
use time::OffsetDateTime;

/// Print a single tick with both wall clock (if anchored) and monotonic time.
fn print_tick(unix: u64, mono_ns: u64) {
    if unix == 0 {
        info!("CLOCK: unix={} utc=<unanchored> mono_ns={}", unix, mono_ns);
        return;
    }

    let unix_i64 = match i64::try_from(unix) {
        Ok(val) => val,
        Err(_) => {
            info!("CLOCK: unix={} utc=<out_of_range> mono_ns={}", unix, mono_ns);
            return;
        }
    };

    let dt = match OffsetDateTime::from_unix_timestamp(unix_i64) {
        Ok(val) => val,
        Err(_) => {
            info!("CLOCK: unix={} utc=<invalid> mono_ns={}", unix, mono_ns);
            return;
        }
    };

    info!(
        "CLOCK: unix={} utc={:04}-{:02}-{:02} {:02}:{:02}:{:02} mono_ns={}",
        unix,
        dt.year(),
        dt.month() as u8,
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
        mono_ns
    );
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
