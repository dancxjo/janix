use kernel::memory::boot_frame_alloc::BootFrameAllocator;
use kernel::memory::paging::PageFlags;
use kernel::memory::frame_alloc::{PhysFrame, FRAME_ALLOCATOR};
use core::arch::asm;
use crate::requests::HHDM_REQUEST;

// LoongArch64 PTE definitions
const PTE_V: u64 = 1 << 0;  // Valid
const PTE_D: u64 = 1 << 1;  // Dirty (implies Writable if D=1)
const PTE_PLV_MASK: u64 = 3 << 2;
const PTE_PLV_KERN: u64 = 0 << 2;
const PTE_PLV_USER: u64 = 3 << 2;
const PTE_MAT_MASK: u64 = 3 << 4;
const PTE_MAT_CC: u64 = 1 << 4; // Coherent Cached
const PTE_G: u64 = 1 << 6;  // Global
const PTE_HUGE: u64 = 1 << 7; // Huge Page (if supported at this level)
const PTE_NX: u64 = 1 << 63; // No Execute (Architecture dependent position, using 63 as common placeholder or high bit)
const PTE_RPLV: u64 = 1 << 63; // Unused but often restricted?

pub fn phys_to_virt_offset() -> u64 {
    HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0)
}

fn phys_to_virt(phys: u64) -> u64 {
    phys + phys_to_virt_offset()
}

pub fn tlb_flush_page(virt: u64) {
    unsafe {
        // invtlb 0x01, $r_addr (0x01 = current ASID, individual page)
        // format: invtlb op, rj, rk
        // op=4 (all), op=5 (all, g=1), op=6 (all, asid)
        // Linux uses invtlb 5 for full flush?
        // For page: invtlb 3?
        // Let's just flush all for safety and simplicity initially
        // asm!("invtlb 0x00, $r0, $r0"); // Invalidates everything?
        // 0x00 is invalid opcode for invtlb potentially?
        // Use 0x06 (INVTLB_ALL_G_ASID)? 
        // 0x04 = INVTLB_ALL
        asm!("invtlb 0x04, $r0, $r0");
    }
}

pub fn tlb_flush_all() {
    unsafe {
        asm!("invtlb 0x04, $r0, $r0");
    }
}

fn get_pgd() -> u64 {
    let mut pgd: u64;
    unsafe { asm!("csrrd {}, 0x19", out(reg) pgd); } // CSR_PGDL = 0x19
    pgd
}

// Convert abstract PageFlags to LoongArch PTE flags
fn flags_to_pte(flags: PageFlags) -> u64 {
    let mut hw = PTE_V | PTE_MAT_CC;

    // LoongArch: D bit controls Write.
    // If we want Writable, set D.
    if flags.contains(PageFlags::WRITABLE) {
        hw |= PTE_D;
    }
    
    // User accessible
    if flags.contains(PageFlags::USER_ACCESSIBLE) {
        hw |= PTE_PLV_USER;
    } else {
        hw |= PTE_PLV_KERN;
    }

    if flags.contains(PageFlags::GLOBAL) {
        hw |= PTE_G;
    }
    
    // NX is tricky. LoongArch has NX bit in some revisions.
    // Assuming bit 63 for now if needed, but 'D' covers write.
    // Read is implicit?
    
    hw
}

// Assuming 3-level paging (Sv39) which is standard for LoongArch64 with < 1TB RAM typical configs.
const LEVELS: usize = 3;

pub fn map_page(virt: u64, phys: PhysFrame, flags: PageFlags) -> Result<(), ()> {
    let root_phys = get_pgd();
    if root_phys == 0 { return Err(()); }

    let vpn = [
        (virt >> 12) & 0x1FF,
        (virt >> 21) & 0x1FF,
        (virt >> 30) & 0x1FF, 
    ];

    let mut table_phys = root_phys;

    for level in (1..LEVELS).rev() {
        let table_ptr = phys_to_virt(table_phys) as *mut u64;
        unsafe {
            table_phys = ensure_table_global(table_ptr, vpn[level])?;
        }
    }

    let l0_ptr = phys_to_virt(table_phys) as *mut u64;
    let entry_val = (phys.0 & 0xFFFFFFFFF000) | flags_to_pte(flags);
    unsafe {
        *l0_ptr.add(vpn[0] as usize) = entry_val;
    }
    tlb_flush_page(virt);
    Ok(())
}

pub fn unmap_page(virt: u64) -> Result<Option<PhysFrame>, ()> {
    let root_phys = get_pgd();
    if root_phys == 0 { return Err(()); }

    let vpn = [
        (virt >> 12) & 0x1FF,
        (virt >> 21) & 0x1FF,
        (virt >> 30) & 0x1FF, 
    ];

    let mut table_phys = root_phys;

    for level in (1..LEVELS).rev() {
        let table_ptr = phys_to_virt(table_phys) as *const u64;
        unsafe {
            let entry = *table_ptr.add(vpn[level] as usize);
            if (entry & PTE_V) == 0 { return Ok(None); }
             // Check if huge page? (Not handling unmap of HUGE for now)
            table_phys = entry & 0xFFFFFFFFF000;
        }
    }

    let l0_ptr = phys_to_virt(table_phys) as *mut u64;
    unsafe {
        let entry = *l0_ptr.add(vpn[0] as usize);
        if (entry & PTE_V) == 0 { return Ok(None); }
        
        *l0_ptr.add(vpn[0] as usize) = 0;
        tlb_flush_page(virt);
        
        // Extract phys address
        let phys = entry & 0xFFFFFFFFF000;
        Ok(Some(PhysFrame(phys)))
    }
}

pub fn translate(virt: u64) -> Option<PhysFrame> {
    let root_phys = get_pgd();
     if root_phys == 0 { return None; }

    let vpn = [
        (virt >> 12) & 0x1FF,
        (virt >> 21) & 0x1FF,
        (virt >> 30) & 0x1FF, 
    ];

    let mut table_phys = root_phys;
    
    for level in (0..LEVELS).rev() {
        let table_ptr = phys_to_virt(table_phys) as *const u64;
        unsafe {
            let entry = *table_ptr.add(vpn[level] as usize);
            if (entry & PTE_V) == 0 { return None; }
            
            // Check for leaf (Huge page or L0)
            if level == 0 {
                 let phys = (entry & 0xFFFFFFFFF000) | (virt & 0xFFF);
                 return Some(PhysFrame(phys));
            }
            
            table_phys = entry & 0xFFFFFFFFF000;
        }
    }
    None
}

pub fn map_bootheap_page(virt: u64, phys: u64, allocator: &mut BootFrameAllocator) {
    let root_phys = get_pgd();
    // In bootheap, we assume PGDL is set up.
    if root_phys == 0 { panic!("map_bootheap_page: PGDL is 0"); }
    
    let vpn = [
        (virt >> 12) & 0x1FF,
        (virt >> 21) & 0x1FF,
        (virt >> 30) & 0x1FF, 
    ];

    let mut table_phys = root_phys;

    for level in (1..LEVELS).rev() {
         let table_ptr = phys_to_virt(table_phys) as *mut u64;
         unsafe {
             table_phys = ensure_table(table_ptr, vpn[level], allocator);
         }
    }

    let l0_ptr = phys_to_virt(table_phys) as *mut u64;
    // Map L0
    let flags = PTE_V | PTE_D | PTE_MAT_CC | PTE_G; // Kernel RW
    let entry_val = (phys & 0xFFFFFFFFF000) | flags;
    
    unsafe {
        *l0_ptr.add(vpn[0] as usize) = entry_val;
    }
    tlb_flush_page(virt);
}

// Helpers

unsafe fn ensure_table(table: *mut u64, index: u64, allocator: &mut BootFrameAllocator) -> u64 {
    let entry = unsafe { *table.add(index as usize) };
    if (entry & PTE_V) != 0 {
        entry & 0xFFFFFFFFF000
    } else {
        let frame = allocator.alloc_frame().expect("OOM in bootheap map");
        unsafe {
            let virt_ptr = phys_to_virt(frame) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            
            let new_entry = (frame & 0xFFFFFFFFF000) | PTE_V | PTE_MAT_CC;
            *table.add(index as usize) = new_entry;
        }
        frame
    }
}

unsafe fn ensure_table_global(table: *mut u64, index: u64) -> Result<u64, ()> {
    let entry = unsafe { *table.add(index as usize) };
    if (entry & PTE_V) != 0 {
         Ok(entry & 0xFFFFFFFFF000)
    } else {
        let frame = FRAME_ALLOCATOR.with_lock(|alloc| {
            alloc.alloc_contiguous(1).map(|r| r.base)
        }).ok_or(())?;
        
        unsafe {
             let virt_ptr = phys_to_virt(frame.0) as *mut u8;
             core::ptr::write_bytes(virt_ptr, 0, 4096);
             
             let new_entry = (frame.0 & 0xFFFFFFFFF000) | PTE_V | PTE_MAT_CC; 
             *table.add(index as usize) = new_entry;
        }
        Ok(frame.0)
    }
}

