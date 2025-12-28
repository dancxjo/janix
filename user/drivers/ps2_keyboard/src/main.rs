#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std as std;
use abi::{ThingId};
use abi::wire::driver::{DriverEvent, DriverPublish};
use abi::wire::input::{RawKeyEvent, RawKeyKind, KeyState};
use models::core::input::{KeyboardBody, RawKeyEventStreamBody};
use models::builtins::ids::{
    THING_KEYBOARD_KIND, 
    THING_RAW_KEY_EVENT_STREAM_KIND, 
    THING_EMITS_KIND, 
    THING_LINK_KIND, 
    THING_BOOT_ROOT, 
    THING_HAS_KEYBOARD_KIND
};
use models::builtins::symbols::{SYM_PS2};
use models::Thing;

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();
    std::debug::log("PS/2 Driver starting...\n");

    // Create Keyboard Device Thing
    let mut keyboard = Thing {
        id: ThingId(3000),
        kind: THING_KEYBOARD_KIND,
        body: models::ThingBody::from(&KeyboardBody { bus: SYM_PS2 }).expect("body"),
    };

    if let Some(id) = publish_thing(&keyboard) {
        keyboard.id = id;
    }
    std::debug::log("Published Keyboard Device\n");

    // Link Root -> HAS_KEYBOARD -> Keyboard
    let root_link_id = ThingId(3003);
    let root_link_body = models::link::LinkBody {
        from: THING_BOOT_ROOT,
        to: keyboard.id,
        predicate: THING_HAS_KEYBOARD_KIND,
    };
    publish_link(root_link_id, root_link_body);
    std::debug::log("Linked Root -> Keyboard\n");

    // Create/Publish RawKeyEventStream
    let mut stream_id = ThingId(3001);
    let mut stream_body = RawKeyEventStreamBody {
        head_seq: 0,
        capacity: 32,
        dropped: 0,
        events: Vec::new(),
    };

    let stream_thing = Thing {
        id: stream_id,
        kind: THING_RAW_KEY_EVENT_STREAM_KIND,
        body: models::ThingBody::from(&stream_body).expect("stream body"),
    };
    
    if let Some(id) = publish_thing(&stream_thing) {
        stream_id = id;
    }
    std::debug::log("Published RawKeyEventStream\n");

    // Link: Keyboard -> EMITS -> Stream
    let link_id = ThingId(3002);
    let link_body = models::link::LinkBody {
        from: keyboard.id,
        to: stream_id,
        predicate: THING_EMITS_KIND,
    };
    publish_link(link_id, link_body);
    std::debug::log("Linked Keyboard -> Raw Stream\n");

    let mut out_buf = [0u8; 128];
    let mut e0_pending = false;

    loop {
        match std::syscalls::driver_wait(&mut out_buf) {
            Ok(len) => {
                let slice = &out_buf[..len];
                if let Ok(event) = postcard::from_bytes::<DriverEvent>(slice) {
                    match event {
                        DriverEvent::Ps2Scancode { scancode } => {
                            if scancode == 0xE0 {
                                e0_pending = true;
                                continue;
                            }
                            
                            let is_release = (scancode & 0x80) != 0;
                            let code = (scancode & 0x7F) as u32;
                            let flags = if e0_pending { 1 } else { 0 };
                            e0_pending = false;

                            let raw_event = RawKeyEvent {
                                source: keyboard.id,
                                time_ns: 0,
                                kind: RawKeyKind::ScancodeSet1,
                                code,
                                state: if is_release { KeyState::Up } else { KeyState::Down },
                                flags,
                            };

                            // Update Stream Body
                            stream_body.head_seq += 1;
                            stream_body.events.push(raw_event);
                            
                            // Sliding window
                            if stream_body.events.len() > stream_body.capacity as usize {
                                stream_body.events.remove(0);
                                stream_body.dropped += 1;
                            }
                            
                            // Publish Update
                            let thing = Thing {
                                id: stream_id,
                                kind: THING_RAW_KEY_EVENT_STREAM_KIND,
                                body: models::ThingBody::from(&stream_body).unwrap(),
                            };
                            publish_thing(&thing);
                        },
                        _ => {}
                    }
                }
            },
            Err(_) => {
                // Spin/Yield to avoid burning CPU
                for _ in 0..1000 { core::hint::spin_loop(); }
            }
        }
    }
}

fn publish_thing(thing: &Thing) -> Option<ThingId> {
    let pub_bytes = postcard::to_allocvec(thing).expect("serialize thing");
    let payload = DriverPublish::Observation { thing_bytes: pub_bytes };
    let payload_bytes = postcard::to_allocvec(&payload).expect("serialize payload");
    let res = std::syscalls::driver_publish(&payload_bytes);
    match res {
        Ok(id_val) => Some(ThingId(id_val as u64)),
        Err(_) => None,
    }
}

fn publish_link(id: ThingId, body: models::link::LinkBody) {
    use models::builtins::ids::THING_LINK_KIND;

    let link_bytes = postcard::to_allocvec(&body).expect("serialize link body");
    let typed = abi::wire::typed::TypedBytes {
        type_id: abi::wire::typed::TypeId(2003), // THING_LINK_SCHEMA
        codec_id: abi::wire::typed::CodecId::POSTCARD,
        bytes: link_bytes,
    };

    let thing = Thing {
        id, 
        kind: THING_LINK_KIND,
        body: models::ThingBody::from(&typed).expect("link thing body"),
    };
    publish_thing(&thing);
}
