//! Echo: Bristle Event Display
//!
//! Reads BristleEvents and prints them to the console.
//! Demonstrates normalized input - keys and pointer events.

#![no_std]
#![no_main]

use abi::hid::{
    BristleEventHeader, Key, KeyEventPayload, Mods, PointerButtonPayload, PointerMovePayload,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use stem::info;
use stem::syscall::{port_recv, PortHandle};

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
        _ => " +Mods", // Changed from +? to +Mods for clarity
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
        return;
    }
    if header.version != BRISTLE_EVENT_VERSION {
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
            // PointerMove
            if buf.len() >= 24 {
                let payload: PointerMovePayload = unsafe {
                    core::ptr::read_unaligned(buf.as_ptr().add(20) as *const PointerMovePayload)
                };
                // Copy to locals to avoid packed struct field reference
                let dx = payload.dx;
                let dy = payload.dy;
                // info!("PointerMove dx={} dy={}", dx, dy);
            }
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
fn main(evt_read_handle: usize) -> ! {
    let handle = evt_read_handle as PortHandle;

    info!("echo: online (handle={})", handle);
    info!("echo: ready for Bristle events (keyboard + mouse)");

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
