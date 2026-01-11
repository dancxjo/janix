#![no_std]
#![no_main]

use stem::kprintln;
use stem::rtc_time;
use stem::sleep;
use core::time::Duration;

#[no_mangle]
pub fn main(_arg: usize) -> i32 {
    kprintln!("CLOCK: starting");
    
    loop {
        sleep(Duration::from_secs(3));
        if let Ok(time) = rtc_time() {
             kprintln!(
                 "CLOCK: {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                 time.year, time.month, time.day,
                 time.hour, time.minute, time.second
             );
        }
    }
}
