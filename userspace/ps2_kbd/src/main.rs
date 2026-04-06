#![feature(restricted_std)]
#![no_main]

use stem::syscall::{ioport_read, irq_subscribe, irq_wait, port_send_all, PortHandle};
use stem::thing::sys as thingsys;
use stem::{info, warn};

/// PS/2 controller status register
const PS2_STATUS: usize = 0x64;
/// PS/2 controller data register  
const PS2_DATA: usize = 0x60;

/// Status: output buffer full
const STATUS_OUTPUT_FULL: usize = 0x01;
/// Status: data from aux port (mouse) - skip
const STATUS_AUX_DATA: usize = 0x20;

/// IRQ1 vector (keyboard) - legacy IRQ1 maps to vector 0x21 after IOAPIC remap
const KBD_VECTOR: u8 = 0x21;

/// Polling interval in milliseconds for degraded mode
const POLLING_INTERVAL_MS: u64 = 10;

/// Driver state node kind
const KIND_DRV_PS2_KBD: &str = "drv.Ps2Keyboard";

#[stem::main]
fn main(raw_write_handle: usize) -> ! {
    let handle = raw_write_handle as PortHandle;

    info!("ps2_kbd: online (handle={})", handle);

    // Create a graph node to represent this driver instance
    let drv_node = match thingsys::create_node(KIND_DRV_PS2_KBD) {
        Ok(id) => {
            info!("ps2_kbd: created driver node {}", id.to_u64_lossy());
            Some(id)
        }
        Err(e) => {
            warn!("ps2_kbd: failed to create driver node: {:?}", e);
            None
        }
    };

    // Helper to update input mode
    let set_mode = |mode: &str| {
        if let Some(id) = drv_node {
            if let Ok(sym) = thingsys::intern(mode) {
                let _ = thingsys::prop_set(id, "dev.InputMode", sym as u64);
            }
        }
    };

    // Subscribe to keyboard interrupt
    match irq_subscribe(KBD_VECTOR) {
        Ok(()) => {
            info!("ps2_kbd: subscribed to IRQ1 (vector 0x{:02x})", KBD_VECTOR);
            set_mode("Interrupt");
            interrupt_loop(handle);
        }
        Err(e) => {
            info!(
                "ps2_kbd: IRQ subscribe failed ({:?}), falling back to polling",
                e
            );
            set_mode("Polling");
            polling_loop(handle);
        }
    }
}

mod normalizer;
mod thigmonasty;

use abi::hid::{
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, BristleEventHeader, EventType,
    KeyEventPayload,
};
use thigmonasty::{KeyboardState, KeyEdge};

/// Drain all pending keyboard data from the controller
fn drain_keyboard_data(handle: PortHandle, state: &mut KeyboardState, drop_counter: &mut u32) {
    // Read while data is available (handle burst of scancodes)
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL == 0 {
            break; // No more data
        }

        if status & STATUS_AUX_DATA == 0 {
            // Keyboard data - read and send
            let scancode = ioport_read(PS2_DATA, 1) as u8;
            if let Some(edge) = state.process_ps2(scancode) {
                send_key_event(handle, edge, drop_counter);
            }
        } else {
            // If aux data (mouse), stop draining - let ps2_mouse handle it
            stem::info!("ps2_kbd: yield on AUX data (mouse packet)");
            break;
        }
    }
}

fn send_key_event(handle: PortHandle, edge: KeyEdge, drop_counter: &mut u32) {
    let timestamp_ns = stem::monotonic_ns();
    let mut buf = [0u8; 24]; // Max size is header + 4 byte payload
    
    let (event_type, key, mods, repeat) = match edge {
        KeyEdge::Down { key, mods, repeat } => (EventType::KeyDown, key, mods, repeat),
        KeyEdge::Up { key, mods } => (EventType::KeyUp, key, mods, false),
    };

    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: event_type as u16,
        timestamp_ns,
        payload_len: KeyEventPayload::SIZE as u32,
    };
    
    let payload = KeyEventPayload {
        key: key as u16,
        mods: mods.0,
        flags: if repeat { 1 } else { 0 },
    };

    buf[0..20].copy_from_slice(&header.to_bytes());
    buf[20..24].copy_from_slice(&payload.to_bytes());
    
    if port_send_all(handle, &buf[..24]).is_err() {
        *drop_counter = drop_counter.wrapping_add(1);
        if *drop_counter <= 4 || *drop_counter % 100 == 0 {
            warn!(
                "ps2_kbd: dropped {} key events because raw input port {} is full",
                *drop_counter,
                handle
            );
        }
    }
}

/// Primary interrupt-driven loop.
fn interrupt_loop(handle: PortHandle) -> ! {
    info!("ps2_kbd: entering interrupt-driven loop");
    let mut state = KeyboardState::new();
    let mut drop_counter = 0u32;

    // Clear any already-pending keyboard bytes before waiting. On an edge-triggered
    // line, a latched byte can keep IRQ1 asserted without generating a fresh edge.
    drain_keyboard_data(handle, &mut state, &mut drop_counter);

    loop {
        match irq_wait(KBD_VECTOR) {
            Ok(_) => drain_keyboard_data(handle, &mut state, &mut drop_counter),
            Err(_) => stem::yield_now(),
        }
    }
}

/// Fallback polling loop (if IRQ subscribe fails)
fn polling_loop(handle: PortHandle) -> ! {
    info!(
        "ps2_kbd: using polling mode ({}ms interval)",
        POLLING_INTERVAL_MS
    );
    let mut state = KeyboardState::new();
    let mut drop_counter = 0u32;
    loop {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL != 0 {
            if status & STATUS_AUX_DATA == 0 {
                let scancode = ioport_read(PS2_DATA, 1) as u8;
                if let Some(edge) = state.process_ps2(scancode) {
                    send_key_event(handle, edge, &mut drop_counter);
                }
            } else {
                // Leave mouse bytes queued for ps2_mouse.
                stem::sleep_ms(1);
            }
        } else {
            // Rate limit the polling to avoid burning CPU
            stem::sleep_ms(POLLING_INTERVAL_MS);
        }
    }
}
