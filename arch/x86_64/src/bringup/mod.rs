pub mod context;
pub mod cpu;
pub mod fpu;
pub mod gdt;
pub mod interrupts;
pub mod paging;
pub mod user;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ArchContext(pub [u64; 34]);

impl Default for ArchContext {
    fn default() -> Self {
        Self([0; 34])
    }
}

/// Perform arch bring-up required before kernel can run:
/// - cpu state (fp/simd)
/// - interrupts vectors
/// - syscall entry mechanism
/// - paging prerequisites
///
/// # Safety
/// Must be called exactly once during boot.
pub unsafe fn init_early() {
    // GDT, IDT, PIC, and syscall are initialized via Bridge::init().
    // This function exists for API symmetry with aarch64.
}

/// Called after the kernel has its allocator / graph / etc.
/// Use sparingly; many arches won't need it.
///
/// # Safety
/// Must be called after init_early and kernel initialization.
pub unsafe fn init_late() {
    // Not needed on x86_64 currently.
    // This function exists for API symmetry with aarch64.
}

/// Returns the architecture name.
pub fn arch_name() -> &'static str {
    "x86_64"
}
