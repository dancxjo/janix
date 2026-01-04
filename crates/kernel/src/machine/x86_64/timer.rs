// use x86_64::instructions::port::Port;
// Local APIC Base Address MSR
const _IA32_APIC_BASE: u32 = 0x1b;

// Offsets
const _APIC_EOI: u32 = 0x0b0;
const _APIC_SVR: u32 = 0x0f0;
const _APIC_LVT_TIMER: u32 = 0x320;
const _APIC_TIMER_INIT: u32 = 0x380;
const _APIC_TIMER_CURRENT: u32 = 0x390;
const _APIC_TIMER_DIV: u32 = 0x3e0;

use x86_64::instructions::port::Port;

// PIC ports
const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

pub unsafe fn init() {
    // 1. Initialize PIT (already correct)
    init_pit();

    // 2. Remap PICs (Master -> 32, Slave -> 40)
    remap_pics();

    // 3. Unmask IRQ0 on PIC
    // Unmask IRQ0 (bit 0 of PIC1_DATA)
    let mut data = Port::<u8>::new(PIC1_DATA);
    let mask = data.read();
    // Ensure only IRQ0 is unmasked? Or preserve others?
    // Usually mask all others.
    // Unmask IRQ0 (Timer) and IRQ1 (Keyboard)
    // Safety: Mask 0xFC (11111100).
    data.write(mask & 0xFC);
}

unsafe fn remap_pics() {
    let mut cmd1 = Port::<u8>::new(PIC1_CMD);
    let mut data1 = Port::<u8>::new(PIC1_DATA);
    let mut cmd2 = Port::<u8>::new(PIC2_CMD);
    let mut data2 = Port::<u8>::new(PIC2_DATA);

    let _a1 = data1.read();
    let _a2 = data2.read();

    // ICW1: Init
    cmd1.write(0x11);
    io_wait();
    cmd2.write(0x11);
    io_wait();

    // ICW2: Offset
    data1.write(0x20); // 32
    io_wait();
    data2.write(0x28); // 40
    io_wait();

    // ICW3: Cascade
    data1.write(4);
    io_wait();
    data2.write(2);
    io_wait();

    // ICW4: 8086
    data1.write(0x01);
    io_wait();
    data2.write(0x01);
    io_wait();

    // Restore masks (or mask all?)
    // data1.write(a1);
    // data2.write(a2);

    // Mask all for safety, then unmask specific ones later.
    data1.write(0xff);
    data2.write(0xff);
}

unsafe fn io_wait() {
    let mut p = Port::<u8>::new(0x80);
    p.write(0);
}

unsafe fn init_pit() {
    let mut command = Port::<u8>::new(0x43);
    command.write(0x36); // Channel 0, LSB/MSB, Mode 3 (Square Wave), Binary

    // 1193182 / 100 Hz = 11931
    let divisor = 11931u16;
    let mut data = Port::<u8>::new(0x40);
    data.write((divisor & 0xff) as u8);
    data.write((divisor >> 8) as u8);
}

pub unsafe fn ack() {
    // Send EOI to PIC1
    let mut cmd = Port::<u8>::new(PIC1_CMD);
    cmd.write(0x20);
}
