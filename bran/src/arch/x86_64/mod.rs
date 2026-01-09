mod simd;
mod serial;
mod runtime;
mod gdt;
mod percpu;
mod syscall;
mod paging;
mod task;

use core::arch::global_asm;
global_asm!(include_str!("syscall_entry.S"));

pub use runtime::{Runtime, hcf};

pub const fn create_runtime() -> Runtime {
    Runtime::new(runtime::X86_64Runtime::new())
}
