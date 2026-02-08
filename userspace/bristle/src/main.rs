//! Bristle: Unified HID Broker
//!
//! The sole input authority. Drivers send raw reports, apps receive
//! normalized events. Apps never see scancodes, drivers never see apps.

#![no_std]
#![no_main]

use abi::hid::{
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, BristleEventHeader, EventType, Key,
    KeyEventPayload, Mods, PointerButtonPayload, PointerMovePayload,
};
use bristle::mouse::{MouseState, PointerEvent};
use bristle::thigmonasty::{KeyEdge, KeyboardState};
use stem::info;
use stem::syscall::{PortHandle, port_recv, port_send, port_wait};
#[cfg(feature = "diagnostic-apps")]
use stem::syscall::{port_create, spawn_process};
use stem::thing::sys as thingsys;

/// Register Bristle in the Root graph and return the node ID
fn register_in_graph() -> Option<stem::thing::ThingId> {
    match thingsys::create_node(abi::schema::hid::SVC_INPUT) {
        Ok(node_id) => {
            info!(
                "bristle: registered in graph as svc.Input (id={})",
                node_id.to_u64_lossy()
            );
            Some(node_id)
        }
        Err(e) => {
            info!("bristle: failed to register in graph: {:?}", e);
            None
        }
    }
}

/// Maximum number of dynamic event subscribers
const MAX_SUBSCRIBERS: usize = 16;

/// A subscriber entry with port handle and optional filter
#[derive(Clone, Copy, Default)]
struct Subscriber {
    port: PortHandle,
    filter: u64, // 0=all, 1=keyboard, 2=pointer, 4=button
}

/// Scan the graph for registered input event subscribers.
/// Returns the count of subscribers found (up to MAX_SUBSCRIBERS).
fn scan_subscribers(subscribers: &mut [Subscriber; MAX_SUBSCRIBERS]) -> usize {
    use abi::schema::input::{SUBSCRIBER_FILTER, SUBSCRIBER_PORT, SVC_INPUT_SUBSCRIBER};
    use stem::thing::ThingId;

    let mut node_buf = [ThingId::default(); MAX_SUBSCRIBERS];
    let count = match thingsys::find(SVC_INPUT_SUBSCRIBER, &mut node_buf) {
        Ok(c) => c.min(MAX_SUBSCRIBERS),
        Err(_) => 0,
    };

    let mut valid = 0;
    for i in 0..count {
        let node = node_buf[i];
        // Get the subscriber's port handle
        if let Ok(port) = thingsys::prop_get(node, SUBSCRIBER_PORT) {
            if port > 0 && port <= 0xFFFF_FFFF {
                // Get optional filter (default to 0 = all)
                let filter = thingsys::prop_get(node, SUBSCRIBER_FILTER).unwrap_or(0);
                subscribers[valid] = Subscriber {
                    port: port as PortHandle,
                    filter,
                };
                valid += 1;
            }
        }
    }

    valid
}

/// Check if event matches subscriber filter
#[inline]
fn matches_filter(filter: u64, event_kind: u64) -> bool {
    filter == 0 || (filter & event_kind) != 0
}


/// Serialize a KeyDown event
fn serialize_key_down(
    key: Key,
    mods: Mods,
    repeat: bool,
    timestamp_ns: u64,
    buf: &mut [u8],
) -> usize {
    if buf.len() < 24 {
        return 0;
    }
    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: EventType::KeyDown as u16,
        timestamp_ns,
        payload_len: 4,
    };
    let payload = KeyEventPayload {
        key: key as u16,
        mods: mods.0,
        flags: if repeat { 1 } else { 0 },
    };
    unsafe {
        core::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(
            &payload as *const _ as *const u8,
            buf.as_mut_ptr().add(20),
            4,
        );
    }
    24
}

/// Serialize a KeyUp event
fn serialize_key_up(key: Key, mods: Mods, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 {
        return 0;
    }
    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: EventType::KeyUp as u16,
        timestamp_ns,
        payload_len: 4,
    };
    let payload = KeyEventPayload {
        key: key as u16,
        mods: mods.0,
        flags: 0,
    };
    unsafe {
        core::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(
            &payload as *const _ as *const u8,
            buf.as_mut_ptr().add(20),
            4,
        );
    }
    24
}

/// Serialize a PointerMove event
fn serialize_pointer_move(dx: i16, dy: i16, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 {
        return 0;
    }
    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: EventType::PointerMove as u16,
        timestamp_ns,
        payload_len: 4,
    };
    let payload = PointerMovePayload { dx, dy };
    unsafe {
        core::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(
            &payload as *const _ as *const u8,
            buf.as_mut_ptr().add(20),
            4,
        );
    }
    24
}

/// Serialize a PointerButtonDown event
fn serialize_pointer_button_down(button: u8, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 {
        return 0;
    }
    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: EventType::PointerButtonDown as u16,
        timestamp_ns,
        payload_len: 2,
    };
    let payload = PointerButtonPayload { button, _pad: 0 };
    unsafe {
        core::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(
            &payload as *const _ as *const u8,
            buf.as_mut_ptr().add(20),
            2,
        );
    }
    22
}

/// Serialize a PointerButtonUp event
fn serialize_pointer_button_up(button: u8, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 {
        return 0;
    }
    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: EventType::PointerButtonUp as u16,
        timestamp_ns,
        payload_len: 2,
    };
    let payload = PointerButtonPayload { button, _pad: 0 };
    unsafe {
        core::ptr::copy_nonoverlapping(&header as *const _ as *const u8, buf.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(
            &payload as *const _ as *const u8,
            buf.as_mut_ptr().add(20),
            2,
        );
    }
    22
}

#[stem::main]
fn main(packed_handles: usize) -> ! {
    // Layout: kbd_raw_read[63:48] | mouse_raw_read[47:32] | evt_write[31:16] | evt_echo_write[15:0]
    let packed = packed_handles as u64;
    let kbd_read = ((packed >> 48) & 0xFFFF) as PortHandle;
    let mouse_read = ((packed >> 32) & 0xFFFF) as PortHandle;
    let legacy_evt_write = ((packed >> 16) & 0xFFFF) as PortHandle;
    let legacy_evt_echo_write = (packed & 0xFFFF) as PortHandle;

    info!(
        "bristle: online (kbd={}, mouse={}, evt={}, echo={})",
        kbd_read, mouse_read, legacy_evt_write, legacy_evt_echo_write
    );

    let mut kbd_state = KeyboardState::new();
    let mut mouse_state = MouseState::new();
    let mut _keyboard_gen: u64 = 0;

    let mut kbd_buf = [0u8; 64];
    let mut mouse_buf = [0u8; 64];
    let mut send_buf = [0u8; 64];
    let mut mouse_packet = [0u8; 3];
    let mut mouse_packet_len = 0usize;
    let mut drop_counter: u32 = 0;
    let mut event_count: u64 = 0;

    let wait_handles = [kbd_read, mouse_read];
    loop {
        let ready_handle = match port_wait(&wait_handles, abi::syscall::port_wait::READABLE) {
            Ok(h) => h,
            Err(_) => {
                stem::yield_now();
                continue;
            }
        };

        // Process keyboard input
        if ready_handle == kbd_read {
            if let Ok(n) = port_recv(kbd_read, &mut kbd_buf) {
                if n > 0 {
                for &byte in &kbd_buf[..n] {
                    if let Some(edge) = kbd_state.process_ps2(byte) {
                        let timestamp_ns = stem::monotonic_ns();
                        let len = match edge {
                            KeyEdge::Down { key, mods, repeat } => {
                                serialize_key_down(key, mods, repeat, timestamp_ns, &mut send_buf)
                            }
                            KeyEdge::Up { key, mods } => {
                                serialize_key_up(key, mods, timestamp_ns, &mut send_buf)
                            }
                        };
                        if len > 0 {
                            // Check for F10 (Trigger Graph Dump)
                            if let KeyEdge::Down { key: Key::F10, .. } = edge {
                                info!("bristle: F10 pressed - dumping graph...");
                                let _ = thingsys::dump_graph(0);
                            }

                            let mut sent = false;
                            if port_send(legacy_evt_write, &send_buf[..len]).is_ok() {
                                sent = true;
                            } else {
                                drop_counter += 1;
                            }

                            if legacy_evt_echo_write != 0 {
                                if port_send(legacy_evt_echo_write, &send_buf[..len]).is_ok() {
                                    sent = true;
                                } else {
                                    drop_counter += 1;
                                }
                            }

                            if sent {
                                event_count += 1;
                            }
                        }
                    }
                }
                }
            }
        }

        // Process mouse input
        if ready_handle == mouse_read {
            if let Ok(n) = port_recv(mouse_read, &mut mouse_buf) {
                for &byte in &mouse_buf[..n] {
                    // Keep packet framing across recv calls: the stream can split 3-byte packets.
                    if mouse_packet_len == 0 && (byte & 0x08) == 0 {
                        continue;
                    }

                    mouse_packet[mouse_packet_len] = byte;
                    mouse_packet_len += 1;

                    if mouse_packet_len == 3 {
                        let (events, count) = mouse_state.process_packet(&mouse_packet);
                        for i in 0..count {
                            if let Some(evt) = events[i] {
                                let timestamp_ns = stem::monotonic_ns();
                                let (len, _filter_kind) = match evt {
                                    PointerEvent::Move { dx, dy } => (
                                        serialize_pointer_move(dx, dy, timestamp_ns, &mut send_buf),
                                        abi::schema::input::FILTER_POINTER,
                                    ),
                                    PointerEvent::ButtonDown { button } => (
                                        serialize_pointer_button_down(
                                            button,
                                            timestamp_ns,
                                            &mut send_buf,
                                        ),
                                        abi::schema::input::FILTER_BUTTON,
                                    ),
                                    PointerEvent::ButtonUp { button } => (
                                        serialize_pointer_button_up(button, timestamp_ns, &mut send_buf),
                                        abi::schema::input::FILTER_BUTTON,
                                    ),
                                };

                                if len > 0 {
                                    let mut sent = false;
                                    if port_send(legacy_evt_write, &send_buf[..len]).is_ok() {
                                        sent = true;
                                    } else {
                                        drop_counter += 1;
                                    }

                                    if legacy_evt_echo_write != 0 {
                                        if port_send(legacy_evt_echo_write, &send_buf[..len]).is_ok() {
                                            sent = true;
                                        } else {
                                            drop_counter += 1;
                                        }
                                    }

                                    if sent {
                                        event_count += 1;
                                    }
                                }
                            }
                        }
                        mouse_packet_len = 0;
                    }
                }
            }
        }
        // Rate-limited drop logging
        if drop_counter > 0 && drop_counter % 100 == 0 {
            info!("bristle: dropped {} events (port full)", drop_counter);
        }

    }
}
