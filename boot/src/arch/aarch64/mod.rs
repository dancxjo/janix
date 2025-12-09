pub mod rtc;
pub mod time;
pub mod trap;

use super::{Arch, UserEntryRegs};

pub struct AArch64Arch;

impl Arch for AArch64Arch {
    fn enter_user_mode(_regs: &UserEntryRegs) -> ! {
        loop {}
    }

    fn install_syscall_handler() {
        trap::init();
        time::init_arch_timer();
        rtc::init_arch_rtc();
    }
}

pub fn alloc_user_stack() -> u64 {
    0
}
pub unsafe fn init_user_stack(_phys_mem_offset: u64) {}
