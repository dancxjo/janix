//! Architecture abstraction layer
//!
//! Provides CPU-level primitives (IRQ, halt, idle) for the kernel.



#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::setup_new_task_stack;
#[cfg(target_arch = "aarch64")]
pub use aarch64::setup_new_task_stack;

#[cfg(target_arch = "x86_64")]
pub use crate::machine::x86_64::init;
#[cfg(not(target_arch = "x86_64"))]
pub fn init() {}



// Context is now in crate::machine::Context
pub use crate::machine::Context;

