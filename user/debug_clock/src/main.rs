#![no_std]
#![no_main]

use thing_os::prelude::*;
use thing_os::SystemClock;

#[thing_os::main]
fn main() {
    println!("debug_clock: starting...");

    let clock = SystemClock::discover().expect("SystemClock not found");
    let tick_hz = clock.tick_hz().max(1);
    let start_ticks = clock.uptime_ticks();
    let base_seconds = clock.now().0;

    let mut last_logged = None;

    loop {
        let ticks = clock.uptime_ticks();
        let elapsed_secs = ticks.saturating_sub(start_ticks) / tick_hz as u64;
        
        if last_logged.map(|prev| prev != elapsed_secs).unwrap_or(true) {
            let (hour, minute, second) = seconds_to_hms(base_seconds + elapsed_secs as i64);
            // let time_str = format!("{:02}:{:02}:{:02}", hour, minute, second);
            println!("debug_clock: Time: {:02}:{:02}:{:02}", hour, minute, second);
            last_logged = Some(elapsed_secs);
        }

        sleep_ms(250);
    }
}

fn seconds_to_hms(seconds: i64) -> (u32, u32, u32) {
    let secs = seconds.rem_euclid(86_400);
    let hour = (secs / 3_600) as u32;
    let minute = ((secs % 3_600) / 60) as u32;
    let second = (secs % 60) as u32;
    (hour, minute, second)
}
