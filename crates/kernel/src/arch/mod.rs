//! Architecture abstraction layer
//!
//! Provides CPU-level primitives (IRQ, halt, idle) for the kernel.

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;

/// Common architecture trait
pub trait Arch: Sync + Send {
    fn irq_disable(&self) -> u64;
    fn irq_restore(&self, token: u64);
    fn halt(&self) -> !;
    fn idle(&self);
    fn cpu_id(&self) -> u32;
}

/// Global architecture handles
#[cfg(target_arch = "x86_64")]
pub static ARCH: x86_64::X86Arch = x86_64::X86Arch;

#[cfg(target_arch = "aarch64")]
pub static ARCH: aarch64::Aarch64Arch = aarch64::Aarch64Arch;

#[cfg(target_arch = "riscv64")]
pub static ARCH: riscv64::Riscv64Arch = riscv64::Riscv64Arch;

#[cfg(target_arch = "loongarch64")]
pub static ARCH: loongarch64::Loongarch64Arch = loongarch64::Loongarch64Arch;
