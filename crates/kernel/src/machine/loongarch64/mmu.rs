//! LoongArch64 MMU implementation
//! 
//! Implements 4-level page tables using PGDL/PGDH CSRs for address space management.

use crate::memory::map::{MapPerms, MapResult, MapError};
use alloc::alloc::{Layout, alloc_zeroed};
use core::sync::atomic::{AtomicU64, Ordering};

/// Captured kernel PGDL from bootloader - contains kernel page tables
static KERNEL_PGDL: AtomicU64 = AtomicU64::new(0);

// CSR numbers for LoongArch64 MMU (for documentation)
// CSR_PGDL = 0x19 - Page Global Directory for Lower half (user)
// CSR_PGDH = 0x1a - Page Global Directory for Higher half (kernel)

/// Page table entry flags for LoongArch64
/// 
/// PTE format: [PFN(bits 12+)] [FLAGS(bits 0-11)]
/// - V (bit 0): Valid
/// - D (bit 1): Dirty (writable)
/// - PLV (bits 2-3): Privilege Level (0=kernel, 3=user)
/// - MAT (bits 4-5): Memory Attribute Type (0=coherent, 1=strong order)
/// - G (bit 6): Global
/// - P (bit 7): Present (page vs table)
/// - W (bit 8): Writable
/// - NR (bit 61): No Read
/// - NX (bit 62): No Execute
const PTE_V: u64 = 1 << 0;    // Valid
const PTE_D: u64 = 1 << 1;    // Dirty
const PTE_PLV_USER: u64 = 3 << 2;  // Privilege level 3 (user)
const PTE_MAT_CC: u64 = 1 << 4;    // Coherent Cached
const PTE_P: u64 = 1 << 7;    // Page (not table pointer)
const PTE_W: u64 = 1 << 8;    // Writable
const PTE_NX: u64 = 1 << 62;  // No Execute

/// Initialize MMU by capturing the current PGDL from bootloader.
/// Called during kernel init before any address space switching.
pub fn init() {
    let pgdl: u64;
    unsafe {
        core::arch::asm!("csrrd {}, 0x19", out(reg) pgdl);  // CSR_PGDL
    }
    KERNEL_PGDL.store(pgdl, Ordering::Release);
}

/// Get the kernel PGDL value (bootloader's page tables).
pub fn kernel_pgdl() -> u64 {
    KERNEL_PGDL.load(Ordering::Acquire)
}

#[derive(Clone, Copy)]
pub struct AddressSpace {
    /// Physical address of the Page Global Directory (L0 table)
    pub pgd: u64,
}

impl AddressSpace {
    /// Create a new address space with its own page tables.
    /// Copies kernel mappings from the current PGDL's upper half.
    pub fn new() -> MapResult<Self> {
        // Allocate L0 table (PGD)
        let phys = unsafe { alloc_subtable()? };
        
        // Copy kernel half (upper 256 entries) from kernel PGDL
        let kernel_pgd = kernel_pgdl();
        if kernel_pgd != 0 {
            let new_table = phys_to_virt(phys) as *mut u64;
            let kernel_table = phys_to_virt(kernel_pgd) as *const u64;
            
            // Copy upper half (entries 256-511) for kernel space
            unsafe {
                for i in 256..512 {
                    let entry = kernel_table.add(i).read();
                    new_table.add(i).write(entry);
                }
            }
        }
        
        Ok(Self { pgd: phys })
    }

    /// Create from an existing PGD physical address.
    pub fn from_existing(pgd: u64) -> Self {
        Self { pgd }
    }

    /// Activate this address space by writing to PGDL CSR.
    pub fn activate(&self) {
        let current: u64;
        unsafe {
            core::arch::asm!("csrrd {}, 0x19", out(reg) current);  // Read PGDL
        }
        
        // Only switch if different from current
        if current != self.pgd {
            unsafe {
                core::arch::asm!("csrwr {}, 0x19", in(reg) self.pgd);  // Write PGDL
                // Invalidate TLB
                core::arch::asm!("invtlb 0x0, $r0, $r0");
            }
        }
    }

    /// Map a virtual address range to physical addresses.
    pub fn map(&mut self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        let pages = (len + 4095) / 4096;
        for i in 0..pages {
            let offset = i as u64 * 4096;
            unsafe {
                self.map_page(virt + offset, phys + offset, perms)?;
            }
        }
        
        // Invalidate TLB after mapping
        unsafe {
            core::arch::asm!("invtlb 0x0, $r0, $r0");
        }
        
        Ok(())
    }

    unsafe fn map_page(&mut self, virt: u64, phys: u64, perms: MapPerms) -> MapResult<()> {
        // 4-level page table walk
        // VA structure (48-bit): [L0:9][L1:9][L2:9][L3:9][offset:12]
        
        let l0 = phys_to_virt(self.pgd) as *mut u64;
        let l0_idx = ((virt >> 39) & 0x1ff) as usize;
        let l1 = ensure_table(l0, l0_idx)?;
        
        let l1_idx = ((virt >> 30) & 0x1ff) as usize;
        let l2 = ensure_table(l1, l1_idx)?;
        
        let l2_idx = ((virt >> 21) & 0x1ff) as usize;
        let l3 = ensure_table(l2, l2_idx)?;
        
        let l3_idx = ((virt >> 12) & 0x1ff) as usize;
        let entry_ptr = l3.add(l3_idx);
        
        // Build page entry with flags
        let mut desc = (phys & !0xfff) | PTE_V | PTE_P;
        
        // Memory attribute: Coherent Cached for normal memory
        if !perms.contains(MapPerms::DEVICE) {
            desc |= PTE_MAT_CC;
        }
        
        // User access
        if perms.contains(MapPerms::USER) {
            desc |= PTE_PLV_USER;
        }
        
        // Writable
        if perms.contains(MapPerms::WRITE) {
            desc |= PTE_W | PTE_D;
        }
        
        // No Execute
        if !perms.contains(MapPerms::EXEC) {
            desc |= PTE_NX;
        }
        
        entry_ptr.write(desc);
        
        Ok(())
    }
}

/// Allocate a zeroed 4KB page table
unsafe fn alloc_subtable() -> MapResult<u64> {
    let layout = Layout::from_size_align(4096, 4096).map_err(|_| MapError::Oom)?;
    let ptr = alloc_zeroed(layout);
    if ptr.is_null() {
        return Err(MapError::Oom);
    }
    // Convert to physical address
    Ok(crate::machine::machine().virt_to_phys(ptr as u64))
}

/// Convert physical address to virtual using HHDM
fn phys_to_virt(phys: u64) -> u64 {
    crate::boot::get_boot_ctx().hhdm_offset.wrapping_add(phys)
}

/// Ensure a table entry points to a valid subtable, allocating if needed
unsafe fn ensure_table(table: *mut u64, index: usize) -> MapResult<*mut u64> {
    let entry_ptr = table.add(index);
    let entry = entry_ptr.read();
    
    if entry & PTE_V == 0 {
        // Invalid, allocate new subtable
        let new_table_phys = alloc_subtable()?;
        // Table entry: addr | V (tables don't have P bit set)
        let new_entry = (new_table_phys & !0xfff) | PTE_V;
        entry_ptr.write(new_entry);
        Ok(phys_to_virt(new_table_phys) as *mut u64)
    } else {
        // Extract physical address from existing entry
        let phys = entry & !0xfff;
        Ok(phys_to_virt(phys) as *mut u64)
    }
}
