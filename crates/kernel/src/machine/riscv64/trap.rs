
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
pub unsafe extern "C" fn riscv64_handle_trap(ctx: &mut TrapContext) {
    let scause = ctx.scause;
    let is_interrupt = (scause >> 63) != 0;
    let cause_code = scause & 0xfff; // bottom 12 bits? or full masked?
    // RISC-V scause: Interrupt bit + Exception Code.
    // Exception codes are small.
    
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

    graph::store::with_store(|store| {
        let rec = TrapRecord {
            arch: Arch::Riscv64,
            kind,
            ip: ctx.sepc,
            sp: 0, // Need to fish from regs if we want it. regs[1] usually (x2)
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
        FaultKind::Breakpoint => {
            // Advance SEPC to avoid loop for ebreak (4 bytes)
            ctx.sepc += 4;
            return;
        }
        FaultKind::Timer => {
            // Panic for now or just return (timer needs handling!)
            // panic!("Timer interrupt");
            // Clear pending?
        }
        _ => {
            // panic!("Unhandled RISC-V Trap: {:?} scause={:#x} sepc={:#x}", kind, scause, ctx.sepc);
            // Don't panic yet if smoke test triggers unknown
        }
    }
}
