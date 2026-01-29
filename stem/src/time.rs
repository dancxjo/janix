//! Time primitives for userspace.
//!
//! All time in Thing-OS derives from a single monotonic timebase.
//! Use `now()` to get the current `Instant`.

use crate::syscall;

// Re-export the ABI types for convenience
pub use abi::types::instant::{Duration, Instant};

/// Returns the current monotonic instant.
///
/// This is the primary way to get the current time in userspace.
/// The returned `Instant` is guaranteed to be monotonically increasing.
#[inline]
pub fn now() -> Instant {
    Instant::from_nanos(syscall::monotonic_ns())
}

/// Returns the current Unix time in nanoseconds.
///
/// Returns 0 if the system clock is not yet anchored.
pub fn now_unix_nanos() -> u64 {
    unsafe {
        match syscall::syscall6(abi::syscall::SYS_TIME_NOW, 0, 0, 0, 0, 0, 0) {
            x if x >= 0 => x as u64,
            _ => 0, // Error fallback (e.g., EAGAIN before anchoring)
        }
    }
}

/// Returns the current Unix time in seconds.
///
/// Returns 0 if the system clock is not yet anchored.
pub fn now_unix_seconds() -> u64 {
    now_unix_nanos() / 1_000_000_000
}

/// Returns raw monotonic nanoseconds since boot.
///
/// Prefer using `now()` which returns a type-safe `Instant`.
#[inline]
pub fn monotonic_ns() -> u64 {
    syscall::monotonic_ns()
}

/// Sleep for the specified duration.
///
/// Accepts both `abi::types::instant::Duration` and `core::time::Duration`.
pub fn sleep(duration: impl Into<Duration>) {
    syscall::sleep_ns(duration.into().as_nanos());
}

/// Sleep for the specified number of milliseconds.
pub fn sleep_ms(ms: u64) {
    syscall::sleep_ns(ms * 1_000_000);
}

/// Sleep for the specified number of nanoseconds.
#[inline]
pub fn sleep_ns(ns: u64) {
    syscall::sleep_ns(ns);
}
