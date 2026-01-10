use kernel::memory::boot_frame_alloc::BootFrameAllocator;
use kernel::memory::paging::PageFlags;
use kernel::memory::frame_alloc::PhysFrame;

use core::arch::asm;
use crate::requests::HHDM_REQUEST;

pub fn phys_to_virt_offset() -> u64 {
    HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0)
}

pub unsafe fn phys_to_virt(phys: u64) -> u64 {
    phys + phys_to_virt_offset()
}

// AArch64 Flags
const DESC_VALID: u64 = 1;
const DESC_TABLE: u64 = 2; // For L0-L2
const DESC_PAGE: u64 = 3;  // For L3 (Valid | Page)
const DESC_AF: u64 = 1 << 10; // Access Flag
const DESC_SH_INNER: u64 = 3 << 8; // Inner Shareable

pub fn map_bootheap_page(virt: u64, phys: u64, allocator: &mut BootFrameAllocator) {
    unsafe {
        // Read TTBR1_EL1 (Kernel Table Base)
        let ttbr1: u64;
        asm!("mrs {}, ttbr1_el1", out(reg) ttbr1, options(nomem, nostack));
        // TTBR1 has ASID in top bits, mask them if needed (usually 48 bits phys)
        // BADDR is [47:1] usually.
        let l0_phys = ttbr1 & 0x0000_FFFF_FFFF_F000;
        
        // Indices (same as x86 for 4KB pages)
        let l0_idx = (virt >> 39) & 0x1FF;
        let l1_idx = (virt >> 30) & 0x1FF;
        let l2_idx = (virt >> 21) & 0x1FF;
        let l3_idx = (virt >> 12) & 0x1FF;

        let l0_ptr = phys_to_virt(l0_phys) as *mut u64;
        let l1_phys = ensure_table_boot(l0_ptr, l0_idx, allocator);
        
        let l1_ptr = phys_to_virt(l1_phys) as *mut u64;
        let l2_phys = ensure_table_boot(l1_ptr, l1_idx, allocator);
        
        let l2_ptr = phys_to_virt(l2_phys) as *mut u64;
        let l3_phys = ensure_table_boot(l2_ptr, l2_idx, allocator);
        
        let l3_ptr = phys_to_virt(l3_phys) as *mut u64;
        
        // Leaf Entry (L3)
        // Valid | Page | AF | InnerShareable | AP=RW(Priv) 
        // AP=00 (RW Priv), AP=01 (RW User/Priv) -> We want 00.
        // AttrIndx = 0 (assuming MAIR[0] is Normal Memory, which Limine usually sets)
        let entry = phys | DESC_PAGE | DESC_AF | DESC_SH_INNER; 
        *l3_ptr.add(l3_idx as usize) = entry;
        
        tlb_flush_page(virt);
    }
}

unsafe fn ensure_table_boot(table: *mut u64, index: u64, allocator: &mut BootFrameAllocator) -> u64 {
    unsafe {
        let entry = *table.add(index as usize);
        if (entry & DESC_VALID) != 0 {
            // Assume it's a table, not a block for now?
            // Block descriptors (L1/L2) have bit 1 = 0.
            // Table descriptors (L0-L2) have bit 1 = 1.
            // We really hope Limine didn't map this range with Blocks if we are subdividing it.
            // But BootHeap is usually in free space.
            entry & 0x0000_FFFF_FFFF_F000
        } else {
            let frame = allocator.alloc_frame().expect("OOM allocating page table for bootheap");
            let virt_ptr = phys_to_virt(frame) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            
            // Create Table Descriptor
            // Valid | Table
            let new_entry = frame | DESC_VALID | DESC_TABLE | DESC_AF; // AF not strictly needed for tables but good practice?
            *table.add(index as usize) = new_entry;
            
            frame
        }
    }
}

use kernel::memory::frame_alloc::FRAME_ALLOCATOR;

pub fn map_page(virt: u64, phys: PhysFrame, flags: PageFlags) -> Result<(), ()> {
    unsafe {
        // Read TTBR1_EL1
        let ttbr1: u64;
        asm!("mrs {}, ttbr1_el1", out(reg) ttbr1, options(nomem, nostack));
        let l0_phys = ttbr1 & 0x0000_FFFF_FFFF_F000;
        
        let l0_idx = (virt >> 39) & 0x1FF;
        let l1_idx = (virt >> 30) & 0x1FF;
        let l2_idx = (virt >> 21) & 0x1FF;
        let l3_idx = (virt >> 12) & 0x1FF;

        let l0_ptr = phys_to_virt(l0_phys) as *mut u64;
        let l1_phys = ensure_table_global(l0_ptr, l0_idx)?;
        
        let l1_ptr = phys_to_virt(l1_phys) as *mut u64;
        let l2_phys = ensure_table_global(l1_ptr, l1_idx)?;
        
        let l2_ptr = phys_to_virt(l2_phys) as *mut u64;
        let l3_phys = ensure_table_global(l2_ptr, l2_idx)?;
        
        let l3_ptr = phys_to_virt(l3_phys) as *mut u64;
        
        // Leaf Entry (L3)
        // Valid | Page | AF | InnerShareable
        // AP: 
        // 00 = Priv RW
        // 01 = User RW (if PageFlags::USER)
        // 10 = Priv RO
        // 11 = User RO
        // We assume RW for now unless flags say otherwise.
        
        let mut desc_ap = 0; // Priv RW
        if flags.contains(PageFlags::USER_ACCESSIBLE) {
             desc_ap |= 1 << 6; // AP[1]=1 -> User/Priv RW (if AP[2]=0)
        }
        // TODO: Handle RO, NX
        
        let entry = phys.0 | DESC_PAGE | DESC_AF | DESC_SH_INNER | desc_ap; 
        *l3_ptr.add(l3_idx as usize) = entry;
        
        tlb_flush_page(virt);
        Ok(())
    }
}

unsafe fn ensure_table_global(table: *mut u64, index: u64) -> Result<u64, ()> {
    unsafe {
        let entry = *table.add(index as usize);
        if (entry & DESC_VALID) != 0 {
            // Assume Table
            Ok(entry & 0x0000_FFFF_FFFF_F000)
        } else {
            // Allocate from Global Allocator
            let frame = FRAME_ALLOCATOR.with_lock(|alloc| {
                 alloc.alloc_contiguous(1).map(|r| r.base.0)
            }).ok_or(())?;
            
            let virt_ptr = phys_to_virt(frame) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            
            // Link - allow user access to table descriptors so they can reach user pages?
            // AP for tables:
            // "The AP/APTable bits in the descriptors for the subsequent levels of lookup."
            // Actually AP in Table descriptor limits access for subsequent levels!
            // APTable = 00 (No effect).
            // UXNTable / PXNTable...
            // We just leave them 0.
            
            let new_entry = frame | DESC_VALID | DESC_TABLE | DESC_AF;
            *table.add(index as usize) = new_entry;
            
            Ok(frame)
        }
    }
}
pub fn unmap_page(_virt: u64) -> Result<Option<PhysFrame>, ()> { Ok(None) }
pub fn translate(_virt: u64) -> Option<PhysFrame> { None }

pub fn tlb_flush_page(virt: u64) {
    unsafe {
        // TLBI VAAE1IS, x0
        // virt is 48 bit but TLBI takes it shifted by 12 usually?
        let page = virt >> 12;
        asm!("tlbi vaae1is, {}", in(reg) page, options(nostack, preserves_flags));
        asm!("dsb ish", options(nostack, preserves_flags));
        asm!("isb", options(nostack, preserves_flags));
    }
}
pub fn tlb_flush_all() {
    unsafe {
        asm!("tlbi vmalle1is", options(nostack, preserves_flags));
        asm!("dsb ish", options(nostack, preserves_flags));
        asm!("isb", options(nostack, preserves_flags));
    }
}
