use core::arch::asm;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const PIC_EOI: u8 = 0x20;

/// Initialize the 8259 PICs and remap them to offsets 32 and 40.
/// This unmasks IRQ0 (Timer) only.
pub unsafe fn init() {
    // Save masks
    let _m1 = inb(PIC1_DATA);
    let _m2 = inb(PIC2_DATA);

    // ICW1: Init
    outb(PIC1_COMMAND, 0x11);
    wait();
    outb(PIC2_COMMAND, 0x11);
    wait();

    // ICW2: Vector Offset
    outb(PIC1_DATA, 32); // Master -> 32
    wait();
    outb(PIC2_DATA, 40); // Slave -> 40
    wait();

    // ICW3: Cascading
    outb(PIC1_DATA, 4);
    wait();
    outb(PIC2_DATA, 2);
    wait();

    // ICW4: 8086 mode
    outb(PIC1_DATA, 0x01);
    wait();
    outb(PIC2_DATA, 0x01);
    // Verify Wait
    wait();

    // Restore masks? No, we want to control them explicitly.
    // Unmask Timer (IRQ0) and Keyboard (IRQ1) and Cascade (IRQ2).
    outb(PIC1_DATA, 0b11111000); // 0xF8
                                 // Unmask Mouse (IRQ12 = Slave IRQ4)
    outb(PIC2_DATA, 0b11101111); // 0xEF

    init_pit();
}

unsafe fn init_pit() {
    const PIT_COMMAND: u16 = 0x43;
    const PIT_CHANNEL0: u16 = 0x40;
    const PIT_FREQUENCY: u32 = 1193182;
    const TARGET_FREQUENCY: u32 = 100;
    let divisor = PIT_FREQUENCY / TARGET_FREQUENCY;

    // Command: Channel 0, Access Lo/Hi, Mode 3 (Square Wave), Binary
    outb(PIT_COMMAND, 0x36);
    // Low byte
    outb(PIT_CHANNEL0, (divisor & 0xFF) as u8);
    // High byte
    outb(PIT_CHANNEL0, ((divisor >> 8) & 0xFF) as u8);
}

pub unsafe fn notify_end_of_interrupt(interrupt_id: u8) {
    let irq = interrupt_id.saturating_sub(32);
    if irq >= 8 {
        outb(PIC2_COMMAND, PIC_EOI);
    }
    outb(PIC1_COMMAND, PIC_EOI);
}

#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
    ret
}

#[inline]
unsafe fn wait() {
    outb(0x80, 0);
}

pub unsafe fn disable() {
    // Mask all interrupts on both PICs
    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);
}
