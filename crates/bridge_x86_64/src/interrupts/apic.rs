use core::arch::asm;
use crate::Bridge;

// Local APIC Registers (Offsets)
const LAPIC_ID: u32 = 0x020;
const LAPIC_EOI: u32 = 0x0B0;
const LAPIC_SIV: u32 = 0x0F0; // Spurious Interrupt Vector
const LAPIC_ICR_LO: u32 = 0x300;
const LAPIC_ICR_HI: u32 = 0x310;
const LAPIC_LVT_TIMER: u32 = 0x320;
const LAPIC_TIMER_INIT: u32 = 0x380;
const LAPIC_TIMER_CUR: u32 = 0x390;
const LAPIC_TIMER_DIV: u32 = 0x3E0;

static mut LAPIC_BASE: u64 = 0;

pub unsafe fn init(base_phys: u64) {
    let bridge = Bridge;
    
    // We need to map this? Ideally Main maps MMIO region.
    // We rely on HHDM for access if base_phys is low enough?
    // No, APIC is usually high (0xFEE00000). 
    // We must use the HHDM offset + phys address mechanism.
    // But `acpi::to_virt` already does that.
    
    // We assume `acpi::LOCAL_APIC_ADDR` was set (passed here as argument)
    LAPIC_BASE = base_phys; 
    
    // 1. Enable LAPIC via SIV (Spurious Interrupt Vector)
    // Set bit 8 (Enable) and vector 0xFF (Spurious)
    write_reg(LAPIC_SIV, 0x1FF);
    
    bridge.log("LAPIC: Enabled\n");
}

pub unsafe fn end_of_interrupt() {
    write_reg(LAPIC_EOI, 0);
}

pub unsafe fn id() -> u32 {
    read_reg(LAPIC_ID) >> 24
}

// Read/Write Registers
unsafe fn read_reg(offset: u32) -> u32 {
    let phys = LAPIC_BASE + offset as u64;
    let virt = crate::acpi::to_virt(phys).as_ptr() as *const u32;
    core::ptr::read_volatile(virt)
}

unsafe fn write_reg(offset: u32, val: u32) {
    let phys = LAPIC_BASE + offset as u64;
    let virt = crate::acpi::to_virt(phys).as_ptr() as *mut u32;
    core::ptr::write_volatile(virt, val);
}

pub unsafe fn enable_timer(vector: u8) {
    let bridge = Bridge;
    // 1. Divide by 16 (Value 0x3)
    write_reg(LAPIC_TIMER_DIV, 0x3);
    
    // 2. Set Vector & Periodic Mode (Bit 17)
    // 0x20000 = Periodic
    let lvt = (vector as u32) | 0x20000;
    write_reg(LAPIC_LVT_TIMER, lvt);
    
    // 3. Calibrate?
    // We'll just set a large count for now to verify.
    // 10ms approx? Assumes APIC bus speed.
    // If bus is 100MHz, div 16 = 6.25MHz. 10ms = 62,500 ticks.
    let init_count = 100_0000; 
    write_reg(LAPIC_TIMER_INIT, init_count);
    
    bridge.log("LAPIC: Timer Enabled\n");
}
