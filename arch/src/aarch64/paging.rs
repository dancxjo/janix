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
pub const ATTR_NORMAL: u64 = 1 << 2; // MAIR index 1 (Normal Write-Back)

pub const DESC_AP_EL0: u64 = 1 << 6; // AP[1]=1, AP[2]=0 => RW at EL0
pub const DESC_UXN: u64 = 1 << 54; // Unprivileged Execute Never

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
    // Diagnostic: print HHDM offset and root table so we can verify mapping
    let hhdm_offset = kernel::memory::get_hhdm_offset();
    kernel::println!(
        "map_device_region: phys={:#x} len={:#x} root_table={:#x} hhdm_offset={:#x} t1sz={} va_bits={}",
        phys,
        len,
        root_table,
        hhdm_offset,
        t1sz,
        va_bits
    );
    let hhdm_offset = kernel::memory::get_hhdm_offset();

    let start = phys;
    let end = phys + len;

    let mut curr = start;
    while curr < end {
        let virt = curr + hhdm_offset;
        // Print the first mapping for visibility
        if curr == start {
            kernel::println!(
                "map_device_region: mapping first page phys={:#x} -> virt={:#x}",
                curr,
                virt
            );
        }
        map_page(root_table, virt, curr, va_bits);
        curr += 4096;
    }

    // TLB flush
    asm!("tlbi vmalle1");
    asm!("dsb ish");
    asm!("isb");
}

pub unsafe fn update_page_flags(virt: u64, flags_to_set: u64, flags_to_clear: u64) {
    let tcr = get_tcr();
    let t1sz = (tcr >> 16) & 0x3F;
    let va_bits = 64 - t1sz;

    let root = get_ttbr1();

    let l0_idx = (virt >> 39) & 0x1FF;
    let l1_idx = (virt >> 30) & 0x1FF;
    let l2_idx = (virt >> 21) & 0x1FF;
    let l3_idx = (virt >> 12) & 0x1FF;

    let mut table = root;

    if va_bits > 39 {
        let entry = read_table(table, l0_idx as usize);
        if entry & DESC_VALID == 0 { return; }
        table = entry & 0x0000_FFFF_FFFF_F000;
    }

    let entry = read_table(table, l1_idx as usize);
    if entry & DESC_VALID == 0 { return; }
    table = entry & 0x0000_FFFF_FFFF_F000;

    let entry = read_table(table, l2_idx as usize);
    if entry & DESC_VALID == 0 { return; }
    table = entry & 0x0000_FFFF_FFFF_F000;

    // L3
    let entry = read_table(table, l3_idx as usize);
    if entry & DESC_VALID != 0 {
        let new_entry = (entry & !flags_to_clear) | flags_to_set;
        write_table(table, l3_idx as usize, new_entry);

        // TLB flush
        asm!("tlbi vmalle1");
        asm!("dsb ish");
        asm!("isb");
    }
}

/// Diagnostic helper: walk page tables for `virt` using current TTBR1 and
/// print entries found at each level. Call from kernel context to verify
/// whether a virtual address is actually mapped.
pub unsafe fn dump_page_table_for(virt: u64) {
    let root = get_ttbr1();
    let l0_idx = (virt >> 39) & 0x1FF;
    let l1_idx = (virt >> 30) & 0x1FF;
    let l2_idx = (virt >> 21) & 0x1FF;
    let l3_idx = (virt >> 12) & 0x1FF;

    kernel::println!("dump_page_table_for: virt={:#x} root={:#x}", virt, root);
    let mut table = root;
    if (read_table(table, l0_idx as usize) & DESC_VALID) == 0 {
        kernel::println!(
            "L0 entry not present: idx={} val={:#x}",
            l0_idx,
            read_table(table, l0_idx as usize)
        );
        return;
    }
    let e0 = read_table(table, l0_idx as usize);
    kernel::println!("L0[{}] = {:#x}", l0_idx, e0);
    table = e0 & 0x0000_FFFF_FFFF_F000;

    let e1 = read_table(table, l1_idx as usize);
    if (e1 & DESC_VALID) == 0 {
        kernel::println!("L1 entry not present: idx={} val={:#x}", l1_idx, e1);
        return;
    }
    kernel::println!("L1[{}] = {:#x}", l1_idx, e1);
    table = e1 & 0x0000_FFFF_FFFF_F000;

    let e2 = read_table(table, l2_idx as usize);
    if (e2 & DESC_VALID) == 0 {
        kernel::println!("L2 entry not present: idx={} val={:#x}", l2_idx, e2);
        return;
    }
    kernel::println!("L2[{}] = {:#x}", l2_idx, e2);
    table = e2 & 0x0000_FFFF_FFFF_F000;

    let e3 = read_table(table, l3_idx as usize);
    kernel::println!("L3[{}] = {:#x}", l3_idx, e3);
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
        table_phys = ensure_next_level(table_phys, l0_idx as usize, 0);
    }

    // Level 1
    table_phys = ensure_next_level(table_phys, l1_idx as usize, 1);

    // Level 2
    table_phys = ensure_next_level(table_phys, l2_idx as usize, 2);

    // Level 3 (Page)
    let entry = phys | DESC_VALID | DESC_PAGE | DESC_ACCESS_FLAG | DESC_SH_INNER | ATTR_DEVICE;
    // Note: We overwrite existing mapping if any.
    kernel::println!(
        "map_page: l3_idx={} table_phys={:#x} entry={:#x}",
        l3_idx,
        table_phys,
        entry
    );
    write_table(table_phys, l3_idx as usize, entry);
}

unsafe fn ensure_next_level(table_phys: u64, index: usize, level: u8) -> u64 {
    let entry = read_table(table_phys, index);
    if entry & DESC_VALID != 0 {
        // Check if it's a block mapping (Bit 1 is 0)
        // DESC_TABLE = 1<<1 = 2. DESC_BLOCK = 0<<1 = 0.
        // If it is a block, we must split it into a new table to allow more granular mapping
        // while preserving existing attributes.
        if (entry & 2) == 0 {
            // Found a block mapping. We must split it into a new table.
            if level == 0 {
                panic!("Encountered L0 block mapping, which is invalid for 4KB granule");
            }

            // Allocate new table
            let frame = if let Some(f) = allocate_frame() {
                f
            } else {
                panic!("OOM during block splitting");
            };
            let new_table_phys = frame.start_address;
            let new_table_virt = phys_to_virt(new_table_phys);
            let ptr = new_table_virt as *mut u64;

            // Extract info from the block entry
            let addr_mask = 0x0000_FFFF_FFFF_F000;
            let base_phys = entry & addr_mask;
            // Keep attributes: everything except address and type bit (bit 1)
            // Note: Bit 0 (Valid) is kept. Bit 1 (Type) is 0 in entry, so masking doesn't change it.
            let attributes = entry & !addr_mask;

            let (step_size, new_type) = if level == 1 {
                // Splitting L1 (1GB) -> L2 (2MB blocks)
                (0x200000, DESC_BLOCK)
            } else if level == 2 {
                // Splitting L2 (2MB) -> L3 (4KB pages)
                (0x1000, DESC_PAGE)
            } else {
                panic!("Unexpected level for block mapping: {}", level);
            };

            // Fill the new table
            for i in 0..512 {
                let sub_phys = base_phys + (i as u64 * step_size);
                // Combine address, attributes, and new type
                // attributes has bit 0 (Valid) set.
                // new_type sets bit 1 appropriately (0 for Block, 1 for Page).
                let new_val = sub_phys | attributes | new_type;
                *ptr.add(i) = new_val;
            }

            // Link the new table in the current table, replacing the block entry.
            // DESC_TABLE has bit 1 set.
            let table_entry = new_table_phys | DESC_VALID | DESC_TABLE;
            write_table(table_phys, index, table_entry);

            // We do not need to flush TLB for the table entry change immediately if we assume
            // the caller will eventually flush or if the hardware handles CoW-like splits (which it doesn't always).
            // However, since we are replacing a valid entry with another valid entry (Block -> Table),
            // a TLB flush is generally required for correctness if the Block was cached.
            // But we don't have easy access to flush just this VA range here without more context.
            // The caller `map_device_region` does a full VMALLE1 flush at the end.

            new_table_phys
        } else {
            // It is a Table descriptor.
            // Mask out attributes to get address
            entry & 0x0000_FFFF_FFFF_F000
        }
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
