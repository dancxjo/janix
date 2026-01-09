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
        // 1. Allocate a new PML4 table
        // We use the global frame allocator
        let root = FRAME_ALLOCATOR.with_lock(|alloc| {
            alloc.alloc_contiguous(1).expect("Failed to allocate PML4").base
        });
        
        let aspace = Self { root };
        
        // 2. Initialize it
        // We need to copy the kernel half (top 256 entries) from the current CR3
        unsafe {
            let virt = phys_to_virt(root.0) as *mut u64;
            // Zero lower half
            core::ptr::write_bytes(virt, 0, 256);
            
            // Copy upper half
            let current_cr3: u64;
            asm!("mov {}, cr3", out(reg) current_cr3, options(nomem, nostack, preserves_flags));
            let current_pml4 = phys_to_virt(current_cr3 & !0xFFF) as *const u64;
            
            core::ptr::copy_nonoverlapping(current_pml4.add(256), virt.add(256), 256);
        }
        
        aspace
    }

    pub fn active() -> Self {
        let cr3: u64;
        unsafe {
             asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags));
        }
        Self { root: PhysFrame(cr3 & !0xFFF) }
    }
    
    pub fn switch(&self) {
        unsafe {
            asm!("mov cr3, {}", in(reg) self.root.0, options(nostack, preserves_flags));
        }
    }
    
    pub fn map_page(&mut self, virt: u64, phys: PhysFrame, flags: PageFlags) -> Result<(), ()> {
        let p4_idx = (virt >> 39) & 0x1FF;
        let p3_idx = (virt >> 30) & 0x1FF;
        let p2_idx = (virt >> 21) & 0x1FF;
        let p1_idx = (virt >> 12) & 0x1FF;
        
        unsafe {
            let p4_ptr = phys_to_virt(self.root.0) as *mut u64;
            let p3_phys = ensure_table(p4_ptr, p4_idx)?;
            
            let p3_ptr = phys_to_virt(p3_phys) as *mut u64;
            let p2_phys = ensure_table(p3_ptr, p3_idx)?;
            
            let p2_ptr = phys_to_virt(p2_phys) as *mut u64;
            let p1_phys = ensure_table(p2_ptr, p2_idx)?;
            
            let p1_ptr = phys_to_virt(p1_phys) as *mut u64;
            
            // Convert PageFlags to x86 flags
            let mut hw_flags = 1; // Present
            if flags.contains(PageFlags::WRITABLE) { hw_flags |= 2; }
            if flags.contains(PageFlags::USER_ACCESSIBLE) { hw_flags |= 4; }
            // If WriteThrough (bit 3) or CacheDisable (bit 4) required:
            if flags.contains(PageFlags::NO_CACHE) { hw_flags |= 1 << 4; }
            if flags.contains(PageFlags::NO_EXECUTE) { hw_flags |= 1 << 63; }
            if flags.contains(PageFlags::GLOBAL) { hw_flags |= 1 << 8; }
            
            let entry = phys.0 | hw_flags;
            *p1_ptr.add(p1_idx as usize) = entry;
            
            // Invalidate TLB if it's the current address space?
            // User can call flush hooks explicitly.
        }
        
        Ok(())
    }
    
    pub fn unmap_page(&mut self, virt: u64) -> Result<Option<PhysFrame>, ()> {
        let p4_idx = (virt >> 39) & 0x1FF;
        let p3_idx = (virt >> 30) & 0x1FF;
        let p2_idx = (virt >> 21) & 0x1FF;
        let p1_idx = (virt >> 12) & 0x1FF;
        
        unsafe {
            let p4_ptr = phys_to_virt(self.root.0) as *const u64;
            if (*p4_ptr.add(p4_idx as usize) & 1) == 0 { return Ok(None); }
            let p3_phys = *p4_ptr.add(p4_idx as usize) & 0x000FFFFFFFFFF000;
            
            let p3_ptr = phys_to_virt(p3_phys) as *const u64;
            if (*p3_ptr.add(p3_idx as usize) & 1) == 0 { return Ok(None); }
            let p2_phys = *p3_ptr.add(p3_idx as usize) & 0x000FFFFFFFFFF000;
            
            let p2_ptr = phys_to_virt(p2_phys) as *const u64;
            if (*p2_ptr.add(p2_idx as usize) & 1) == 0 { return Ok(None); }
            let p1_phys = *p2_ptr.add(p2_idx as usize) & 0x000FFFFFFFFFF000;
            
            let p1_ptr = phys_to_virt(p1_phys) as *mut u64;
            let entry = *p1_ptr.add(p1_idx as usize);
            
            if (entry & 1) == 0 { return Ok(None); }
            
            // Clear entry
            *p1_ptr.add(p1_idx as usize) = 0;
            
            tlb_flush_page(virt);
            
            Ok(Some(PhysFrame(entry & 0x000FFFFFFFFFF000)))
        }
    }
    
    pub fn translate(&self, virt: u64) -> Option<PhysFrame> {
         let p4_idx = (virt >> 39) & 0x1FF;
        let p3_idx = (virt >> 30) & 0x1FF;
        let p2_idx = (virt >> 21) & 0x1FF;
        let p1_idx = (virt >> 12) & 0x1FF;
        
        unsafe {
            let p4_ptr = phys_to_virt(self.root.0) as *const u64;
            if (*p4_ptr.add(p4_idx as usize) & 1) == 0 { return None; }
            let p3_phys = *p4_ptr.add(p4_idx as usize) & 0x000FFFFFFFFFF000;
            
            let p3_ptr = phys_to_virt(p3_phys) as *const u64;
            if (*p3_ptr.add(p3_idx as usize) & 1) == 0 { return None; }
            let p2_phys = *p3_ptr.add(p3_idx as usize) & 0x000FFFFFFFFFF000;
            
            let p2_ptr = phys_to_virt(p2_phys) as *const u64;
            if (*p2_ptr.add(p2_idx as usize) & 1) == 0 { return None; }
            let p1_phys = *p2_ptr.add(p2_idx as usize) & 0x000FFFFFFFFFF000;
            
            let p1_ptr = phys_to_virt(p1_phys) as *const u64;
            let entry = *p1_ptr.add(p1_idx as usize);
            
            if (entry & 1) == 0 { return None; }
            
            Some(PhysFrame(entry & 0x000FFFFFFFFFF000))
        }
    }
}

unsafe fn ensure_table(table: *mut u64, index: u64) -> Result<u64, ()> {
    let entry = unsafe { *table.add(index as usize) };
    if (entry & 1) != 0 {
        if (entry & 0x80) != 0 {
            // Huge page, cannot treat as table
             return Err(());
        }
        Ok(entry & 0x000FFFFFFFFFF000)
    } else {
        // Allocate new table
        let frame = FRAME_ALLOCATOR.with_lock(|alloc| {
             alloc.alloc_contiguous(1).map(|r| r.base)
        }).ok_or(())?;
        
        unsafe {
            let virt_ptr = phys_to_virt(frame.0) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            
            // Link it with Present | Writable | User (so user pages in bottom can be accessed)
             let new_entry = frame.0 | 1 | 2 | 4;
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
        asm!("invlpg [{}]", in(reg) virt, options(nostack, preserves_flags));
    }
}

pub fn tlb_flush_all() {
    unsafe {
        let cr3: u64;
        asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags));
        asm!("mov cr3, {}", in(reg) cr3, options(nostack, preserves_flags));
    }
}


const PAGE_PRESENT: u64 = 1;
const PAGE_WRITABLE: u64 = 2;
const PAGE_HUGE: u64 = 0x80;

/// Map a single 4KiB page for the boot heap.
/// This assumes the higher-level tables (PML4, PDP, PD) might need to be allocated.
/// Uses the provided allocator for new page tables.
pub fn map_bootheap_page(virt: u64, phys: u64, allocator: &mut BootFrameAllocator) {
    unsafe {
         // This legacy function needs refactoring or keeping as is for legacy boot use.
         // Keeping as logic helper, but locally copying the ensure logic since it uses BootFrameAllocator
         // which is different from global FrameAllocator.
         
        let cr3: u64;
        asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags));
        
        let pml4_phys = cr3 & !0xFFF;
        let pml4_idx = (virt >> 39) & 0x1FF;
        let pdp_idx = (virt >> 30) & 0x1FF;
        let pd_idx = (virt >> 21) & 0x1FF;
        let pt_idx = (virt >> 12) & 0x1FF;

        let pml4 = phys_to_virt(pml4_phys) as *mut u64;
        let pdp_phys = ensure_table_boot(pml4, pml4_idx, allocator);
        
        let pdp = phys_to_virt(pdp_phys) as *mut u64;
        let pd_phys = ensure_table_boot(pdp, pdp_idx, allocator);
        
        let pd = phys_to_virt(pd_phys) as *mut u64;
        let pt_phys = ensure_table_boot(pd, pd_idx, allocator);
        
        let pt = phys_to_virt(pt_phys) as *mut u64;
        let entry = phys | PAGE_PRESENT | PAGE_WRITABLE;
        *pt.add(pt_idx as usize) = entry;
        
        tlb_flush_page(virt);
    }
}

unsafe fn ensure_table_boot(table: *mut u64, index: u64, allocator: &mut BootFrameAllocator) -> u64 {
    let entry = unsafe { *table.add(index as usize) };
    if (entry & PAGE_PRESENT) != 0 {
        if (entry & PAGE_HUGE) != 0 {
             panic!("Huge page encountered while walking tables for bootheap!");
        }
        entry & 0x000FFFFFFFFFF000
    } else {
        let frame = allocator.alloc_frame().expect("OOM allocating page table for bootheap");
        unsafe {
            let virt_ptr = phys_to_virt(frame) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            let new_entry = frame | PAGE_PRESENT | PAGE_WRITABLE;
            *table.add(index as usize) = new_entry;
        }

        frame
    }
}

pub fn test_paging() {
    use crate::kinfo;
    
    kinfo!("Testing paging subsystem...");
    let mut aspace = AddressSpace::new();
    let virt = 0xDEAD_BEEF_0000;
    
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

    // Switch smoke test
    // We only switch if we are confident the kernel mappings are correct.
    // Our new() copies the top 256 entries (high half).
    // Kernel code and heap should be in high half.
    aspace.switch();
    kinfo!("Switched to new address space");
    
    // Switch back to "initial"? We don't have a handle to the initial one easily here without reading CR3 before.
    // But since `aspace` is a valid kernel duplicate, we can stay on it.
    
    kinfo!("Paging subsystem test passed");
}
