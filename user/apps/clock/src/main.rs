#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};
use models::Thing;
use abi::ids::ThingId;
use abi::wire::graph::{GraphOp, GraphReply};
use serde::Deserialize;

#[derive(Deserialize)]
struct SystemTimeProps {
    unix_seconds: u64,
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let g = GraphClient::new();
    let c = StdoutConsole;
    c.write_str("CLOCK: Starting...\n");
    
    let mut buf = [0u8; 2048]; // Increased buffer for full Thing serialization
    let mut sys_time_id: Option<ThingId> = None;
    
    loop {
        // 1. Locate SystemTime if unknown
        if sys_time_id.is_none() {
             // Scan simple range
             // Scan dynamic range
             for i in 268435456..268435556 {
                  let op = GraphOp::GetThing { id: abi::ids::ThingId(i) };
                  if let Ok(GraphReply::Thing { bytes }) = g.call_op(&op, &mut buf) {
                        if let Ok(thing) = postcard::from_bytes::<Thing>(&bytes) {
                             // Workaround: Kernel might produce Kind 0 for dynamically created things in v0.2
                             if thing.kind.0 == 200 || thing.kind.0 == 0 {
                                  sys_time_id = Some(thing.id);
                                  c.write_str("CLOCK: Found SystemTime\n");
                                  break;
                             }
                        }
                  }
             }
        }
        
        if let Some(id) = sys_time_id {
             let op = GraphOp::GetThing { id: abi::ids::ThingId(id.0) };
             if let Ok(GraphReply::Thing { bytes }) = g.call_op(&op, &mut buf) {
                  if let Ok(thing) = postcard::from_bytes::<Thing>(&bytes) {
                        // Decode body
                        if let Ok(props) = postcard::from_bytes::<SystemTimeProps>(&thing.body.bytes) {
                             let (h, m, s) = format_hms(props.unix_seconds);
                             c.write_str("\rCLOCK: ");
                             c.write_2d(h);
                             c.write_str(":");
                             c.write_2d(m);
                             c.write_str(":");
                             c.write_2d(s);
                             c.write_str("   \n"); // Using newline for now to avoid scrolling issues/artifacts
                        } else {
                             // c.write_str("decode props failed\n");
                        }
                  }
             }
        } else {
             c.write_str("CLOCK: Waiting for SystemTime...\n");
        }
        
        thing_std::time::sleep_ms(&g, 1000);
    }
}

fn format_hms(total: u64) -> (u64, u64, u64) {
    let s = total % 60;
    let m = (total / 60) % 60;
    let h = (total / 3600) % 24;
    (h, m, s)
}
