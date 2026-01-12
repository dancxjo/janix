#![no_std]
#![no_main]

extern crate alloc;
use stem::println;
use stem::thing::{sys as thingsys, ThingId};
use stem::abi::types::RootWatchEvent;

#[no_mangle]
pub extern "C" fn main(_arg: usize) {
    println!("CLOCK: Starting...");
    
    let rtc = wait_for_device("dev.rtc.Cmos");
    println!("CLOCK: Found RTC: {:?}", rtc);
    
    let src = wait_for_edge_target(rtc, "provides");
    println!("CLOCK: Found time source: {:?}", src);
    
    let instant = wait_for_edge_target(src, "current");
    println!("CLOCK: Found time instant: {:?}", instant);
    
    let watch = thingsys::watch_subscribe(instant, 0xFF).expect("Subscribe failed");
    
    println!("CLOCK: Watching time...");
    let mut evt = RootWatchEvent::default();
    
    loop {
        match thingsys::stream_poll(watch, &mut evt) {
            Ok(1) => {
                 if evt.target == instant.0 {
                     println!("CLOCK: Time Update! val={}", evt.value);
                 }
            },
            Ok(_) => {
                stem::yield_now();
            },
            Err(e) => {
                println!("CLOCK: Poll error: {:?}", e);
                stem::sleep(core::time::Duration::from_secs(1));
            },
        }
    }
}

fn wait_for_device(kind: &str) -> ThingId {
    let mut buf = [ThingId(0); 1];
    println!("CLOCK: wait_for_device '{}'", kind);
    loop {
        println!("CLOCK: calling find...");
        let res = thingsys::find(kind, &mut buf);
        println!("CLOCK: find returned {:?}", res);
        match res {
            Ok(1) => {
                println!("CLOCK: find success, result={:?}", buf[0]);
                return buf[0];
            }
            Ok(n) => {
                 println!("CLOCK: find returned count {}", n);
            }
            Err(e) => {
                 println!("CLOCK: find error {:?}", e);
            }
        }
        stem::sleep(core::time::Duration::from_millis(500));
    }
}

fn wait_for_edge_target(src: ThingId, rel: &str) -> ThingId {
    // Large formatting buffer for stack safety check?
    // Reduce size to be safe.
    let mut buf = [0u8; 512]; 
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
