#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec; // explicitly import Vec

use thing_std as std;
use abi::{ThingId, SymbolId};
use abi::wire::driver::{DriverEvent, DriverPublish};
use models::core::input::{KeyboardBody, KeyEventBody};
use models::builtins::ids::{THING_KEYBOARD_KIND, THING_KEY_EVENT_KIND};
use models::builtins::symbols::{SYM_KEYBOARD, SYM_PS2, SYM_KEY_EVENT};
use models::Thing;

#[no_mangle]
fn main() {
    std::init();
    std::debug::log("PS/2 Driver starting...\n");

    // Create Keyboard Device Thing if not exists?
    // V0: Just create one locally to reference its ID.
    // In a real driver, we'd check if it exists or "claim" the bus.
    // We'll create it and insert it (Observation).
    // We need a stable ID or just generate one.
    // Let's use a random-ish ID for now or a fixed one if we can't generate.
    // Actually, `sys_driver_publish` takes a `Thing`. We assign the ID.
    // We can use 3000 as pin suggested.
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
    
    match std::syscalls::driver_publish(&payload_bytes) {
        Ok(_) => std::debug::log("Published Keyboard Device\n"),
        Err(e) => std::debug::log("Failed to publish Keyboard Device\n"), 
    }

    // Loop
    let mut next_event_id = ThingId(4000);

    let mut out_buf = [0u8; 128];
    loop {
        match std::syscalls::driver_wait(&mut out_buf) {
            Ok(len) => {
                let slice = &out_buf[..len];
                if let Ok(event) = postcard::from_bytes::<DriverEvent>(slice) {
                    match event {
                        DriverEvent::Ps2Scancode { scancode } => {
                            // Determine up/down
                            let is_release = (scancode & 0x80) != 0;
                            // Make Key Event
                            next_event_id.0 += 1;
                            
                            let body = KeyEventBody {
                                device: keyboard_id,
                                scancode,
                                is_release,
                            };
                            
                            let thing = Thing {
                                id: next_event_id,
                                kind: THING_KEY_EVENT_KIND,
                                body: models::ThingBody::from(&body).unwrap(),
                            };

                            
                            // Publish
                             let pub_bytes = postcard::to_allocvec(&thing).unwrap();
                             let payload = DriverPublish::Observation { thing_bytes: pub_bytes };
                             let payload_bytes = postcard::to_allocvec(&payload).unwrap();
                             let _ = std::syscalls::driver_publish(&payload_bytes);
                        },
                        _ => {}
                    }
                }
            },
            Err(_) => {}
        }
    }
}
