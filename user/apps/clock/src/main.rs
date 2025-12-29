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
    
    let mut buf = [0u8; 4096]; // Increased buffer
    let mut sys_time_id: Option<ThingId> = None;
    let mut window_id: Option<ThingId> = None;

    // Create Window
    {
        c.write_str("CLOCK: Creating Window...\n");
        let mut content: heapless::String<1024> = heapless::String::new();
        let _ = content.push_str("Loading...");
        
        let mut title: heapless::String<64> = heapless::String::new();
        let _ = title.push_str("Clock");

        let window_body = models::schema::window::WindowBody {
            title,
            x: 50,
            y: 50,
            width: 200,
            height: 100,
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
             c.write_str("CLOCK: Window Created!\n");
             
             let link_op = GraphOp::AddLink {
                 from: models::builtins::ids::THING_BOOT_ROOT,
                 to: id,
                 kind: models::builtins::ids::THING_OWNS_KIND
             };
             let _ = g.call_op(&link_op, &mut buf);
        } else {
             c.write_str("CLOCK: Failed to create window\n");
        }
    }

    loop {
        // 1. Locate TimeNow if unknown
        if sys_time_id.is_none() {
             let op = GraphOp::ScanLinks { 
                 from: Some(models::builtins::ids::THING_BOOT_ROOT), 
                 to: None, 
                 kind: Some(models::builtins::ids::THING_HAS_TIME_NOW_KIND) 
             };
             
             if let Ok(GraphReply::Links(list)) = g.call_op(&op, &mut buf) {
                 if let Some((_, target, _)) = list.first() {
                     sys_time_id = Some(*target);
                 }
             }
        }
        
        if let Some(id) = sys_time_id {
             let op = GraphOp::GetThing { id: abi::ids::ThingId(id.0) };
             if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                        if let Ok(props) = postcard::from_bytes::<TimeNow>(&tb.bytes) {
                             let (h, m, s) = format_hms(props.system_ns / 1_000_000_000);
                             
                             // Update Window
                             if let Some(wid) = window_id {
                                 use core::fmt::Write;
                                 let mut content: heapless::String<1024> = heapless::String::new();
                                 write!(content, "{:02}:{:02}:{:02}", h, m, s).unwrap();
                                 
                                 let mut title: heapless::String<64> = heapless::String::new();
                                 let _ = title.push_str("Clock");

                                 let new_body = models::schema::window::WindowBody {
                                     title,
                                     x: 350, // Center-ish
                                     y: 200,
                                     width: 300,
                                     height: 150,
                                     content,
                                     seq: props.system_ns, // Use time as seq
                                 };
                                 
                                 let update_op = GraphOp::UpdateThing {
                                     id: wid,
                                     value: abi::wire::typed::TypedBytes {
                                         type_id: abi::wire::typed::TypeId(models::builtins::ids::THING_WINDOW_SCHEMA.0.into()),
                                         codec_id: abi::wire::typed::CodecId::POSTCARD,
                                         bytes: postcard::to_allocvec(&new_body).unwrap(),
                                     } 
                                 };
                                 
                                 let _ = g.call_op(&update_op, &mut buf);
                             }
                        }
             }
        }
        
        // Poll faster than 1s to match time better? Or just 1s.
        let _ = thing_std::time::sleep_ms(&g, 1000);
    }
}

fn format_hms(total: u64) -> (u64, u64, u64) {
    let s = total % 60;
    let m = (total / 60) % 60;
    let h = (total / 3600) % 24;
    (h, m, s)
}
