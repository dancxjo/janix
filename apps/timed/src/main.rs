#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use models::*; // RtcDevice, SystemClock, Thing, Service
use thing_std::graph::{thing_find, thing_get_body, thing_create, thing_register_name, thing_set_body, symbol_intern};

#[allow(unused)]
fn decode_rtc(id: abi::ids::ThingId) -> Option<RtcDevice> {
    let (body, _) = thing_get_body(id)?;
    RtcDevice::decode_full(&body).ok()
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    log_info("TIMED: Starting...");

    // 1. Wait for relevant RTC service
    // We check for known RTC drivers.
    let rtc_services = ["service.rtc_cmos", "service.rtc_pl031"];
    let mut found_service = false;

    for _ in 0..50 { // Wait up to 5 seconds for a service to appear
        for name in rtc_services.iter() {
            if let Some(svc_id) = thing_find(name) {
                // Service found, wait for Ready
                loop {
                    if let (body, _) = thing_get_body(svc_id).unwrap() {
                         if let Ok(svc) = Service::decode_full(&body) {
                             if svc.state == 1 {
                                 found_service = true;
                                 break;
                             }
                         }
                    }
                    thing_std::time::sleep_ms(100);
                }
            }
            if found_service { break; }
        }
        if found_service { break; }
        thing_std::time::sleep_ms(100);
    }

    if !found_service {
        log_info("TIMED: Warning: No ready RTC service found. Proceeding anyway...");
    }

    // 2. Find device.rtc0
    // Retry loop just in case
    let mut rtc_id = None;
    for _ in 0..100 { 
        if let Some(id) = thing_find("device.rtc0") {
            rtc_id = Some(id);
            break;
        }
        thing_std::time::sleep_ms(100);
    }
    
    if rtc_id.is_none() {
        log_info("TIMED: Could not find device.rtc0");
        return;
    }
    let rtc_id = rtc_id.unwrap();
    
    // 3. Read Base Time
    let rtc_dev = decode_rtc(rtc_id);
    if rtc_dev.is_none() {
        log_info("TIMED: Failed to decode RTC device body");
        return;
    }
    let rtc_dev = rtc_dev.unwrap();
    
    log_info(&alloc::format!("TIMED: Found RTC base: {}s at mono {}ns", rtc_dev.base_seconds, rtc_dev.base_mono_ns));
    
    // 4. Create system.time
    let kind_sys_clock = symbol_intern("kind.SystemClock");
    let time_thing = thing_create(kind_sys_clock, abi::ids::ThingId(0));
    thing_register_name(time_thing, "system.time");
    
    let sys_clock = SystemClock {
        unix_epoch_ns: (rtc_dev.base_seconds as i64) * 1_000_000_000,
        status: 1, // Set
        last_set_mono_ns: rtc_dev.base_mono_ns,
        accuracy_ns: rtc_dev.accuracy_ns,
        source: rtc_dev.source,
        _pad: 0,
    };
    
    let _ = thing_set_body(time_thing, &sys_clock.encode_full());
    log_info("TIMED: Published system.time");

    loop {
        thing_std::time::sleep_ms(10000);
    }
}
