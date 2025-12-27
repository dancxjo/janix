#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};
use abi::ids::ThingId;
use abi::wire::graph::{GraphOp, GraphReply};
use models::core::time::TimeNow;



#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { thing_std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    let g = GraphClient::new();
    let c = StdoutConsole;
    c.write_str("CLOCK: Starting...\n");
    
    let mut buf = [0u8; 2048]; // Increased buffer for full Thing serialization
    let mut sys_time_id: Option<ThingId> = None;
    
    loop {
        // 1. Locate TimeNow if unknown
        if sys_time_id.is_none() {
             // Link based discovery
             // Root -> HAS_TIME_NOW -> ?
             let op = GraphOp::ScanLinks { 
                 from: Some(models::builtins::ids::THING_BOOT_ROOT), 
                 to: None, 
                 kind: Some(models::builtins::ids::THING_HAS_TIME_NOW_KIND) 
             };
             
             if let Ok(GraphReply::Links(list)) = g.call_op(&op, &mut buf) {
                 if let Some((_, target, _)) = list.first() {
                     sys_time_id = Some(*target);
                     c.write_str("CLOCK: Found TimeNow via link\n");
                 }
             }
        }
        
        if let Some(id) = sys_time_id {
             let op = GraphOp::GetThing { id: abi::ids::ThingId(id.0) };
             if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                        // Decode body
                        if let Ok(props) = postcard::from_bytes::<TimeNow>(&tb.bytes) {
                             // SystemTime is ns
                             let (h, m, s) = format_hms(props.system_ns / 1_000_000_000);
                             c.write_str("CLOCK: ");
                             c.write_2d(h);
                             c.write_str(":");
                             c.write_2d(m);
                             c.write_str(":");
                             c.write_2d(s);
                             c.write_str("   \r");
                        }
             } else {
                 c.write_str("CLOCK: GetThing failed\n");
             }
        } else {
             c.write_str("CLOCK: Waiting for TimeNow link...\n");
        }
        
        let _ = thing_std::time::sleep_ms(&g, 1000);
    }
}

fn format_hms(total: u64) -> (u64, u64, u64) {
    let s = total % 60;
    let m = (total / 60) % 60;
    let h = (total / 3600) % 24;
    (h, m, s)
}
