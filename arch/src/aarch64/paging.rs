use core::arch::asm;
use kernel::memory::allocate_frame;

// Page Table Entry Flags (AArch64 VMSA)
pub const PTE_VALID: u64 = 1 << 0;
pub const PTE_TABLE: u64 = 1 << 1; // L0-L2 points to next-level table
pub const PTE_PAGE: u64 = 1 << 1;  // L3 page descriptor (same bit pattern as TABLE at L3)

// AttrIndx[2:0] lives at [4:2] in the descriptor.
// This code assumes MAIR is set up such that these indices mean:
// 0 = Normal (Write-Back)
// 1 = Device (nGnRE / nGnRnE depending on platform)
// If Limine (or your early init) uses different MAIR indices, adjust these.
pub const PTE_ATTR_NORMAL: u64 = 0 << 2; // AttrIndx = 0
pub const PTE_ATTR_DEVICE: u64 = 1 << 2; // AttrIndx = 1

pub const PTE_NS: u64 = 1 << 5;          // Non-secure
pub const PTE_AP_RW_EL1: u64 = 0 << 6;   // RW at EL1 only
pub const PTE_AP_RW_EL0: u64 = 1 << 6;   // RW at EL1/EL0
pub const PTE_SH_INNER: u64 = 3 << 8;    // Inner shareable
pub const PTE_AF: u64 = 1 << 10;         // Access Flag
pub const PTE_PXN: u64 = 1 << 53;        // Privileged Execute Never
pub const PTE_UXN: u64 = 1 << 54;        // Unprivileged Execute Never

// Aliases for compatibility with older naming used elsewhere.
pub const DESC_AP_EL0: u64 = PTE_AP_RW_EL0;
pub const DESC_UXN: u64 = PTE_UXN;
pub const ATTR_NORMAL: u64 = PTE_ATTR_NORMAL;

const TABLE_MASK: u64 = 0x0000_FFFF_FFFF_F000;

pub fn init() {
    // AArch64 paging init
    // (Kept as a stub here, since this file primarily provides helpers used elsewhere.)
}

unsafe fn read_ttbr1_el1() -> u64 {
    let val: u64;
    asm!("mrs {}, ttbr1_el1", out(reg) val);
    val
}

unsafe fn flush_tlb() {
    asm!("tlbi vmalle1is");
    asm!("dsb ish");
    asm!("isb");
}

pub unsafe fn map_device_region(phys: u64, size: usize, hhdm_offset: u64) {
    let start = phys;
    let end = phys + size as u64;

    // Align start/end to 4KB
    let start_page = start & !0xFFF;
    let end_page = (end + 0xFFF) & !0xFFF;

    let ttbr1 = read_ttbr1_el1();

    let mut tcr: u64;
    asm!("mrs {}, tcr_el1", out(reg) tcr);
    let t1sz = (tcr >> 16) & 0x3F;
    let levels = if t1sz == 16 { 4 } else { 3 };

    let root_table_phys = ttbr1 & TABLE_MASK;

    let mut current_addr = start_page;
    while current_addr < end_page {
        let virt = current_addr + hhdm_offset;
        map_page(root_table_phys, current_addr, virt, levels);
        current_addr += 4096;
    }

    flush_tlb();
}

unsafe fn map_page(root_table_phys: u64, phys: u64, virt: u64, levels: usize) {
    let indexes = [
        (virt >> 39) & 0x1FF, // L0
        (virt >> 30) & 0x1FF, // L1
        (virt >> 21) & 0x1FF, // L2
        (virt >> 12) & 0x1FF, // L3
    ];

    let start_level = if levels == 4 { 0 } else { 1 };

    let mut table_phys = root_table_phys;

    for level in start_level..3 {
        let index = indexes[level] as usize;
        let table_ptr = (table_phys + kernel::memory::get_hhdm_offset()) as *mut u64;
        let entry = table_ptr.add(index).read();

        if (entry & PTE_VALID) == 0 {
            let frame = allocate_frame().expect("OOM mapping device");
            let frame_phys = frame.start_address;

            let frame_ptr = (frame_phys + kernel::memory::get_hhdm_offset()) as *mut u64;
            frame_ptr.write_bytes(0, 512);

            // Table descriptor for next level.
            // NOTE: For table descriptors, the AttrIndx bits are ignored; AF is also not meaningful.
            // Keeping SH/AP here is harmless but not required. We keep it simple and consistent.
            let new_entry = frame_phys | PTE_TABLE | PTE_VALID;
            table_ptr.add(index).write(new_entry);

            table_phys = frame_phys;
        } else {
            // If this is a block mapping at L1/L2, we can't descend.
            if (level == 1 || level == 2) && (entry & 0x2) == 0 {
                return;
            }
            table_phys = entry & TABLE_MASK;
        }
    }

    let index = indexes[3] as usize;
    let table_ptr = (table_phys + kernel::memory::get_hhdm_offset()) as *mut u64;

    let entry = phys
        | PTE_VALID
        | PTE_PAGE
        | PTE_AF
        | PTE_SH_INNER
        | PTE_AP_RW_EL1
        | PTE_UXN
        | PTE_PXN
        | PTE_ATTR_DEVICE;

    table_ptr.add(index).write(entry);
}

pub unsafe fn update_page_flags(virt: u64, set: u64, clear: u64) {
    // We assume kernel paging (TTBR1) or user (TTBR0) based on address half.
    // Positive (low half) => TTBR0, negative (high half) => TTBR1.
    let is_ttbr0 = (virt as i64) >= 0;

    let root_phys = if is_ttbr0 {
        let val: u64;
        asm!("mrs {}, ttbr0_el1", out(reg) val);
        val & TABLE_MASK
    } else {
        read_ttbr1_el1() & TABLE_MASK
    };

    if root_phys == 0 {
        return;
    }

    let mut tcr: u64;
    asm!("mrs {}, tcr_el1", out(reg) tcr);
    let t1sz = (tcr >> 16) & 0x3F;
    let levels = if t1sz == 16 { 4 } else { 3 };

    let indexes = [
        (virt >> 39) & 0x1FF,
        (virt >> 30) & 0x1FF,
        (virt >> 21) & 0x1FF,
        (virt >> 12) & 0x1FF,
    ];

    let start_level = if levels == 4 { 0 } else { 1 };
    let mut table_phys = root_phys;

    for level in start_level..3 {
        let index = indexes[level] as usize;
        let table_ptr = (table_phys + kernel::memory::get_hhdm_offset()) as *mut u64;
        let entry = table_ptr.add(index).read();

        if (entry & PTE_VALID) == 0 {
            return; // Not present
        }
        if (level == 1 || level == 2) && (entry & 0x2) == 0 {
            return; // Block mapping, not handled
        }

        table_phys = entry & TABLE_MASK;
    }

    // L3
    let index = indexes[3] as usize;
    let table_ptr = (table_phys + kernel::memory::get_hhdm_offset()) as *mut u64;
    let mut entry = table_ptr.add(index).read();

    if (entry & PTE_VALID) != 0 {
        entry = (entry & !clear) | set;
        table_ptr.add(index).write(entry);
        flush_tlb(); // Naive full flush
    }
}
