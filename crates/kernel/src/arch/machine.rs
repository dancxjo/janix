//! Architecture-provided machine interface.
//!
//! Re-exports the per-arch `ArchMachine` and helpers so the core kernel can
//! install the right backend without cfg soup.

#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::machine::*;

#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::machine::*;

#[cfg(target_arch = "riscv64")]
pub use crate::arch::riscv64::machine::*;

#[cfg(target_arch = "loongarch64")]
pub use crate::arch::loongarch64::machine::*;
