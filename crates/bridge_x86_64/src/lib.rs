#![no_std]
#![allow(clippy::missing_safety_doc)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod interrupts;
pub mod user;
pub mod gdt;

#[cfg(target_arch = "x86_64")]
use core::arch::asm;
use hw::HardwareBridge;

pub struct Bridge;

// Hook for scheduler. Only set by kernel binary.
pub static mut TICK_HOOK: Option<fn(&mut interrupts::trap::TrapFrame)> = None;

pub fn set_tick_hook(hook: fn(&mut interrupts::trap::TrapFrame)) {
    unsafe { TICK_HOOK = Some(hook); }
}

#[cfg(target_arch = "x86_64")]
impl Bridge {
    pub unsafe fn init() {
        gdt::init();
        interrupts::idt::init();
        interrupts::pic::init();
        interrupts::syscall::init();

        // We do NOT enable interrupts here yet. We let the kernel do it when ready.
        // Or wait, kernel loops idle().
        // If we don't enable, we hang.
        // So we should enable.
        x86_64::instructions::interrupts::enable();
    }

}

#[cfg(target_arch = "x86_64")]
impl HardwareBridge for Bridge {
    fn log(&self, msg: &str) {
        unsafe {
            for b in msg.bytes() {
                // Wait for Transmit Holding Register Empty (Bit 5)
                let mut status: u8;
                loop {
                    asm!("in al, dx", out("al") status, in("dx") 0x3F8u16 + 5);
                    if status & 0x20 != 0 {
                        break;
                    }
                    core::hint::spin_loop();
                }
                asm!("out dx, al", in("dx") 0x3F8u16, in("al") b);
            }
        }
    }

    fn ticks(&self) -> u64 {
        let eax: u32;
        let edx: u32;
        unsafe {
            asm!("rdtsc", out("eax") eax, out("edx") edx, options(nomem, nostack));
        }
        ((edx as u64) << 32) | (eax as u64)
    }

    fn system_now(&self) -> u64 {
        0
    }

    fn idle(&self) {
        unsafe {
            asm!("hlt");
        }
    }

    fn shutdown(&self) -> ! {
        unsafe {
            // QEMU ISA debug exit
            asm!("out dx, ax", in("dx") 0x604u16, in("ax") 0x2000u16);
            loop {
                asm!("hlt");
            }
        }
    }

    fn irq_disable(&self) {
        unsafe {
            asm!("cli");
        }
    }

    fn irq_enable(&self) {
        unsafe {
            asm!("sti");
        }
    }

    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> [u64; 20] {
        // [r15...rax, rip, cs, rflags, rsp, ss]
        // 15 GPRs: r15..r8, rcx, rdx, rsi, rdi, rax (rbx, rbp?)
        // Let's check user::resume_user_mode_asm layout:
        // pop r15, r14, r13, r12, rbp, rbx, r11, r10, r9, r8, rcx, rdx, rsi, rdi, rax
        // 15 regs.
        // Then rip, cs, rflags, rsp, ss. Total 20.
        // Index 13 is rdi (arg). 
        // Index 15 is rip.
        // Index 16 is cs. 
        // Index 17 is rflags.
        // Index 18 is rsp.
        // Index 19 is ss.

        let mut ctx = [0u64; 20];
        // RDI = arg
        ctx[13] = arg;

        // RIP
        ctx[15] = entry;
        
         // CS: User Code
        ctx[16] = unsafe { gdt::USER_CODE_SELECTOR.0 as u64 | 3 };

        // RFLAGS: IF enabled (0x200) | Reserved (0x2)
        ctx[17] = 0x202;

        // RSP
        ctx[18] = stack;

        // SS: User Data
        ctx[19] = unsafe { gdt::USER_DATA_SELECTOR.0 as u64 | 3 };

        ctx
    }

    fn resume_user_mode(&self, context: &[u64]) -> ! {
        crate::user::enter::resume_user_mode(context, &kernel_core::sched::fpu::FpuContext::default())
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl HardwareBridge for Bridge {
    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn system_now(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> [u64; 20] {
        [0; 20]
    }
    fn resume_user_mode(&self, _context: &[u64]) -> ! {
        loop {}
    }
}
