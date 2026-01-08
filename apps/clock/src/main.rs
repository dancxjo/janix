#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use thing_std::*;
use models::{SystemClock, Thing};

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    log_info("CLOCK: Starting simple time ticker...");

    let mut time_thing = None;

    loop {
        // Try to find system.time if we haven't yet
        if time_thing.is_none() {
            time_thing = thing_std::graph::thing_find("system.time");
        }

        let time_str = if let Some(tid) = time_thing {
            if let Some((body, _)) = thing_std::graph::thing_get_body(tid) {
                if let Ok(sys_clock) = SystemClock::decode_full(&body) {
                    let now_mono = monotonic_now();
                    let elapsed_ns = now_mono - sys_clock.last_set_mono_ns;
                    let current_ns = (sys_clock.unix_epoch_ns as u64) + elapsed_ns;
                    let current_secs = current_ns / 1_000_000_000;
                    format_time(current_secs)
                } else {
                    String::from("--:--:--")
                }
            } else {
                String::from("--:--:--")
            }
        } else {
            String::from("--:--:--")
        };

        log_info(&format!("CLOCK: {}", time_str));
        sleep_ms(1000);
    }
}

fn format_time(unix_secs: u64) -> String {
    let secs_of_day = unix_secs % 86400;
    let hours = secs_of_day / 3600;
    let minutes = (secs_of_day % 3600) / 60;
    let seconds = secs_of_day % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
