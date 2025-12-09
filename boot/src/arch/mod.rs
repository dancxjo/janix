#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UserEntryRegs {
    pub entry_point: u64,
    pub user_stack: u64,
    pub arg0: u64,
}

pub trait Arch {
    fn enter_user_mode(regs: &UserEntryRegs) -> !;
    fn resume_user_mode(context: &[u64]) -> !;
    fn install_syscall_handler();
    // fn syscall_stub(num: u64, arg0: u64, arg1: u64, arg2: u64) -> u64; // This is for userland, not kernel
}

#[cfg(target_arch = "x86_64")]
pub use x86_64 as current;
#[cfg(target_arch = "x86_64")]
pub use x86_64::X86Arch as CurrentArch;

#[cfg(target_arch = "aarch64")]
pub use aarch64 as current;
#[cfg(target_arch = "aarch64")]
pub use aarch64::AArch64Arch as CurrentArch;

#[cfg(target_arch = "riscv64")]
pub use riscv64 as current;
#[cfg(target_arch = "riscv64")]
pub use riscv64::Riscv64Arch as CurrentArch;

#[cfg(target_arch = "loongarch64")]
pub use loongarch64 as current;
#[cfg(target_arch = "loongarch64")]
pub use loongarch64::LoongArch64Arch as CurrentArch;
