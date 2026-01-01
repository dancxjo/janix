use core::ptr::{read_volatile, write_volatile};

// QEMU virt GICv2 addresses
const GICD_BASE: u64 = 0x08000000;
const GICC_BASE: u64 = 0x08010000;

const GICD_CTLR: u64 = 0x000;
const GICD_ISENABLER: u64 = 0x100;
const GICD_IPRIORITYR: u64 = 0x400;
const GICD_ITARGETSR: u64 = 0x800;

const GICC_CTLR: u64 = 0x000;
const GICC_PMR: u64 = 0x004;
const GICC_EOIR: u64 = 0x010;
const GICC_IAR: u64 = 0x00C;

pub unsafe fn init() {
    // 1. Distributor: Enable
    write_volatile((GICD_BASE + GICD_CTLR) as *mut u32, 1);
    
    // 2. CPU Interface: Enable + Priority Mask
    write_volatile((GICC_BASE + GICC_PMR) as *mut u32, 0xF0); // Priority mask
    write_volatile((GICC_BASE + GICC_CTLR) as *mut u32, 1);   // Enable
}

pub unsafe fn enable_irq(id: u32) {
    let n = id / 32;
    let offset = id % 32;
    
    // Set Enable bit
    let addr = (GICD_BASE + GICD_ISENABLER + (n as u64 * 4)) as *mut u32;
    let val = read_volatile(addr);
    write_volatile(addr, val | (1 << offset));
    
    // Route to CPU0 (Target Register)
    // First 32 interrupts are local (SGI/PPI), ITARGETSR usually RO or ignored for them?
    // PPI (16-31) are per-CPU.
    // SPI (32+) need routing.
    // If id >= 32:
    if id >= 32 {
         let t_offset = (id / 4) * 4;
         let t_addr = (GICD_BASE + GICD_ITARGETSR + t_offset as u64) as *mut u32;
         // Byte-accessible? 
         // Assuming word access. Write 0x01010101 for CPU0 target to all 4 in this word?
         // For simplicity, skip proper targeting for now if QEMU defaults to CPU0.
    }
}

pub unsafe fn ack_irq() -> u32 {
    let id = read_volatile((GICC_BASE + GICC_IAR) as *const u32);
    id
}

pub unsafe fn eoi(id: u32) {
    write_volatile((GICC_BASE + GICC_EOIR) as *mut u32, id);
}
