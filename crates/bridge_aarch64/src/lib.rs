#![no_std]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

#[cfg(target_arch = "aarch64")]
use core::arch::asm;
use hw::HardwareBridge;

pub mod interrupts;
pub mod paging;
pub mod user;

pub struct Bridge;

static mut UART_BASE: u64 = 0x09000000;

pub unsafe fn set_uart_base(base: u64) {
    UART_BASE = base;
}

#[cfg(target_arch = "aarch64")]
impl Bridge {
    pub unsafe fn init() {
        interrupts::trap::init();
    }
}

#[cfg(target_arch = "aarch64")]
impl HardwareBridge for Bridge {
    fn log(&self, msg: &str) {
        // PL011 UART
        let uart_base = unsafe { UART_BASE };
        let uart_ptr = uart_base as *mut u8;
        // FR Register offset 0x18. TXFF is bit 5.
        let uart_fr = (uart_base + 0x18) as *mut u32;

        for b in msg.bytes() {
            unsafe {
                // Wait while TXFF (bit 5) is set
                // while (core::ptr::read_volatile(uart_fr) & (1 << 5)) != 0 {
                //    core::hint::spin_loop();
                // }
                core::ptr::write_volatile(uart_ptr, b);
            }
        }
    }

    fn ticks(&self) -> u64 {
        let cntpct: u64;
        unsafe { asm!("mrs {}, cntpct_el0", out(reg) cntpct); }
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
        if cntfrq == 0 { return 0; }
        // (cntpct * 1_000_000_000) / cntfrq
        // Be careful of overflow. 
        // 1GHz ticks = 1e9 per sec. 
        // u64 max is 1.8e19. 
        // If uptime is small, we are fine.
        // For robustness, use u128?
        ((cntpct as u128 * 1_000_000_000) / (cntfrq as u128)) as u64
    }

    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> [u64; 34] {
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

        ctx
    }

    fn resume_user_mode(&self, context: &[u64]) -> ! {
        user::enter::resume_user_mode(context, &kernel_core::sched::fpu::FpuContext::default())
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
        if is_leap { days_in_month[1] = 29; }

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
}

#[cfg(not(target_arch = "aarch64"))]
impl HardwareBridge for Bridge {
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
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> [u64; 34] {
        [0; 34]
    }
    fn resume_user_mode(&self, _context: &[u64]) -> ! {
        loop {}
    }
    fn set_kernel_stack(&self, _stack: u64) {}
    fn rtc_read(&self, _out: &mut abi::wire::time::RtcSample) {}
}
