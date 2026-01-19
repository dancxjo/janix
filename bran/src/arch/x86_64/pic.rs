//! Legacy 8259 PIC Management
//!
//! This module provides functions to disable and mask the legacy 8259 PICs.
//! Required before IOAPIC can take over interrupt routing.

use kernel::{ioport_read_u8, ioport_write_u8};

// PIC ports
const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

// ICW1: Initialization Command Word 1
const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;

// ICW4: Initialization Command Word 4
const ICW4_8086: u8 = 0x01;

/// Remap PIC vectors to avoid conflicts with CPU exceptions (0-31).
/// Master PIC: vectors 0x20-0x27
/// Slave PIC: vectors 0x28-0x2F
/// Then mask all IRQs.
pub fn disable_pic() {
    // Save existing masks (for debugging)
    let _mask1 = ioport_read_u8(PIC1_DATA);
    let _mask2 = ioport_read_u8(PIC2_DATA);

    // ICW1: Start initialization sequence
    ioport_write_u8(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
    io_wait();
    ioport_write_u8(PIC2_CMD, ICW1_INIT | ICW1_ICW4);
    io_wait();

    // ICW2: Set vector offsets
    ioport_write_u8(PIC1_DATA, 0x20); // Master PIC starts at vector 0x20
    io_wait();
    ioport_write_u8(PIC2_DATA, 0x28); // Slave PIC starts at vector 0x28
    io_wait();

    // ICW3: Configure cascading
    ioport_write_u8(PIC1_DATA, 0x04); // Slave on IRQ2
    io_wait();
    ioport_write_u8(PIC2_DATA, 0x02); // Cascade identity
    io_wait();

    // ICW4: 8086 mode
    ioport_write_u8(PIC1_DATA, ICW4_8086);
    io_wait();
    ioport_write_u8(PIC2_DATA, ICW4_8086);
    io_wait();

    // Mask all IRQs on both PICs
    ioport_write_u8(PIC1_DATA, 0xFF);
    io_wait();
    ioport_write_u8(PIC2_DATA, 0xFF);
    io_wait();
}

/// Small I/O delay for PIC programming
#[inline]
fn io_wait() {
    // Write to unused port 0x80 for ~1µs delay
    ioport_write_u8(0x80, 0);
}

/// Send End-of-Interrupt to legacy PIC (not used when IOAPIC is active)
#[allow(dead_code)]
pub fn send_eoi(irq: u8) {
    if irq >= 8 {
        ioport_write_u8(PIC2_CMD, 0x20);
    }
    ioport_write_u8(PIC1_CMD, 0x20);
}
