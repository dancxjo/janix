use core::arch::asm;
use alloc::alloc::{alloc, Layout};
use bridge_aarch64::paging::{
    PTE_VALID, PTE_TABLE, PTE_PAGE, PTE_ATTR_DEVICE, PTE_AF, PTE_SH_INNER, 
    PTE_AP_RW_EL1, PTE_UXN, PTE_PXN, 
};

static mut HHDM_OFFSET: u64 = 0;

pub unsafe fn init(hhdm: u64) {
    HHDM_OFFSET = hhdm;
    
    // Configure MAIR_EL1
    // Attr 0: Normal Write-Back (0xFF)
    // Attr 1: Device-nGnRnE (0x00)
    let mair: u64 = 0x00_FF; 
    asm!("msr mair_el1, {}", in(reg) mair);

    // Register hook
    bridge_aarch64::paging::UPDATE_FLAGS_FN = Some(update_page_flags);
}

fn allocate_frame() -> Option<(u64, u64)> {
    // Returns (Phys, Virt)
    let layout = Layout::from_size_align(4096, 4096).ok()?;
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            return None;
        }
        // Zero it
        core::ptr::write_bytes(ptr, 0, 4096);
        
        let virt = ptr as u64;
        let hhdm = HHDM_OFFSET;
        // Verify virt is in HHDM? 
        // We assume Global Allocator allocates from HHDM region (Limine Heap).
        // If so, Phys = Virt - HHDM.
        // HHDM mapping: Virt = Phys + HHDM.
        if virt < hhdm {
            // Panic or error?
            // If alloc returns low address, it's not HHDM. 
            // But main.rs initializes "heap" using limine heap which is usually high address.
            return None; 
        }
        let phys = virt - hhdm;
        Some((phys, virt))
    }
}

const TABLE_MASK: u64 = 0x0000_FFFF_FFFF_F000;

unsafe fn read_ttbr1_el1() -> u64 {
    let val: u64;
    asm!("mrs {}, ttbr1_el1", out(reg) val);
    val
}

unsafe fn flush_tlb() {
    asm!("tlbi vmalle1is");
    asm!("dsb ish");
    asm!("isb");
}

pub unsafe fn map_device_region(phys: u64, size: usize) {
    let hhdm_offset = HHDM_OFFSET;
    let start = phys;
    let end = phys + size as u64;

    // Align start/end to 4KB
    let start_page = start & !0xFFF;
    let end_page = (end + 0xFFF) & !0xFFF;

    let ttbr1 = read_ttbr1_el1();

    let mut tcr: u64;
    asm!("mrs {}, tcr_el1", out(reg) tcr);
    let t1sz = (tcr >> 16) & 0x3F;
    let levels = if t1sz == 16 { 4 } else { 3 };

    let root_table_phys = ttbr1 & TABLE_MASK;

    let mut current_addr = start_page;
    while current_addr < end_page {
        let virt = current_addr + hhdm_offset;
        map_page(root_table_phys, current_addr, virt, levels);
        current_addr += 4096;
    }

    flush_tlb();
}

unsafe fn map_page(root_table_phys: u64, phys: u64, virt: u64, levels: usize) {
    let indexes = [
        (virt >> 39) & 0x1FF, // L0
        (virt >> 30) & 0x1FF, // L1
        (virt >> 21) & 0x1FF, // L2
        (virt >> 12) & 0x1FF, // L3
    ];

    let start_level = if levels == 4 { 0 } else { 1 };

    let mut table_phys = root_table_phys;

    for level in start_level..3 {
        let index = indexes[level] as usize;
        let table_ptr = (table_phys + HHDM_OFFSET) as *mut u64;
        let entry = table_ptr.add(index).read();

        if (entry & PTE_VALID) == 0 {
            let (frame_phys, _frame_virt) = allocate_frame().expect("OOM mapping device");
            // frame_virt is already zeroed by allocate_frame

            let new_entry = frame_phys | PTE_TABLE | PTE_VALID;
            table_ptr.add(index).write(new_entry);

            table_phys = frame_phys;
        } else {
            if (level == 1 || level == 2) && (entry & 0x2) == 0 {
                return;
            }
            table_phys = entry & TABLE_MASK;
        }
    }

    let index = indexes[3] as usize;
    let table_ptr = (table_phys + HHDM_OFFSET) as *mut u64;

    let entry = phys
        | PTE_VALID
        | PTE_PAGE
        | PTE_AF
        | PTE_SH_INNER
        | PTE_AP_RW_EL1
        | PTE_UXN
        | PTE_PXN
        | PTE_ATTR_DEVICE;

    table_ptr.add(index).write(entry);
}

pub fn update_page_flags(virt: u64, set: u64, clear: u64) {
    unsafe {
        // We assume kernel paging (TTBR1) or user (TTBR0)
        let is_ttbr0 = (virt as i64) >= 0;

        let root_phys = if is_ttbr0 {
            let val: u64;
            asm!("mrs {}, ttbr0_el1", out(reg) val);
            val & TABLE_MASK
        } else {
            read_ttbr1_el1() & TABLE_MASK
        };

        if root_phys == 0 {
            return;
        }

        let mut tcr: u64;
        asm!("mrs {}, tcr_el1", out(reg) tcr);
        let t1sz = (tcr >> 16) & 0x3F;
        let levels = if t1sz == 16 { 4 } else { 3 };

        let indexes = [
            (virt >> 39) & 0x1FF,
            (virt >> 30) & 0x1FF,
            (virt >> 21) & 0x1FF,
            (virt >> 12) & 0x1FF,
        ];

        let start_level = if levels == 4 { 0 } else { 1 };
        let mut table_phys = root_phys;

        for level in start_level..3 {
            let index = indexes[level] as usize;
            let table_ptr = (table_phys + HHDM_OFFSET) as *mut u64;
            let entry = table_ptr.add(index).read();

            if (entry & PTE_VALID) == 0 {
                return; 
            }
            if (level == 1 || level == 2) && (entry & 0x2) == 0 {
                return;
            }

            table_phys = entry & TABLE_MASK;
        }

        // L3
        let index = indexes[3] as usize;
        let table_ptr = (table_phys + HHDM_OFFSET) as *mut u64;
        let mut entry = table_ptr.add(index).read();

        if (entry & PTE_VALID) != 0 {
            entry = (entry & !clear) | set;
            table_ptr.add(index).write(entry);
            flush_tlb(); 
        }
    }
}
