use kernel::memory::boot_frame_alloc::BootFrameAllocator;
use kernel::memory::paging::PageFlags;
use kernel::memory::frame_alloc::{PhysFrame, FRAME_ALLOCATOR};
use core::arch::asm;
use crate::requests::HHDM_REQUEST;

pub fn phys_to_virt_offset() -> u64 {
    HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0)
}

pub unsafe fn phys_to_virt(phys: u64) -> u64 {
    phys + phys_to_virt_offset()
}

pub fn tlb_flush_page(_virt: u64) {
    unsafe {
        asm!("sfence.vma");
    }
}

pub fn tlb_flush_all() {
    unsafe {
        asm!("sfence.vma");
    }
}

// Sv48/Sv39 constants
const PTE_V: u64 = 1 << 0;
const PTE_R: u64 = 1 << 1;
const PTE_W: u64 = 1 << 2;
const PTE_X: u64 = 1 << 3;
const PTE_U: u64 = 1 << 4;
const PTE_G: u64 = 1 << 5;
const PTE_A: u64 = 1 << 6;
const PTE_D: u64 = 1 << 7;

fn internal_map_get_flags(flags: PageFlags) -> u64 {
    let mut hw_flags = PTE_V | PTE_R | PTE_A | PTE_D; // Default specific to this kernel: Valid, Read, Accessed, Dirty
    
    if flags.contains(PageFlags::WRITABLE) { hw_flags |= PTE_W; }
    if flags.contains(PageFlags::USER_ACCESSIBLE) { hw_flags |= PTE_U; }
    // Execution: If NO_EXECUTE is NOT set, we assume Executable.
    if !flags.contains(PageFlags::NO_EXECUTE) { hw_flags |= PTE_X; }
    if flags.contains(PageFlags::GLOBAL) { hw_flags |= PTE_G; }
    
    hw_flags
}

pub fn map_page(virt: u64, phys: PhysFrame, flags: PageFlags) -> Result<(), ()> {
    unsafe {
        let satp: u64;
        asm!("csrr {}, satp", out(reg) satp);
        let mode = satp >> 60;
        
        let levels = match mode {
            9 => 4,
            8 => 3,
             _ => return Err(()), 
        };

        let ppn = satp & 0x00000FFFFFFFFFFF;
        let root_phys = ppn << 12;
        
        let vpn = [
            (virt >> 12) & 0x1FF,
            (virt >> 21) & 0x1FF,
            (virt >> 30) & 0x1FF,
            (virt >> 39) & 0x1FF,
        ];
        
        let mut table_phys = root_phys;
        
        for level in (1..levels).rev() {
            let table_ptr = phys_to_virt(table_phys) as *mut u64;
            table_phys = ensure_table_global(table_ptr, vpn[level])?;
        }
        
        let l0_ptr = phys_to_virt(table_phys) as *mut u64;
        let entry_flags = internal_map_get_flags(flags);
        let pte_ppn = (phys.0 >> 12) & 0x0FFFFFFFFFFF;
        let entry = (pte_ppn << 10) | entry_flags;
        
        *l0_ptr.add(vpn[0] as usize) = entry;
        
        tlb_flush_page(virt);
        Ok(())
    }
}

pub fn unmap_page(virt: u64) -> Result<Option<PhysFrame>, ()> {
     unsafe {
        let satp: u64;
        asm!("csrr {}, satp", out(reg) satp);
        let mode = satp >> 60;
        let ppn = satp & 0x00000FFFFFFFFFFF;
        let mut table_phys = ppn << 12;
        
        let levels = match mode {
            9 => 4,
            8 => 3,
             _ => return Err(()), 
        };

        let vpn = [
            (virt >> 12) & 0x1FF,
            (virt >> 21) & 0x1FF,
            (virt >> 30) & 0x1FF,
            (virt >> 39) & 0x1FF,
        ];
        
        for level in (1..levels).rev() {
             let table_ptr = phys_to_virt(table_phys) as *const u64;
             let entry = *table_ptr.add(vpn[level] as usize);
             if (entry & PTE_V) == 0 { return Ok(None); }
             if (entry & (PTE_R | PTE_W | PTE_X)) != 0 { return Ok(None); } // Huge page in path?
             
             let pte_ppn = (entry >> 10) & 0x0FFFFFFFFFFF;
             table_phys = pte_ppn << 12;
        }
        
        // L0
        let table_ptr = phys_to_virt(table_phys) as *mut u64;
        let entry = *table_ptr.add(vpn[0] as usize);
        
        if (entry & PTE_V) == 0 { return Ok(None); }
        
        // Clear it
        *table_ptr.add(vpn[0] as usize) = 0;
        tlb_flush_page(virt);
        
        let pte_ppn = (entry >> 10) & 0x0FFFFFFFFFFF;
        Ok(Some(PhysFrame(pte_ppn << 12)))
    }
}

pub fn translate(virt: u64) -> Option<PhysFrame> {
    unsafe {
        let satp: u64;
        asm!("csrr {}, satp", out(reg) satp);
        let mode = satp >> 60;
        let ppn = satp & 0x00000FFFFFFFFFFF; 
        let mut table_phys = ppn << 12;
        
        let levels = match mode {
            8 => 3, // Sv39
            9 => 4, // Sv48
            _ => return None,
        };

        let vpn = [
            (virt >> 12) & 0x1FF, // L0
            (virt >> 21) & 0x1FF, // L1
            (virt >> 30) & 0x1FF, // L2
            (virt >> 39) & 0x1FF, // L3
        ];
        
        for level in (0..levels).rev() {
             let table_ptr = phys_to_virt(table_phys) as *const u64;
             let entry = *table_ptr.add(vpn[level] as usize);
             
             if (entry & PTE_V) == 0 { return None; }
             
             if (entry & (PTE_R | PTE_W | PTE_X)) != 0 {
                 // Leaf
                 if level > 0 {
                     return None; // Huge page
                 }
                 // Level 0 leaf
                 let pte_ppn = (entry >> 10) & 0x0FFFFFFFFFFF;
                 return Some(PhysFrame(pte_ppn << 12));
             }
             
             // Next level
             let pte_ppn = (entry >> 10) & 0x0FFFFFFFFFFF;
             table_phys = pte_ppn << 12;
        }
        None
    }
}

pub fn map_bootheap_page(virt: u64, phys: u64, allocator: &mut BootFrameAllocator) {
    unsafe {
        let satp: u64;
        asm!("csrr {}, satp", out(reg) satp);
        let mode = satp >> 60;
        
        let levels = match mode {
            9 => 4, // Sv48
            8 => 3, // Sv39
             _ => panic!("RISC-V: Unsupported paging mode {} (Expected 8/Sv39 or 9/Sv48). SATP={:#x}", mode, satp),
        };

        let ppn = satp & 0x00000FFFFFFFFFFF;
        let root_phys = ppn << 12;
        
        let vpn = [
            (virt >> 12) & 0x1FF, // L0
            (virt >> 21) & 0x1FF, // L1
            (virt >> 30) & 0x1FF, // L2
            (virt >> 39) & 0x1FF, // L3
        ];
        
        let mut table_phys = root_phys;
        
        for level in (1..levels).rev() {
            let table_ptr = phys_to_virt(table_phys) as *mut u64;
            table_phys = ensure_table(table_ptr, vpn[level], allocator);
        }
        
        let l0_ptr = phys_to_virt(table_phys) as *mut u64;
        
        // Map L0 entry
        let flags = PTE_V | PTE_R | PTE_W | PTE_A | PTE_D | PTE_G;
        
        let pte_ppn = (phys >> 12) & 0x0FFFFFFFFFFF;
        let entry = (pte_ppn << 10) | flags;
        
        *l0_ptr.add(vpn[0] as usize) = entry;
        
        tlb_flush_page(virt);
    }
}

unsafe fn ensure_table(table: *mut u64, index: u64, allocator: &mut BootFrameAllocator) -> u64 {
    let entry = unsafe { *table.add(index as usize) };
    if (entry & PTE_V) != 0 {
        let pte_ppn = (entry >> 10) & 0x0FFFFFFFFFFF;
        pte_ppn << 12
    } else {
        let frame = allocator.alloc_frame().expect("OOM in bootheap map");
        unsafe {
            let virt_ptr = phys_to_virt(frame) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            
            let pte_ppn = (frame >> 12) & 0x0FFFFFFFFFFF;
            let new_entry = (pte_ppn << 10) | PTE_V; 
            *table.add(index as usize) = new_entry;
        }
        
        frame
    }
}

unsafe fn ensure_table_global(table: *mut u64, index: u64) -> Result<u64, ()> {
    let entry = unsafe { *table.add(index as usize) };
    if (entry & PTE_V) != 0 {
        let pte_ppn = (entry >> 10) & 0x0FFFFFFFFFFF;
        Ok(pte_ppn << 12)
    } else {
        let frame = FRAME_ALLOCATOR.with_lock(|alloc| {
            alloc.alloc_contiguous(1).map(|r| r.base)
        }).ok_or(())?;
        
        unsafe {
            let virt_ptr = phys_to_virt(frame.0) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            
            let pte_ppn = (frame.0 >> 12) & 0x0FFFFFFFFFFF;
            let new_entry = (pte_ppn << 10) | PTE_V; 
            *table.add(index as usize) = new_entry;
        }
        
        Ok(frame.0)
    }
}
