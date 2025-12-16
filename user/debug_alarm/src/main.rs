#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
mod app {
    extern crate alloc;
    use thing_os::prelude::*;
    use examples_support::{log, sleep_ms, UserlandSys};
    use alloc::format;

    #[unsafe(no_mangle)]
    pub fn main() {
        let mut sys = examples_support::init();
        if let Err(_) = run(&mut sys) {
            log(&mut sys, "debug_alarm", "Error running app");
        }
    }

    fn run(sys: &mut UserlandSys) -> Result<(), ()> {
        log(sys, "debug_alarm", "starting");

        let clock = SystemClock::discover(sys).ok_or(())?; 
        let (now_secs, now_nanos) = clock.now(sys);
        
        // Set alarm for 5 seconds in future
        let target_secs = now_secs.saturating_add(5);
        let mut alarm = Alarm::request_at(sys, target_secs, now_nanos).ok_or(())?;

        let (hour, minute, second) = seconds_to_hms(target_secs);
        log(sys, "debug_alarm", &format!("waiting for alarm at {:02}:{:02}:{:02}", hour, minute, second));

        loop {
            if let Some(state) = alarm.state(sys) {
                if state == "Fired" {
                    let (fired_secs, _) = clock.now(sys);
                    let (fh, fm, fs) = seconds_to_hms(fired_secs);
                    log(sys, "debug_alarm", &format!("alarm fired at {:02}:{:02}:{:02}", fh, fm, fs));

                    // Re-arm for 5s later
                    let next_target_secs = fired_secs.saturating_add(5);
                    let (_, nanos) = clock.now(sys);
                    
                    if let Some(new_alarm) = Alarm::request_at(sys, next_target_secs, nanos) {
                        alarm = new_alarm;
                        let (nh, nm, ns) = seconds_to_hms(next_target_secs);
                        log(sys, "debug_alarm", &format!("next alarm scheduled at {:02}:{:02}:{:02}", nh, nm, ns));
                    } else {
                        log(sys, "debug_alarm", "failed to re-arm");
                        return Err(());
                    }
                } else if state == "Cancelled" {
                    log(sys, "debug_alarm", "alarm cancelled");
                    return Ok(());
                }
            }
            
            sleep_ms(sys, 100);
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
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
