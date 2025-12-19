#![no_std]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

pub mod boot;
pub mod cpu;
pub mod io;
pub mod pci;
pub mod platform;
pub mod shared_buffer;

pub mod user;

#[cfg(target_arch = "x86_64")]
pub mod gdt;

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
    fn resume_user_mode(context: &[u64], fpu_context: &kernel::sched::FpuContext) -> !;
    fn install_syscall_handler();
    fn activate_user_address_space(token: Option<u64>);
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

pub fn read_boot_rtc_epoch_seconds() -> i64 {
    #[cfg(target_arch = "x86_64")]
    {
        return x86_64::rtc::read_rtc_unix_epoch_seconds();
    }
    #[cfg(target_arch = "aarch64")]
    {
        return aarch64::rtc::read_rtc_unix_epoch_seconds();
    }
    #[cfg(target_arch = "riscv64")]
    {
        return riscv64::rtc::read_rtc_unix_epoch_seconds();
    }
    #[cfg(target_arch = "loongarch64")]
    {
        return loongarch64::rtc::read_rtc_unix_epoch_seconds();
    }
    #[allow(unreachable_code)]
    0
}
