use crate::memory::map::{MapPerms, MapResult};
use core::sync::atomic::{AtomicU64, Ordering};

/// Captured SATP from bootloader - contains kernel page tables
static KERNEL_SATP: AtomicU64 = AtomicU64::new(0);

/// Initialize MMU by capturing the current SATP from bootloader.
/// Called during kernel init before any address space switching.
pub fn init() {
    let satp: u64;
    unsafe {
        core::arch::asm!("csrr {}, satp", out(reg) satp);
    }
    KERNEL_SATP.store(satp, Ordering::Release);
}

/// Get the kernel SATP value (bootloader's page tables).
pub fn kernel_satp() -> u64 {
    KERNEL_SATP.load(Ordering::Acquire)
}

#[derive(Clone, Copy)]
pub struct AddressSpace {
    pub satp: u64,
}

impl AddressSpace {
    /// Create a new address space that shares kernel mappings.
    /// For now, reuses the bootloader's page tables entirely.
    pub fn new() -> MapResult<Self> {
        // Use the kernel's SATP to share kernel mappings
        // TODO: Eventually allocate a new page table and copy upper-half entries
        Ok(Self { satp: kernel_satp() })
    }

    /// Create from an existing SATP value.
    pub fn from_existing(satp: u64) -> Self {
        Self { satp }
    }

    /// Activate this address space by writing to SATP.
    pub fn activate(&self) {
        let current: u64;
        unsafe {
            core::arch::asm!("csrr {}, satp", out(reg) current);
        }
        
        // Only switch if different from current
        if current != self.satp {
            unsafe {
                core::arch::asm!("csrw satp, {}", in(reg) self.satp);
                core::arch::asm!("sfence.vma");
            }
        }
    }

    pub fn map(&mut self, _virt: u64, _phys: u64, _len: usize, _perms: MapPerms) -> MapResult<()> {
        // TODO: Implement actual page table manipulation
        Ok(())
    }
}
