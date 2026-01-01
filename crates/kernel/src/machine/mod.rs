//! Machine interface for architecture-specific I/O.
//!
//! Provides a single choke point the kernel uses for early console output and
//! MMIO mappings. Architectures install their implementation during boot.

use bitflags::bitflags;

/// Physical MMIO range.
pub struct MmioRange {
    pub phys: u64,
    pub len: usize,
}

/// Virtual mapping returned by the machine backend.
pub struct MmioMapping {
    pub virt: u64,
    pub len: usize,
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct MmioFlags: u32 {
        const DEVICE = 1 << 0;
        const UNCACHED = 1 << 1;
        const READ = 1 << 2;
        const WRITE = 1 << 3;
    }
}

/// Machine operations exposed to the kernel.
pub trait Machine: Sync {
    /// Write bytes to the early console.
    fn console_write(&self, bytes: &[u8]) -> usize;

    /// Map a physical MMIO range and return a virtual mapping.
    /// Implementations must not assume an HHDM covers device ranges.
    fn mmio_map(&self, range: MmioRange, flags: MmioFlags) -> Option<MmioMapping>;
}

static mut MACHINE: Option<&'static dyn Machine> = None;

/// Install the architecture-provided machine implementation.
///
/// Safety: must be called exactly once during boot by the architecture code
/// before any machine() calls occur.
pub unsafe fn install(machine: &'static dyn Machine) {
    MACHINE = Some(machine);
}

/// Access the installed machine implementation.
pub fn machine() -> &'static dyn Machine {
    unsafe { MACHINE.expect("machine not installed") }
}
