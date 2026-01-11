#![no_std]
#![no_main]

use stem::kprintln;
use stem::device::rtc_read_time;
use stem::syscall::sleep_ms;

#[no_mangle]
pub fn main() -> i32 {
    kprintln!("Clock App v0");
    let mut ticks = 0;
    loop {
        if let Ok(time) = rtc_read_time() {
             // kprintln!("\x1b[2J\x1b[H"); // clear screen
             kprintln!(
                 "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                 time.year, time.month, time.day,
                 time.hour, time.minute, time.second
             );
        } else {
             // Fallback
             kprintln!("Tick: {}", ticks);
             ticks += 1;
        }
        let _ = sleep_ms(1000);
    }
}
