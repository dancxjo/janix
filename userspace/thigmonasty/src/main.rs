//! Thigmonasty: Keyboard Event Broker
//!
//! Reads raw scancodes from ps2_kbd, maintains keyboard state,
//! and emits KeyEventV0 records to the focused app.

#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};

/// Modifier key bits
const MOD_SHIFT: u8 = 1 << 0;
const MOD_CTRL: u8 = 1 << 1;
const MOD_ALT: u8 = 1 << 2;
#[allow(dead_code)]
const MOD_SUPER: u8 = 1 << 3;

/// KeyEventV0 wire format (4 bytes, packed)
/// kind: 1=down, 2=up
/// mods: bitmask of modifiers
/// scancode: raw scancode (low 7 bits)
/// flags: bit0=extended
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct KeyEventV0 {
    kind: u8,
    mods: u8,
    scancode: u8,
    flags: u8,
}

/// Keyboard state tracker
struct KeyboardState {
    mods: u8,
    e0_prefix: bool,
}

impl KeyboardState {
    fn new() -> Self {
        Self {
            mods: 0,
            e0_prefix: false,
        }
    }

    /// Process a raw scancode byte, optionally emitting an event
    fn process(&mut self, byte: u8) -> Option<KeyEventV0> {
        // Handle E0 extended prefix
        if byte == 0xE0 {
            self.e0_prefix = true;
            return None;
        }

        let is_break = byte & 0x80 != 0;
        let scancode = byte & 0x7F;
        let extended = self.e0_prefix;
        self.e0_prefix = false;

        // Update modifier state
        self.update_mods(scancode, !is_break, extended);

        Some(KeyEventV0 {
            kind: if is_break { 2 } else { 1 },
            mods: self.mods,
            scancode,
            flags: if extended { 1 } else { 0 },
        })
    }

    fn update_mods(&mut self, scancode: u8, pressed: bool, _extended: bool) {
        let bit = match scancode {
            0x2A | 0x36 => MOD_SHIFT, // Left/Right Shift
            0x1D => MOD_CTRL,          // Ctrl
            0x38 => MOD_ALT,           // Alt
            _ => return,
        };

        if pressed {
            self.mods |= bit;
        } else {
            self.mods &= !bit;
        }
    }
}

#[stem::main]
fn main(packed_handles: usize) -> ! {
    // Unpack handles: (raw_read << 16) | evt_write
    let raw_read = ((packed_handles >> 16) & 0xFFFF) as PortHandle;
    let evt_write = (packed_handles & 0xFFFF) as PortHandle;

    info!("thigmonasty: online (raw={}, evt={})", raw_read, evt_write);

    let mut state = KeyboardState::new();
    let mut buf = [0u8; 64];
    let mut drop_counter: u32 = 0;

    loop {
        match port_recv(raw_read, &mut buf) {
            Ok(n) if n > 0 => {
                for &byte in &buf[..n] {
                    if let Some(event) = state.process(byte) {
                        // Transmute event to bytes (safe because repr(C, packed))
                        let bytes: [u8; 4] = unsafe { core::mem::transmute(event) };
                        
                        if let Err(_) = port_send(evt_write, &bytes) {
                            // Bounded loss: drop and rate-limit logging
                            drop_counter += 1;
                            if drop_counter == 1 || drop_counter % 100 == 0 {
                                info!("thigmonasty: event port full, dropped {} events", drop_counter);
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
