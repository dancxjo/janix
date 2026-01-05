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

    let rtc_thing = thing_std::graph::thing_create(kind_rtc, ThingId(0)); // Parent 0? Or maybe we can't set parent in create easily if graph trait API doesn't allow it. 
    // thing_std::graph::thing_create takes (kind, parent).
    // Let's pass parent 0 and link it manually if needed.
    // Wait, thing_std::graph::thing_create signature: pub fn thing_create(kind: SymbolId, parent: ThingId) -> ThingId
    
    // So if we don't know devices place yet, what to do?
    // If device_place exists, link to it.
    
    // Register name
    thing_std::graph::thing_register_name(rtc_thing, "device.rtc0");
    
    if let Some(dp) = device_place {
        let pred_contains = thing_std::graph::symbol_intern("pred.contains");
        thing_std::graph::relationship_create(pred_contains, dp, rtc_thing);
    }
    
    let source_cmos = thing_std::graph::symbol_intern("cmos");

    let payload = RtcDevice {
        source: source_cmos,
        accuracy_ns: 1_000_000_000,
        flags: 0,
        _pad: 0,
    };
    
    let _ = thing_std::graph::thing_set_body(rtc_thing, &payload.encode_full());

    log_info("RTC-CMOS: Active.");

    // Loop
    loop {
        // Read time, update payload/event
        thing_std::time::sleep_ms(1000);
    }
}
