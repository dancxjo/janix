use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicU64, Ordering};

// Physical addresses for mapping
pub const GICD_PHYS: u64 = 0x08000000;
pub const GICC_PHYS: u64 = 0x08010000;

static GICD_BASE: AtomicU64 = AtomicU64::new(0);
static GICC_BASE: AtomicU64 = AtomicU64::new(0);

const GICD_CTLR: u64 = 0x000;
const GICD_ISENABLER: u64 = 0x100;
const GICD_ITARGETSR: u64 = 0x800;

const GICC_CTLR: u64 = 0x000;
const GICC_PMR: u64 = 0x004;
const GICC_EOIR: u64 = 0x010;
const GICC_IAR: u64 = 0x00C;

pub unsafe fn init(dist_base: u64, cpu_base: u64) {
    GICD_BASE.store(dist_base, Ordering::Relaxed);
    GICC_BASE.store(cpu_base, Ordering::Relaxed);

    // 1. Distributor: Enable
    write_volatile((dist_base + GICD_CTLR) as *mut u32, 1);
    
    // 2. CPU Interface: Enable + Priority Mask
    write_volatile((cpu_base + GICC_PMR) as *mut u32, 0xF0); // Priority mask
    write_volatile((cpu_base + GICC_CTLR) as *mut u32, 1);   // Enable
}

fn dist() -> u64 {
    GICD_BASE.load(Ordering::Relaxed)
}

fn cpu() -> u64 {
    GICC_BASE.load(Ordering::Relaxed)
}

pub unsafe fn enable_irq(id: u32) {
    let base = dist();
    if base == 0 { return; }

    let n = id / 32;
    let offset = id % 32;
    
    // Set Enable bit
    let addr = (base + GICD_ISENABLER + (n as u64 * 4)) as *mut u32;
    let val = read_volatile(addr);
    write_volatile(addr, val | (1 << offset));
    
    // Route to CPU0 (Target Register)
    if id >= 32 {
         let t_offset = (id / 4) * 4;
         let _t_addr = (base + GICD_ITARGETSR + t_offset as u64) as *mut u32;
         // TODO: Set target
    }
}

pub unsafe fn ack_irq() -> u32 {
    let base = cpu();
    if base == 0 { return 0x3ff; } // Spurious
    read_volatile((base + GICC_IAR) as *const u32)
}

pub unsafe fn eoi(id: u32) {
    let base = cpu();
    if base == 0 { return; }
    write_volatile((base + GICC_EOIR) as *mut u32, id);
}
