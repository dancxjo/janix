//! Timer interface for x86_64.
//!
//! Stub on x86_64; required for API symmetry.
//! The actual timer is managed via HPET/APIC in the kernel drivers.

/// Initialize the timer.
pub fn init() {
    // Stub on x86_64 until implemented; required for API symmetry.
    // Timer is managed via HPET in kernel::drivers::hpet.
}

/// Get monotonic time in nanoseconds.
///
/// Currently delegated to HPET via kernel driver.
pub fn monotonic_ns() -> u64 {
    // Stub: returns 0. Actual implementation uses kernel::drivers::hpet::read_ns().
    0
}

/// Busy-wait for the specified number of nanoseconds.
pub fn sleep_ns(_ns: u64) {
    // Stub on x86_64 until implemented; required for API symmetry.
}
