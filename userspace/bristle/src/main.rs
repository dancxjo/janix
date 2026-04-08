//! Bristle: Unified HID Broker
//!
//! The sole input authority. Drivers send raw reports, apps receive
//! normalized events. Apps never see scancodes, drivers never see apps.

#![feature(restricted_std)]
#![no_main]

use abi::hid::{
    BristleEventHeader, EventType, Key, KeyEventPayload, PointerButtonPayload, PointerMovePayload,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use stem::info;
use stem::syscall::{channel_recv, channel_send_all, ChannelHandle};
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

    if let Err(e) = thingsys::prop_set(node_id, pointer::POINTER_X, state.pointer_x as u64) {
        info!(
            "bristle: failed to set pointer.x on {}: {:?}",
            node_id.to_u64_lossy(),
            e
        );
    }
    if let Err(e) = thingsys::prop_set(node_id, pointer::POINTER_Y, state.pointer_y as u64) {
        info!(
            "bristle: failed to set pointer.y on {}: {:?}",
            node_id.to_u64_lossy(),
            e
        );
    }
    if let Err(e) = thingsys::prop_set(node_id, pointer::POINTER_BUTTONS, state.pointer_buttons) {
        info!(
            "bristle: failed to set pointer.buttons on {}: {:?}",
            node_id.to_u64_lossy(),
            e
        );
    }
    if let Err(e) = thingsys::prop_set(node_id, kb::KEYBOARD_GEN, state.keyboard_gen) {
        info!(
            "bristle: failed to set keyboard.gen on {}: {:?}",
            node_id.to_u64_lossy(),
            e
        );
    }
}

fn update_keyboard_graph_state(
    node_id: stem::thing::ThingId,
    state: &mut InputGraphState,
    payload: KeyEventPayload,
    is_down: bool,
) {
    use abi::schema::keyboard as kb;

    state.keyboard_gen = state.keyboard_gen.wrapping_add(1);
    if let Err(e) = thingsys::prop_set(node_id, kb::KEYBOARD_LAST_KEY, payload.key as u64) {
        info!("bristle: failed to set keyboard.last_key: {:?}", e);
    }
    if let Err(e) = thingsys::prop_set(node_id, kb::KEYBOARD_KEY_EDGE, if is_down { 1 } else { 0 })
    {
        info!("bristle: failed to set keyboard.key_edge: {:?}", e);
    }
    if let Err(e) = thingsys::prop_set(node_id, kb::KEYBOARD_MODS, payload.mods as u64) {
        info!("bristle: failed to set keyboard.mods: {:?}", e);
    }
    if let Err(e) = thingsys::prop_set(node_id, kb::KEYBOARD_GEN, state.keyboard_gen) {
        info!("bristle: failed to set keyboard.gen: {:?}", e);
    }
}

fn update_pointer_move_graph_state(
    node_id: stem::thing::ThingId,
    state: &mut InputGraphState,
    payload: PointerMovePayload,
) {
    use abi::schema::pointer;

    state.pointer_x = state.pointer_x.saturating_add(payload.dx as i32);
    state.pointer_y = state.pointer_y.saturating_add(payload.dy as i32);
    if let Err(e) = thingsys::prop_set(node_id, pointer::POINTER_X, state.pointer_x as u64) {
        info!("bristle: failed to set pointer.x: {:?}", e);
    }
    if let Err(e) = thingsys::prop_set(node_id, pointer::POINTER_Y, state.pointer_y as u64) {
        info!("bristle: failed to set pointer.y: {:?}", e);
    }
    let readback_x = thingsys::prop_get(node_id, pointer::POINTER_X).unwrap_or(u64::MAX) as i32;
    let readback_y = thingsys::prop_get(node_id, pointer::POINTER_Y).unwrap_or(u64::MAX) as i32;
    info!(
        "bristle: graph pointer now ({}, {}) readback=({}, {})",
        state.pointer_x, state.pointer_y, readback_x, readback_y
    );
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
    if let Err(e) = thingsys::prop_set(node_id, pointer::POINTER_BUTTONS, state.pointer_buttons) {
        info!("bristle: failed to set pointer.buttons: {:?}", e);
    }
    let readback = thingsys::prop_get(node_id, pointer::POINTER_BUTTONS).unwrap_or(u64::MAX);
    info!(
        "bristle: graph pointer.buttons now {:x} readback={:x}",
        state.pointer_buttons, readback
    );
}

// (serialization functions removed as devices emit serialized events directly)

/// Kill all userspace tasks except Bristle, then respawn Sprout.
fn reset_userspace_and_respawn_sprout() {
    info!("bristle: reset_userspace_and_respawn_sprout is deprecated and currently disabled.");
}

#[stem::main]
fn main(packed_handles: usize) -> ! {
    // Layout: kbd_raw_read[63:48] | mouse_raw_read[47:32] | bloom_evt_write[31:16] | evt_input_echo_write[15:0]
    let packed = packed_handles as u64;
    let kbd_read = ((packed >> 48) & 0xFFFF) as ChannelHandle;
    let mouse_read = ((packed >> 32) & 0xFFFF) as ChannelHandle;
    let bloom_evt_write = ((packed >> 16) & 0xFFFF) as ChannelHandle;
    let evt_input_echo_write = (packed & 0xFFFF) as ChannelHandle;

    stem::info!("BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY");

    info!(
        "bristle: online (kbd={}, mouse={}, bloom_evt={}, input_echo={})",
        kbd_read, mouse_read, bloom_evt_write, evt_input_echo_write
    );

    let node_id = register_in_graph();
    let mut graph_state = InputGraphState::default();
    if let Some(node) = node_id {
        publish_initial_graph_state(node, &graph_state);
    }
    let mut recv_buf = [0u8; 128];
    let mut event_accum = [0u8; 64];
    let mut accum_len = 0usize;
    let mut drop_counter: u32 = 0;
    let mut resync_counter: u32 = 0;

    let mut ws = stem::wait_set::WaitSet::new();
    let mut kbd_tok = None;
    let mut mouse_tok = None;

    if kbd_read != 0 {
        match ws.add_port_readable(kbd_read as u64) {
            Ok(tok) => kbd_tok = Some(tok),
            Err(e) => info!(
                "bristle: failed to watch kbd_read port {}: {:?}",
                kbd_read, e
            ),
        }
    }
    if mouse_read != 0 {
        match ws.add_port_readable(mouse_read as u64) {
            Ok(tok) => mouse_tok = Some(tok),
            Err(e) => info!(
                "bristle: failed to watch mouse_read port {}: {:?}",
                mouse_read, e
            ),
        }
    }

    if kbd_tok.is_none() && mouse_tok.is_none() {
        info!("bristle: no valid input ports to watch, shutting down loop");
        loop {
            stem::sleep_ms(1000);
        }
    }

    loop {
        let events = match ws.wait(None::<stem::time::Duration>) {
            Ok(evs) => evs,
            Err(e) => {
                stem::warn!("bristle: ws.wait error: {:?}", e);
                stem::time::sleep_ms(10);
                continue;
            }
        };

        for ev in events {
            if !ev.is_readable() {
                continue;
            }

            let ready_handle = if Some(ev.token()) == kbd_tok {
                kbd_read
            } else if Some(ev.token()) == mouse_tok {
                mouse_read
            } else {
                continue;
            };

            if let Ok(n) = channel_recv(ready_handle, &mut recv_buf) {
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
                                let total_len =
                                    BristleEventHeader::SIZE + header.payload_len as usize;
                                if accum_len >= total_len {
                                    let event_bytes = &event_accum[..total_len];

                                    // Parse hotkeys and contract logs
                                    if header.event_type == EventType::KeyDown as u16
                                        && header.payload_len >= 4
                                    {
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
                                            update_keyboard_graph_state(
                                                node,
                                                &mut graph_state,
                                                payload,
                                                true,
                                            );
                                        }

                                        match payload.key() {
                                            Key::F2 => {
                                                info!("bristle: F2 pressed - dumping tasks...");
                                                stem::syscall::task_dump();
                                            }
                                            Key::F10 => {
                                                info!("bristle: F10 pressed - graph dump is disabled.");
                                            }
                                            Key::Delete => {
                                                if payload.mods().has_ctrl()
                                                    && payload.mods().has_alt()
                                                {
                                                    info!(
                                                        "bristle: Ctrl+Alt+Del - rebooting system..."
                                                    );
                                                    stem::syscall::reboot();
                                                }
                                            }
                                            Key::F12 => {
                                                info!(
                                                    "bristle: F12 pressed - resetting userspace and respawning sprout..."
                                                );
                                                reset_userspace_and_respawn_sprout();
                                            }
                                            _ => {}
                                        }
                                    } else if header.event_type == EventType::KeyUp as u16
                                        && header.payload_len >= 4
                                    {
                                        let mut p = [0u8; 4];
                                        p.copy_from_slice(&event_bytes[20..24]);
                                        let payload = KeyEventPayload::from_bytes(&p);

                                        crate::info!(
                                            "[CONTRACT] CONTRACT: input key_event key={} edge=up mods=0x{:02x}",
                                            payload.key().name(),
                                            payload.mods
                                        );
                                        if let Some(node) = node_id {
                                            update_keyboard_graph_state(
                                                node,
                                                &mut graph_state,
                                                payload,
                                                false,
                                            );
                                        }
                                    } else if header.event_type == EventType::PointerMove as u16
                                        && header.payload_len >= 4
                                    {
                                        let mut p = [0u8; 4];
                                        p.copy_from_slice(&event_bytes[20..24]);
                                        let payload = PointerMovePayload::from_bytes(&p);
                                        if let Some(node) = node_id {
                                            update_pointer_move_graph_state(
                                                node,
                                                &mut graph_state,
                                                payload,
                                            );
                                        }
                                        let dx = payload.dx;
                                        let dy = payload.dy;
                                        crate::info!(
                                            "[CONTRACT] CONTRACT: input pointer_move dx={} dy={}",
                                            dx,
                                            dy
                                        );
                                    } else if header.event_type
                                        == EventType::PointerButtonDown as u16
                                        && header.payload_len >= PointerButtonPayload::SIZE as u32
                                    {
                                        let mut p = [0u8; PointerButtonPayload::SIZE];
                                        p.copy_from_slice(
                                            &event_bytes[20..20 + PointerButtonPayload::SIZE],
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
                                            &event_bytes[20..20 + PointerButtonPayload::SIZE],
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

                                    if channel_send_all(bloom_evt_write, event_bytes).is_err() {
                                        drop_counter += 1;
                                    }

                                    if evt_input_echo_write != 0
                                        && channel_send_all(evt_input_echo_write, event_bytes)
                                            .is_err()
                                    {
                                        drop_counter += 1;
                                    }

                                    // Shift remaining bytes
                                    accum_len -= total_len;
                                    if accum_len > 0 {
                                        event_accum
                                            .copy_within(total_len..total_len + accum_len, 0);
                                    }
                                } else {
                                    // Wait for more payload bytes
                                    break;
                                }
                            } else {
                                // Invalid header, drop 1 byte to try resync
                                resync_counter = resync_counter.wrapping_add(1);
                                if resync_counter <= 4 || resync_counter % 100 == 0 {
                                    info!(
                                        "bristle: resyncing raw stream after invalid header (count={}, accum_len={})",
                                        resync_counter, accum_len
                                    );
                                }
                                accum_len -= 1;
                                if accum_len > 0 {
                                    event_accum.copy_within(1..1 + accum_len, 0);
                                }
                            }
                        }
                    }
                }
            } // close if Ok(n)
        } // close for ev in events

        if drop_counter > 0 && drop_counter % 100 == 0 {
            info!("bristle: dropped {} events (port full)", drop_counter);
        }
    }
}
