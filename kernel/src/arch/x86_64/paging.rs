use crate::memory::boot_frame_alloc::BootFrameAllocator;
use core::arch::asm;

static mut HHDM_OFFSET: u64 = 0;

pub fn init(hhdm_offset: u64) {
    unsafe { HHDM_OFFSET = hhdm_offset; }
}

const PAGE_PRESENT: u64 = 1;
const PAGE_WRITABLE: u64 = 2;
const PAGE_HUGE: u64 = 0x80;

/// Map a single 4KiB page for the boot heap.
/// This assumes the higher-level tables (PML4, PDP, PD) might need to be allocated.
/// Uses the provided allocator for new page tables.
pub fn map_bootheap_page(virt: u64, phys: u64, allocator: &mut BootFrameAllocator) {
    unsafe {
        let cr3: u64;
        asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags));
        
        let pml4_phys = cr3 & !0xFFF;
        let pml4_idx = (virt >> 39) & 0x1FF;
        let pdp_idx = (virt >> 30) & 0x1FF;
        let pd_idx = (virt >> 21) & 0x1FF;
        let pt_idx = (virt >> 12) & 0x1FF;

        let pml4 = phys_to_virt(pml4_phys) as *mut u64;
        let pdp_phys = ensure_table(pml4, pml4_idx, allocator);
        
        let pdp = phys_to_virt(pdp_phys) as *mut u64;
        let pd_phys = ensure_table(pdp, pdp_idx, allocator);
        
        let pd = phys_to_virt(pd_phys) as *mut u64;
        let pt_phys = ensure_table(pd, pd_idx, allocator);
        
        let pt = phys_to_virt(pt_phys) as *mut u64;
        // Set the PTE
        // Use Present | Writable | NoExecute (assume NX bit is 63, but let's just stick to minimal permissions first)
        // If system supports NX, we should set it, but for now strict minimal.
        let entry = phys | PAGE_PRESENT | PAGE_WRITABLE;
        *pt.add(pt_idx as usize) = entry;
        
        // Invalidate TLB for this page
        asm!("invlpg [{}]", in(reg) virt, options(nostack, preserves_flags));
    }
}

unsafe fn ensure_table(table: *mut u64, index: u64, allocator: &mut BootFrameAllocator) -> u64 {
    let entry = unsafe { *table.add(index as usize) };
    if (entry & PAGE_PRESENT) != 0 {
        // Check for huge page? If huge page is encountered where we expect a table, we are in trouble.
        // For kernel heap at -2GB, standard Limine might use 2MB pages for part of it, 
        // but `BOOTHEAP` is at a distinct range usually.
        // If huge bit is set, panic or return the addr.
        if (entry & PAGE_HUGE) != 0 {
             panic!("Huge page encountered while walking tables for bootheap!");
        }
        entry & 0x000FFFFFFFFFF000
    } else {
        // Allocate new table
        let frame = allocator.alloc_frame().expect("OOM allocating page table for bootheap");
        // Zero the new table!
        unsafe {
            let virt_ptr = phys_to_virt(frame) as *mut u8;
            core::ptr::write_bytes(virt_ptr, 0, 4096);
            
            // Link it
            let new_entry = frame | PAGE_PRESENT | PAGE_WRITABLE;
            *table.add(index as usize) = new_entry;
        }
        
        frame
    }
}

unsafe fn phys_to_virt(phys: u64) -> u64 {
    unsafe { phys + HHDM_OFFSET }
}
