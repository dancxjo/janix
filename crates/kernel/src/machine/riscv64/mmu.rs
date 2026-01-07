use crate::memory::map::{MapError, MapPerms, MapResult};
use alloc::boxed::Box;
use core::sync::atomic::{AtomicU64, Ordering};

/// Captured SATP from bootloader - contains kernel page tables
static KERNEL_SATP: AtomicU64 = AtomicU64::new(0);

/// Page size is 4KB
const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: usize = 12;

/// Sv39/48 PTE bits
const PTE_V: u64 = 1 << 0; // Valid
const PTE_R: u64 = 1 << 1; // Readable
const PTE_W: u64 = 1 << 2; // Writable
const PTE_X: u64 = 1 << 3; // Executable
const PTE_U: u64 = 1 << 4; // User-accessible
const PTE_G: u64 = 1 << 5; // Global
const PTE_A: u64 = 1 << 6; // Accessed
const PTE_D: u64 = 1 << 7; // Dirty

/// SATP modes (RV64)
const SATP_MODE_SV39: u64 = 8;
const SATP_MODE_SV48: u64 = 9;

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

        if perms.contains(MapPerms::READ) {
            flags |= PTE_R;
        }
        if perms.contains(MapPerms::WRITE) {
            flags |= PTE_W;
        }
        if perms.contains(MapPerms::EXEC) {
            flags |= PTE_X;
        }
        if user {
            flags |= PTE_U;
        }

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
    // Debug aid: capture the boot SATP so new address spaces can mirror kernel mappings.
    crate::serial::write(b"MMU: boot satp=");
    crate::serial::write_hex(satp);
    crate::serial::write(b"\n");
    KERNEL_SATP.store(satp, Ordering::Release);
}

/// Get the kernel SATP value (bootloader's page tables).
pub fn kernel_satp() -> u64 {
    KERNEL_SATP.load(Ordering::Acquire)
}

fn current_mode() -> u64 {
    let mode = kernel_satp() >> 60;
    if mode != 0 {
        mode
    } else {
        SATP_MODE_SV39
    }
}

fn levels() -> usize {
    match current_mode() {
        SATP_MODE_SV48 => 4, // L3 root
        _ => 3,              // Sv39 L2 root
    }
}

/// Get the root page table physical address from a SATP value
fn satp_to_root_phys(satp: u64) -> u64 {
    (satp & 0xFFF_FFFF_FFFF) << PAGE_SHIFT
}

/// Convert physical address to virtual (using HHDM offset)
fn phys_to_virt(phys: u64) -> u64 {
    let offset = super::ARCH_MACHINE_IMPL.hhdm_offset.load(Ordering::Relaxed);
    phys.wrapping_add(offset)
}

/// Convert virtual address to physical (for HHDM addresses)
fn virt_to_phys(virt: u64) -> u64 {
    let offset = super::ARCH_MACHINE_IMPL.hhdm_offset.load(Ordering::Relaxed);
    virt.wrapping_sub(offset)
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

        let kernel_satp = kernel_satp();
        if kernel_satp != 0 {
            crate::serial::write(b"AS: kernel satp=");
            crate::serial::write_hex(kernel_satp);
            crate::serial::write(b"\n");
            let kernel_root_phys = satp_to_root_phys(kernel_satp);
            let kernel_root_virt = phys_to_virt(kernel_root_phys);
            let kernel_root = unsafe { &*(kernel_root_virt as *const PageTable) };
            let new_root = unsafe { &mut *root_ptr };

            // Share the kernel's mappings wholesale so traps, stacks, and HHDM stay valid
            // after we switch SATP to a task address space.
            // Sv39 uses 256..512 for the upper half (kernel space).
            for i in 256..512 {
                new_root.entries[i] = kernel_root.entries[i];
            }
        }

        // Build SATP: Mode (match kernel) | ASID(0) | PPN
        let satp_mode = current_mode() << 60;
        let satp = satp_mode | (root_phys >> PAGE_SHIFT);

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

    pub fn unmap(&mut self, virt: u64, len: usize) -> MapResult<()> {
        if len == 0 {
            return Ok(());
        }

        let mut vaddr = virt & !(PAGE_SIZE as u64 - 1);
        let end = virt + len as u64;

        while vaddr < end {
            self.unmap_page(vaddr)?;
            vaddr += PAGE_SIZE as u64;
        }

        unsafe {
            core::arch::asm!("sfence.vma");
        }

        Ok(())
    }

    /// Map a single 4KB page.
    fn map_page(&mut self, virt: u64, phys: u64, perms: MapPerms, user: bool) -> MapResult<()> {
        let mut table_phys = self.root_phys;
        let mut level = levels() - 1;

        loop {
            let table_virt = phys_to_virt(table_phys);
            let table = unsafe { &mut *(table_virt as *mut PageTable) };
            let idx = vpn(virt, level);

            if level == 0 {
                table.entries[idx] = PageTableEntry::new_leaf(phys, perms, user);
                break;
            }

            if !table.entries[idx].is_valid() {
                let next = Box::new(PageTable::new());
                let next_ptr = Box::into_raw(next);
                let next_phys = virt_to_phys(next_ptr as u64);
                table.entries[idx] = PageTableEntry::new_table(next_phys);
            }

            table_phys = table.entries[idx].phys_addr();
            level -= 1;
        }

        Ok(())
    }

    fn unmap_page(&mut self, virt: u64) -> MapResult<()> {
        let mut table_phys = self.root_phys;
        let mut level = levels() - 1;

        loop {
            let table_virt = phys_to_virt(table_phys);
            let table = unsafe { &mut *(table_virt as *mut PageTable) };
            let idx = vpn(virt, level);
            let entry = table.entries[idx];

            if !entry.is_valid() {
                return Ok(());
            }

            if entry.is_leaf() {
                table.entries[idx] = PageTableEntry::empty();
                return Ok(());
            }

            if level == 0 {
                return Ok(());
            }

            table_phys = entry.phys_addr();
            level -= 1;
        }
    }

    pub fn user_range_end(&self) -> u64 {
        0x8000_0000_0000_0000
    }

    pub fn probe_user_range(&self, start: u64, len: usize, perms: MapPerms) -> bool {
        if len == 0 {
            return true;
        }
        let Some(end) = start.checked_add(len as u64) else {
            return false;
        };
        let mut addr = start & !(PAGE_SIZE as u64 - 1);
        while addr < end {
            if !self.probe_user_page(addr, perms) {
                return false;
            }
            addr = addr.saturating_add(PAGE_SIZE as u64);
        }
        true
    }

    fn probe_user_page(&self, virt: u64, perms: MapPerms) -> bool {
        let mut table_phys = self.root_phys;
        let mut level = levels() - 1;

        loop {
            let table_virt = phys_to_virt(table_phys);
            let table = unsafe { &*(table_virt as *const PageTable) };
            let idx = vpn(virt, level);
            let entry = table.entries[idx];

            if !entry.is_valid() {
                return false;
            }

            if entry.is_leaf() {
                return entry_permits(entry, perms);
            }

            if level == 0 {
                return false;
            }

            table_phys = entry.phys_addr();
            level -= 1;
        }
    }
}

fn entry_permits(entry: PageTableEntry, perms: MapPerms) -> bool {
    let flags = entry.0;
    if flags & PTE_U == 0 {
        return false;
    }
    if perms.contains(MapPerms::READ) && flags & PTE_R == 0 {
        return false;
    }
    if perms.contains(MapPerms::WRITE) && flags & PTE_W == 0 {
        return false;
    }
    if perms.contains(MapPerms::EXEC) && flags & PTE_X == 0 {
        return false;
    }
    true
}
