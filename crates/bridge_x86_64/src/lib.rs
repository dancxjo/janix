#![no_std]
#![allow(clippy::missing_safety_doc)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod interrupts;
pub mod user;
pub mod gdt;
pub mod acpi;
pub mod pci;
pub mod hpet;
pub mod ps2;
pub mod serial;

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
    pub unsafe fn init(rsdp_addr: Option<u64>, hhdm: u64) {
        use hw::HardwareBridge;
        let b = Bridge;
        b.log("BRIDGE: gdt::init\n");
        gdt::init();
        b.log("BRIDGE: idt::init\n");
        interrupts::idt::init();
        b.log("BRIDGE: pic::init\n");
        interrupts::pic::init();
        b.log("BRIDGE: syscall::init\n");
        interrupts::syscall::init();


        b.log("BRIDGE: ps2::init\n");
        ps2::init();
        
        b.log("BRIDGE: serial::init\n");
        serial::init();

        // ACPI init moved to explicit call
    }

    pub unsafe fn init_acpi(rsdp_addr: u64, hhdm: u64) {
        use hw::HardwareBridge;
        let b = Bridge;
        b.log("BRIDGE: acpi::init\n");
        acpi::init(rsdp_addr, hhdm);
    }

}

pub fn print_u64(val: u64) {
    let bridge = Bridge;
    use hw::HardwareBridge;
    
    if val == 0 {
        bridge.log("0");
        return;
    }

    let mut buffer = [0u8; 20];
    let mut i = 0;
    let mut n = val;
    
    while n > 0 {
        buffer[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    
    while i > 0 {
        i -= 1;
        bridge.log(core::str::from_utf8(&[buffer[i]]).unwrap());
    }
}

pub fn print_hex(val: u64) {
    let bridge = Bridge;
    use hw::HardwareBridge;
    bridge.log("0x");
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        let c = if digit < 10 { digit as u8 + b'0' } else { digit as u8 - 10 + b'a' };
        bridge.log(core::str::from_utf8(&[c]).unwrap());
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

    fn monotonic_now(&self) -> u64 {
        hpet::read_ns()
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

    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> [u64; 34] {
        // [r15...rax, rip, cs, rflags, rsp, ss]
        // 15 GPRs: r15..r8, rcx, rdx, rsi, rdi, rax (rbx, rbp?)
        // Let's check user::resume_user_mode_asm layout:
        // pop r15, r14, r13, r12, rbp, rbx, r11, r10, r9, r8, rcx, rdx, rsi, rdi, rax
        // 15 regs.
        // Then rip, cs, rflags, rsp, ss. Total 20.
        // We now pad to 34 for AArch64 compatibility.

        let mut ctx = [0u64; 34];
        // RDI = arg
        ctx[13] = arg;

        // RIP
        ctx[15] = entry;
        
         // CS: User Code
        ctx[16] = unsafe { gdt::USER_CODE_SELECTOR.0 as u64 | 3 };

        // RFLAGS: Interrupts enabled (0x200). IOPL 3 (0x3000) -> 0x3202
        ctx[17] = 0x3202;

        // RSP
        ctx[18] = stack;

        // SS: User Data
        ctx[19] = unsafe { gdt::USER_DATA_SELECTOR.0 as u64 | 3 };

        ctx
    }

    fn resume_user_mode(&self, context: &[u64]) -> ! {
        crate::user::enter::resume_user_mode(context, &kernel_core::sched::fpu::FpuContext::default())
    }

    fn set_kernel_stack(&self, stack_top: u64) {
        unsafe {
            gdt::set_kernel_stack(stack_top);
            interrupts::syscall::set_kernel_stack(stack_top);
        }
    }

    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample) {
        unsafe {
            // Helper to read CMOS register
            let mut read_reg = |reg: u8| -> u8 {
                asm!("out dx, al", in("dx") 0x70u16, in("al") reg);
                let val: u8;
                asm!("in al, dx", out("al") val, in("dx") 0x71u16);
                val
            };

            // Wait for update in progress (Register A, bit 7)
            while (read_reg(0x0A) & 0x80) != 0 {
                core::hint::spin_loop();
            }

            let mut sec = read_reg(0x00);
            let mut min = read_reg(0x02);
            let mut hour = read_reg(0x04);
            let mut day = read_reg(0x07);
            let mut mon = read_reg(0x08);
            let mut year = read_reg(0x09) as u16;

            let reg_b = read_reg(0x0B);

            // BCD conversion (if Bit 2 of Reg B is 0)
            if (reg_b & 0x04) == 0 {
                sec = (sec & 0x0F) + ((sec / 16) * 10);
                min = (min & 0x0F) + ((min / 16) * 10);
                hour = ((hour & 0x0F) + ((hour & 0x70) / 16 * 10)) | (hour & 0x80);
                day = (day & 0x0F) + ((day / 16) * 10);
                mon = (mon & 0x0F) + ((mon / 16) * 10);
                year = (year & 0x0F) as u16 + ((year / 16) as u16 * 10);
            }

            // 12-hour format (if Bit 1 of Reg B is 0)
            if (reg_b & 0x02) == 0 && (hour & 0x80) != 0 {
                hour = ((hour & 0x7F) + 12) % 24;
            }

            // Century guessing (2000-2099)
            // Real OS reads ACPI FADT century byte, we'll just assume 20xx
            out.year = 2000 + year;
            out.mon = mon;
            out.day = day;
            out.hour = hour;
            out.min = min;
            out.sec = sec;
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl HardwareBridge for Bridge {
    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn monotonic_now(&self) -> u64 {
        0
    }
    fn system_now(&self) -> u64 {
        0
    }
    fn monotonic_now(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> [u64; 34] {
        [0; 34]
    }
    fn resume_user_mode(&self, _context: &[u64]) -> ! {
        loop {}
    }
    fn set_kernel_stack(&self, _: u64) {}
    fn rtc_read(&self, _out: &mut abi::wire::time::RtcSample) {}
}
