use crate::memory::boot_frame_alloc::BootFrameAllocator;
use crate::memory::paging::PageFlags;
use crate::memory::frame_alloc::{PhysFrame, FRAME_ALLOCATOR};
use core::arch::asm;

static mut HHDM_OFFSET: u64 = 0;

pub fn init(hhdm_offset: u64) {
    unsafe { HHDM_OFFSET = hhdm_offset; }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressSpace {
    pub root: PhysFrame,
}

impl AddressSpace {
    pub fn new() -> Self {
        // Allocate a new L0 table for TTBR0
        let root = FRAME_ALLOCATOR.with_lock(|alloc| {
            alloc.alloc_contiguous(1).expect("Failed to allocate L0 table").base
        });
        
        // Zero it out
        unsafe {
            let virt = phys_to_virt(root.0) as *mut u64;
            core::ptr::write_bytes(virt, 0, 512); // L0 has 512 entries
        }
        
        Self { root }
    }
    
    pub fn active() -> Self {
        let ttbr1: u64;
        unsafe {
             asm!("mrs {}, ttbr1_el1", out(reg) ttbr1, options(nomem, nostack, preserves_flags));
        }
        // Mask out ASID or other bits if present? Usually bits [47:1] are address.
        // But for safety let's assume it's clean physical.
        // TTBRn_EL1: [47:1] BADDR.
        Self { root: PhysFrame(ttbr1 & 0x0000_FFFF_FFFF_F000) }
    }

    pub fn switch(&self) {
        unsafe {
            // Set TTBR0_EL1
            let phys = self.root.0;
            asm!("msr ttbr0_el1, {}", in(reg) phys, options(nostack, preserves_flags));
            asm!("isb", options(nostack, preserves_flags));
            asm!("tlbi vmalle1is", options(nostack, preserves_flags));
            asm!("dsb ish", options(nostack, preserves_flags));
            asm!("isb", options(nostack, preserves_flags));
        }
    }
    
    pub fn map_page(&mut self, virt: u64, phys: PhysFrame, flags: PageFlags) -> Result<(), ()> {
        let l0_idx = (virt >> 39) & 0x1FF;
        let l1_idx = (virt >> 30) & 0x1FF;
        let l2_idx = (virt >> 21) & 0x1FF;
        let l3_idx = (virt >> 12) & 0x1FF;
        
        unsafe {
            let l0_ptr = phys_to_virt(self.root.0) as *mut u64;
            let l1_phys = ensure_table(l0_ptr, l0_idx)?;
            
            let l1_ptr = phys_to_virt(l1_phys) as *mut u64;
            let l2_phys = ensure_table(l1_ptr, l1_idx)?;
            
            let l2_ptr = phys_to_virt(l2_phys) as *mut u64;
            let l3_phys = ensure_table(l2_ptr, l2_idx)?;
            
            let l3_ptr = phys_to_virt(l3_phys) as *mut u64;
            
            // Descriptor bits
            // Valid=1, Table/Page=1 (for L3)
            let mut desc = 3; 
            
            if !flags.contains(PageFlags::WRITABLE) {
                desc |= 1 << 7; // AP[2] = 1 (RO)
            }
            
            if flags.contains(PageFlags::USER_ACCESSIBLE) {
                desc |= 1 << 6; // AP[1] = 1 (User)
            }
            
            if flags.contains(PageFlags::NO_EXECUTE) {
                desc |= 1 << 53; // PXN (Privileged Execute-Never)
                desc |= 1 << 54; // UXN (Unprivileged Execute-Never)
            }
            
            // Access flag (bit 10).
            desc |= 1 << 10;
            
            // Shareability. Inner Shareable (3<<8)
            desc |= 3 << 8;
            
            let entry = phys.0 | desc;
            *l3_ptr.add(l3_idx as usize) = entry;
        }
        
        Ok(())
    }
    
    pub fn unmap_page(&mut self, virt: u64) -> Result<Option<PhysFrame>, ()> {
         let l0_idx = (virt >> 39) & 0x1FF;
        let l1_idx = (virt >> 30) & 0x1FF;
        let l2_idx = (virt >> 21) & 0x1FF;
        let l3_idx = (virt >> 12) & 0x1FF;
        
        unsafe {
            let l0_ptr = phys_to_virt(self.root.0) as *const u64;
            if (*l0_ptr.add(l0_idx as usize) & 1) == 0 { return Ok(None); }
            let l1_phys = *l0_ptr.add(l0_idx as usize) & 0x0000_FFFF_FFFF_F000;
            
            let l1_ptr = phys_to_virt(l1_phys) as *const u64;
            if (*l1_ptr.add(l1_idx as usize) & 1) == 0 { return Ok(None); }
            let l2_phys = *l1_ptr.add(l1_idx as usize) & 0x0000_FFFF_FFFF_F000;
            
            let l2_ptr = phys_to_virt(l2_phys) as *const u64;
            if (*l2_ptr.add(l2_idx as usize) & 1) == 0 { return Ok(None); }
            let l3_phys = *l2_ptr.add(l2_idx as usize) & 0x0000_FFFF_FFFF_F000;
            
            let l3_ptr = phys_to_virt(l3_phys) as *mut u64;
            let entry = *l3_ptr.add(l3_idx as usize);
            
            if (entry & 1) == 0 { return Ok(None); }
            
            // Clear
            *l3_ptr.add(l3_idx as usize) = 0;
            
            tlb_flush_page(virt);
            
            Ok(Some(PhysFrame(entry & 0x0000_FFFF_FFFF_F000)))
        }
    }
    
    pub fn translate(&self, virt: u64) -> Option<PhysFrame> {
         let l0_idx = (virt >> 39) & 0x1FF;
        let l1_idx = (virt >> 30) & 0x1FF;
        let l2_idx = (virt >> 21) & 0x1FF;
        let l3_idx = (virt >> 12) & 0x1FF;
        
        unsafe {
            let l0_ptr = phys_to_virt(self.root.0) as *const u64;
            if (*l0_ptr.add(l0_idx as usize) & 1) == 0 { return None; }
            let l1_phys = *l0_ptr.add(l0_idx as usize) & 0x0000_FFFF_FFFF_F000;
            
            let l1_ptr = phys_to_virt(l1_phys) as *const u64;
            if (*l1_ptr.add(l1_idx as usize) & 1) == 0 { return None; }
            let l2_phys = *l1_ptr.add(l1_idx as usize) & 0x0000_FFFF_FFFF_F000;
            
            let l2_ptr = phys_to_virt(l2_phys) as *const u64;
            if (*l2_ptr.add(l2_idx as usize) & 1) == 0 { return None; }
            let l3_phys = *l2_ptr.add(l2_idx as usize) & 0x0000_FFFF_FFFF_F000;
            
            let l3_ptr = phys_to_virt(l3_phys) as *const u64;
            let entry = *l3_ptr.add(l3_idx as usize);
            
            if (entry & 1) == 0 { return None; }
            
            Some(PhysFrame(entry & 0x0000_FFFF_FFFF_F000))
        }
    }
}

unsafe fn ensure_table(table: *mut u64, index: u64) -> Result<u64, ()> {
    let entry = unsafe { *table.add(index as usize) };
    if (entry & 1) != 0 {
        if (entry & 2) == 0 { return Err(()); }
        Ok(entry & 0x0000_FFFF_FFFF_F000)
    } else {
         let frame = FRAME_ALLOCATOR.with_lock(|alloc| {
             alloc.alloc_contiguous(1).map(|r| r.base)
        }).ok_or(())?;
        
        unsafe {
            let virt_ptr = phys_to_virt(frame.0) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            let new_entry = frame.0 | 3;
            *table.add(index as usize) = new_entry;
        }
        
        Ok(frame.0)
    }
}

unsafe fn phys_to_virt(phys: u64) -> u64 {
    unsafe { phys + HHDM_OFFSET }
}

pub fn tlb_flush_page(virt: u64) {
    unsafe {
        asm!("tlbi vaae1is, {}", in(reg) (virt >> 12), options(nostack, preserves_flags));
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

// 4KiB page size
const PAGE_VALID: u64 = 1;

const PAGE_ACCESS: u64 = 1 << 10;
const PAGE_SH_INNER: u64 = 3 << 8;

// AP[2] (bit 7): 0=RW, 1=RO.
// AP[1] (bit 6): 0=Kernel, 1=User.
// We want RW Kernel -> AP[2]=0, AP[1]=0.

pub fn map_bootheap_page(virt: u64, phys: u64, allocator: &mut BootFrameAllocator) {
    unsafe {
        let ttbr1: u64;
        asm!("mrs {}, ttbr1_el1", out(reg) ttbr1, options(nomem, nostack, preserves_flags));
        
        // Remove ASID/Attributes from TTBR to get physical base
        let root_phys = ttbr1 & 0x0000_FFFF_FFFF_F000;
        
        let l0_idx = (virt >> 39) & 0x1FF;
        let l1_idx = (virt >> 30) & 0x1FF;
        let l2_idx = (virt >> 21) & 0x1FF;
        let l3_idx = (virt >> 12) & 0x1FF;
        
        let l0_ptr = phys_to_virt(root_phys) as *mut u64;
        let l1_phys = ensure_table_boot(l0_ptr, l0_idx, allocator);
        
        let l1_ptr = phys_to_virt(l1_phys) as *mut u64;
        let l2_phys = ensure_table_boot(l1_ptr, l1_idx, allocator);
        
        let l2_ptr = phys_to_virt(l2_phys) as *mut u64;
        let l3_phys = ensure_table_boot(l2_ptr, l2_idx, allocator);
        
        let l3_ptr = phys_to_virt(l3_phys) as *mut u64;
        
        // Entry bits: Valid(1) | Table/Page(3 for L3... wait, for L3: bit 1=1 is Page)
        // At L3, bit 1=1 is "Page descriptor".
        // Base Attrs: Valid | Page | Access | InnerShareable
        let mut desc = PAGE_VALID | 2; // Bit 1=1 for Page at L3
        desc |= PAGE_ACCESS;
        desc |= PAGE_SH_INNER;
        // Default is Kernel RW (AP=00)
        // If we needed Execute, we are fine. For heap, usually NX, so PXN(bit 53) UXN(bit 54)
        desc |= 1 << 53; // PXN
        desc |= 1 << 54; // UXN
        
        // Mair index 0 (assuming Normal)
        
        let entry = phys | desc;
        *l3_ptr.add(l3_idx as usize) = entry;
        
        tlb_flush_page(virt);
    }
}

unsafe fn ensure_table_boot(table: *mut u64, index: u64, allocator: &mut BootFrameAllocator) -> u64 {
    let entry = *table.add(index as usize);
    if (entry & 1) != 0 {
        // Check for block mappings (which shouldn't be here for tables we traverse)
        // For L0/L1/L2, bit 1=1 is Table. bit 1=0 is Block.
        if (entry & 2) == 0 {
             panic!("Huge page/Block encountered while walking tables for bootheap!");
        }
        entry & 0x0000_FFFF_FFFF_F000
    } else {
        let frame = allocator.alloc_frame().expect("OOM allocating page table for bootheap");
        let virt_ptr = phys_to_virt(frame) as *mut u8;
        core::ptr::write_bytes(virt_ptr, 0, 4096);
        
        // Table descriptor: Valid(1) | Table(1) = 3
        let new_entry = frame | 3;
        *table.add(index as usize) = new_entry;
        
        frame
    }
}

pub fn test_paging() {
    crate::kinfo!("Testing paging subsystem (aarch64)...");
    let mut aspace = AddressSpace::new();
    let virt = 0x1000_0000; // Use a lower address for TTBR0
    
    // Alloc a frame to map
    let frame = FRAME_ALLOCATOR.with_lock(|alloc| alloc.alloc().expect("test_paging alloc failed"));
    
    // Map
    aspace.map_page(virt, frame, PageFlags::PRESENT | PageFlags::WRITABLE).expect("map_page failed");
    
    // Translate
    if let Some(f) = aspace.translate(virt) {
        if f != frame { panic!("Translate returned wrong frame"); }
    } else {
        panic!("Translate returned None after mapping");
    }
    
    // Unmap
    match aspace.unmap_page(virt) {
        Ok(Some(f)) => if f != frame { panic!("Unmap returned wrong frame"); },
        Ok(None) => panic!("Unmap returned None"),
        Err(_) => panic!("Unmap failed"),
    }
    
    // Translate again
    if aspace.translate(virt).is_some() {
        panic!("Translate returned Some after unmap");
    }
    
    aspace.switch();
    crate::kinfo!("Switched to new address space");
    crate::kinfo!("Paging subsystem test passed");
}
