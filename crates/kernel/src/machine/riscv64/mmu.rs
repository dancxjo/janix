use crate::memory::map::{MapError, MapPerms, MapResult};
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::boxed::Box;

/// Captured SATP from bootloader - contains kernel page tables
static KERNEL_SATP: AtomicU64 = AtomicU64::new(0);

/// Page size is 4KB
const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: usize = 12;

/// Sv39 PTE bits
const PTE_V: u64 = 1 << 0;  // Valid
const PTE_R: u64 = 1 << 1;  // Readable
const PTE_W: u64 = 1 << 2;  // Writable
const PTE_X: u64 = 1 << 3;  // Executable
const PTE_U: u64 = 1 << 4;  // User-accessible
const PTE_G: u64 = 1 << 5;  // Global
const PTE_A: u64 = 1 << 6;  // Accessed
const PTE_D: u64 = 1 << 7;  // Dirty

/// SATP mode for Sv39
const SATP_MODE_SV39: u64 = 8 << 60;

/// A single page table entry (64 bits)
#[derive(Clone, Copy)]
#[repr(transparent)]
struct PageTableEntry(u64);

impl PageTableEntry {
    const fn empty() -> Self {
        Self(0)
    }
    
    fn is_valid(&self) -> bool {
        self.0 & PTE_V != 0
    }
    
    fn is_leaf(&self) -> bool {
        // Leaf if any of R, W, X is set
        self.0 & (PTE_R | PTE_W | PTE_X) != 0
    }
    
    /// Get the physical page number (bits 10-53)
    fn ppn(&self) -> u64 {
        (self.0 >> 10) & 0xFFF_FFFF_FFFF // 44 bits
    }
    
    /// Get the physical address this PTE points to
    fn phys_addr(&self) -> u64 {
        self.ppn() << PAGE_SHIFT
    }
    
    /// Create a non-leaf PTE pointing to next page table
    fn new_table(phys_addr: u64) -> Self {
        let ppn = phys_addr >> PAGE_SHIFT;
        Self((ppn << 10) | PTE_V)
    }
    
    /// Create a leaf PTE mapping a page
    fn new_leaf(phys_addr: u64, perms: MapPerms, user: bool) -> Self {
        let ppn = phys_addr >> PAGE_SHIFT;
        let mut flags = PTE_V | PTE_A | PTE_D; // Valid, Accessed, Dirty
        
        if perms.contains(MapPerms::READ) { flags |= PTE_R; }
        if perms.contains(MapPerms::WRITE) { flags |= PTE_W; }
        if perms.contains(MapPerms::EXEC) { flags |= PTE_X; }
        if user { flags |= PTE_U; }
        
        Self((ppn << 10) | flags)
    }
}

/// A page table (512 entries, 4KB aligned)
#[repr(C, align(4096))]
struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    fn new() -> Self {
        Self {
            entries: [PageTableEntry::empty(); 512],
        }
    }
}

/// Initialize MMU by capturing the current SATP from bootloader.
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

/// Get the root page table physical address from a SATP value
fn satp_to_root_phys(satp: u64) -> u64 {
    (satp & 0xFFF_FFFF_FFFF) << PAGE_SHIFT
}

/// Convert physical address to virtual (using HHDM offset)
fn phys_to_virt(phys: u64) -> u64 {
    // HHDM base from limine is typically 0xffff800000000000
    phys.wrapping_add(0xffff_8000_0000_0000)
}

/// Convert virtual address to physical (for HHDM addresses)
fn virt_to_phys(virt: u64) -> u64 {
    virt.wrapping_sub(0xffff_8000_0000_0000)
}

/// Extract VPN[level] from virtual address (level 0, 1, or 2)
fn vpn(virt: u64, level: usize) -> usize {
    ((virt >> (PAGE_SHIFT + level * 9)) & 0x1FF) as usize
}

#[derive(Clone, Copy)]
pub struct AddressSpace {
    /// Root page table physical address (L2)
    root_phys: u64,
    /// SATP value for this address space
    pub satp: u64,
}

impl AddressSpace {
    /// Create a new address space with kernel mappings shared.
    pub fn new() -> MapResult<Self> {
        // Allocate a new root page table
        let root = Box::new(PageTable::new());
        let root_ptr = Box::into_raw(root);
        let root_virt = root_ptr as u64;
        let root_phys = virt_to_phys(root_virt);
        
        // Copy kernel page table entries (upper half: VPN[2] >= 256)
        let kernel_satp = kernel_satp();
        if kernel_satp != 0 {
            let kernel_root_phys = satp_to_root_phys(kernel_satp);
            let kernel_root_virt = phys_to_virt(kernel_root_phys);
            let kernel_root = unsafe { &*(kernel_root_virt as *const PageTable) };
            let new_root = unsafe { &mut *root_ptr };
            
            // Copy upper half (kernel space: indices 256-511)
            for i in 256..512 {
                new_root.entries[i] = kernel_root.entries[i];
            }
        }
        
        // Build SATP: Mode(Sv39) | ASID(0) | PPN
        let satp = SATP_MODE_SV39 | (root_phys >> PAGE_SHIFT);
        
        Ok(Self { root_phys, satp })
    }

    /// Create from an existing SATP value.
    pub fn from_existing(satp: u64) -> Self {
        let root_phys = satp_to_root_phys(satp);
        Self { root_phys, satp }
    }

    /// Activate this address space by writing to SATP.
    pub fn activate(&self) {
        let current: u64;
        unsafe {
            core::arch::asm!("csrr {}, satp", out(reg) current);
        }
        
        if current != self.satp {
            unsafe {
                core::arch::asm!("csrw satp, {}", in(reg) self.satp);
                core::arch::asm!("sfence.vma");
            }
        }
    }

    /// Map a virtual address range to physical memory.
    pub fn map(&mut self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        let mut vaddr = virt & !(PAGE_SIZE as u64 - 1); // Page-align
        let mut paddr = phys & !(PAGE_SIZE as u64 - 1);
        let end = virt + len as u64;
        
        // Determine if this is a user mapping (lower half of address space)
        let is_user = virt < 0x8000_0000_0000_0000;
        
        while vaddr < end {
            self.map_page(vaddr, paddr, perms, is_user)?;
            vaddr += PAGE_SIZE as u64;
            paddr += PAGE_SIZE as u64;
        }
        
        // Flush TLB after mapping
        unsafe {
            core::arch::asm!("sfence.vma");
        }
        
        Ok(())
    }
    
    /// Map a single 4KB page.
    fn map_page(&mut self, virt: u64, phys: u64, perms: MapPerms, user: bool) -> MapResult<()> {
        let root_virt = phys_to_virt(self.root_phys);
        let root = unsafe { &mut *(root_virt as *mut PageTable) };
        
        // Walk page table, creating entries as needed
        // Level 2 (root)
        let idx2 = vpn(virt, 2);
        if !root.entries[idx2].is_valid() {
            // Allocate L1 page table
            let l1 = Box::new(PageTable::new());
            let l1_ptr = Box::into_raw(l1);
            let l1_phys = virt_to_phys(l1_ptr as u64);
            root.entries[idx2] = PageTableEntry::new_table(l1_phys);
        }
        
        let l1_phys = root.entries[idx2].phys_addr();
        let l1_virt = phys_to_virt(l1_phys);
        let l1 = unsafe { &mut *(l1_virt as *mut PageTable) };
        
        // Level 1
        let idx1 = vpn(virt, 1);
        if !l1.entries[idx1].is_valid() {
            // Allocate L0 page table
            let l0 = Box::new(PageTable::new());
            let l0_ptr = Box::into_raw(l0);
            let l0_phys = virt_to_phys(l0_ptr as u64);
            l1.entries[idx1] = PageTableEntry::new_table(l0_phys);
        }
        
        let l0_phys = l1.entries[idx1].phys_addr();
        let l0_virt = phys_to_virt(l0_phys);
        let l0 = unsafe { &mut *(l0_virt as *mut PageTable) };
        
        // Level 0 (leaf)
        let idx0 = vpn(virt, 0);
        l0.entries[idx0] = PageTableEntry::new_leaf(phys, perms, user);
        
        Ok(())
    }
}
