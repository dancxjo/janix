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
    
    // Schedule next tick (freq ~10MHz? QEMU is usually 10MHz or 32kHz?)
    // QEMU riscv virt is 10MHz usually?
    // Let's guess 100_000 cycles for 10ms.
    let next_time = current_time + 100_000;
    
    // Call SBI Set Timer (Legacy EID=0, FID=0)
    // Func: sbi_set_timer(stime_value)
    // a0 = stime_value
    // a7 = 0x0
    asm!(
        "ecall",
        in("a0") next_time,
        in("a7") 0,
        options(nostack)
    );
}
