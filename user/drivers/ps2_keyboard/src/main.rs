#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec; // explicitly import Vec

use thing_std as std;
use abi::{ThingId, SymbolId};
use abi::wire::driver::{DriverEvent, DriverPublish};
use models::core::input::{KeyboardBody, KeyEventStreamBody, KeyEventCompact};
use models::builtins::ids::{THING_KEYBOARD_KIND, THING_KEY_EVENT_STREAM_KIND, THING_EMITS_KIND, THING_LINK_KIND};
use models::builtins::symbols::{SYM_KEYBOARD, SYM_PS2, SYM_KEY_EVENT};
use models::Thing;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    std::init();
    std::debug::log("PS/2 Driver starting...\n");

    // Create Keyboard Device Thing
    let keyboard_id = ThingId(3000);
    
    let keyboard = Thing {
        id: keyboard_id,
        kind: THING_KEYBOARD_KIND,
        body: models::ThingBody::from(&KeyboardBody { bus: SYM_PS2 }).expect("body"),
    };

    // Publish Keyboard Device
    let pub_bytes = postcard::to_allocvec(&keyboard).expect("serialize thing");
    let payload = DriverPublish::Observation { thing_bytes: pub_bytes };
    let payload_bytes = postcard::to_allocvec(&payload).expect("serialize payload");
    let _ = std::syscalls::driver_publish(&payload_bytes);

    std::debug::log("Published Keyboard Device\n");

    // Create/Publish KeyEventStream
    let stream_id = ThingId(3001);
    let mut stream_body = KeyEventStreamBody {
        head_seq: 0,
        capacity: 16,
        dropped: 0,
        events: Vec::new(),
    };

    let stream_thing = Thing {
        id: stream_id,
        kind: THING_KEY_EVENT_STREAM_KIND,
        body: models::ThingBody::from(&stream_body).expect("stream body"),
    };
    
    let pub_bytes = postcard::to_allocvec(&stream_thing).expect("serialize stream");
    let payload = DriverPublish::Observation { thing_bytes: pub_bytes };
    let payload_bytes = postcard::to_allocvec(&payload).expect("serialize payload");
    let _ = std::syscalls::driver_publish(&payload_bytes);
    
    std::debug::log("Published KeyEventStream\n");

    // Link: Keyboard -> EMITS -> Stream
    let link_id = ThingId(3002);
    let link_body = models::link::LinkBody {
        from: keyboard_id,
        to: stream_id,
        predicate: THING_EMITS_KIND,
    };
    
    let link_thing = Thing {
        id: link_id,
        kind: THING_LINK_KIND,
        body: models::ThingBody::from(&link_body).expect("link body"),
    };
    
    let pub_bytes = postcard::to_allocvec(&link_thing).expect("serialize link");
    let payload = DriverPublish::Observation { thing_bytes: pub_bytes };
    let payload_bytes = postcard::to_allocvec(&payload).expect("serialize payload");
    let _ = std::syscalls::driver_publish(&payload_bytes);
    
    std::debug::log("Linked Keyboard -> Stream\n");

    let mut out_buf = [0u8; 128];
    loop {
        match std::syscalls::driver_wait(&mut out_buf) {
            Ok(len) => {
                let slice = &out_buf[..len];
                if let Ok(event) = postcard::from_bytes::<DriverEvent>(slice) {
                    match event {
                        DriverEvent::Ps2Scancode { scancode } => {
                            let is_release = (scancode & 0x80) != 0;
                            
                            // Update Stream Body
                            stream_body.head_seq += 1;
                            let compact = KeyEventCompact { scancode, is_release };
                            stream_body.events.push(compact);
                            
                            // Sliding window
                            if stream_body.events.len() > stream_body.capacity as usize {
                                stream_body.events.remove(0);
                                stream_body.dropped += 1;
                            }
                            
                            // Publish Update
                            let thing = Thing {
                                id: stream_id,
                                kind: THING_KEY_EVENT_STREAM_KIND,
                                body: models::ThingBody::from(&stream_body).unwrap(),
                            };
                            
                            let pub_bytes = postcard::to_allocvec(&thing).unwrap();
                            let payload = DriverPublish::Observation { thing_bytes: pub_bytes };
                            let payload_bytes = postcard::to_allocvec(&payload).unwrap();
                            let _ = std::syscalls::driver_publish(&payload_bytes);
                        },
                        _ => {}
                    }
                }
            },
            Err(_) => {
                // Should potentially spin or yield, but syscall blocks/yields effectively now (or spins in user)
            }
        }
    }
}
