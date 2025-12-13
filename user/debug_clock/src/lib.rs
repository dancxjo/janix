#![no_std]

use thing_os::prelude::*;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "debug_clock: starting");

    let Some(clock) = SystemClock::discover(sys) else {
        println(sys, "debug_clock: TimeSource not found");
        sys.exit_thread();
    };

    let tick_hz = clock.tick_hz(sys).max(1);
    let start_ticks = clock.uptime_ticks(sys);
    let base_seconds = clock.now(sys).0;

    let mut last_logged = None;
    loop {
        let ticks = clock.uptime_ticks(sys);
        let elapsed_secs = ticks.saturating_sub(start_ticks) / tick_hz as u64;
        if last_logged.map(|prev| prev != elapsed_secs).unwrap_or(true) {
            let (hour, minute, second) = seconds_to_hms(base_seconds + elapsed_secs as i64);
            let ticks = clock.uptime_ticks(sys);
            log_dynamic(
                sys,
                format_args!(
                    "debug_clock: {:02}:{:02}:{:02} UTC (ticks={} uptime_secs={})",
                    hour, minute, second, ticks, elapsed_secs
                ),
            );
            last_logged = Some(elapsed_secs);
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

#[cfg(test)]
mod tests {
    use super::seconds_to_hms;

    #[test]
    fn formats_small_time_values() {
        assert_eq!(seconds_to_hms(7 * 3_600 + 5 * 60 + 9), (7, 5, 9));
    }

    #[test]
    fn wraps_at_midnight() {
        assert_eq!(seconds_to_hms(86_400), (0, 0, 0));
        assert_eq!(seconds_to_hms(-1), (23, 59, 59));
    }
}
