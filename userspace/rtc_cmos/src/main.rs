#![no_std]
#![no_main]

use stem::kprintln;
use stem::device::rtc_read_time;
use stem::syscall::sleep_ms;

#[no_mangle]
pub fn main() -> i32 {
    kprintln!("RTC CMOS Driver v0");
    
    loop {
        match rtc_read_time() {
            Ok(time) => {
                 kprintln!(
                     "RTC: {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                     time.year, time.month, time.day,
                     time.hour, time.minute, time.second
                 );
            }
            Err(e) => {
                kprintln!("RTC Error: {:?}", e);
            }
        }
        let _ = sleep_ms(1000);
    }
}
