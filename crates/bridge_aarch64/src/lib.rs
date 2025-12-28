#![no_std]
#![allow(unused)]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

#[cfg(target_arch = "aarch64")]
use core::arch::asm;
use kernel::bridge::HardwareBridge;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ArchContext(pub [u64; 34]);

impl Default for ArchContext {
    fn default() -> Self {
        Self([0; 34])
    }
}

pub mod interrupts;
pub mod paging;
pub mod user;

pub struct Bridge;

use core::sync::atomic::{AtomicU64, Ordering};
pub static HHDM_OFFSET: AtomicU64 = AtomicU64::new(0);

static mut UART_BASE: u64 = 0x09000000;

pub unsafe fn set_uart_base(base: u64) {
    UART_BASE = base;
}

#[cfg(target_arch = "aarch64")]
impl Bridge {
    pub unsafe fn init(hhdm: u64) {
        HHDM_OFFSET.store(hhdm, Ordering::Relaxed);
        set_uart_base(0x09000000 + hhdm);
        interrupts::trap::init();
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl Bridge {
    pub unsafe fn init(_hhdm: u64) {}
}

#[cfg(target_arch = "aarch64")]
impl HardwareBridge for Bridge {
    type Context = ArchContext;

    fn log(&self, msg: &str) {
        // PL011 UART
        let uart_base = unsafe { UART_BASE };
        let uart_ptr = uart_base as *mut u8;
        // FR Register offset 0x18. TXFF is bit 5.
        // let _uart_fr = (uart_base + 0x18) as *mut u32;

        for b in msg.bytes() {
            unsafe {
                // Wait while TXFF (bit 5) is set
                // while (core::ptr::read_volatile(uart_fr) & (1 << 5)) != 0 {
                //    core::hint::spin_loop();
                // }
                core::ptr::write_volatile(uart_ptr, b);
            }
        }
        unsafe {
            // Ensure write completes
             asm!("dc cvac, {0}", in(reg) uart_ptr);
             asm!("dsb ish");
        }
    }

    fn ticks(&self) -> u64 {
        let cntpct: u64;
        unsafe {
            asm!("mrs {}, cntpct_el0", out(reg) cntpct);
        }
        cntpct
    }

    fn idle(&self) {
        unsafe {
            asm!("wfi");
        }
    }

    fn shutdown(&self) -> ! {
        // PSCI SYSTEM_OFF or QEMU semihosting logic could go here.
        // For now, loop forever.
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }

    fn irq_disable(&self) {
        unsafe {
            asm!("msr daifset, #2");
        }
    }

    fn irq_enable(&self) {
        unsafe {
            asm!("msr daifclr, #2");
        }
    }

    fn system_now(&self) -> u64 {
        self.ticks()
    }

    fn monotonic_now(&self) -> u64 {
        let cntpct: u64;
        let cntfrq: u64;
        unsafe {
            asm!("mrs {}, cntpct_el0", out(reg) cntpct);
            asm!("mrs {}, cntfrq_el0", out(reg) cntfrq);
        }
        if cntfrq == 0 {
            return 0;
        }
        // (cntpct * 1_000_000_000) / cntfrq
        // Be careful of overflow.
        // 1GHz ticks = 1e9 per sec.
        // u64 max is 1.8e19.
        // If uptime is small, we are fine.
        // For robustness, use u128?
        ((cntpct as u128 * 1_000_000_000) / (cntfrq as u128)) as u64
    }

    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> Self::Context {
        // [x0..x29, x30, sp_el0, elr, spsr]
        let mut ctx = [0u64; 34];

        let _uer = user::UserEntryRegs {
            entry_point: entry,
            user_stack: stack,
            arg0: arg,
        };

        // We can't easily pack UserEntryRegs into regs.
        // But wait! enter_user_mode uses UserEntryRegs struct pointer.
        // resume_user_mode uses array.

        // We need to set up the array such that `resume_user_mode_asm` restores it correctly.
        // resume_user_mode_asm:
        // x0..x29, x30, sp_el0, elr_el1, spsr_el1.

        // Return to EL0:
        // SPSR_EL1 [3:0] = 0000 (EL0t). M[3:0]=0000.
        // SPSR = 0.
        ctx[33] = 0;

        // ELR_EL1 = entry
        ctx[32] = entry;

        // SP_EL0 = stack
        ctx[31] = stack;

        // x0 = arg
        ctx[0] = arg;

        ArchContext(ctx)
    }

    fn resume_user_mode(&self, context: &Self::Context) -> ! {
        user::enter::resume_user_mode(&context.0, &kernel::sched::fpu::FpuContext::default())
    }

    fn set_kernel_stack(&self, _stack: u64) {}

    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample) {
        // QEMU Virt PL031 check
        let pl031_base = 0x09010000 as *const u32;
        let mut t = unsafe { core::ptr::read_volatile(pl031_base) } as u64;

        // Convert t (unix seconds) to YMD
        let mut year = 1970;
        let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        loop {
            let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            let days = if is_leap { 366 } else { 365 };
            let sec_year = days * 86400;
            if t < sec_year {
                break;
            }
            t -= sec_year;
            year += 1;
        }

        out.year = year as u16;
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        if is_leap {
            days_in_month[1] = 29;
        }

        let mut mon = 0;
        loop {
            let sec_mon = days_in_month[mon] * 86400;
            if t < sec_mon {
                break;
            }
            t -= sec_mon;
            mon += 1;
        }
        out.mon = (mon + 1) as u8;

        let days = t / 86400;
        t %= 86400;
        out.day = (days + 1) as u8;

        out.hour = (t / 3600) as u8;
        t %= 3600;
        out.min = (t / 60) as u8;
        out.sec = (t % 60) as u8;
    }
    fn save_fpu(&self, area: &mut [u8; 512]) {
        unsafe {
            let ptr = area.as_mut_ptr();
            asm!(
                "stp q0, q1, [{0}, #0]",
                "stp q2, q3, [{0}, #32]",
                "stp q4, q5, [{0}, #64]",
                "stp q6, q7, [{0}, #96]",
                "stp q8, q9, [{0}, #128]",
                "stp q10, q11, [{0}, #160]",
                "stp q12, q13, [{0}, #192]",
                "stp q14, q15, [{0}, #224]",
                "stp q16, q17, [{0}, #256]",
                "stp q18, q19, [{0}, #288]",
                "stp q20, q21, [{0}, #320]",
                "stp q22, q23, [{0}, #352]",
                "stp q24, q25, [{0}, #384]",
                "stp q26, q27, [{0}, #416]",
                "stp q28, q29, [{0}, #448]",
                "stp q30, q31, [{0}, #480]",
                in(reg) ptr,
            );
        }
    }

    fn restore_fpu(&self, area: &[u8; 512]) {
        unsafe {
            let ptr = area.as_ptr();
            asm!(
                "ldp q0, q1, [{0}, #0]",
                "ldp q2, q3, [{0}, #32]",
                "ldp q4, q5, [{0}, #64]",
                "ldp q6, q7, [{0}, #96]",
                "ldp q8, q9, [{0}, #128]",
                "ldp q10, q11, [{0}, #160]",
                "ldp q12, q13, [{0}, #192]",
                "ldp q14, q15, [{0}, #224]",
                "ldp q16, q17, [{0}, #256]",
                "ldp q18, q19, [{0}, #288]",
                "ldp q20, q21, [{0}, #320]",
                "ldp q22, q23, [{0}, #352]",
                "ldp q24, q25, [{0}, #384]",
                "ldp q26, q27, [{0}, #416]",
                "ldp q28, q29, [{0}, #448]",
                "ldp q30, q31, [{0}, #480]",
                in(reg) ptr,
            );
        }
    }

    fn hhdm_offset(&self) -> u64 {
        HHDM_OFFSET.load(Ordering::Relaxed)
    }

    fn port_outb(&self, _port: u16, _val: u8) { unimplemented!("Port IO not supported on AArch64") }
    fn port_inb(&self, _port: u16) -> u8 { unimplemented!("Port IO not supported on AArch64") }
    fn port_outw(&self, _port: u16, _val: u16) { unimplemented!("Port IO not supported on AArch64") }
    fn port_inw(&self, _port: u16) -> u16 { unimplemented!("Port IO not supported on AArch64") }
    fn port_outd(&self, _port: u16, _val: u32) { unimplemented!("Port IO not supported on AArch64") }
    fn port_ind(&self, _port: u16) -> u32 { unimplemented!("Port IO not supported on AArch64") }
}

#[cfg(not(target_arch = "aarch64"))]
impl HardwareBridge for Bridge {
    type Context = ArchContext;

    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
    fn system_now(&self) -> u64 {
        0
    }
    fn monotonic_now(&self) -> u64 {
        0
    }
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> Self::Context {
        ArchContext([0; 34])
    }
    fn resume_user_mode(&self, _context: &Self::Context) -> ! {
        loop {}
    }
    fn set_kernel_stack(&self, _stack: u64) {}
    fn rtc_read(&self, _out: &mut abi::wire::time::RtcSample) {}

    fn hhdm_offset(&self) -> u64 { 0 }
    fn port_outb(&self, _port: u16, _val: u8) {}
    fn port_inb(&self, _port: u16) -> u8 { 0 }
    fn port_outw(&self, _port: u16, _val: u16) {}
    fn port_inw(&self, _port: u16) -> u16 { 0 }
    fn port_outd(&self, _port: u16, _val: u32) {}
    fn port_ind(&self, _port: u16) -> u32 { 0 }
    fn save_fpu(&self, _area: &mut [u8; 512]) {}
    fn restore_fpu(&self, _area: &[u8; 512]) {}
}
