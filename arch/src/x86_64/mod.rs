pub mod apic;
pub mod enter;
pub mod fpu;
pub mod paging;
pub mod pic;
pub mod pit;
pub mod rtc;
pub mod syscall;
pub mod time;
pub mod trap;

use super::{Arch, UserEntryRegs};

pub struct X86Arch;

impl Arch for X86Arch {
    fn enter_user_mode(regs: &UserEntryRegs) -> ! {
        enter::enter_user_mode(regs)
    }

    fn resume_user_mode(context: &[u64], fpu_context: &kernel::sched::FpuContext) -> ! {
        enter::resume_user_mode(context, fpu_context)
    }

    fn install_syscall_handler() {
        unsafe {
            use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};
            let mut cr0 = Cr0::read();
            cr0.remove(Cr0Flags::from_bits_truncate(1 << 2));
            cr0.insert(Cr0Flags::MONITOR_COPROCESSOR);
            Cr0::write(cr0);

            let mut cr4 = Cr4::read();
            cr4.insert(Cr4Flags::OSFXSR);
            cr4.insert(Cr4Flags::OSXMMEXCPT_ENABLE);
            Cr4::write(cr4);
        }
        syscall::install_handler();
        time::init_arch_timer();
        rtc::init_arch_rtc();
    }

    fn activate_user_address_space(token: Option<u64>) {
        enter::activate_address_space(token);
    }
}

pub use enter::{alloc_user_stack, init_user_stack, resume_user_mode};
