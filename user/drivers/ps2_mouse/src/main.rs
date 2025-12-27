#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std as std;
use thing_std::{StdoutConsole, Console};
use abi::{ThingId, SymbolId};
use abi::wire::driver::{DriverEvent, DriverPublish};
use models::core::input::{MouseBody, PointerEventStreamBody, PointerEventCompact};
use models::builtins::ids::{
    THING_MOUSE_KIND, THING_POINTER_EVENT_STREAM_KIND, 
    THING_EMITS_KIND, THING_LINK_KIND, THING_BOOT_ROOT, THING_HAS_DEVICE_KIND
};
use models::builtins::symbols::{SYM_MOUSE, SYM_PS2};
use models::Thing;

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe {
        // Init Heap
        std::rt::init_heap(heap_start as usize, 256 * 1024);
    }
    
    std::init();
    use std::{StdoutConsole, Console};
    use core::fmt::Write;
    use thing_std::debug::PortWrites;
    
    let _ = PortWrites.write_str("PS/2 Mouse: PortWrites Alive\n");
    let _ = PortWrites.write_fmt(format_args!("Heap Init at: {:x}\n", heap_start));
    
    // HEAP TEST
    {
        let mut v = alloc::vec::Vec::new();
        v.push(1u8);
    }

    // Delay removed

    // StdoutConsole removed to isolate crash
    // let c = StdoutConsole;

    // 1. Create Mouse Device Thing (ID 3010)
    let mouse = Thing {
        id: ThingId(3010),
        kind: THING_MOUSE_KIND,
        body: models::ThingBody::from(&MouseBody { bus: SYM_PS2 }).expect("mouse body"),
    };

    publish_thing(&mouse);
    let _ = PortWrites.write_str("PS/2 Mouse: Mouse Device Published\n");

    // 2. Link Root -> HAS_DEVICE -> Mouse (ID 3013)
    let root_link_id = ThingId(3013);
    let root_link_body = models::link::LinkBody {
        from: THING_BOOT_ROOT,
        to: mouse.id,
        predicate: THING_HAS_DEVICE_KIND,
    };
    publish_link(root_link_id, root_link_body);
    let _ = PortWrites.write_str("PS/2 Mouse: Link Root->Mouse Published\n");

    
    // 3. Create PointerEventStream Thing (ID 3011)
    let stream_id = ThingId(3011);
    let mut stream_body = PointerEventStreamBody {
        head_seq: 0,
        capacity: 128,
        dropped: 0,
        events: Vec::with_capacity(128),
    };

    let stream_thing = Thing {
        id: stream_id,
        kind: THING_POINTER_EVENT_STREAM_KIND,
        body: models::ThingBody::from(&stream_body).expect("stream body"),
    };
    publish_thing(&stream_thing);
    let _ = PortWrites.write_str("PS/2 Mouse: Stream Published\n");

    // 4. Link Mouse -> EMITS -> Stream (ID 3012)
    let link_id = ThingId(3012);
    let link_body = models::link::LinkBody {
        from: mouse.id,
        to: stream_id,
        predicate: THING_EMITS_KIND,
    };
    publish_link(link_id, link_body);
    let _ = PortWrites.write_str("PS/2 Mouse: Link Mouse->Stream Published\n");
    let _ = PortWrites.write_str("!!! MOUSE FULLY ALIVE !!!\n");

    // Work Loop
    let mut packet = [0u8; 3];
    let mut packet_idx = 0;
    
    let mut out_buf = [0u8; 128];
    loop {
        match std::syscalls::driver_wait(&mut out_buf) {
            Ok(len) => {
                let slice = &out_buf[..len];
                if let Ok(event) = postcard::from_bytes::<DriverEvent>(slice) {
                    match event {
                        DriverEvent::Ps2MouseByte { byte } => {
                            // Packet Sync logic
                            if packet_idx == 0 && (byte & 0x08) == 0 {
                                // Byte 0 must have bit 3 set always
                                // If not, we are out of sync or it's garbage. 
                                // Ignore and wait for next byte (hoping it aligns)
                                // c.write_str("Mouse: Sync error\n");
                                continue;
                            }
                            
                            packet[packet_idx] = byte;
                            packet_idx += 1;
                            
                            if packet_idx == 3 {
                                process_packet(packet, &mut stream_body);
                                packet_idx = 0;
                                
                                // Publish Update (batched? No, every packet for now)
                                // Mouse can be high frequency (100Hz). 
                                // Ideally we batch or update shm. For Graph, we update Thing.
                                let thing = Thing {
                                    id: stream_id,
                                    kind: THING_POINTER_EVENT_STREAM_KIND,
                                    body: models::ThingBody::from(&stream_body).unwrap(),
                                };
                                let pub_bytes = postcard::to_allocvec(&thing).unwrap();
                                let payload = DriverPublish::Observation { thing_bytes: pub_bytes };
                                let payload_bytes = postcard::to_allocvec(&payload).unwrap();
                let res = std::syscalls::driver_publish(&payload_bytes);
                                if let Err(_e) = res {
                                     let _ = PortWrites.write_str("Stream Update Failed!\n");
                                }
                            }
                        },
                        _ => {}
                    }
                }
            },
            Err(_) => {
                for _ in 0..1000 { core::hint::spin_loop(); }
            }
        }
    }
}

fn process_packet(packet: [u8; 3], body: &mut PointerEventStreamBody) {
    let b0 = packet[0];
    let b1 = packet[1];
    let b2 = packet[2];
    
    // X
    let mut x: i16 = b1 as i16;
    if (b0 & 0x10) != 0 { x |= 0xFF00u16 as i16; } // Sign extend
    
    // Y
    let mut y: i16 = b2 as i16;
    if (b0 & 0x20) != 0 { y |= 0xFF00u16 as i16; } // Sign extend
    
    // Invert Y? Usually mouse sends +Y UP. Screen is +Y DOWN.
    // Let's invert it so userspace gets "Screen Delta".
    y = -y; 
    
    // Buttons (Left=1, Right=2, Middle=4)
    // Packet: Left=Bit0, Right=Bit1, Middle=Bit2
    let buttons = b0 & 0x07;
    
    let event = PointerEventCompact {
        dx: x,
        dy: y,
        scroll: 0,
        buttons,
    };
    
    body.head_seq += 1;
    body.events.push(event);
    
    // Sliding Window
    if body.events.len() > body.capacity as usize {
        body.events.remove(0);
        body.dropped += 1;
    }
}

fn publish_thing(thing: &Thing) {
    use core::fmt::Write;
    use thing_std::debug::PortWrites;

    let pub_bytes = postcard::to_allocvec(thing).expect("serialize thing");
    let payload = DriverPublish::Observation { thing_bytes: pub_bytes };
    let payload_bytes = postcard::to_allocvec(&payload).expect("serialize payload");
    // c.write_str("Calling driver_publish...\n");
    let res = std::syscalls::driver_publish(&payload_bytes);
    if let Err(e) = res {
        let _ = PortWrites.write_str("Publish Thing Failed\n");
    }
}

fn publish_link(id: ThingId, body: models::link::LinkBody) {
    let thing = Thing {
        id, 
        kind: THING_LINK_KIND,
        body: models::ThingBody::from(&body).expect("link body"),
    };
    publish_thing(&thing);
}
