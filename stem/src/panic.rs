//! Panic handling shims for `stem` consumers.
//!
//! - Kernel (`target_os = "none"`): `bran` provides its own `#[panic_handler]`.
//! - Userspace (`target_os = "thingos"`): `stem` provides a minimal aborting handler.

#[cfg(target_os = "thingos")]
use core::panic::PanicInfo;

/// Userspace panic handler for Thing-OS binaries.
///
/// This keeps userspace crates `#![no_std]` while still surfacing panic
/// diagnostics over the debug channel before aborting the task.
#[cfg(target_os = "thingos")]
#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    crate::pal::abort::debug_write_str("panic in userspace task: ");
    if let Some(message) = info.message().as_str() {
        crate::pal::abort::debug_write_str(message);
    }
    crate::pal::abort::debug_write_str("\n");
    crate::pal::abort::abort(101)
}
