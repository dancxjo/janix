//! CPU bring-up for x86_64.
//!
//! This module wraps GDT/CPU initialization for API symmetry with AArch64.

use crate::bringup::gdt;

/// Initialize CPU state early in boot.
///
/// # Safety
/// Must be called once during boot.
pub unsafe fn init_cpu_early() {
    // GDT is initialized via Bridge::init() which calls gdt::init().
    // This function exists for API symmetry with aarch64.
}

/// Returns the current CPU ID.
///
/// On x86_64, this uses CPUID or APIC ID. For now, returns 0.
pub fn cpu_id() -> u16 {
    // TODO: Implement proper APIC ID reading.
    0
}
