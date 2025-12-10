#![no_std]

use userland::prelude::*;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "alarm_demo: starting");

    let Some(clock) = SystemClock::discover(sys) else {
        println(sys, "alarm_demo: TimeSource not found");
        sys.exit_thread();
    };

    let (now_secs, now_nanos) = clock.now(sys);
    let target_secs = now_secs.saturating_add(5);
    let Some(alarm) = Alarm::request_at(sys, target_secs, now_nanos) else {
        println(sys, "alarm_demo: failed to create AlarmRequest");
        sys.exit_thread();
    };

    let (hour, minute, second) = seconds_to_hms(target_secs);
    log_dynamic(
        sys,
        format!(
            "alarm_demo: waiting for alarm at {:02}:{:02}:{:02}",
            hour, minute, second
        ),
    );

    loop {
        if let Some(state) = alarm.state(sys) {
            if state == "Fired" {
                let (fired_secs, _) = clock.now(sys);
                let (fh, fm, fs) = seconds_to_hms(fired_secs);
                log_dynamic(
                    sys,
                    format!("alarm_demo: alarm fired at {:02}:{:02}:{:02}", fh, fm, fs),
                );
                break;
            } else if state == "Cancelled" {
                log_dynamic(sys, "alarm_demo: alarm cancelled".into());
                break;
            }
        }
        sys.yield_now();
    }

    sys.exit_thread();
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
