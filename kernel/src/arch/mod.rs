#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "x86_64")]
pub type ArchContext = x86_64::Context;
#[cfg(target_arch = "x86_64")]
pub type ArchTrapFrame = x86_64::TrapFrame;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

// Placeholder types for other arches until implemented
#[cfg(target_arch = "aarch64")]
pub type ArchContext = usize; // Temporary
#[cfg(target_arch = "aarch64")]
pub type ArchTrapFrame = usize; // Temporary

#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;

#[cfg(target_arch = "loongarch64")]
pub use loongarch64::*;

#[cfg(target_arch = "loongarch64")]
pub type ArchContext = usize; // Temporary
#[cfg(target_arch = "loongarch64")]
pub type ArchTrapFrame = usize; // Temporary

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "riscv64")]
pub use riscv64::*;

#[cfg(target_arch = "riscv64")]
pub type ArchContext = usize; // Temporary
#[cfg(target_arch = "riscv64")]
pub type ArchTrapFrame = usize; // Temporary
