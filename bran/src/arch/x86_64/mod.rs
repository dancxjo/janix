pub mod simd;
pub mod serial;
pub mod runtime;
pub mod gdt;
pub mod percpu;
pub mod syscall;
pub mod paging;
pub mod idt;
pub mod interrupt;
pub mod timer;
pub mod exception;
pub mod trap;

use core::arch::global_asm;
global_asm!(include_str!("syscall_entry.S"));
global_asm!(include_str!("interrupt_entry.S"));

pub use runtime::{Runtime, hcf};

pub const fn create_runtime() -> Runtime {
    Runtime::new(runtime::X86_64Runtime::new())
}

pub unsafe fn init() {
    gdt::init();
    interrupt::init();
    percpu::init_gs_base();
}
