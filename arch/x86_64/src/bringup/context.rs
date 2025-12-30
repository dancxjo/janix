//! Thread context initialization for x86_64.
//!
//! Stub on x86_64; required for API symmetry with AArch64.
//! The actual context initialization lives in bridge/mod.rs.

/// Initialize thread context subsystem.
///
/// On x86_64, this is a no-op as context is managed directly.
pub fn init_context() {
    // Stub on x86_64 until implemented; required for API symmetry.
}
