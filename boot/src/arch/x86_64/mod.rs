pub mod enter;
pub mod syscall;
pub mod time;
pub mod trap;

use super::{Arch, UserEntryRegs};

pub struct X86Arch;

impl Arch for X86Arch {
    fn enter_user_mode(regs: &UserEntryRegs) -> ! {
        enter::enter_user_mode(regs)
    }

    fn install_syscall_handler() {
        syscall::install_handler();
        time::init_arch_timer();
    }
}

pub use enter::{alloc_user_stack, init_user_stack};
