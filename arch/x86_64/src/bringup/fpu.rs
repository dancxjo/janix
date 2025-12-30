//! FPU initialization for x86_64.
//!
//! This module forwards to SIMD initialization for API symmetry.

/// Initialize the FPU/SIMD unit.
///
/// # Safety
/// Must be called after CR0/CR4 are configured.
pub unsafe fn init_fpu() {
    // FPU/SSE is initialized in _start assembly and simd::init_simd().
    // This function exists for API symmetry with aarch64.
    crate::simd::init_simd();
}
