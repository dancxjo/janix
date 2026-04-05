//! Bristle: Unified HID Broker
//!
//! The sole input authority. Drivers send raw reports, apps receive
//! normalized events. Apps never see scancodes, drivers never see apps.

#![feature(restricted_std)]
#![no_main]

use abi::hid::{
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, BristleEventHeader, EventType, Key,
    KeyEventPayload, PointerButtonPayload, PointerMovePayload,
};
use stem::info;
use stem::syscall::{PortHandle, port_recv, port_send, port_wait, topic_create, topic_publish};
use stem::thing::sys as thingsys;

/// Register Bristle in the Root graph and return the node ID
fn register_in_graph(topic_id: Option<u32>) -> Option<stem::thing::ThingId> {
    use abi::schema::input::INPUT_TOPIC_ID;
    match thingsys::create_node(abi::schema::hid::SVC_INPUT) {
        Ok(node_id) => {
            info!(
                "bristle: registered in graph as svc.Input (id={})",
                node_id.to_u64_lossy()
            );
            // Publish the topic ID so subscribers can find it
            if let Some(tid) = topic_id {
                let _ = thingsys::prop_set(node_id, INPUT_TOPIC_ID, tid as u64);
            }
            Some(node_id)
        }
        Err(e) => {
            info!("bristle: failed to register in graph: {:?}", e);
            None
        }
    }
}

#[derive(Clone, Copy)]
struct InputGraphState {
    pointer_x: i32,
    pointer_y: i32,
    pointer_buttons: u64,
    keyboard_gen: u64,
}

impl Default for InputGraphState {
    fn default() -> Self {
        Self {
            // Match existing viewport defaults so graph-state consumers do not jump to 0,0.
            pointer_x: 400,
            pointer_y: 300,
            pointer_buttons: 0,
            keyboard_gen: 0,
        }
    }
}

fn publish_initial_graph_state(node_id: stem::thing::ThingId, state: &InputGraphState) {
    use abi::schema::{keyboard as kb, pointer};

    let _ = thingsys::prop_set(node_id, pointer::POINTER_X, state.pointer_x as u64);
    let _ = thingsys::prop_set(node_id, pointer::POINTER_Y, state.pointer_y as u64);
    let _ = thingsys::prop_set(node_id, pointer::POINTER_BUTTONS, state.pointer_buttons);
    let _ = thingsys::prop_set(node_id, kb::KEYBOARD_GEN, state.keyboard_gen);
}

fn update_keyboard_graph_state(
    node_id: stem::thing::ThingId,
    state: &mut InputGraphState,
    payload: KeyEventPayload,
    is_down: bool,
) {
    use abi::schema::keyboard as kb;

    state.keyboard_gen = state.keyboard_gen.wrapping_add(1);
    let _ = thingsys::prop_set(node_id, kb::KEYBOARD_LAST_KEY, payload.key as u64);
    let _ = thingsys::prop_set(node_id, kb::KEYBOARD_KEY_EDGE, if is_down { 1 } else { 0 });
    let _ = thingsys::prop_set(node_id, kb::KEYBOARD_MODS, payload.mods as u64);
    let _ = thingsys::prop_set(node_id, kb::KEYBOARD_GEN, state.keyboard_gen);
}

fn update_pointer_move_graph_state(
    node_id: stem::thing::ThingId,
    state: &mut InputGraphState,
    payload: PointerMovePayload,
) {
    use abi::schema::pointer;

    state.pointer_x = state.pointer_x.saturating_add(payload.dx as i32);
    state.pointer_y = state.pointer_y.saturating_add(payload.dy as i32);
    let _ = thingsys::prop_set(node_id, pointer::POINTER_X, state.pointer_x as u64);
    let _ = thingsys::prop_set(node_id, pointer::POINTER_Y, state.pointer_y as u64);
}

fn update_pointer_button_graph_state(
    node_id: stem::thing::ThingId,
    state: &mut InputGraphState,
    payload: PointerButtonPayload,
    is_down: bool,
) {
    use abi::schema::pointer;

    let bit = 1u64 << (payload.button as u64);
    if is_down {
        state.pointer_buttons |= bit;
    } else {
        state.pointer_buttons &= !bit;
    }
    let _ = thingsys::prop_set(node_id, pointer::POINTER_BUTTONS, state.pointer_buttons);
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

// (serialization functions removed as devices emit serialized events directly)

/// Kill all userspace tasks except Bristle, then respawn Sprout.
fn reset_userspace_and_respawn_sprout() {
    use abi::schema::{keys, kinds};
    use stem::thing::ThingId;

    // Find all proc.Thread nodes
    let mut thread_buf = [ThingId::default(); 256];
    let count = match thingsys::find(kinds::PROC_THREAD, &mut thread_buf) {
        Ok(c) => c.min(256),
        Err(_) => {
            info!("bristle: failed to find proc.Thread nodes");
            return;
        }
    };

    let mut killed = 0u64;
    for i in 0..count {
        let node = thread_buf[i];

        // Only target userspace tasks.
        let is_user = match thingsys::prop_get(node, keys::PROC_IS_USER) {
            Ok(v) => v != 0,
            Err(_) => false,
        };
        if !is_user {
            continue;
        }

        // Get the interned name symbol
        let name_sym = match thingsys::prop_get(node, keys::PROC_NAME) {
            Ok(v) => v,
            Err(_) => {
                // If name is missing, still kill userspace task.
                let tid = match thingsys::prop_get(node, keys::PROC_TID) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                match stem::syscall::task_kill(tid) {
                    Ok(()) => killed += 1,
                    Err(e) => info!(
                        "bristle: failed to kill unnamed userspace tid={}: {:?}",
                        tid, e
                    ),
                }
                continue;
            }
        };

        // Resolve the symbol to a string
        let mut name_buf = [0u8; 64];
        let name_len = match thingsys::describe_symbol(name_sym as u32, &mut name_buf) {
            Ok(n) => n,
            Err(_) => continue,
        };

        let name = match core::str::from_utf8(&name_buf[..name_len]) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Keep Bristle alive so it can perform the respawn.
        if name.ends_with("/bristle") || name == "bristle" {
            continue;
        }

        // Get the TID
        let tid = match thingsys::prop_get(node, keys::PROC_TID) {
            Ok(v) => v,
            Err(_) => continue,
        };

        info!("bristle: killing userspace task {} (tid={})", name, tid);
        match stem::syscall::task_kill(tid) {
            Ok(()) => killed += 1,
            Err(e) => info!("bristle: failed to kill {}: {:?}", name, e),
        }
    }

    match stem::syscall::spawn_process("/boot/sprout", 0) {
        Ok(tid) => info!(
            "bristle: userspace reset complete (killed {}), respawned sprout tid={}",
            killed, tid
        ),
        Err(e) => info!(
            "bristle: userspace reset killed {}, but failed to respawn sprout: {:?}",
            killed, e
        ),
    }
}

#[stem::main]
fn main(packed_handles: usize) -> ! {
    // Layout: kbd_raw_read[63:48] | mouse_raw_read[47:32] | evt_write[31:16] | evt_echo_write[15:0]
    let packed = packed_handles as u64;
    let kbd_read = ((packed >> 48) & 0xFFFF) as PortHandle;
    let mouse_read = ((packed >> 32) & 0xFFFF) as PortHandle;
    let legacy_evt_write = ((packed >> 16) & 0xFFFF) as PortHandle;
    let legacy_evt_echo_write = (packed & 0xFFFF) as PortHandle;

    stem::info!("BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY");




    info!(
        "bristle: online (kbd={}, mouse={}, evt={}, echo={})",
        kbd_read, mouse_read, legacy_evt_write, legacy_evt_echo_write
    );

    // Create the broadcast topic
    let topic_id = match topic_create() {
        Ok(id) => {
            info!("bristle: created broadcast topic {}", id);
            Some(id)
        }
        Err(e) => {
            info!("bristle: FAILED to create broadcast topic: {:?}", e);
            None // Fallback to legacy only
        }
    };

    let node_id = register_in_graph(topic_id);
    let mut graph_state = InputGraphState::default();
    if let Some(node) = node_id {
        publish_initial_graph_state(node, &graph_state);
    }

    let mut recv_buf = [0u8; 128];
    let mut event_accum = [0u8; 64];
    let mut accum_len = 0usize;
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

        if let Ok(n) = port_recv(ready_handle, &mut recv_buf) {
            if n > 0 {
                let mut cursor = 0;
                while cursor < n {
                    let to_copy = (n - cursor).min(64 - accum_len);
                    event_accum[accum_len..accum_len + to_copy]
                        .copy_from_slice(&recv_buf[cursor..cursor + to_copy]);
                    accum_len += to_copy;
                    cursor += to_copy;

                    while accum_len >= BristleEventHeader::SIZE {
                        // Check if we have enough bytes for the header + payload
                        let mut header_bytes = [0u8; BristleEventHeader::SIZE];
                        header_bytes.copy_from_slice(&event_accum[..BristleEventHeader::SIZE]);
                        
                        if let Ok(header) = BristleEventHeader::from_bytes(&header_bytes) {
                            let total_len = BristleEventHeader::SIZE + header.payload_len as usize;
                            if accum_len >= total_len {
                                let event_bytes = &event_accum[..total_len];

                                // Parse hotkeys and contract logs
                                if header.event_type == EventType::KeyDown as u16 && header.payload_len >= 4 {
                                    let mut p = [0u8; 4];
                                    p.copy_from_slice(&event_bytes[20..24]);
                                    let payload = KeyEventPayload::from_bytes(&p);

                                    crate::info!(
                                        "[CONTRACT] CONTRACT: input key_event key={} edge=down mods=0x{:02x} repeat={}",
                                        payload.key().name(),
                                        payload.mods,
                                        payload.is_repeat()
                                    );
                                    if let Some(node) = node_id {
                                        update_keyboard_graph_state(node, &mut graph_state, payload, true);
                                    }

                                    match payload.key() {
                                        Key::F2 => {
                                            info!("bristle: F2 pressed - dumping tasks...");
                                            stem::syscall::task_dump();
                                        }
                                        Key::F10 => {
                                            info!("bristle: F10 pressed - dumping graph...");
                                            let _ = thingsys::dump_graph(0);
                                        }
                                        Key::F12 => {
                                            info!("bristle: F12 pressed - resetting userspace and respawning sprout...");
                                            reset_userspace_and_respawn_sprout();
                                        }
                                        Key::Delete => {
                                            if payload.mods().has_ctrl() && payload.mods().has_alt() {
                                                info!("bristle: Ctrl+Alt+Del - rebooting system...");
                                                stem::syscall::reboot();
                                            }
                                        }
                                        _ => {}
                                    }
                                } else if header.event_type == EventType::KeyUp as u16 && header.payload_len >= 4 {
                                    let mut p = [0u8; 4];
                                    p.copy_from_slice(&event_bytes[20..24]);
                                    let payload = KeyEventPayload::from_bytes(&p);

                                    crate::info!(
                                        "[CONTRACT] CONTRACT: input key_event key={} edge=up mods=0x{:02x}",
                                        payload.key().name(),
                                        payload.mods
                                    );
                                    if let Some(node) = node_id {
                                        update_keyboard_graph_state(node, &mut graph_state, payload, false);
                                    }
                                } else if header.event_type == EventType::PointerMove as u16 && header.payload_len >= 4 {
                                    let mut p = [0u8; 4];
                                    p.copy_from_slice(&event_bytes[20..24]);
                                    let payload = PointerMovePayload::from_bytes(&p);
                                    if let Some(node) = node_id {
                                        update_pointer_move_graph_state(node, &mut graph_state, payload);
                                    }
                                    let dx = payload.dx;
                                    let dy = payload.dy;
                                    crate::info!(
                                        "[CONTRACT] CONTRACT: input pointer_move dx={} dy={}",
                                        dx,
                                        dy
                                    );
                                } else if header.event_type == EventType::PointerButtonDown as u16
                                    && header.payload_len >= PointerButtonPayload::SIZE as u32
                                {
                                    let mut p = [0u8; PointerButtonPayload::SIZE];
                                    p.copy_from_slice(
                                        &event_bytes[20..20 + PointerButtonPayload::SIZE]
                                    );
                                    let payload = PointerButtonPayload::from_bytes(&p);
                                    if let Some(node) = node_id {
                                        update_pointer_button_graph_state(
                                            node,
                                            &mut graph_state,
                                            payload,
                                            true,
                                        );
                                    }
                                } else if header.event_type == EventType::PointerButtonUp as u16
                                    && header.payload_len >= PointerButtonPayload::SIZE as u32
                                {
                                    let mut p = [0u8; PointerButtonPayload::SIZE];
                                    p.copy_from_slice(
                                        &event_bytes[20..20 + PointerButtonPayload::SIZE]
                                    );
                                    let payload = PointerButtonPayload::from_bytes(&p);
                                    if let Some(node) = node_id {
                                        update_pointer_button_graph_state(
                                            node,
                                            &mut graph_state,
                                            payload,
                                            false,
                                        );
                                    }
                                }

                                // Forward the event to subscribers
                                let mut _sent_to_legacy = false;
                                if port_send(legacy_evt_write, event_bytes).is_ok() {
                                    _sent_to_legacy = true;
                                } else {
                                    drop_counter += 1;
                                }

                                if legacy_evt_echo_write != 0 {
                                    if port_send(legacy_evt_echo_write, event_bytes).is_ok() {
                                        _sent_to_legacy = true;
                                    } else {
                                        drop_counter += 1;
                                    }
                                }

                                event_count += 1;
                                if let Some(tid) = topic_id {
                                    let _ = topic_publish(tid, event_bytes);
                                }

                                // Shift remaining bytes
                                accum_len -= total_len;
                                if accum_len > 0 {
                                    event_accum.copy_within(total_len..total_len + accum_len, 0);
                                }
                            } else {
                                // Wait for more payload bytes
                                break;
                            }
                        } else {
                            // Invalid header, drop 1 byte to try resync
                            accum_len -= 1;
                            if accum_len > 0 {
                                event_accum.copy_within(1..1 + accum_len, 0);
                            }
                        }
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
