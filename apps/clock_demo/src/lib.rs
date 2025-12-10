#![no_std]

use userland::prelude::*;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "clock_demo: starting");

    let Some(clock) = SystemClock::discover(sys) else {
        println(sys, "clock_demo: TimeSource not found");
        sys.exit_thread();
    };

    let mut last_logged = None;
    loop {
        let (unix_seconds, _) = clock.now(sys);
        if last_logged.map(|prev| prev != unix_seconds).unwrap_or(true) {
            let (hour, minute, second) = seconds_to_hms(unix_seconds);
            let ticks = clock.uptime_ticks(sys);
            log_dynamic(
                sys,
                format!(
                    "clock_demo: {:02}:{:02}:{:02} UTC (ticks={})",
                    hour, minute, second, ticks
                ),
            );
            last_logged = Some(unix_seconds);
        }
        sys.yield_now();
    }
}

fn seconds_to_hms(seconds: i64) -> (u32, u32, u32) {
    let secs_in_day = 86_400_i64;
    let mut secs = seconds.rem_euclid(secs_in_day);
    if secs < 0 {
        secs += secs_in_day;
    }
    let hour = (secs / 3_600) as u32;
    secs %= 3_600;
    let minute = (secs / 60) as u32;
    let second = (secs % 60) as u32;
    (hour, minute, second)
}
