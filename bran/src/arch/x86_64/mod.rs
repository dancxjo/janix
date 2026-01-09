mod simd;
mod serial;
mod runtime;
mod gdt;
mod percpu;
mod syscall;

pub use runtime::{Runtime, hcf};

pub const fn create_runtime() -> Runtime {
    Runtime::new(runtime::X86_64Runtime::new())
}
