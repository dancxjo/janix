// Utilities for executing idle/halt instructions in a target-specific way.

/// Execute a single idle instruction (e.g., `hlt`, `wfi`, `idle 0`).
#[inline]
pub fn wait_for_interrupt() {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("hlt");

        #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
        core::arch::asm!("wfi");

        #[cfg(target_arch = "loongarch64")]
        core::arch::asm!("idle 0");
    }
}

/// Enable interrupts (STI / MSR write / etc).
#[inline]
pub fn enable_interrupts() {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("sti");
        
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!("msr daifclr, #2"); // Enable IRQ

        #[cfg(target_arch = "riscv64")]
        core::arch::asm!("csrsi sstatus, 2"); // SIE bit

        #[cfg(target_arch = "loongarch64")]
        {
            let mut mask: u32 = 0x4; // IE bit
            core::arch::asm!("csrxchg {0}, {0}, 0x0", inout(reg) mask);
        }
    }
}

/// Halt forever by repeatedly issuing the target-specific idle instruction.
#[inline]
pub fn halt_loop() -> ! {
    loop {
        wait_for_interrupt();
    }
}
