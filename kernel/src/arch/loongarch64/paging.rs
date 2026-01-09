use crate::memory::boot_frame_alloc::BootFrameAllocator;
use crate::memory::paging::PageFlags;
use crate::memory::frame_alloc::PhysFrame;
use core::arch::asm;

static mut HHDM_OFFSET: u64 = 0;
static mut LEVELS: usize = 3;

pub fn init(_hhdm_offset: u64) {
    unsafe { 
        // Configure DMW1 for direct mapping (Cached, PLV0) -> 0x9000...
        // Virt Seg: 0x9 (1001), MAT: 1 (CC), PLV0: 1
        let dmw_val: u64 = 0x9000_0000_0000_0011;
        asm!("csrwr {}, 0x181", in(reg) dmw_val); // DMW1
        
        // Use DMW1 window for phys_to_virt instead of Limine's HHDM
        HHDM_OFFSET = 0x9000_0000_0000_0000;
        let hhdm = HHDM_OFFSET;
        crate::kinfo!("LoongArch paging init: DMW1 setup, HHDM override={:#x}", hhdm);
        
        let pwch: u64;
        asm!("csrrd {}, 0x1d", out(reg) pwch);
        if pwch != 0 {
            LEVELS = 4;
            crate::kinfo!("Detected 4-level paging (PWCH={:#x})", pwch);
        } else {
            crate::kinfo!("Detected 3-level paging");
        }
    }
}
 
unsafe fn phys_to_virt(phys: u64) -> u64 {
    // Safety: Caller must ensure HHDM_OFFSET is initialized
    unsafe { phys | HHDM_OFFSET }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressSpace {
    pub root: PhysFrame,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self { root: PhysFrame(0) }
    }
    
    pub fn active() -> Self {
        let val: u64;
        unsafe { asm!("csrrd {}, 0x1a", out(reg) val); }
        Self { root: PhysFrame(val) }
    }
    
    pub fn switch(&self) {
    }
    
    pub fn map_page(&mut self, _virt: u64, _phys: PhysFrame, _flags: PageFlags) -> Result<(), ()> {
        Ok(())
    }
    
    pub fn unmap_page(&mut self, _virt: u64) -> Result<Option<PhysFrame>, ()> {
        Ok(None)
    }
    
    pub fn translate(&self, _virt: u64) -> Option<PhysFrame> {
        None
    }
}

pub fn tlb_flush_page(_virt: u64) {
    unsafe {
        asm!("invtlb 0x0, $r0, $r0"); 
    }
}

pub fn tlb_flush_all() {
    unsafe {
        asm!("invtlb 0x0, $r0, $r0");
    }
}

// LoongArch PTE bits
const PTE_V: u64 = 1 << 0;
const PTE_D: u64 = 1 << 1;     // Dirty/Writable
const PTE_MAT_CC: u64 = 1 << 4; // Cache Coherent

pub fn map_bootheap_page(virt: u64, phys: u64, allocator: &mut BootFrameAllocator) {
    unsafe {
        let levels = LEVELS;
        crate::kinfo!("map_bootheap_page: virt={:#x} phys={:#x} levels={}", virt, phys, levels);
        
        let pgdh: u64;
        asm!("csrrd {}, 0x1a", out(reg) pgdh);
        crate::kinfo!("PGDH={:#x}", pgdh);
        
        let mut table_phys = pgdh & !0xFFF; // Align
        
        if levels >= 4 {
             let l3_idx = (virt >> 39) & 0x1FF;
             let l3_ptr = phys_to_virt(table_phys) as *mut u64;
             crate::kinfo!("L3: idx={} ptr={:p}", l3_idx, l3_ptr);
             table_phys = ensure_table_boot(l3_ptr, l3_idx, allocator);
        }

        let l2_idx = (virt >> 30) & 0x1FF;
        let l2_ptr = phys_to_virt(table_phys) as *mut u64;
        crate::kinfo!("L2: idx={} ptr={:p}", l2_idx, l2_ptr);
        let l1_phys = ensure_table_boot(l2_ptr, l2_idx, allocator);
        
        let l1_ptr = phys_to_virt(l1_phys) as *mut u64;
        let l1_idx = (virt >> 21) & 0x1FF;
        let l0_phys = ensure_table_boot(l1_ptr, l1_idx, allocator);
        
        let l0_ptr = phys_to_virt(l0_phys) as *mut u64;
        let l0_idx = (virt >> 12) & 0x1FF;
        
        // Leaf
        let entry = phys | PTE_V | PTE_D | PTE_MAT_CC;
        
        *l0_ptr.add(l0_idx as usize) = entry;
        
        tlb_flush_page(virt);
    }
}

unsafe fn ensure_table_boot(table: *mut u64, index: u64, allocator: &mut BootFrameAllocator) -> u64 {
    unsafe {
        crate::kinfo!("Reading table entry {:p}", table.add(index as usize));
        let entry = *table.add(index as usize);
        crate::kinfo!("Entry: {:#x}", entry);
        if (entry & PTE_V) != 0 || entry != 0 {
            // Valid or at least present (Limine weirdness workaround)
            // If V=0 but entry!=0, it might be a table that the TLB Refill handler knows about.
            crate::kinfo!("Reusing existing table at {:#x}", entry);
            entry & 0x0000_FFFF_FFFF_F000
        } else {
            crate::kinfo!("Allocating table...");
            let frame = allocator.alloc_frame().expect("OOM allocating page table for bootheap");
            crate::kinfo!("Allocated frame {:#x}", frame);
            let virt_ptr = phys_to_virt(frame) as *mut u64;
            
            // Manual zeroing
            let slice = core::slice::from_raw_parts_mut(virt_ptr as *mut u64, 512);
            for slot in slice { *slot = 0; }
            
            let new_entry = frame | PTE_V;
            *table.add(index as usize) = new_entry;
            
            frame
        }
    }
}

pub fn test_paging() {}
