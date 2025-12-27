#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use alloc::format;

use thing_std as std;
use abi::ThingId;
use models::core::input::{KeyEventStreamBody};
use models::Thing;
use std::GraphClient;
use models::abi::wire::graph::{GraphOp, GraphReply};
use models::builtins::ids::{THING_EMITS_KIND, THING_KEYBOARD_KIND};
use models::abi::wire::typed::TypedBytes; // Ensure access

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();
    std::debug::log("Keylog App starting...\n");

    let g = GraphClient::new();
    let mut buf = [0u8; 4096]; // Larger buffer for responses

    // 1. Find Keyboard Stream
    // Scan for links with Predicate = EMITS
    // For each, check if Source is Keyboard. If so, Destination is Stream.
    
    let mut stream_id = ThingId(0);
    let mut found = false;

    // Retry discovery loop
    while !found {
        let op = GraphOp::ScanLinks { from: None, to: None, kind: Some(THING_EMITS_KIND) };
        if let Ok(GraphReply::Links(links)) = g.call_op(&op, &mut buf) {
             for (src, dst, _pred) in links {
                 // Check src kind
                 let op_get = GraphOp::GetThing { id: src };
                 if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op_get, &mut buf) {
                      // Check if kind is KEYBOARD (Thing type_id is Kind ID)
                      if tb.type_id.0 as u64 == THING_KEYBOARD_KIND.0 {
                           stream_id = dst;
                           found = true;
                           std::debug::log(&format!("Found Keyboard Stream: t{}\n", dst.0));
                           break;
                      }
                 }
             }
        }
        
        if !found {
            // wait and retry
            core::hint::spin_loop();
        }
    }

    let mut last_seq = 0u64;
    
    loop {
        // Poll Stream
        let op_get = GraphOp::GetThing { id: stream_id };
        if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op_get, &mut buf) {
            // Decode Stream Body
             if let Ok(stream) = postcard::from_bytes::<KeyEventStreamBody>(&tb.bytes) {
                  // Process new events
                  // Events are in a sliding window.
                  // We need to print events with seq > last_seq.
                  // We can infer seq of each event by (head_seq - (len - 1 - index)) ?
                  // Or just iterate.
                  
                  // Latest event has seq = head_seq.
                  // Event at index i has seq = head_seq - (len - 1 - i).
                  
                  let len = stream.events.len();
                  if len > 0 {
                       let first_seq_in_buf = stream.head_seq - (len as u64) + 1;
                       
                       for (i, evt) in stream.events.iter().enumerate() {
                            let seq = first_seq_in_buf + (i as u64);
                            if seq > last_seq {
                                 let msg = format!("Key: Scancode={:#x} Rel={} Seq={}\n", evt.scancode, evt.is_release, seq);
                                 std::debug::log(&msg);
                                 last_seq = seq;
                            }
                       }
                  }
             }
        }
        
        // Yield/Sleep
        // Simple busy wait for v0
        for _ in 0..10_000 { core::hint::spin_loop(); }
    }
}
