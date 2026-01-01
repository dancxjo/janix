//! x86_64 architecture implementation

use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange, Context, PreBootInfo};
use core::arch::asm;
use self::serial::Serial;

pub mod abi;
pub mod serial;
pub mod gdt;
pub mod idt;
pub mod percpu;

// -----------------------------------------------------------------------------
// Machine Implementation
// -----------------------------------------------------------------------------

pub struct ArchMachine {
    serial: Serial,
}

static ARCH_MACHINE_IMPL: ArchMachine = ArchMachine::new();
pub static ARCH_MACHINE: &'static dyn Machine = &ARCH_MACHINE_IMPL;

impl ArchMachine {
    pub const fn new() -> Self {
        Self {
            serial: Serial::new(),
        }
    }
}

// BSP PerCore structures
pub static mut PERCPU_BSP: self::percpu::PerCpu = self::percpu::PerCpu::new(0, 0);
pub static mut BSP_GDT: self::gdt::GdtTss = self::gdt::GdtTss::new();

pub fn init() {
    unsafe {
        // use machine::{BSP_GDT, PERCPU_BSP}; // Now local
        use percpu::init_gs_base;
        
        // 1. PerCpu
        init_gs_base(&mut *(&raw mut PERCPU_BSP));
        
        // 2. GDT/TSS
        gdt::init(&mut *(&raw mut BSP_GDT));
        
        // 3. IDT
        idt::init();
    }
}


extern "C" {
    fn x86_switch_context(old_ctx: *mut Context, new_ctx: *const Context);
    fn task_entry();
}

core::arch::global_asm!(
    ".global x86_switch_context",
    "x86_switch_context:",
    "push rbx",
    "push rbp",
    "push r12", 
    "push r13",
    "push r14",
    "push r15",
    "mov [rdi], rsp",
    "mov rsp, [rsi]",
    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop rbp",
    "pop rbx",
    "ret",

    ".global task_entry",
    "task_entry:",
    "pop rdi", // dispatch_ptr
    "pop rsi", // entry_point
    "call task_dispatch",
    "1: hlt",
    "jmp 1b"
);

impl Machine for ArchMachine {
    fn init(&self, _info: PreBootInfo) {
        // Initialize PerCpu, GDT, IDT
        init();
    }

    fn console_write(&self, bytes: &[u8]) -> usize {
        self.serial.write(bytes);
        bytes.len()
    }

    fn mmio_map(&self, _range: MmioRange, _flags: MmioFlags) -> Option<MmioMapping> {
        None
    }

    fn irq_disable(&self) -> u64 {
        let flags: u64;
        unsafe {
            asm!("pushfq; pop {}; cli", out(reg) flags, options(nomem, preserves_flags));
        }
        flags
    }

    fn irq_restore(&self, token: u64) {
        unsafe {
            if token & 0x200 != 0 {
                asm!("sti", options(nomem, preserves_flags));
            }
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe { asm!("cli; hlt"); }
        }
    }

    fn idle(&self) {
        unsafe { asm!("hlt"); }
    }
    


    fn switch_to(&self, old_ctx: &mut Context, new_ctx: &Context) {
        unsafe {
            x86_switch_context(old_ctx, new_ctx);
        }
    }
    
    fn task_entry_stub(&self) -> u64 {
        task_entry as *const () as u64
    }
}
