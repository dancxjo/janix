#![no_std]
#![no_main]

use standard::prelude::*;
use stem::{sys_rtc_cmos_read, sys_yield};

fn bcd_to_binary(val: u8) -> u8 {
    (val & 0x0F) + ((val / 16) * 10)
}

#[no_mangle]
pub fn main() -> i32 {
    loop {
        // Simple read
        // 0x00 sec, 0x02 min, 0x04 hr, 0x07 day, 0x08 mo, 0x09 yr
        // 0x0B status B (bit 2 = binary mode, bit 1 = 24h)
        
        let sec = unsafe { sys_rtc_cmos_read(0x00) };
        let min = unsafe { sys_rtc_cmos_read(0x02) };
        let hour = unsafe { sys_rtc_cmos_read(0x04) };
        
        if sec < 0 {
            println!("rtc_cmos: unsupported or error");
             // Just yield long time
            for _ in 0..100 { unsafe { sys_yield() }; }
            continue;
        }
        
        let sec = bcd_to_binary(sec as u8);
        let min = bcd_to_binary(min as u8);
        let hour = bcd_to_binary(hour as u8);
        
        println!("rtc_cmos: {:02}:{:02}:{:02}", hour, min, sec);
        
        // Yield for a while
        for i in 0..5000000 {
            // tight loop or yield
             // unsafe { sys_yield() }; // yielding makes it super slow in QEMU sometimes if scheduler is round robin 
             // Just spin to waste time for now to simulate delay, but we should yield to be cooperative.
             if i % 1000 == 0 { unsafe { sys_yield() }; }
        }
    }
}
