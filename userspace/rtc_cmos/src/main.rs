#![no_std]
#![no_main]

use stem::kprintln;
use stem::rtc_time;
use stem::sleep;
use core::time::Duration;

#[no_mangle]
pub fn main(_arg: usize) -> i32 {
    kprintln!("RTC: Driver starting");
    
    loop {
        match rtc_time() {
            Ok(time) => {
                 kprintln!(
                     "RTC: {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                     time.year, time.month, time.day,
                     time.hour, time.minute, time.second
                 );
            }
            Err(e) => kprintln!("RTC: Read failed {:?}", e),
        }
        sleep(Duration::from_secs(5));
    }
}
