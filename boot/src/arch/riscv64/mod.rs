use super::{UserEntryRegs, Arch};

pub struct Riscv64Arch;

impl Arch for Riscv64Arch {
    fn enter_user_mode(_regs: &UserEntryRegs) -> ! {
        loop {}
    }

    fn install_syscall_handler() {
    }
}

pub fn alloc_user_stack() -> u64 { 0 }
pub unsafe fn init_user_stack(_phys_mem_offset: u64) {}
