#![no_std]
#![no_main]

extern crate alloc;
use stem::kprintln;
use stem::thing::{sys as thingsys, ThingId};
use stem::abi::types::RootWatchEvent;

#[no_mangle]
pub extern "C" fn main(_arg: usize) {
    kprintln!("CLOCK: Starting...");
    
    let rtc = wait_for_device("dev.rtc.cmos");
    kprintln!("CLOCK: Found RTC: {:?}", rtc);
    
    let src = wait_for_edge_target(rtc, "provides");
    kprintln!("CLOCK: Found time source: {:?}", src);
    
    let instant = wait_for_edge_target(src, "current");
    kprintln!("CLOCK: Found time instant: {:?}", instant);
    
    let watch = thingsys::watch_subscribe(instant, 0xFF).expect("Subscribe failed");
    
    kprintln!("CLOCK: Watching time...");
    let mut evt = RootWatchEvent::default();
    
    loop {
        match thingsys::stream_poll(watch, &mut evt) {
            Ok(1) => {
                 if evt.target == instant.0 {
                     kprintln!("CLOCK: Time Update! val={}", evt.value);
                 }
            },
            Ok(_) => {
                stem::yield_now();
            },
            Err(e) => {
                kprintln!("CLOCK: Poll error: {:?}", e);
                stem::sleep(core::time::Duration::from_secs(1));
            },
        }
    }
}

fn wait_for_device(kind: &str) -> ThingId {
    let mut buf = [ThingId(0); 1];
    loop {
        if let Ok(1) = thingsys::find(kind, &mut buf) {
            return buf[0];
        }
        stem::sleep(core::time::Duration::from_millis(500));
    }
}

fn wait_for_edge_target(src: ThingId, rel: &str) -> ThingId {
    let mut buf = [0u8; 1024];
    loop {
         if let Ok(len) = thingsys::dump_edges(src, &mut buf) {
             let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
             for line in s.lines() {
                 if line.contains(rel) && line.contains("-->") {
                     if let Some(pos) = line.find("-->") {
                         let rest = &line[pos+3..];
                         let parts: alloc::vec::Vec<&str> = rest.split_whitespace().collect();
                         if !parts.is_empty() {
                              if let Ok(id) = parts[0].parse::<u64>() {
                                  return ThingId(id);
                              }
                         }
                     }
                 }
             }
         }
         stem::sleep(core::time::Duration::from_millis(500));
    }
}
