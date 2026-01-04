//! x86_64 Timer and Interrupt Controller Integration.
//!
//! This module provides the timer initialization and EOI handling for x86_64.
//! Since we now use LAPIC for timer, this module:
//! 1. Disables the legacy PIC (8259) completely
//! 2. Initializes the IO-APIC for routing legacy IRQs
//! 3. Delegates timer operations to the LAPIC

use super::apic::{LAPIC, IOAPIC};
use x86_64::instructions::port::Port;

// PIC ports
const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

/// Global flag indicating whether LAPIC is active.
static mut USE_LAPIC: bool = false;

/// Initialize the timer subsystem (Phase 1: disable legacy PIC).
///
/// # Safety
/// Must be called exactly once during boot.
pub unsafe fn init() {
    // 1. Remap PICs to vectors 32-47 (needed even if disabled)
    remap_pics();

    // 2. Mask all IRQs on both PICs (disable legacy PIC)
    disable_pic();

    crate::serial::write(b"TIMER: Legacy PIC disabled\n");
}

/// Initialize LAPIC and IO-APIC (Phase 2: requires HHDM).
///
/// # Safety
/// Must be called after the HHDM is established.
pub unsafe fn init_lapic(hhdm_offset: u64) {
    // Initialize LAPIC first
    LAPIC.init_with_hhdm(hhdm_offset);
    USE_LAPIC = true;

    // Initialize IO-APIC and route legacy IRQs
    IOAPIC.init(hhdm_offset);
    
    // Route keyboard (IRQ 1) to vector 33
    let lapic_id = LAPIC.id() as u8;
    IOAPIC.route_irq(1, 33, lapic_id);
    
    // Route mouse (IRQ 12) to vector 44
    IOAPIC.route_irq(12, 44, lapic_id);
    
    crate::serial::write(b"TIMER: IO-APIC configured for PS/2\n");
}

/// Acknowledge timer interrupt (send EOI).
///
/// # Safety
/// Must be called from interrupt context.
pub unsafe fn ack() {
    if USE_LAPIC {
        LAPIC.send_eoi();
    } else {
        // Fallback to PIC EOI
        let mut cmd = Port::<u8>::new(PIC1_CMD);
        cmd.write(0x20);
    }
}

/// Remap PICs to vectors 32-47.
unsafe fn remap_pics() {
    let mut cmd1 = Port::<u8>::new(PIC1_CMD);
    let mut data1 = Port::<u8>::new(PIC1_DATA);
    let mut cmd2 = Port::<u8>::new(PIC2_CMD);
    let mut data2 = Port::<u8>::new(PIC2_DATA);

    let a1 = data1.read();
    let a2 = data2.read();

    // ICW1: Init + ICW4 needed
    cmd1.write(0x11);
    io_wait();
    cmd2.write(0x11);
    io_wait();

    // ICW2: Vector offsets (Master: 32, Slave: 40)
    data1.write(0x20);
    io_wait();
    data2.write(0x28);
    io_wait();

    // ICW3: Cascade identity
    data1.write(4);
    io_wait();
    data2.write(2);
    io_wait();

    // ICW4: 8086 mode
    data1.write(0x01);
    io_wait();
    data2.write(0x01);
    io_wait();

    // Restore masks temporarily
    data1.write(a1);
    data2.write(a2);
}

/// Disable the legacy PIC by masking all IRQs.
unsafe fn disable_pic() {
    let mut data1 = Port::<u8>::new(PIC1_DATA);
    let mut data2 = Port::<u8>::new(PIC2_DATA);
    data1.write(0xFF);
    data2.write(0xFF);
}

/// Small delay for PIC initialization.
#[inline]
unsafe fn io_wait() {
    let mut p = Port::<u8>::new(0x80);
    p.write(0);
}

/// Get the LAPIC timer frequency in Hz.
pub fn timer_frequency_hz() -> u32 {
    use crate::interrupt::InterruptController;
    LAPIC.timer_frequency_hz()
}

/// Get the LAPIC ID.
pub fn lapic_id() -> u32 {
    LAPIC.id()
}
