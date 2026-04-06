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
use stem::info;
use stem::syscall::{port_create, port_recv, PortHandle};

fn find_bristle_node() -> Option<stem::thing::ThingId> {
    use abi::schema::hid::SVC_INPUT;
    use stem::thing::sys::find;
    use stem::thing::ThingId;

    let mut input_nodes = [ThingId::default(); 16];
    match find(SVC_INPUT, &mut input_nodes) {
        Ok(count) if count > 0 => {
            let count = count.min(input_nodes.len());
            let mut best = input_nodes[0];
            for node in input_nodes.iter().take(count).skip(1) {
                if node.to_u64_lossy() > best.to_u64_lossy() {
                    best = *node;
                }
            }
            Some(best)
        }
        _ => None,
    }
}

fn subscribe_bristle_topic() -> Option<PortHandle> {
    use abi::schema::input::INPUT_TOPIC_ID;
    use stem::syscall::topic_subscribe;
    use stem::thing::sys::prop_get;

    let input_node = find_bristle_node()?;
    if let Ok(topic_id) = prop_get(input_node, INPUT_TOPIC_ID) {
        if let Ok((write, read)) = port_create(4096) {
            if topic_subscribe(topic_id as u32, write).is_ok() {
                info!(
                    "echo: dynamically subscribed to input topic {} on svc.Input {} via port {}",
                    topic_id,
                    input_node.to_u64_lossy(),
                    read
                );
                return Some(read);
            }
        }
    }

    None
}

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

fn key_to_ascii(key: Key, mods: Mods) -> Option<char> {
    let shift = mods.has_shift();
    Some(match key {
        Key::A => if shift { 'A' } else { 'a' },
        Key::B => if shift { 'B' } else { 'b' },
        Key::C => if shift { 'C' } else { 'c' },
        Key::D => if shift { 'D' } else { 'd' },
        Key::E => if shift { 'E' } else { 'e' },
        Key::F => if shift { 'F' } else { 'f' },
        Key::G => if shift { 'G' } else { 'g' },
        Key::H => if shift { 'H' } else { 'h' },
        Key::I => if shift { 'I' } else { 'i' },
        Key::J => if shift { 'J' } else { 'j' },
        Key::K => if shift { 'K' } else { 'k' },
        Key::L => if shift { 'L' } else { 'l' },
        Key::M => if shift { 'M' } else { 'm' },
        Key::N => if shift { 'N' } else { 'n' },
        Key::O => if shift { 'O' } else { 'o' },
        Key::P => if shift { 'P' } else { 'p' },
        Key::Q => if shift { 'Q' } else { 'q' },
        Key::R => if shift { 'R' } else { 'r' },
        Key::S => if shift { 'S' } else { 's' },
        Key::T => if shift { 'T' } else { 't' },
        Key::U => if shift { 'U' } else { 'u' },
        Key::V => if shift { 'V' } else { 'v' },
        Key::W => if shift { 'W' } else { 'w' },
        Key::X => if shift { 'X' } else { 'x' },
        Key::Y => if shift { 'Y' } else { 'y' },
        Key::Z => if shift { 'Z' } else { 'z' },
        Key::Num1 => if shift { '!' } else { '1' },
        Key::Num2 => if shift { '@' } else { '2' },
        Key::Num3 => if shift { '#' } else { '3' },
        Key::Num4 => if shift { '$' } else { '4' },
        Key::Num5 => if shift { '%' } else { '5' },
        Key::Num6 => if shift { '^' } else { '6' },
        Key::Num7 => if shift { '&' } else { '7' },
        Key::Num8 => if shift { '*' } else { '8' },
        Key::Num9 => if shift { '(' } else { '9' },
        Key::Num0 => if shift { ')' } else { '0' },
        Key::Space => ' ',
        Key::Minus => if shift { '_' } else { '-' },
        Key::Equal => if shift { '+' } else { '=' },
        Key::LeftBracket => if shift { '{' } else { '[' },
        Key::RightBracket => if shift { '}' } else { ']' },
        Key::Backslash => if shift { '|' } else { '\\' },
        Key::Semicolon => if shift { ':' } else { ';' },
        Key::Quote => if shift { '"' } else { '\'' },
        Key::Grave => if shift { '~' } else { '`' },
        Key::Comma => if shift { '<' } else { ',' },
        Key::Period => if shift { '>' } else { '.' },
        Key::Slash => if shift { '?' } else { '/' },
        _ => return None,
    })
}

fn parse_and_print_event(buf: &[u8]) {
    let mut offset = 0usize;
    while offset + 20 <= buf.len() {
        let header: BristleEventHeader = unsafe {
            core::ptr::read_unaligned(buf.as_ptr().add(offset) as *const BristleEventHeader)
        };

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

        let payload_len = header.payload_len as usize;
        let total = 20 + payload_len;
        if offset + total > buf.len() {
            return;
        }

        let payload_ptr = unsafe { buf.as_ptr().add(offset + 20) };
        match header.event_type {
            1 => {
                if payload_len >= KeyEventPayload::SIZE {
                    let payload: KeyEventPayload =
                        unsafe { core::ptr::read_unaligned(payload_ptr as *const KeyEventPayload) };
                    let key = Key::from_raw(payload.key);
                    let mods = Mods(payload.mods);
                    let repeat = if payload.flags & 1 != 0 {
                        " (repeat)"
                    } else {
                        ""
                    };
                    info!("KeyDown {}{}{}", key.name(), format_mods(mods), repeat);
                    if let Some(ch) = key_to_ascii(key, mods) {
                        info!("TextInput {:?}", ch);
                    } else if key == Key::Enter {
                        info!("TextInput <Enter>");
                    } else if key == Key::Backspace {
                        info!("TextInput <Backspace>");
                    } else if key == Key::Tab {
                        info!("TextInput <Tab>");
                    }
                }
            }
            2 => {
                if payload_len >= KeyEventPayload::SIZE {
                    let payload: KeyEventPayload =
                        unsafe { core::ptr::read_unaligned(payload_ptr as *const KeyEventPayload) };
                    let key = Key::from_raw(payload.key);
                    let mods = Mods(payload.mods);
                    info!("KeyUp {}{}", key.name(), format_mods(mods));
                }
            }
            3 => {
                if payload_len >= PointerMovePayload::SIZE {
                    let payload: PointerMovePayload = unsafe {
                        core::ptr::read_unaligned(payload_ptr as *const PointerMovePayload)
                    };
                    let dx = payload.dx;
                    let dy = payload.dy;
                    info!("PointerMove dx={} dy={}", dx, dy);
                }
            }
            4 => {
                if payload_len >= PointerButtonPayload::SIZE {
                    let payload: PointerButtonPayload = unsafe {
                        core::ptr::read_unaligned(payload_ptr as *const PointerButtonPayload)
                    };
                    let btn = payload.button;
                    info!("PointerButtonDown {}", button_name(btn));
                }
            }
            5 => {
                if payload_len >= PointerButtonPayload::SIZE {
                    let payload: PointerButtonPayload = unsafe {
                        core::ptr::read_unaligned(payload_ptr as *const PointerButtonPayload)
                    };
                    let btn = payload.button;
                    info!("PointerButtonUp {}", button_name(btn));
                }
            }
            _ => {}
        }

        offset += total;
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    info!("echo: starting up");

    let mut handle = arg as PortHandle;
    if handle == 0 {
        if let Some(topic_handle) = subscribe_bristle_topic() {
            handle = topic_handle;
        }
    }

    info!(
        "echo: ready for Bristle events (main loop using handle {})",
        handle
    );

    let mut buf = [0u8; 256];

    loop {
        if handle == 0 {
            if let Some(new_handle) = subscribe_bristle_topic() {
                handle = new_handle;
                info!("echo: attached to Bristle topic on handle {}", handle);
            } else {
                stem::yield_now();
                continue;
            }
        }

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
