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
            log(&mut sys, "debug_clock", "Error running app");
        }
    }

    fn run(sys: &mut UserlandSys) -> Result<(), ()> {
        log(sys, "debug_clock", "starting...");

        let clock = SystemClock::discover(sys).ok_or(())?; 
        let tick_hz = clock.tick_hz(sys).max(1);
        let start_ticks = clock.uptime_ticks(sys);
        let base_seconds = clock.now(sys).0;

        let mut last_logged = None;

        loop {
            let ticks = clock.uptime_ticks(sys);
            let elapsed_secs = ticks.saturating_sub(start_ticks) / tick_hz as u64;
            
            if last_logged.map(|prev| prev != elapsed_secs).unwrap_or(true) {
                let (hour, minute, second) = seconds_to_hms(base_seconds + elapsed_secs as i64);
                let time_str = format!("{:02}:{:02}:{:02}", hour, minute, second);
                log(sys, "debug_clock", &format!("Time: {}", time_str));
                last_logged = Some(elapsed_secs);
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
