use core::arch::asm;
use kernel::memory::{PhysFrame, allocate_frame, phys_to_virt};

// AArch64 VMSA descriptors
const DESC_VALID: u64 = 1 << 0;
const DESC_TABLE: u64 = 1 << 1; // For L0-L2
const DESC_BLOCK: u64 = 0 << 1; // For L1-L2
const DESC_PAGE: u64 = 1 << 1; // For L3

const DESC_ACCESS_FLAG: u64 = 1 << 10;
const DESC_SH_INNER: u64 = 3 << 8;
const DESC_ATTR_DEV_NGNRNE: u64 = 0 << 2; // MAIR index 0 (Device-nGnRnE) usually?
// We need to check MAIR setup by Limine.
// Limine usually sets:
// 0: Device-nGnRnE
// 1: Normal Write-Back
// ...

// We'll assume MAIR index 0 is Device.
const ATTR_DEVICE: u64 = 0;

// TTBR1_EL1 points to L0 table (for 48-bit VA) or L1 (for 39-bit VA).
// Limine usually uses 4-level (48-bit) or 3-level?
// We can check TCR_EL1.T1SZ.

unsafe fn get_ttbr1() -> u64 {
    let val: u64;
    asm!("mrs {}, ttbr1_el1", out(reg) val);
    val & 0x0000_FFFF_FFFF_F000 // Mask out ASID etc
}

unsafe fn get_tcr() -> u64 {
    let val: u64;
    asm!("mrs {}, tcr_el1", out(reg) val);
    val
}

unsafe fn read_table(phys: u64, index: usize) -> u64 {
    let virt = phys_to_virt(phys);
    let ptr = virt as *const u64;
    *ptr.add(index)
}

unsafe fn write_table(phys: u64, index: usize, val: u64) {
    let virt = phys_to_virt(phys);
    let ptr = virt as *mut u64;
    *ptr.add(index) = val;
}

pub unsafe fn map_device_region(phys: u64, len: u64) {
    let tcr = get_tcr();
    let t1sz = (tcr >> 16) & 0x3F;
    let va_bits = 64 - t1sz;

    // Assuming 4KB granule (TG1 = 0 usually)

    let root_table = get_ttbr1();
    let hhdm_offset = kernel::memory::get_hhdm_offset();

    let start = phys;
    let end = phys + len;

    let mut curr = start;
    while curr < end {
        let virt = curr + hhdm_offset;
        map_page(root_table, virt, curr, va_bits);
        curr += 4096;
    }

    // TLB flush
    asm!("tlbi vmalle1");
    asm!("dsb ish");
    asm!("isb");
}

unsafe fn map_page(root_phys: u64, virt: u64, phys: u64, va_bits: u64) {
    // Calculate indices
    // L0: 47-39 (if 48 bit)
    // L1: 38-30
    // L2: 29-21
    // L3: 20-12

    let l0_idx = (virt >> 39) & 0x1FF;
    let l1_idx = (virt >> 30) & 0x1FF;
    let l2_idx = (virt >> 21) & 0x1FF;
    let l3_idx = (virt >> 12) & 0x1FF;

    let mut table_phys = root_phys;

    // Level 0 (only if va_bits > 39)
    if va_bits > 39 {
        table_phys = ensure_next_level(table_phys, l0_idx as usize);
    }

    // Level 1
    table_phys = ensure_next_level(table_phys, l1_idx as usize);

    // Level 2
    table_phys = ensure_next_level(table_phys, l2_idx as usize);

    // Level 3 (Page)
    let entry = phys | DESC_VALID | DESC_PAGE | DESC_ACCESS_FLAG | DESC_SH_INNER | ATTR_DEVICE;
    // Note: We overwrite existing mapping if any.
    write_table(table_phys, l3_idx as usize, entry);
}

unsafe fn ensure_next_level(table_phys: u64, index: usize) -> u64 {
    let entry = read_table(table_phys, index);
    if entry & DESC_VALID != 0 {
        // TODO: Check if it's a block mapping? Assuming tables for now.
        // Mask out attributes to get address
        entry & 0x0000_FFFF_FFFF_F000
    } else {
        // Allocate new table
        let frame = if let Some(f) = allocate_frame() {
            f
        } else {
            panic!("OOM during page table alloc");
        };
        let new_table_phys = frame.start_address;

        // Zero it

        let new_table_virt = phys_to_virt(new_table_phys);
        core::ptr::write_bytes(new_table_virt as *mut u8, 0, 4096);

        // Link it
        let new_entry = new_table_phys | DESC_VALID | DESC_TABLE;
        write_table(table_phys, index, new_entry);

        new_table_phys
    }
}
