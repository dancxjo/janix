//! Machine-agnostic input interface.
//!
//! This module provides a uniform way for the kernel to access input data
//! (scancodes) regardless of the underlying architecture (PS/2, USB, UART, etc.).

#[cfg(target_arch = "x86_64")]
use crate::machine::x86_64::ps2_keyboard;

#[cfg(target_arch = "x86_64")]
use crate::machine::x86_64::ps2_mouse;

/// Initialize the input subsystem (keyboard only - early init before heap).
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    ps2_keyboard::init();
}

/// Initialize mouse (requires heap - call after heap init).
pub fn init_mouse() {
    #[cfg(target_arch = "x86_64")]
    ps2_mouse::init();
}

/// Set the pointer thing ID for automatic graph publication.
pub fn set_pointer_thing_id(_id: abi::ids::ThingId) {

    #[cfg(target_arch = "x86_64")]
    ps2_mouse::set_pointer_thing_id(_id);
}

/// Set mouse screen bounds for pointer clamping.
pub fn set_mouse_bounds(_width: u32, _height: u32) {

    #[cfg(target_arch = "x86_64")]
    ps2_mouse::set_bounds(_width, _height);
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
