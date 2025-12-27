#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use thing_std as std;
use abi::{ThingId};
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::input::{Key, KeyState};
use models::core::input::{KeyEventStreamBody, TextEventStreamBody};
use models::Thing;
use std::GraphClient;

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();
    std::debug::log("Keylog App (Input Service Client) starting...\n");

    let g = GraphClient::new();
    let mut buf = [0u8; 4096];

    let key_stream_id = ThingId(3100);
    let text_stream_id = ThingId(3200);

    let mut last_key_seq = 0u64;
     let mut last_text_seq = 0u64;
    
    loop {
        // Poll Key Stream
        let op_key = GraphOp::GetThing { id: key_stream_id };
        if let Ok(GraphReply::Thing { bytes }) = g.call_op(&op_key, &mut buf) {
             if let Ok(thing) = postcard::from_bytes::<Thing>(&bytes) {
                  // Decode TypedBytes
                  use abi::wire::typed::TypedBytes;
                  if let Ok(typed) = postcard::from_bytes::<TypedBytes>(&thing.body.bytes) {
                      if let Ok(stream) = postcard::from_bytes::<KeyEventStreamBody>(&thing.body.bytes) {
                           let head = stream.head_seq;
                           if head > last_key_seq {
                               let len = stream.events.len() as u64;
                               let start = head.saturating_sub(len);
                               for (i, evt) in stream.events.iter().enumerate() {
                                   let seq = start + (i as u64) + 1;
                                   if seq > last_key_seq {
                                       let state_str = if evt.state == KeyState::Down { "DN" } else { "UP" };
                                       std::debug::log(&format!("KEY: {:?} {} [Mods: S={} C={} A={}]\n", evt.key, state_str, evt.mods.shift, evt.mods.ctrl, evt.mods.alt));
                                       last_key_seq = seq;
                                   }
                               }
                           }
                      }
                  }
             }
        }
        
        // Poll Text Stream
        let op_text = GraphOp::GetThing { id: text_stream_id };
        if let Ok(GraphReply::Thing { bytes }) = g.call_op(&op_text, &mut buf) {
             if let Ok(thing) = postcard::from_bytes::<Thing>(&bytes) {
                  use abi::wire::typed::TypedBytes;
                  if let Ok(typed) = postcard::from_bytes::<TypedBytes>(&thing.body.bytes) {
                      if let Ok(stream) = postcard::from_bytes::<TextEventStreamBody>(&thing.body.bytes) {
                           let head = stream.head_seq;
                           if head > last_text_seq {
                               let len = stream.events.len() as u64;
                               let start = head.saturating_sub(len);
                               for (i, evt) in stream.events.iter().enumerate() {
                                   let seq = start + (i as u64) + 1;
                                   if seq > last_text_seq {
                                       std::debug::log(&format!("TEXT: '{}' (Kind: {:?})\n", evt.text, evt.kind));
                                       last_text_seq = seq;
                                   }
                               }
                           }
                      }
                  }
             }
        }
        
        for _ in 0..10_000 { core::hint::spin_loop(); }
    }
}
