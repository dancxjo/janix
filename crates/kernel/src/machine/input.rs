//! Machine-agnostic input interface.
//!
//! This module provides a uniform way for the kernel to access input data
//! (scancodes) regardless of the underlying architecture (PS/2, USB, UART, etc.).

#[cfg(target_arch = "x86_64")]
use crate::machine::x86_64::ps2_keyboard;

/// Initialize the input subsystem.
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    ps2_keyboard::init();
}

/// Read raw scancodes from the input buffer.
///
/// Returns the number of bytes read.
pub fn read_scancodes(dst: &mut [u8]) -> usize {
    #[cfg(target_arch = "x86_64")]
    return ps2_keyboard::read_scancodes(dst);

    #[cfg(not(target_arch = "x86_64"))]
    {
        // TODO: Implement for other architectures
        let _ = dst;
        0
    }
}

/// Debug dump of the input ring buffer state.
pub fn debug_dump_input_ring() {
    #[cfg(target_arch = "x86_64")]
    ps2_keyboard::debug_dump();
}
