use core::ptr::{read_volatile, write_volatile};

// QEMU Virt Machine GICv2 Addresses
pub const GIC_DIST_BASE: u64 = 0x08000000;
pub const GIC_CPU_BASE: u64 = 0x08010000;

// Offsets
const GICD_CTLR: u64 = 0x000;
const GICD_ISENABLER: u64 = 0x100;

const GICC_CTLR: u64 = 0x000;
const GICC_PMR: u64 = 0x004;
const GICC_IAR: u64 = 0x00C;
const GICC_EOIR: u64 = 0x010;

static mut GICD_BASE_VIRT: u64 = 0;
static mut GICC_BASE_VIRT: u64 = 0;

pub unsafe fn init(hhdm_offset: u64) {
    GICD_BASE_VIRT = GIC_DIST_BASE + hhdm_offset;
    GICC_BASE_VIRT = GIC_CPU_BASE + hhdm_offset;

    // Dist: Enable
    write_reg(GICD_BASE_VIRT, GICD_CTLR, 1);

    // CPU: Enable | Priority Mask 0xF0 (Allow all)
    write_reg(GICC_BASE_VIRT, GICC_PMR, 0xF0); // Priority Mask
    write_reg(GICC_BASE_VIRT, GICC_CTLR, 1);   // Enable

    // Enable Timer IRQ (ID 30 for Non-Secure Physical, or 27 for Virtual)
    // QEMU usually maps CNTP (Physical EL1) to 30.
    enable_irq(30);
}

unsafe fn write_reg(base: u64, offset: u64, val: u32) {
    let ptr = (base + offset) as *mut u32;
    write_volatile(ptr, val);
}

unsafe fn read_reg(base: u64, offset: u64) -> u32 {
    let ptr = (base + offset) as *const u32;
    read_volatile(ptr)
}

pub unsafe fn enable_irq(id: u32) {
    let n = id / 32;
    let offset = GICD_ISENABLER + (n as u64 * 4);
    let shift = id % 32;
    let old = read_reg(GICD_BASE_VIRT, offset);
    write_reg(GICD_BASE_VIRT, offset, old | (1 << shift));
}

/// Acknowledge Interrupt. Returns (ID, CPUID).
pub unsafe fn acknowledge_irq() -> u32 {
    read_reg(GICC_BASE_VIRT, GICC_IAR)
}

/// End Of Interrupt.
pub unsafe fn end_of_irq(id: u32) {
    write_reg(GICC_BASE_VIRT, GICC_EOIR, id);
}
