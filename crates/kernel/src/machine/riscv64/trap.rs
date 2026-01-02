
use crate::trap::{self, TrapRecord, FaultKind, Arch};
use graph::store;
use core::arch::asm;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapContext {
    pub regs: [u64; 31], // x1-x31
    pub sstatus: u64,
    pub sepc: u64,
    pub scause: u64,
    pub stval: u64,
}

#[no_mangle]
pub unsafe extern "C" fn riscv64_handle_trap(ctx: &mut TrapContext) -> u64 {
    let scause = ctx.scause;
    let is_interrupt = (scause >> 63) != 0;
    let cause_code = scause & 0xfff;
    
    // DEBUG: Print scause to understand what trap we're handling
    crate::serial::write(b"TRAP: scause=");
    crate::serial::write_hex(scause);
    crate::serial::write(b" int=");
    crate::serial::write_num(if is_interrupt { 1 } else { 0 });
    crate::serial::write(b" code=");
    crate::serial::write_num(cause_code);
    crate::serial::write(b"\n");
    
    let mut kind = FaultKind::Unknown;
    let mut addr = None;
    
    if is_interrupt {
        match cause_code {
            1 => kind = FaultKind::SoftwareInterrupt, // Supervisor software interrupt
            5 => kind = FaultKind::Timer, // Supervisor timer interrupt
            9 => kind = FaultKind::ExternalInterrupt, // Supervisor external interrupt
            _ => kind = FaultKind::Irq,
        }
    } else {
        match cause_code {
            0 => kind = FaultKind::InstructionAbort, // Instruction address misaligned
            1 => {
                 kind = FaultKind::InstructionAbort; // Instruction access fault
                 addr = Some(ctx.stval);
            }
            2 => kind = FaultKind::IllegalInstruction,
            3 => kind = FaultKind::Breakpoint,
            5 => kind = FaultKind::AccessFault, // Load access fault
            6 => kind = FaultKind::AccessFault, // Store/AMO address misaligned
            7 => {
                 kind = FaultKind::AccessFault; // Store/AMO access fault
                 addr = Some(ctx.stval);
            }
            8 => kind = FaultKind::Syscall, // Ecall from U-mode
            9 => kind = FaultKind::Syscall, // Ecall from S-mode
            12 => kind = FaultKind::PageFault, // Instruction page fault
            13 => {
                kind = FaultKind::PageFault; // Load page fault
                addr = Some(ctx.stval);
            }
            15 => {
                kind = FaultKind::PageFault; // Store/AMO page fault
                addr = Some(ctx.stval);
            }
            _ => kind = FaultKind::Unknown,
        }
    }
    
    // Check privilege from sstatus? SPP bit.
    // sstatus.SPP (bit 8) = 1 (Supervisor), 0 (User)
    let in_kernel = (ctx.sstatus & (1 << 8)) != 0;

    // Handle ALL interrupts EARLY (before any lock acquisition) to avoid deadlock
    if is_interrupt {
        match cause_code {
            5 => {
                // Supervisor Timer Interrupt
                super::timer::ack();
                let current_sp = ctx as *mut TrapContext as u64;
                let new_sp = crate::sched::tick(current_sp);
                return if new_sp != 0 { new_sp } else { 0 };
            }
            1 => {
                // Supervisor Software Interrupt - just clear and return
                // TODO: Handle IPI
                return 0;
            }
            9 => {
                // Supervisor External Interrupt
                // TODO: Handle external IRQ via PLIC
                return 0;
            }
            _ => {
                // Unknown interrupt, just return
                return 0;
            }
        }
    }
    
    // Handle exceptions that can be resolved quickly
    if kind == FaultKind::Breakpoint {
        ctx.sepc += 4; // Skip ebreak instruction
        return 0;
    }

    // Only record serious faults (not Timer/Breakpoint)
    graph::store::with_store(|store| {
        let rec = TrapRecord {
            arch: Arch::Riscv64,
            kind,
            ip: ctx.sepc,
            sp: ctx.regs[1], // x2 = sp
            addr,
            code: scause,
            vector: cause_code as u32,
            cpu: 0,
            in_kernel,
            task: None,
        };
        trap::record_fault(store, &rec);
    });

    match kind {
        FaultKind::Syscall => {
            // TODO: Handle syscalls
            panic!("RISC-V syscall not implemented");
        }
        _ => {
            panic!("Unhandled RISC-V Trap: {:?} scause={:#x} sepc={:#x}", kind, scause, ctx.sepc);
        }
    }
}
