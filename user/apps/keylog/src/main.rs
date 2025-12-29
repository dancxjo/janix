#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use thing_std as std;
use abi::{ThingId};
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::typed::TypedBytes;
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

    let mut window_id: Option<ThingId> = None;
    let mut log_lines: heapless::Deque<heapless::String<64>, 10> = heapless::Deque::new();

    // Create Window
    {
        std::debug::log("KEYLOG: Creating Window...\n");
        let mut content: heapless::String<1024> = heapless::String::new();
        let _ = content.push_str("Waiting for input...");
        
        let mut title: heapless::String<64> = heapless::String::new();
        let _ = title.push_str("Input Log");

        let window_body = models::schema::window::WindowBody {
            title,
            x: 800,
            y: 50,
            width: 300,
            height: 400,
            content,
            seq: 0,
        };
        
        let op = GraphOp::CreateThing {
            kind: models::builtins::ids::THING_WINDOW_KIND,
            value: abi::wire::typed::TypedBytes {
                 type_id: abi::wire::typed::TypeId(models::builtins::ids::THING_WINDOW_SCHEMA.0.into()),
                 codec_id: abi::wire::typed::CodecId::POSTCARD,
                 bytes: postcard::to_allocvec(&window_body).unwrap(),
            }
        };
        
        if let Ok(GraphReply::Created{ id }) = g.call_op(&op, &mut buf) {
             window_id = Some(id);
             std::debug::log("KEYLOG: Window Created!\n");
             
             let link_op = GraphOp::AddLink {
                 from: models::builtins::ids::THING_BOOT_ROOT,
                 to: id,
                 kind: models::builtins::ids::THING_OWNS_KIND
             };
             let _ = g.call_op(&link_op, &mut buf);
        }
    }
    
    loop {
        let mut changed = false;
        // Poll Key Stream
        let op_key = GraphOp::GetThing { id: key_stream_id };
        if let Ok(GraphReply::Thing { bytes }) = g.call_op(&op_key, &mut buf) {
             if let Ok(thing) = postcard::from_bytes::<Thing>(&bytes) {
                  if let Ok(typed) = postcard::from_bytes::<TypedBytes>(&thing.body.bytes) {
                        if let Ok(stream) = postcard::from_bytes::<KeyEventStreamBody>(&typed.bytes) {
                               let head = stream.head_seq;
                               if head > last_key_seq {
                                   let len = stream.events.len() as u64;
                                   let start = head.saturating_sub(len);
                                   for (i, evt) in stream.events.iter().enumerate() {
                                       let seq = start + (i as u64) + 1;
                                       if seq > last_key_seq {
                                           let state_str = if evt.state == KeyState::Down { "DN" } else { "UP" };
                                           let line = format!("KEY: {:?} {}", evt.key, state_str);
                                           
                                           if log_lines.is_full() { let _ = log_lines.pop_front(); }
                                           let mut s: heapless::String<64> = heapless::String::new();
                                           let _ = s.push_str("Err"); // Fallback
                                           let _ = log_lines.push_back(line.as_str().try_into().unwrap_or(s));
                                           
                                           std::debug::log(&format!("{}\n", line));
                                           last_key_seq = seq;
                                           changed = true;
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
                  if let Ok(typed) = postcard::from_bytes::<TypedBytes>(&thing.body.bytes) {
                        if let Ok(stream) = postcard::from_bytes::<TextEventStreamBody>(&typed.bytes) {
                               let head = stream.head_seq;
                               if head > last_text_seq {
                                   let len = stream.events.len() as u64;
                                   let start = head.saturating_sub(len);
                                   for (i, evt) in stream.events.iter().enumerate() {
                                       let seq = start + (i as u64) + 1;
                                       if seq > last_text_seq {
                                           let line = format!("TXT: '{}'", evt.text);
                                           
                                           if log_lines.is_full() { let _ = log_lines.pop_front(); }
                                           let mut s: heapless::String<64> = heapless::String::new();
                                           let _ = s.push_str("Err");
                                           let _ = log_lines.push_back(line.as_str().try_into().unwrap_or(s));

                                           std::debug::log(&format!("{}\n", line));
                                           last_text_seq = seq;
                                           changed = true;
                                       }
                                   }
                               }
                          }
                  }
             }
        }
        
        if changed {
             if let Some(wid) = window_id {
                 let mut content: heapless::String<1024> = heapless::String::new();
                 for line in log_lines.iter() {
                     use core::fmt::Write;
                     if !content.is_empty() { let _ = content.write_char('\n'); }
                     let _ = content.write_str(line.as_str());
                 }

                 let mut title: heapless::String<64> = heapless::String::new();
                 let _ = title.push_str("Input Log");

                 let update_body = models::schema::window::WindowBody {
                     title,
                     x: 800,
                     y: 50,
                     width: 300,
                     height: 400,
                     content,
                     seq: last_key_seq.max(last_text_seq),
                 };
                 
                 let update_op = GraphOp::UpdateThing {
                     id: wid,
                     value: abi::wire::typed::TypedBytes {
                         type_id: abi::wire::typed::TypeId(models::builtins::ids::THING_WINDOW_SCHEMA.0.into()),
                         codec_id: abi::wire::typed::CodecId::POSTCARD,
                         bytes: postcard::to_allocvec(&update_body).unwrap(),
                     }
                 };
                 let _ = g.call_op(&update_op, &mut buf);
             }
        }
        
        let _ = thing_std::time::sleep_ms(&g, 50);
    }
}
