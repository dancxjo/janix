#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use thing_std::*;
use models::{SystemClock, Thing};

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    log_info("CLOCK: Starting...");
    
    let mut time_thing = None;
    let mut ticks = 0;
    
    loop {
        // Try to find if not yet found
        if time_thing.is_none() {
            time_thing = thing_std::graph::thing_find("system.time");
        }

        match time_thing {
            Some(tid) => {
                 if let Some((body, _)) = thing_std::graph::thing_get_body(tid) {
                     if let Ok(sys_clock) = SystemClock::decode_full(&body) {
                         let now_mono = monotonic_now();
                         let elapsed_ns = now_mono - sys_clock.last_set_mono_ns;
                         let current_ns = (sys_clock.unix_epoch_ns as u64) + elapsed_ns;
                         
                         let current_secs = current_ns / 1_000_000_000;
                         let time_str = format_time(current_secs);
                         log_info(&format!("CLOCK: {}", time_str));
                     } else {
                         log_info("CLOCK: failed to decode system.time");
                     }
                 } else {
                     log_info("CLOCK: Failed to get body of system.time");
                 }
            },
            None => {
                ticks += 1;
                log_info(&format!("CLOCK: Waiting for system.time... ({})", ticks));
            }
        }

        sleep_ms(10000);
    }
}

fn format_time(unix_secs: u64) -> String {
    let days_since_epoch = unix_secs / 86400;
    let secs_of_day = unix_secs % 86400;
    let hours = secs_of_day / 3600;
    let minutes = (secs_of_day % 3600) / 60;
    let seconds = secs_of_day % 60;

    let mut year = 1970;
    let mut days = days_since_epoch;

    loop {
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if is_leap { 366 } else { 365 };

        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap {
        days_in_month[1] = 29;
    }

    let mut month = 0;
    for (i, &d) in days_in_month.iter().enumerate() {
        if days < d {
            month = i + 1;
            break;
        }
        days -= d;
    }
    
    let day = days + 1; 

    format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hours, minutes, seconds)
}
