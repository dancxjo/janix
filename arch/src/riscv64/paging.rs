use core::arch::asm;
use kernel::memory::{allocate_frame, get_hhdm_offset};

const PTE_V: u64 = 1 << 0;
const PTE_R: u64 = 1 << 1;
const PTE_W: u64 = 1 << 2;
const PTE_X: u64 = 1 << 3;
const PTE_U: u64 = 1 << 4;
const PTE_G: u64 = 1 << 5;
const PTE_A: u64 = 1 << 6;
const PTE_D: u64 = 1 << 7;

// Valid leaf for MMIO: V | R | W | A | D
const PTE_MMIO: u64 = PTE_V | PTE_R | PTE_W | PTE_A | PTE_D;

pub fn init() {}

unsafe fn read_satp() -> u64 {
    let val: u64;
    asm!("csrr {}, satp", out(reg) val);
    val
}

unsafe fn flush_tlb() {
    asm!("sfence.vma x0, x0");
}

pub unsafe fn map_device_region(phys: u64, size: usize, hhdm_offset: u64) {
    let start = phys;
    let end = phys + size as u64;

    // Align to 4KB
    let start_page = start & !0xFFF;
    let end_page = (end + 0xFFF) & !0xFFF;

    let satp = read_satp();
    let mode = satp >> 60;

    let levels = match mode {
        8 => 3, // Sv39
        9 => 4, // Sv48
        0 => return, // Bare mode
        _ => panic!("Unknown SATP mode: {}", mode),
    };

    // satp PPN is bits 0-43
    let root_table_phys = (satp & 0x0000_0FFFF_FFFF_FFF) << 12;

    let mut current_addr = start_page;
    while current_addr < end_page {
        let virt = current_addr + hhdm_offset;
        map_page(root_table_phys, current_addr, virt, levels);
        current_addr += 4096;
    }

    flush_tlb();
}

unsafe fn map_page(root_table_phys: u64, phys: u64, virt: u64, levels: usize) {
    let mut table_phys = root_table_phys;

    // Iterate from levels-1 down to 1
    for level in (1..levels).rev() {
        let shift = 12 + level * 9;
        let index = (virt >> shift) & 0x1FF;

        let table_ptr = (table_phys + kernel::memory::get_hhdm_offset()) as *mut u64;
        let entry = table_ptr.add(index as usize).read();

        if (entry & PTE_V) == 0 {
            // Allocate
            let frame = allocate_frame().expect("OOM mapping device RISC-V");
            // frame.start_address is u64
            let frame_phys = frame.start_address;
            let frame_ptr = (frame_phys + kernel::memory::get_hhdm_offset()) as *mut u64;
            frame_ptr.write_bytes(0, 512);

            // PPN is bits 10-53
            let ppn = (frame_phys >> 12) << 10;
            // Valid, pointers have R=0, W=0, X=0
            let new_entry = ppn | PTE_V;
            table_ptr.add(index as usize).write(new_entry);

            table_phys = frame_phys;
        } else if (entry & (PTE_R | PTE_W | PTE_X)) != 0 {
             // Block/Huge page
             return;
        } else {
             // Extract phys from PPN (bits 10-53)
             let ppn = (entry >> 10) & 0x0FFF_FFFF_FFFF;
             table_phys = ppn << 12;
        }
    }

    // Leaf (Level 0)
    let index = (virt >> 12) & 0x1FF;
    let table_ptr = (table_phys + kernel::memory::get_hhdm_offset()) as *mut u64;

    let ppn = (phys >> 12) << 10;
    let entry = ppn | PTE_MMIO;
    table_ptr.add(index as usize).write(entry);
}
