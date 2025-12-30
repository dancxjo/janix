use core::ptr::{read_volatile, write_volatile};
use kernel::bridge::CpuBridge;

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

    // Enable Group 0 and Group 1
    write_reg(GICC_BASE_VIRT, GICC_CTLR, 3); // Enable Grp0 and Grp1

    // Enable Timer IRQ (ID 30 for Non-Secure Physical, or 27 for Virtual)
    // QEMU usually maps CNTP (Physical EL1) to 30.

    // Set to Group 1 (Non-Secure)
    let old_group = read_reg(GICD_BASE_VIRT, GICD_IGROUPR);
    write_reg(GICD_BASE_VIRT, GICD_IGROUPR, old_group | (1 << 30));

    set_priority(30, 0x00); // Priority 0 (High), unmasked by PMR 0xF0
    enable_irq(30);

    // Barrier to ensure GIC config is visible
    core::arch::asm!("dsb sy");
    crate::bridge::Bridge.log("GIC: Init Complete\n");
}

const GICD_IGROUPR: u64 = 0x080;
const GICD_IPRIORITYR: u64 = 0x400;

pub unsafe fn set_priority(id: u32, priority: u8) {
    let offset = GICD_IPRIORITYR + (id as u64); // Byte accessible
                                                // Note: GIC registers are usually 32-bit aligned. Byte access might not be supported directly by write_volatile of u8
                                                // depending on implementation, but standard GIC allows byte access for priority registers.
                                                // However, write_reg uses u32. Let's use RMW on 32-bit register.

    let reg_offset = GICD_IPRIORITYR + (id / 4) as u64 * 4;
    let shift = (id % 4) * 8;
    let mask = 0xFF << shift;
    let val = (priority as u32) << shift;

    let old = read_reg(GICD_BASE_VIRT, reg_offset);
    let new = (old & !mask) | val;
    write_reg(GICD_BASE_VIRT, reg_offset, new);
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
