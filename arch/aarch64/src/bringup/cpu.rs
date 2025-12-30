use core::arch::asm;

pub fn ticks() -> u64 {
    let cntpct: u64;
    unsafe {
        asm!("mrs {}, cntpct_el0", out(reg) cntpct);
    }
    cntpct
}

pub fn monotonic_now() -> u64 {
    let cntpct: u64;
    let cntfrq: u64;
    unsafe {
        asm!("mrs {}, cntpct_el0", out(reg) cntpct);
        asm!("mrs {}, cntfrq_el0", out(reg) cntfrq);
    }
    if cntfrq == 0 {
        return 0;
    }
    ((cntpct as u128 * 1_000_000_000) / (cntfrq as u128)) as u64
}

pub fn idle() {
    unsafe {
        asm!("wfi");
    }
}

pub fn shutdown() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

pub fn irq_disable() {
    unsafe {
        asm!("msr daifset, #2");
    }
}

pub fn irq_enable() {
    unsafe {
        asm!("msr daifclr, #2");
    }
}

pub unsafe fn enter_user_mode(entry: u64, stack: u64, arg: u64) -> ! {
    asm!(
        "msr sp_el0, {stack}",
        "msr elr_el1, {entry}",
        "msr spsr_el1, {spsr}",
        "mov x0, {arg}",
        "eret",
        stack = in(reg) stack,
        entry = in(reg) entry,
        spsr = in(reg) 0u64,
        arg = in(reg) arg,
        options(noreturn)
    );
}
