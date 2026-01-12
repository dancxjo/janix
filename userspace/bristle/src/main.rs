//! Bristle: Unified HID Broker
//!
//! The sole input authority. Drivers send raw reports, apps receive
//! normalized events. Apps never see scancodes, drivers never see apps.

#![no_std]
#![no_main]

mod normalizer;
mod thigmonasty;

use abi::hid::{
    BristleEventHeader, EventType, Key, KeyEventPayload, Mods,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::{sys as thingsys, ThingId};
use thigmonasty::{KeyEdge, KeyboardState};

/// Register Bristle in the Root graph
fn register_in_graph() {
    // Create svc.Input node
    match thingsys::create_node(abi::schema::hid::SVC_INPUT) {
        Ok(node_id) => {
            info!("bristle: registered in graph as svc.Input (id={})", node_id.0);
        }
        Err(e) => {
            info!("bristle: failed to register in graph: {:?}", e);
        }
    }
}

/// Serialize a KeyDown event to wire format
fn serialize_key_down(key: Key, mods: Mods, repeat: bool, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 {
        return 0;
    }

    // Header (20 bytes)
    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: EventType::KeyDown as u16,
        timestamp_ns,
        payload_len: 4,
    };

    // Payload (4 bytes)
    let payload = KeyEventPayload {
        key: key as u16,
        mods: mods.0,
        flags: if repeat { 1 } else { 0 },
    };

    // Copy to buffer
    unsafe {
        core::ptr::copy_nonoverlapping(
            &header as *const _ as *const u8,
            buf.as_mut_ptr(),
            20,
        );
        core::ptr::copy_nonoverlapping(
            &payload as *const _ as *const u8,
            buf.as_mut_ptr().add(20),
            4,
        );
    }

    24
}

/// Serialize a KeyUp event to wire format
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
        core::ptr::copy_nonoverlapping(
            &header as *const _ as *const u8,
            buf.as_mut_ptr(),
            20,
        );
        core::ptr::copy_nonoverlapping(
            &payload as *const _ as *const u8,
            buf.as_mut_ptr().add(20),
            4,
        );
    }

    24
}

#[stem::main]
fn main(packed_handles: usize) -> ! {
    // Unpack handles: (raw_read << 16) | evt_write
    let raw_read = ((packed_handles >> 16) & 0xFFFF) as PortHandle;
    let evt_write = (packed_handles & 0xFFFF) as PortHandle;

    info!("bristle: online (raw={}, evt={})", raw_read, evt_write);
    
    // Register in the Root graph
    register_in_graph();

    let mut kbd_state = KeyboardState::new();
    let mut recv_buf = [0u8; 64];
    let mut send_buf = [0u8; 64];
    let mut drop_counter: u32 = 0;
    let mut event_count: u64 = 0;

    loop {
        match port_recv(raw_read, &mut recv_buf) {
            Ok(n) if n > 0 => {
                // Get a monotonic timestamp (approximate via event count for now)
                // TODO: Use actual monotonic time syscall
                let timestamp_ns = event_count * 1_000_000; // fake ~1ms per event

                for &byte in &recv_buf[..n] {
                    if let Some(edge) = kbd_state.process_ps2(byte) {
                        let len = match edge {
                            KeyEdge::Down { key, mods, repeat } => {
                                serialize_key_down(key, mods, repeat, timestamp_ns, &mut send_buf)
                            }
                            KeyEdge::Up { key, mods } => {
                                serialize_key_up(key, mods, timestamp_ns, &mut send_buf)
                            }
                        };

                        if len > 0 {
                            if let Err(_) = port_send(evt_write, &send_buf[..len]) {
                                drop_counter += 1;
                                if drop_counter == 1 || drop_counter % 100 == 0 {
                                    info!("bristle: event port full, dropped {} events", drop_counter);
                                }
                            } else {
                                event_count += 1;
                            }
                        }
                    }
                }
            }
            _ => {
                // No data, yield to avoid busy-spin
                stem::yield_now();
            }
        }
    }
}
