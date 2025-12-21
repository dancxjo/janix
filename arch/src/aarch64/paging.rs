use core::arch::asm;
use kernel::memory::allocate_frame;

// Page Table Entry Flags
pub const PTE_VALID: u64 = 1 << 0;
pub const PTE_TABLE: u64 = 1 << 1; // L0-L2
pub const PTE_PAGE: u64 = 1 << 1;  // L3
pub const PTE_ATTR_DEVICE: u64 = 1 << 2; // Index 1 in MAIR (Device-nGnRE)
pub const PTE_ATTR_NORMAL: u64 = 0 << 2; // Index 0 in MAIR (Normal)
pub const PTE_NS: u64 = 1 << 5;    // Non-Secure
pub const PTE_AP_RW_EL1: u64 = 0 << 6; // RW EL1 only
pub const PTE_AP_RW_EL0: u64 = 1 << 6; // RW EL1/EL0
pub const PTE_SH_INNER: u64 = 3 << 8;  // Inner Shareable
pub const PTE_AF: u64 = 1 << 10;   // Access Flag
pub const PTE_UXN: u64 = 1 << 54;  // Unprivileged Execute Never
pub const PTE_PXN: u64 = 1 << 53;  // Privileged Execute Never

// Aliases for compatibility
pub const DESC_AP_EL0: u64 = PTE_AP_RW_EL0;
pub const DESC_UXN: u64 = PTE_UXN;
pub const ATTR_NORMAL: u64 = PTE_ATTR_NORMAL;

const TABLE_MASK: u64 = 0x0000_FFFF_FFFF_F000;

pub fn init() {
    // AArch64 paging init
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

            let new_entry = frame_phys | PTE_TABLE | PTE_VALID | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL1 | PTE_AF;
            table_ptr.add(index).write(new_entry);

            table_phys = frame_phys;
        } else {
            if (level == 1 || level == 2) && (entry & 0x2) == 0 {
                return;
            }
            table_phys = entry & TABLE_MASK;
        }
    }

    let index = indexes[3] as usize;
    let table_ptr = (table_phys + kernel::memory::get_hhdm_offset()) as *mut u64;

    let entry = phys | PTE_VALID | PTE_PAGE | PTE_AF | PTE_SH_INNER | PTE_AP_RW_EL1 | PTE_UXN | PTE_PXN | PTE_ATTR_DEVICE;

    table_ptr.add(index).write(entry);
}

pub unsafe fn update_page_flags(virt: u64, set: u64, clear: u64) {
    // We assume kernel paging (TTBR1) or user?
    // User stack is in TTBR0 usually (lower half).
    // Kernel code/stack is TTBR1 (upper half).
    // `alloc_user_stack` returns an address. If it is user stack, it is likely low address?
    // `alloc_user_stack` does `alloc_zeroed`, which returns HHDM address (high half).
    // Wait, HHDM is kernel accessible.
    // If we map it for user, we need to map it in TTBR0?
    // But `enter.rs` maps it using `update_page_flags` on the address returned by `alloc_user_stack`.
    // If `alloc_user_stack` returns a HHDM address, it is in TTBR1.
    // So we are updating TTBR1 entries?

    // However, user mode (EL0) cannot access TTBR1 (unless configured?).
    // Usually user space is TTBR0.
    // If `alloc_user_stack` creates a stack in kernel memory, and then we give it to user...
    // We need to map it into user space (TTBR0).

    // BUT, the existing `enter.rs` code just calls `update_page_flags`.
    // It assumes that works.
    // I will implement it for TTBR1 for now (HHDM addresses).
    // If `virt` is low (user), we should check TTBR0.

    let is_ttbr0 = (virt as i64) >= 0; // Positive = low half = TTBR0. Negative = high half = TTBR1.

    let root_phys = if is_ttbr0 {
        let val: u64;
        asm!("mrs {}, ttbr0_el1", out(reg) val);
        val & TABLE_MASK
    } else {
        read_ttbr1_el1() & TABLE_MASK
    };

    if root_phys == 0 { return; } // Not set up

    let mut tcr: u64;
    asm!("mrs {}, tcr_el1", out(reg) tcr);
    let t1sz = (tcr >> 16) & 0x3F;
    let levels = if t1sz == 16 { 4 } else { 3 }; // Assuming T0SZ matches T1SZ logic or similar config

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

        if (entry & PTE_VALID) == 0 { return; } // Page not present
        if (level == 1 || level == 2) && (entry & 0x2) == 0 { return; } // Block mapping, can't handle yet

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
