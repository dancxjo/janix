use core::arch::asm;

pub unsafe fn init() {
    // 1. Config Timer (TCFG - CSR 0x41)
    // Val[63:2] | Periodic[1] | En[0]
    // Interval = 1_000_000 ticks
    let interval: u64 = 1_000_000;
    let tcfg = (interval << 2) | (1 << 1) | 1;
    asm!("csrwr {}, 0x41", in(reg) tcfg);

    // 2. Enable Timer Interrupt in ECFG (CSR 0x4)
    // Timer is IS11 (bit 11)
    let mut ecfg: u64;
    asm!("csrrd {}, 0x4", out(reg) ecfg);
    ecfg |= 1 << 11;
    asm!("csrwr {}, 0x4", in(reg) ecfg);
}

pub unsafe fn ack() {
    // Clear interrupt: TICLR (CSR 0x44) -> Write 1
    let val: u64 = 1;
    asm!("csrwr {}, 0x44", in(reg) val);
}
