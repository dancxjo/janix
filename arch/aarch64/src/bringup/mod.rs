pub mod context;
pub mod cpu;
pub mod fpu;
pub mod interrupts;
pub mod paging;
pub mod user;

pub use context::ArchContext;

/// Perform arch bring-up required before kernel can run:
/// - cpu state (fp/simd)
/// - interrupts vectors
/// - syscall entry mechanism
/// - paging prerequisites
///
/// # Safety
/// Must be called exactly once during boot.
pub unsafe fn init_early() {
    // CPU and FPU initialization is done in _start assembly.
    // Trap vectors are initialized in Bridge::init().
    // This function exists for API symmetry with x86_64.
}

/// Called after the kernel has its allocator / graph / etc.
/// Use sparingly; many arches won't need it.
///
/// # Safety
/// Must be called after init_early and kernel initialization.
pub unsafe fn init_late() {
    // Not needed on AArch64 currently.
    // This function exists for API symmetry with x86_64.
}

/// Returns the architecture name.
pub fn arch_name() -> &'static str {
    "aarch64"
}
