pub mod time;
pub mod rtc;
pub mod trap;

use super::{UserEntryRegs, Arch};

pub struct Riscv64Arch;

impl Arch for Riscv64Arch {
    fn enter_user_mode(_regs: &UserEntryRegs) -> ! {
        loop {}
    }

    fn resume_user_mode(_context: &[u64]) -> ! {
        loop {}
    }

    fn install_syscall_handler() {
        trap::init();
        time::init_arch_timer();
        rtc::init_arch_rtc();
    }
}

pub fn alloc_user_stack() -> u64 { 0 }
pub unsafe fn init_user_stack(_phys_mem_offset: u64) {}
