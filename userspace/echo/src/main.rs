//! Echo: Bristle Event Display
//!
//! Reads BristleEvents and prints them to the console.
//! Demonstrates normalized key input - no scancodes!

#![no_std]
#![no_main]

use abi::hid::{
    BristleEventHeader, EventType, Key, KeyEventPayload, Mods,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use stem::info;
use stem::syscall::{port_recv, PortHandle};

/// Format modifier flags for display
fn format_mods(mods: Mods) -> &'static str {
    match mods.0 {
        0 => "",
        m if m == Mods::SHIFT => " +Shift",
        m if m == Mods::CTRL => " +Ctrl",
        m if m == Mods::ALT => " +Alt",
        m if m == Mods::META => " +Meta",
        m if m == (Mods::SHIFT | Mods::CTRL) => " +Shift+Ctrl",
        m if m == (Mods::SHIFT | Mods::ALT) => " +Shift+Alt",
        m if m == (Mods::CTRL | Mods::ALT) => " +Ctrl+Alt",
        m if m == (Mods::SHIFT | Mods::CTRL | Mods::ALT) => " +Shift+Ctrl+Alt",
        _ => " +?",
    }
}

/// Parse a BristleEvent from buffer
fn parse_event(buf: &[u8]) -> Option<(EventType, Key, Mods, bool)> {
    if buf.len() < 24 {
        return None;
    }

    // Parse header (20 bytes)
    let header: BristleEventHeader = unsafe {
        core::ptr::read_unaligned(buf.as_ptr() as *const BristleEventHeader)
    };

    // Validate magic
    if header.magic != BRISTLE_EVENT_MAGIC {
        return None;
    }

    // Validate version
    if header.version != BRISTLE_EVENT_VERSION {
        return None;
    }

    // Parse event type
    let event_type = match header.event_type {
        1 => EventType::KeyDown,
        2 => EventType::KeyUp,
        _ => return None,
    };

    // Parse payload (4 bytes at offset 20)
    let payload: KeyEventPayload = unsafe {
        core::ptr::read_unaligned(buf.as_ptr().add(20) as *const KeyEventPayload)
    };

    let key = Key::from_raw(payload.key);
    let mods = Mods(payload.mods);
    let repeat = payload.flags & 1 != 0;

    Some((event_type, key, mods, repeat))
}

#[stem::main]
fn main(evt_read_handle: usize) -> ! {
    let handle = evt_read_handle as PortHandle;
    
    info!("echo: online (handle={})", handle);
    info!("echo: ready to receive Bristle events");

    let mut buf = [0u8; 128];
    let mut event_count: u64 = 0;

    loop {
        match port_recv(handle, &mut buf) {
            Ok(n) if n >= 24 => {
                // Process events (24 bytes each)
                let mut offset = 0;
                while offset + 24 <= n {
                    if let Some((event_type, key, mods, repeat)) = parse_event(&buf[offset..]) {
                        event_count += 1;
                        
                        match event_type {
                            EventType::KeyDown => {
                                let repeat_str = if repeat { " (repeat)" } else { "" };
                                info!(
                                    "[echo] KeyDown {}{}{}",
                                    key.name(),
                                    format_mods(mods),
                                    repeat_str
                                );
                            }
                            EventType::KeyUp => {
                                info!(
                                    "[echo] KeyUp {}{}",
                                    key.name(),
                                    format_mods(mods)
                                );
                            }
                            _ => {}
                        }
                    }
                    offset += 24;
                }
            }
            _ => {
                // No data, yield to avoid busy-spin
                stem::yield_now();
            }
        }
    }
}
