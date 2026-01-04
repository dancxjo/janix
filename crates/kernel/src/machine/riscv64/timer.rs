use core::arch::asm;

pub unsafe fn init() {
    // Enable Supervisor Timer Interrupt (SIE bit 5)
    // sie (CSR 0x104)
    asm!("csrrs zero, sie, {}", in(reg) 0x20);

    // Set initial timer
    ack();
}

pub unsafe fn ack() {
    // Read current time
    let current_time: u64;
    asm!("rdtime {}", out(reg) current_time);

    // Schedule next tick (QEMU riscv virt is 10MHz)
    // 100_000 cycles = 10ms at 10MHz
    let next_time = current_time.wrapping_add(100_000);

    // Call SBI Set Timer (Legacy EID=0, FID=0)
    asm!(
        "ecall",
        in("a0") next_time,
        in("a7") 0,
        options(nostack)
    );
}
