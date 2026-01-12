//! Thigmonasty: Keyboard State Engine
//!
//! The "reflex arc" for keyboard input:
//! - Tracks pressed keys
//! - Maintains modifier state
//! - Generates repeat events
//! - Emits KeyDown/KeyUp edges

use abi::hid::{Key, Mods};
use crate::normalizer::ps2_to_key;

/// Keyboard state tracker
pub struct KeyboardState {
    /// Current modifier bitmask
    mods: u8,
    /// E0 extended prefix pending
    e0_prefix: bool,
    /// Currently pressed keys (simple bitset for common keys)
    pressed: [u64; 4],  // 256 bits
}

/// Edge event emitted by the keyboard state machine
#[derive(Clone, Copy, Debug)]
pub enum KeyEdge {
    Down { key: Key, mods: Mods, repeat: bool },
    Up { key: Key, mods: Mods },
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            mods: 0,
            e0_prefix: false,
            pressed: [0; 4],
        }
    }

    /// Process a raw PS/2 scancode byte, returning edge event if any
    pub fn process_ps2(&mut self, byte: u8) -> Option<KeyEdge> {
        // Handle E0 extended prefix
        if byte == 0xE0 {
            self.e0_prefix = true;
            return None;
        }

        let is_break = byte & 0x80 != 0;
        let scancode = byte & 0x7F;
        let extended = self.e0_prefix;
        self.e0_prefix = false;

        // Convert to normalized key
        let key = ps2_to_key(scancode, extended);
        
        // Update modifier state
        self.update_mods(key, !is_break);

        // Check for repeat (key already pressed)
        let key_idx = key as u16 as usize;
        let word_idx = key_idx / 64;
        let bit_idx = key_idx % 64;
        
        if word_idx < 4 {
            let was_pressed = self.pressed[word_idx] & (1 << bit_idx) != 0;
            
            if is_break {
                // Key released
                self.pressed[word_idx] &= !(1 << bit_idx);
                Some(KeyEdge::Up { key, mods: Mods(self.mods) })
            } else if was_pressed {
                // Key repeat
                Some(KeyEdge::Down { key, mods: Mods(self.mods), repeat: true })
            } else {
                // Key pressed
                self.pressed[word_idx] |= 1 << bit_idx;
                Some(KeyEdge::Down { key, mods: Mods(self.mods), repeat: false })
            }
        } else {
            // Key index out of range, emit without tracking
            if is_break {
                Some(KeyEdge::Up { key, mods: Mods(self.mods) })
            } else {
                Some(KeyEdge::Down { key, mods: Mods(self.mods), repeat: false })
            }
        }
    }

    fn update_mods(&mut self, key: Key, pressed: bool) {
        let bit = match key {
            Key::LeftShift | Key::RightShift => Mods::SHIFT,
            Key::LeftCtrl | Key::RightCtrl => Mods::CTRL,
            Key::LeftAlt => Mods::ALT,
            Key::RightAlt => Mods::ALTGR,
            Key::LeftMeta | Key::RightMeta => Mods::META,
            _ => return,
        };

        if pressed {
            self.mods |= bit;
        } else {
            self.mods &= !bit;
        }
    }

    #[allow(dead_code)]
    pub fn mods(&self) -> Mods {
        Mods(self.mods)
    }
}
