#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use alloc::format;

use thing_std as std;
use abi::ThingId;
use models::core::input::{KeyEventBody};
use models::Thing;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    std::init();
    std::debug::log("Keylog App starting...\n");

    let mut last_seen = ThingId(0); // Start from beginning or specific point
    let mut out_buf = [0u8; 1024];

    loop {
        // Query next event
        let params = postcard::to_allocvec(&last_seen).unwrap();
        // Since we stubbed `graph_query` in thing_std to fail, we need to FIX thing_std/src/syscalls.rs FIRST!
        // But let's assume I fix it.
        // Or I use raw syscall here if I can?
        // No, I should fix `thing_std`.
        
        // Wait, I left `graph_query` as Err in step 138.
        // I Must fix it.
        // But let's write the code assuming it works.
        match std::syscalls::graph_query("input.key_events.next", &params, &mut out_buf) {
            Ok(len) => {
                let slice = &out_buf[..len];
                if let Ok(thing) = postcard::from_bytes::<Thing>(slice) {
                    last_seen = thing.id;
                    
                    // Decode body
                    if let Ok(event) = postcard::from_bytes::<KeyEventBody>(&thing.body.bytes) {

                        let msg = format!("Key Event: Scancode={:#x} Release={}\n", event.scancode, event.is_release);
                        std::debug::log(&msg);
                    } else {
                        std::debug::log("Failed to decode KeyEventBody\n");
                    }
                } else {
                    std::debug::log("Failed to decode Thing\n");
                }
            },
            Err(_) => {
                // Retry or spin
                 core::hint::spin_loop();
            }
        }
    }
}
