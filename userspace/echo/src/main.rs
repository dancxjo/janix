//! Echo: Bristle Event Display
//!
//! Self-registers as an input subscriber via the System Graph.
//! Reads BristleEvents and prints them to the console.
//! Demonstrates normalized input - keys and pointer events.

#![feature(restricted_std)]
#![no_main]

use abi::hid::{
    BristleEventHeader, Key, KeyEventPayload, Mods, PointerButtonPayload, PointerMovePayload,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use abi::schema::input::{SUBSCRIBER_FILTER, SUBSCRIBER_PORT, SVC_INPUT_SUBSCRIBER};
use stem::info;
use stem::syscall::{port_create, port_recv, PortHandle};
use stem::thing::sys::{create_node, prop_set};

fn format_mods(mods: Mods) -> &'static str {
    match mods.0 {
        0 => "",
        Mods::SHIFT => " +Shift",
        Mods::CTRL => " +Ctrl",
        Mods::ALT => " +Alt",
        Mods::META => " +Meta",
        m if m == (Mods::CTRL | Mods::ALT) => " +Ctrl+Alt",
        m if m == (Mods::SHIFT | Mods::CTRL) => " +Shift+Ctrl",
        m if m == (Mods::SHIFT | Mods::ALT) => " +Shift+Alt",
        m if m == (Mods::CTRL | Mods::ALT | Mods::SHIFT) => " +Ctrl+Alt+Shift",
        _ => " +Mods",
    }
}

fn button_name(button: u8) -> &'static str {
    match button {
        0 => "Left",
        1 => "Right",
        2 => "Middle",
        _ => "?",
    }
}

fn parse_and_print_event(buf: &[u8]) {
    if buf.len() < 20 {
        return;
    }

    let header: BristleEventHeader =
        unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const BristleEventHeader) };

    if header.magic != BRISTLE_EVENT_MAGIC {
        let magic = header.magic;
        info!("echo: dropped invalid magic: {:x}", magic);
        return;
    }
    if header.version != BRISTLE_EVENT_VERSION {
        let version = header.version;
        info!("echo: dropped invalid version: {:x}", version);
        return;
    }

    match header.event_type {
        1 => {
            // KeyDown
            if buf.len() >= 24 {
                let payload: KeyEventPayload = unsafe {
                    core::ptr::read_unaligned(buf.as_ptr().add(20) as *const KeyEventPayload)
                };
                let key = Key::from_raw(payload.key);
                let mods = Mods(payload.mods);
                let repeat = if payload.flags & 1 != 0 {
                    " (repeat)"
                } else {
                    ""
                };
                info!("KeyDown {}{}{}", key.name(), format_mods(mods), repeat);
            }
        }
        2 => {
            // KeyUp
            if buf.len() >= 24 {
                let payload: KeyEventPayload = unsafe {
                    core::ptr::read_unaligned(buf.as_ptr().add(20) as *const KeyEventPayload)
                };
                let key = Key::from_raw(payload.key);
                let mods = Mods(payload.mods);
                info!("KeyUp {}{}", key.name(), format_mods(mods));
            }
        }
        3 => {
            // // PointerMove
            // if buf.len() >= 24 {
            //     let payload: PointerMovePayload = unsafe {
            //         core::ptr::read_unaligned(buf.as_ptr().add(20) as *const PointerMovePayload)
            //     };
            //     let dx = payload.dx;
            //     let dy = payload.dy;
            //     info!("PointerMove dx={} dy={}", dx, dy);
            // }
        }
        4 => {
            // PointerButtonDown
            if buf.len() >= 22 {
                let payload: PointerButtonPayload = unsafe {
                    core::ptr::read_unaligned(buf.as_ptr().add(20) as *const PointerButtonPayload)
                };
                let btn = payload.button;
                info!("PointerButtonDown {}", button_name(btn));
            }
        }
        5 => {
            // PointerButtonUp
            if buf.len() >= 22 {
                let payload: PointerButtonPayload = unsafe {
                    core::ptr::read_unaligned(buf.as_ptr().add(20) as *const PointerButtonPayload)
                };
                let btn = payload.button;
                info!("PointerButtonUp {}", button_name(btn));
            }
        }
        _ => {}
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    use abi::schema::hid::SVC_INPUT;
    use abi::schema::input::INPUT_TOPIC_ID;
    use stem::syscall::topic_subscribe;
    use stem::thing::sys::{find, prop_get};
    use stem::thing::ThingId;

    info!("echo: starting up");

    // Input handle: try to use the broadcast topic, fallback to legacy handle
    let mut handle = arg as PortHandle;

    let mut input_nodes = [ThingId::default(); 1];
    if let Ok(count) = find(SVC_INPUT, &mut input_nodes) {
        if count > 0 {
            let input_node = input_nodes[0];
            if let Ok(topic_id) = prop_get(input_node, INPUT_TOPIC_ID) {
                if let Ok((write, read)) = port_create(4096) {
                    if topic_subscribe(topic_id as u32, write).is_ok() {
                        info!(
                            "echo: dynamically subscribed to input topic {} via port {}",
                            topic_id, read
                        );
                        handle = read;
                    }
                }
            }
        }
    }

    if handle == 0 {
        info!("echo: no input handle provided or topic found, exiting");
        loop {
            stem::yield_now();
        }
    }

    info!(
        "echo: ready for Bristle events (main loop using handle {})",
        handle
    );

    let mut buf = [0u8; 256];

    loop {
        match port_recv(handle, &mut buf) {
            Ok(n) if n >= 20 => {
                parse_and_print_event(&buf[..n]);
            }
            _ => {
                stem::yield_now();
            }
        }
    }
}
