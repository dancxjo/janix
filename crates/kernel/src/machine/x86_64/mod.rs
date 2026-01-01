//! x86_64 architecture implementation
global_asm!(include_str!("interrupts.S"));

use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange, Context, PreBootInfo};
use crate::sched;
use core::arch::asm;
use core::arch::global_asm;
use self::serial::Serial;

pub mod abi;
pub mod serial;
pub mod idt;
pub mod percpu;
pub mod timer;
pub mod gdt;
pub mod mmu;
pub use mmu::AddressSpace;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TrapFrame {
    // Pushed by us
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64,
    pub r8:  u64, pub r9:  u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    
    // Pushed by CPU
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[no_mangle]
pub extern "C" fn sched_tick_asm_helper(sp: u64) -> u64 {
    match sched::tick(sp) {
        Some(new_sp) => new_sp,
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn timer_ack_asm_helper() {
    unsafe { timer::ack(); }
}

#[no_mangle]
pub extern "C" fn task_dispatch(_dispatch_ptr: u64, entry: extern "C" fn()) {
    entry();
}

// -----------------------------------------------------------------------------
// Machine Implementation
// -----------------------------------------------------------------------------

pub struct ArchMachine {
    serial: Serial,
    hhdm_offset: core::sync::atomic::AtomicU64,
    kernel_phys_base: core::sync::atomic::AtomicU64,
    kernel_virt_base: core::sync::atomic::AtomicU64,
}

static ARCH_MACHINE_IMPL: ArchMachine = ArchMachine::new();
pub static ARCH_MACHINE: &'static dyn Machine = &ARCH_MACHINE_IMPL;

impl ArchMachine {
    pub const fn new() -> Self {
        Self {
            serial: Serial::new(),
            hhdm_offset: core::sync::atomic::AtomicU64::new(0),
            kernel_phys_base: core::sync::atomic::AtomicU64::new(0),
            kernel_virt_base: core::sync::atomic::AtomicU64::new(0),
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
        
        // 1. GDT/TSS (Reloads Segments, clearing GS Base)
        gdt::init(&mut *(&raw mut BSP_GDT));

        // 2. PerCpu (Sets GS Base)
        init_gs_base(&mut *(&raw mut PERCPU_BSP));
        
        // 3. IDT
        idt::init();

        // 4. Timer
        timer::init();

        // 5. Syscall
        syscall_init();
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
    fn init(&self, info: PreBootInfo) {
        self.hhdm_offset.store(info.hhdm_offset, core::sync::atomic::Ordering::Relaxed);
        self.kernel_phys_base.store(info.kernel_phys_base, core::sync::atomic::Ordering::Relaxed);
        self.kernel_virt_base.store(info.kernel_virt_base, core::sync::atomic::Ordering::Relaxed);

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

    fn set_kernel_stack(&self, top: u64) {
        // Update per-cpu kernel_rsp
        // We can get per-cpu via GS.
        // Or if we are in kernel, we can use the PerCpu structure if we have a pointer.
        // Easier: use asm to write to gs:32.
        unsafe {
            core::arch::asm!("mov {}, %gs:32", in(reg) top, options(att_syntax));
        }
        // Update TSS RSP0
        // We need to access the GDT/TSS.
        // gdt::set_tss_rsp0(top); // Needs to be exposed
        unsafe {
            gdt::set_tss_rsp0(top);
        }
    }

    fn virt_to_phys(&self, virt: u64) -> u64 {
         let hhdm = self.hhdm_offset.load(core::sync::atomic::Ordering::Relaxed);
         let k_virt = self.kernel_virt_base.load(core::sync::atomic::Ordering::Relaxed);
         let k_phys = self.kernel_phys_base.load(core::sync::atomic::Ordering::Relaxed);

         if virt >= k_virt && k_virt != 0 {
             virt - k_virt + k_phys
         } else if virt >= hhdm && hhdm != 0 {
             virt - hhdm
         } else {
             virt
         }
    }
}

pub fn syscall_init() {
    use x86_64::registers::model_specific::{Efer, EferFlags, Msr, Star, LStar, SFMask};
    use x86_64::registers::rflags::RFlags;
    
    extern "C" {
        fn syscall_entry();
    }

    // Enable syscall extension
    unsafe {
        let mut efer = Efer::read();
        efer |= EferFlags::SYSTEM_CALL_EXTENSIONS;
        Efer::write(efer);
        
        let handler_addr = syscall_entry as u64;
        LStar::write(x86_64::VirtAddr::new(handler_addr));
        
        let _ = Star::write(
            x86_64::structures::gdt::SegmentSelector(0x0013), // User Base (CS=Base+16, SS=Base+8)
            x86_64::structures::gdt::SegmentSelector(0x0013),
            x86_64::structures::gdt::SegmentSelector(0x0008), // Kernel Base (CS=Base, SS=Base+8)
            x86_64::structures::gdt::SegmentSelector(0x0008),
        );
        
        // Flags mask (flags to clear on syscall)
        SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG | RFlags::DIRECTION_FLAG); 
    }
}
