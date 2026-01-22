//! Bristle: Unified HID Broker
//!
//! The sole input authority. Drivers send raw reports, apps receive
//! normalized events. Apps never see scancodes, drivers never see apps.

#![no_std]
#![no_main]

mod mouse;
mod normalizer;
mod thigmonasty;

use abi::hid::{
    BristleEventHeader, EventType, Key, KeyEventPayload, Mods,
    PointerMovePayload, PointerButtonPayload,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use mouse::{MouseState, PointerEvent};
use stem::info;
use stem::syscall::{port_recv, port_send, port_wait, PortHandle};
#[cfg(feature = "diagnostic-apps")]
use stem::syscall::{port_create, spawn_process};
use stem::thing::sys as thingsys;
use thigmonasty::{KeyEdge, KeyboardState};
use abi::ids::HandleId;

/// Register Bristle in the Root graph
fn register_in_graph() {
    match thingsys::create_node(abi::schema::hid::SVC_INPUT) {
        Ok(node_id) => {
            info!("bristle: registered in graph as svc.Input (id={})", node_id.to_u64_lossy());
        }
        Err(e) => {
            info!("bristle: failed to register in graph: {:?}", e);
        }
    }
}

/// Serialize a KeyDown event
fn serialize_key_down(key: Key, mods: Mods, repeat: bool, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 { return 0; }
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
        core::ptr::copy_nonoverlapping(&payload as *const _ as *const u8, buf.as_mut_ptr().add(20), 4);
    }
    24
}

/// Serialize a KeyUp event
fn serialize_key_up(key: Key, mods: Mods, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 { return 0; }
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
        core::ptr::copy_nonoverlapping(&payload as *const _ as *const u8, buf.as_mut_ptr().add(20), 4);
    }
    24
}

/// Serialize a PointerMove event
fn serialize_pointer_move(dx: i16, dy: i16, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 { return 0; }
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
        core::ptr::copy_nonoverlapping(&payload as *const _ as *const u8, buf.as_mut_ptr().add(20), 4);
    }
    24
}

/// Serialize a PointerButtonDown event
fn serialize_pointer_button_down(button: u8, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 { return 0; }
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
        core::ptr::copy_nonoverlapping(&payload as *const _ as *const u8, buf.as_mut_ptr().add(20), 2);
    }
    22
}

/// Serialize a PointerButtonUp event
fn serialize_pointer_button_up(button: u8, timestamp_ns: u64, buf: &mut [u8]) -> usize {
    if buf.len() < 24 { return 0; }
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
        core::ptr::copy_nonoverlapping(&payload as *const _ as *const u8, buf.as_mut_ptr().add(20), 2);
    }
    22
}

#[stem::main]
fn main(packed_handles: usize) -> ! {
    // Unpack handles from 64-bit value:
    // bits 48-63: kbd_read
    // bits 32-47: mouse_read
    // bits 16-31: evt_write (bloom)
    // bits  0-15: evt_echo_write
    let packed = packed_handles as u64;
    let kbd_read = ((packed >> 48) & 0xFFFF) as PortHandle;
    let mouse_read = ((packed >> 32) & 0xFFFF) as PortHandle;
    let evt_write = ((packed >> 16) & 0xFFFF) as PortHandle;
    let evt_echo_write = (packed & 0xFFFF) as PortHandle;

    info!(
        "bristle: online (kbd={}, mouse={}, evt={}, evt_echo={})",
        kbd_read, mouse_read, evt_write, evt_echo_write
    );
    
    register_in_graph();

    let mut kbd_state = KeyboardState::new();
    let mut mouse_state = MouseState::new();
    
    #[cfg(feature = "diagnostic-apps")]
    let evt_mouse_write: PortHandle = {
        let mut write_handle: PortHandle = 0;
        match port_create(8192) {
            Ok((write_h, read_h)) => {
                write_handle = write_h;
                match spawn_process("/echo_mouse", read_h as usize) {
                    Ok(pid) => {
                        info!("bristle: spawned echo_mouse (PID={})", pid);
                    }
                    Err(e) => {
                        info!("bristle: failed to spawn echo_mouse: {:?}", e);
                    }
                }
            }
            Err(e) => {
                info!("bristle: failed to create echo_mouse port: {:?}", e);
            }
        }
        write_handle
    };
    
    #[cfg(not(feature = "diagnostic-apps"))]
    let evt_mouse_write: PortHandle = 0;

    let mut kbd_buf = [0u8; 64];
    let mut mouse_buf = [0u8; 64];
    let mut send_buf = [0u8; 64];
    let mut drop_counter: u32 = 0;
    let mut event_count: u64 = 0;

    let wait_handles = [kbd_read, mouse_read];
    loop {
        // Block until keyboard or mouse data arrives
        let _ = port_wait(&wait_handles);

        // Process keyboard input
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
                            if port_send(evt_write, &send_buf[..len]).is_ok() {
                                sent = true;
                            } else {
                                drop_counter += 1;
                            }
                            if evt_echo_write != 0 {
                                if port_send(evt_echo_write, &send_buf[..len]).is_ok() {
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
        
        // Process mouse input  
        if let Ok(n) = port_recv(mouse_read, &mut mouse_buf) {
            if n >= 3 {
                // Process 3-byte packets
                let mut offset = 0;
                while offset + 3 <= n {
                    let packet: [u8; 3] = [
                        mouse_buf[offset],
                        mouse_buf[offset + 1],
                        mouse_buf[offset + 2],
                    ];
                    
                    let (events, count) = mouse_state.process_packet(&packet);
                    for i in 0..count {
                        if let Some(evt) = events[i] {
                            let timestamp_ns = stem::monotonic_ns();
                            let len = match evt {
                                PointerEvent::Move { dx, dy } => {
                                    serialize_pointer_move(dx, dy, timestamp_ns, &mut send_buf)
                                }
                                PointerEvent::ButtonDown { button } => {
                                    serialize_pointer_button_down(button, timestamp_ns, &mut send_buf)
                                }
                                PointerEvent::ButtonUp { button } => {
                                    serialize_pointer_button_up(button, timestamp_ns, &mut send_buf)
                                }
                            };
                            if len > 0 {
                                let mut sent = false;
                                if port_send(evt_write, &send_buf[..len]).is_ok() {
                                    sent = true;
                                } else {
                                    drop_counter += 1;
                                }
                                if evt_echo_write != 0 {
                                    if port_send(evt_echo_write, &send_buf[..len]).is_ok() {
                                        sent = true;
                                    } else {
                                        drop_counter += 1;
                                    }
                                }
                                if evt_mouse_write != 0 {
                                    if port_send(evt_mouse_write, &send_buf[..len]).is_ok() {
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
                    offset += 3;
                }
            }
        }
        
        // Rate-limited drop logging
        if drop_counter > 0 && drop_counter % 100 == 0 {
            info!("bristle: dropped {} events (port full)", drop_counter);
        }
    }
}
