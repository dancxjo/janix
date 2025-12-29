#![no_std]
#![no_main]

extern crate alloc;

use thing_std as std;
use thing_std::{GraphClient, StdoutConsole, Console};
use alloc::format;
use core::fmt::Write;

use thing_models::abi::wire::graph::{GraphOp, GraphReply};
use thing_models::abi::ThingId;
use thing_models::builtins::ids::*;
use thing_models::core::serial::LogStreamBody;

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();
    let g = GraphClient::new();
    let c = StdoutConsole;
    
    c.write_str("LOG_VIEWER: Searching for LogStream...\n");

    let mut found_id: Option<ThingId> = None;
    let mut last_seq = 0u64;
    let mut buf = [0u8; 4096];

    loop {
        if let Some(id) = found_id {
            // Poll
            let op = GraphOp::GetThing { id };
            if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                if let Ok(body) = postcard::from_bytes::<LogStreamBody>(&tb.bytes) {
                     // Check for new entries
                     // The body contains a window of entries. We iterate and print ones > last_seq
                     
                     // Optimization: if body.head_seq == last_seq, sleep.
                     
                     for entry in body.entries {
                         if entry.seq > last_seq {
                             // Print
                             let _ = c.write_str(&format!("[{}.{}] {:?}: {}\n", 
                                 entry.timestamp_ns / 1_000_000_000, 
                                 (entry.timestamp_ns / 1_000_000) % 1000,
                                 entry.level, // TODO: map to CHAR
                                 entry.message));
                             last_seq = entry.seq;
                         }
                     }
                }
            }
        } else {
            // Search (ScanLinks from Boot Root)
            let op = GraphOp::ScanLinks { 
                 from: Some(THING_BOOT_ROOT), 
                 to: None, 
                 kind: Some(THING_EMITS_KIND) 
            };
            if let Ok(GraphReply::Links(links)) = g.call_op(&op, &mut buf) {
                 for (_src, dst, _pred) in links {
                      // Check if dst is LogStream
                      let op_get = GraphOp::GetThing { id: dst };
                      if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op_get, &mut buf) {
                           if ThingId(tb.type_id.0 as u64) == THING_LOG_STREAM_KIND {
                               c.write_str("LOG_VIEWER: Found LogStream!\n");
                               found_id = Some(dst);
                               break;
                           }
                      }
                 }
            }
        }

        // Sleep 100ms
        for _ in 0..10_000_000 { core::hint::spin_loop(); }
    }
}
