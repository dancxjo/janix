use crate::Bridge;
use kernel::bridge::HardwareBridge;

static mut IOAPIC_ADDR: u64 = 0;

// IOAPIC Registers
const IOREGSEL: u32 = 0x00;
const IOWIN: u32 = 0x10;

// IOAPIC Redirection Table Base
const IOREDTBL: u32 = 0x10;

pub unsafe fn init(addr: u64) {
    IOAPIC_ADDR = addr;
    let bridge = Bridge;
    bridge.log("IOAPIC: Init\n");
}

unsafe fn read(reg: u32) -> u32 {
    use core::sync::atomic::Ordering;
    let hhdm = crate::HHDM_OFFSET.load(Ordering::Relaxed);
    let base = kernel::platform::acpi::to_virt(IOAPIC_ADDR, hhdm).as_ptr() as *mut u32;
    // Write Register Index to IOREGSEL
    core::ptr::write_volatile(base, reg);
    // Read Value from IOWIN
    core::ptr::read_volatile(base.add(4)) // 0x10 / 4 = 4 u32s
}

unsafe fn write(reg: u32, val: u32) {
    use core::sync::atomic::Ordering;
    let hhdm = crate::HHDM_OFFSET.load(Ordering::Relaxed);
    let base = kernel::platform::acpi::to_virt(IOAPIC_ADDR, hhdm).as_ptr() as *mut u32;
    core::ptr::write_volatile(base, reg);
    core::ptr::write_volatile(base.add(4), val);
}

/// Route a GSI (Global System Interrupt) to a CPU Vector.
pub unsafe fn set_irq_vector(gsi: u32, vector: u8, dest_apic_id: u8) {
    // Each Entry is 64-bits (2 registers)
    let low_index = IOREDTBL + (gsi * 2);
    let high_index = low_index + 1;

    // Lower 32-bits:
    // Vector (0-7), Delivery Mode (8-10), Dest Mode (11), Status (12),
    // Polarity (13), Remote IRR (14), Trigger Mode (15), Mask (16)

    // Default: Fixed delivery (0), Physical Dest (0), Active High (0), Edge (0), Unmasked (0)
    let low_val = vector as u32;

    // Upper 32-bits: Dest Field (56-63)
    let high_val = (dest_apic_id as u32) << 24;

    write(high_index, high_val);
    write(low_index, low_val);
}
