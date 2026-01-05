#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use thing_std::*;
use models::{SystemClock, Thing};

fn format_time(total_seconds: u64) -> alloc::string::String {
    let mut days = total_seconds / 86400;
    let mut rem_seconds = total_seconds % 86400;
    
    let hours = rem_seconds / 3600;
    rem_seconds %= 3600;
    let minutes = rem_seconds / 60;
    let seconds = rem_seconds % 60;
    
    let mut year = 1970;
    loop {
        let days_in_year = if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) { 366 } else { 365 };
        if days < days_in_year { break; }
        days -= days_in_year;
        year += 1;
    }
    
    let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 1;
    loop {
        let mut dim = days_in_month[month];
        if month == 2 && ((year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)) { dim = 29; }
        
        if days < dim { break; }
        days -= dim;
        month += 1;
    }
    let day = days + 1;
    
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hours, minutes, seconds)
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    log_info("CLOCK: ALIVE");
    
    // Find system.time
    let mut time_thing = None;
    for _ in 0..20 { // Try for 10 seconds (500ms * 20)
        if let Some(id) = thing_std::graph::thing_find("system.time") {
            time_thing = Some(id);
            break;
        }
        sleep_ms(500);
    }
    
    if time_thing.is_none() {
        log_info("CLOCK: could not find system.time");
        // Fallback to tick counter?
    }
    
    let mut ticks = 0;
    loop {
        ticks += 1;
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
                log_info(&format!("CLOCK: tick {}", ticks));
            }
        }

        // Sleep for approx 10 seconds
        sleep_ms(10000);
    }
}
