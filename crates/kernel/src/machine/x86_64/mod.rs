//! x86_64 architecture implementation



pub mod machine;
pub mod serial;
pub mod gdt;
pub mod idt;
pub mod percpu;

// Export the Broad Machine implementation from machine.rs
pub use machine::ARCH_MACHINE;

pub fn init() {
    unsafe {
        use machine::{BSP_GDT, PERCPU_BSP};
        use percpu::init_gs_base;
        
        // 1. PerCpu
        init_gs_base(&mut *(&raw mut PERCPU_BSP));
        
        // 2. GDT/TSS
        gdt::init(&mut *(&raw mut BSP_GDT));
        
        // 3. IDT
        idt::init();
    }
}



