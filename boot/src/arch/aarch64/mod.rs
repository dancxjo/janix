pub mod enter;
pub mod rtc;
pub mod time;
pub mod trap;

use super::{Arch, UserEntryRegs};

pub struct AArch64Arch;

impl Arch for AArch64Arch {
    fn enter_user_mode(regs: &UserEntryRegs) -> ! {
        enter::enter_user_mode(regs)
    }

    fn resume_user_mode(context: &[u64]) -> ! {
        enter::resume_user_mode(context)
    }

    fn install_syscall_handler() {
        trap::init();
        time::init_arch_timer();
        rtc::init_arch_rtc();
    }

    fn activate_user_address_space(_token: Option<u64>) {}
}

pub use enter::{alloc_user_stack, init_user_stack};
