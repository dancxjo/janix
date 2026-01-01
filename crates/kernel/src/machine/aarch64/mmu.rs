use crate::memory::map::{MapPerms, MapResult};
use core::arch::asm;
use alloc::alloc::{Layout, alloc_zeroed};

#[derive(Clone, Copy)]
pub struct AddressSpace {
    pub ttbr0: u64,
}

impl AddressSpace {
    pub fn new() -> Self {
        // Allocate L0 table (512GB range per entry, covers 256TB)
        // AArch64 usually uses 4 levels (48-bit VA). L0 is root.
        let phys = unsafe { alloc_subtable() };
        Self { ttbr0: phys } 
    }

    pub fn activate(&self) {
        unsafe {
            // Set ASID to 0 (all tasks share ASID 0 for now? Or flushes needed)
            // We use ASID 0 and flush TLB on switch.
            // TTBR0_EL1 = BADDR[47:1] | ASID[63:48]
            // We just write BADDR.
            asm!("msr ttbr0_el1, {}", in(reg) self.ttbr0);
            asm!("isb");
            // Invalidate TLB for ASID 0 (or all)
            asm!("tlbi vmalle1"); // Invalidate all EL1&0 regime logic
            asm!("dsb sy");
            asm!("isb");
        }
    }

    pub fn map(&mut self, virt: u64, phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        let pages = (len + 4095) / 4096;
        for i in 0..pages {
            let offset = i as u64 * 4096;
            unsafe {
                self.map_page(virt + offset, phys + offset, perms)?;
            }
        }
        Ok(())
    }

    unsafe fn map_page(&mut self, virt: u64, phys: u64, perms: MapPerms) -> MapResult<()> {
        let l0 = phys_to_virt(self.ttbr0) as *mut u64;
        let l0_idx = ((virt >> 39) & 0x1ff) as usize;
        let l1 = ensure_table(l0, l0_idx)?;

        let l1_idx = ((virt >> 30) & 0x1ff) as usize;
        let l2 = ensure_table(l1, l1_idx)?;

        let l2_idx = ((virt >> 21) & 0x1ff) as usize;
        let l3 = ensure_table(l2, l2_idx)?;

        let l3_idx = ((virt >> 12) & 0x1ff) as usize;
        let entry_ptr = l3.add(l3_idx);

        // Describe page
        // Lower attr (bits 11:2):
        // [10] AF=1 (Accessed)
        // [9:8] SH (Shareability) -> 11 (Inner Shareable)
        // [7:6] AP (Perms). 00=RW_EL1, 01=RW_EL1/EL0, 10=RO_EL1, 11=RO_EL1/EL0
        // [5] NS
        // [4:2] AttrIndx. 
        // 0=Normal, 2=Device. 
        // We assume Normal (0) for now unless mapped as Device.
        // But MapPerms doesn't specify Cacheability?
        // We usually map RAM as Normal (0).
        
        let mut desc = (phys & !0xfff) | 0b11; // Page Entry
        desc |= 1 << 10; // AF
        desc |= 0b11 << 8; // Inner Shareable

        // User permissions (EL0)
        // We need User access.
        // If writable: AP=01 (RW EL0)
        // If disallowed write: AP=11 (RO EL0)
        // Wait, AP[1] = 1 means RO.
        // AP[0] = 1 means EL0 access allowed.
        // So:
        // EL1 RW, EL0 No => 00
        // EL1 RW, EL0 RW => 01
        // EL1 RO, EL0 No => 10
        // EL1 RO, EL0 RO => 11
        
        // We always allow EL1 access (kernel can read user mem).
        // Since this IS AddressSpace for a Task, it implies EL0 access.
        // MapPerms usually implies "User Access" context here.
        
        if perms.contains(MapPerms::WRITE) {
            desc |= 1 << 6; // AP[2:1] = 01
        } else {
            desc |= 3 << 6; // AP[2:1] = 11
        }

        // UXN / PXN
        // If EXEC: UXN=0, PXN=1 (Kernel shouldn't exec user code)
        // If !EXEC: UXN=1, PXN=1
        if !perms.contains(MapPerms::EXEC) {
            desc |= 1 << 54; // UXN
        }
        desc |= 1 << 53; // PXN

        // AttrIndex
        desc |= 0 << 2; // Normal Memory (Index 0)

        entry_ptr.write(desc);
        
        Ok(())
    }
}

unsafe fn alloc_subtable() -> u64 {
    let layout = Layout::from_size_align(4096, 4096).unwrap();
    let ptr = alloc_zeroed(layout);
    if ptr.is_null() {
        panic!("MMU OOM"); // Should handle gracefully
    }
    // Convert to Phys
    
    // We need virt_to_phys.
    // machine() might not be available if not init? (It is init by now)
    crate::machine::machine().virt_to_phys(ptr as u64)
}

fn phys_to_virt(phys: u64) -> u64 {
    // Inverse of simple translation?
    // We assume HHDM or Kernel map.
    // Hack: we need to access physical memory.
    // crate::boot::get_boot_ctx().hhdm_offset + phys
    // But `get_boot_ctx` might be unsafe.
    // `crate::machine::ARCH_MACHINE` has a helper `phys_to_virt`.
    // But it's not exposed in `Machine` trait.
    // We can use `crate::machine::machine()`. IT DOES NOT HAVE `phys_to_virt`.
    // It has `virt_to_phys`.
    
    // Fortunately, `mmu.rs` is architecture specific.
    // We can use `super::ARCH_MACHINE.phys_to_virt(phys)`.
    // Wait, `ARCH_MACHINE` has `phys_to_virt` but it is private?
    // Let's check `mod.rs`.
    // `fn phys_to_virt(&self, phys: u64) -> u64`. It is NOT pub.
    // I need to make `phys_to_virt` public in `mod.rs` too?
    // Yes.
    
    // For now, I will use `crate::boot::get_boot_ctx().hhdm_offset + phys` 
    // assuming HHDM is valid. 
    // Wait, get_boot_ctx() is available.
    crate::boot::get_boot_ctx().hhdm_offset + phys
}

unsafe fn ensure_table(table: *mut u64, index: usize) -> MapResult<*mut u64> {
    let entry_ptr = table.add(index);
    let mut entry = *entry_ptr;
    
    if entry & 1 == 0 {
        // Invalid, allocate
        let new_table_phys = alloc_subtable();
        // Entry: Address | Valid | Table
        // 0b11 = Valid Table.
        entry = (new_table_phys & !0xfff) | 0b11;
        *entry_ptr = entry;
    }
    
    let phys = entry & !0xfff;
    Ok(phys_to_virt(phys) as *mut u64)
} 
