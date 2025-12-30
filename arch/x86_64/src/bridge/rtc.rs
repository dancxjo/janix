//! RTC interface for x86_64.
//!
//! Stub on x86_64; required for API symmetry.
//! The actual RTC reading lives in the `Rtc` impl on the bridge.

/// Initialize the RTC.
pub fn init() {
    // Stub on x86_64 until implemented; required for API symmetry.
    // RTC is accessed via CMOS ports in Bridge::rtc_read.
}

/// Get wall-clock time as Unix nanoseconds.
///
/// Returns None if unsupported.
pub fn wallclock_unix_ns() -> Option<u64> {
    // Stub on x86_64 until implemented; required for API symmetry.
    None
}
