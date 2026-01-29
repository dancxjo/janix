use super::syscall::handle_syscall;
use core::arch::asm;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_sync_el0_rust(tf: &mut UserTrapFrame, esr: u64) {
    let ec = (esr >> 26) & 0x3f;
    if ec == 0x15 {
        unsafe {
            handle_syscall(tf);
        }
    } else {
        let far: u64;
        unsafe {
            asm!("mrs {}, far_el1", out(reg) far, options(nomem, nostack));
        }
        panic!(
            "Unhandled Sync EL0 Exception. ESR={:#x} EC={:#x} ELR={:#x} FAR={:#x} SPSR={:#x}",
            esr, ec, tf.elr_el1, far, tf.spsr_el1
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unhandled_exception_rust(esr: u64, elr: u64, origin: u64, spsr: u64) -> ! {
    panic!(
        "Unhandled Exception! Origin={} ESR={:#x} ELR={:#x} SPSR={:#x}",
        origin, esr, elr, spsr
    );
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UserTrapFrame {
    // x0-x30
    pub regs: [u64; 31],

    // Exception context
    pub sp_el0: u64,
    pub elr_el1: u64,
    pub spsr_el1: u64,
    // Padding to 16-byte align if needed, but [u64; 34] is 272 bytes (16-aligned).
    // 31 + 3 = 34. 34 * 8 = 272. 272 % 16 == 0. Good.
}
