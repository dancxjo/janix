use core::arch::global_asm;
use kernel::time;
use kernel::sched;
use core::sync::atomic::Ordering;

global_asm!(include_str!("trap.S"));

#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(tf: &mut TrapFrame, scause: u64, stval: u64, sepc: u64, sstatus: u64) {
    let is_interrupt = (scause & (1 << 63)) != 0;
    let code = scause & !(1 << 63);

    if is_interrupt {
        match code {
            5 => {
                // Supervisor Timer Interrupt
                timer_interrupt_handler(tf);
            }
            _ => {
                kernel::println!("Unknown interrupt: scause={:#x}, code={}", scause, code);
            }
        }
    } else {
        kernel::println!("EXCEPTION: RISC-V Trap");
        kernel::println!(
            "scause: {:#x}, stval: {:#x}, sepc: {:#x}, sstatus: {:#x}",
            scause,
            stval,
            sepc,
            sstatus
        );
        kernel::println!("{:#?}", tf);
        loop {}
    }
}

fn timer_interrupt_handler(_tf: &mut TrapFrame) {
    // 1. Increment ticks
    sched::TICKS.fetch_add(1, Ordering::Relaxed);

    // 2. Poll time (updates wall clock, alarms)
    time::poll_time();

    // 3. Schedule next interrupt (1ms from now)
    // We use monotonic_now_ns() which uses RDTIME
    let now = time::monotonic_now_ns();
    let next = now + 1_000_000; // 1ms
    time::timer().set_deadline_ns(next);

    // 4. Preemption / Context Switch
    // TODO: Implement context switching once TrapFrame matches kernel thread context
}

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub x30: u64,
    pub x31: u64,
}

pub fn init() {
    unsafe extern "C" {
        static trap_vector: u8;
    }
    unsafe {
        core::arch::asm!(
            "csrw stvec, {}",
            "csrs sstatus, {}", // Enable global interrupts (SIE bit 1)
            "csrs sie, {}",     // Enable Supervisor Timer Interrupt (STIE bit 5)
            in(reg) &trap_vector,
            in(reg) 1 << 1,
            in(reg) 1 << 5,
        );
    }
}
