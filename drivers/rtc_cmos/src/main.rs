#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use thing_std::cap::{CapOp, Cap, CapScope};
use models::{RtcDevice, HardwareInfo, Thing}; // Added Thing trait for encode_full/decode
// use graph::store; // NOT AVAILABLE
// use graph::symbols::{self, sym}; // NOT AVAILABLE

// CMOS Registers
const CMOS_ADDR: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

unsafe fn cmos_read(reg: u8) -> u8 {
    // Disable NMI (bit 7)
    let _ = thing_std::sys_ioport_write8(CMOS_ADDR, (1 << 7) | reg);
    thing_std::sys_ioport_read8(CMOS_DATA).unwrap_or(0)
}

fn bcd_to_binary(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd / 16) * 10)
}

// ... fn rtc_read_time() similar to before ...
fn rtc_read_time() -> Option<u64> {
     // Simplified for brevity in this fix, can restore full logic if needed or copied
    unsafe {
        for _ in 0..10000 {
            if (cmos_read(0x0A) & 0x80) == 0 {
                break;
            }
        }
        let sec = cmos_read(0x00);
        let min = cmos_read(0x02);
        let hour = cmos_read(0x04);
        let register_b = cmos_read(0x0B);

         let (_s, _m, _h) = if (register_b & 0x04) == 0 {
            (bcd_to_binary(sec), bcd_to_binary(min), bcd_to_binary(hour))
        } else {
            (sec, min, hour)
        };
        Some(0)
    }
}

pub fn main() {
    log_info("RTC-CMOS: Starting...");

    // 1. Find our hardware resource
    // Use thing_std::graph::thing_find
    let hw_thing = thing_std::graph::thing_find("hw.rtc0");
    // "hw.rtc0" name registered in boot.rs

    if hw_thing.is_none() {
        log_info("RTC-CMOS: No hardware found. Exiting.");
        return;
    }

    log_info("RTC-CMOS: Hardware found. Publishing device...");

    // 2. Publish device.rtc0
    let kind_rtc = thing_std::graph::symbol_intern("kind.RtcDevice");
    let device_place = thing_std::graph::thing_find("place.devices"); // sym::PLACE_DEVICES is "place.devices"

    let rtc_thing = thing_std::graph::thing_create(kind_rtc, ThingId(0)); 
    
    // Register name
    thing_std::graph::thing_register_name(rtc_thing, "device.rtc0");
    
    if let Some(dp) = device_place {
        let pred_contains = thing_std::graph::symbol_intern("pred.contains");
        thing_std::graph::relationship_create(pred_contains, dp, rtc_thing);
    }
    
    let source_cmos = thing_std::graph::symbol_intern("cmos");

    // Read full time
    let mut year = 0u16;
    let mut month = 0u8;
    let mut day = 0u8;
    let mut hour = 0u8;
    let mut minute = 0u8;
    let mut second = 0u8;

    unsafe {
        for _ in 0..10000 {
           if (cmos_read(0x0A) & 0x80) == 0 { break; }
        }
        second = cmos_read(0x00);
        minute = cmos_read(0x02);
        hour = cmos_read(0x04);
        day = cmos_read(0x07);
        month = cmos_read(0x08);
        year = cmos_read(0x09) as u16;
        
        let reg_b = cmos_read(0x0B);
        
        if (reg_b & 0x04) == 0 {
            second = bcd_to_binary(second);
            minute = bcd_to_binary(minute);
            hour = bcd_to_binary(hour); // Assuming 24h mode or simple handling
            day = bcd_to_binary(day);
            month = bcd_to_binary(month);
            year = bcd_to_binary(year as u8) as u16;
        }
        
        // Century? CMOS usually doesn't have it standard, assume 2000+
        year += 2000;
    }
    
    // Simple calc (ignoring leap seconds/complex leap years for minimal implementation)
    // Days since 1970
    // Simplified: 
    // 1. Years
    let mut days = 0u64;
    for y in 1970..year {
        if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) {
            days += 366;
        } else {
            days += 365;
        }
    }
    
    // 2. Months
    let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for m in 1..month {
        if m == 2 && ((year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)) {
            days += 29;
        } else {
             days += days_in_month[m as usize];
        }
    }
    
    days += (day as u64) - 1;
    
    let total_seconds = days * 86400 + (hour as u64) * 3600 + (minute as u64) * 60 + (second as u64);

    let payload = RtcDevice {
        source: source_cmos,
        accuracy_ns: 1_000_000_000,
        base_seconds: total_seconds,
        base_mono_ns: thing_std::monotonic_now(),
        flags: 0,
        _pad: 0,
    };
    
    let _ = thing_std::graph::thing_set_body(rtc_thing, &payload.encode_full());

    log_info(&alloc::format!("RTC-CMOS: Active. Time: {}-{}-{} {}:{}:{}", year, month, day, hour, minute, second));

    // Loop
    loop {
        // Just sleep to keep process alive
        thing_std::time::sleep_ms(10000);
    }
}
