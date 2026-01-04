use crate::trap::{self, Arch, FaultKind, TrapRecord};
use core::arch::asm;

// Align to 16 bytes so the assembly trap frame uses a 288-byte block.
#[repr(C, align(16))]
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
            5 => kind = FaultKind::Timer,             // Supervisor timer interrupt
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
            8 => kind = FaultKind::Syscall,    // Ecall from U-mode
            9 => kind = FaultKind::Syscall,    // Ecall from S-mode
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
                // Supervisor Software Interrupt
                // Clear SSIP (Supervisor Software Interrupt Pending)
                // sip bit 1. We are in S-mode, so we write to sip (shadowed/aliased?)
                unsafe {
                    // Clear bit 1 of sip
                    asm!("csrc sip, {}", in(reg) 2, options(nomem, preserves_flags));
                }

                // TODO: Handle IPI callbacks
                return 0;
            }
            9 => {
                // Supervisor External Interrupt
                // Acknowledge PLIC if we had one.
                crate::serial::write(b"WARN: External Interrupt\n");
                return 0;
            }
            _ => {
                // Unknown interrupt, just return
                crate::serial::write(b"WARN: Unknown Interrupt\n");
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
    ::graph::store::with_store(|store| {
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
            // Syscall convention:
            // a7: syscall number
            // a0-a5: arguments
            // Return: a0, a1

            let nr = ctx.regs[16]; // x17 = a7
            let a0 = ctx.regs[9]; // x10 = a0
            let a1 = ctx.regs[10]; // x11 = a1
            let a2 = ctx.regs[11]; // x12 = a2
            let a3 = ctx.regs[12]; // x13 = a3
            let a4 = ctx.regs[13]; // x14 = a4
            let a5 = ctx.regs[14]; // x15 = a5

            // Advance SEPC to avoid infinite loop
            ctx.sepc += 4;

            // Re-enable interrupts if they were enabled before trap?
            // Syscalls usually run with interrupts enabled in higher level kernels,
            // but for now run with whatever state we entered.
            // TODO: explicitly enable if we want preemptible syscalls.
            let _ = in_kernel; // Unused for now

            let result = crate::syscall::dispatch::dispatch(nr as u32, a0, a1, a2, a3, a4, a5);

            // Write return values
            ctx.regs[9] = result.val0;
            ctx.regs[10] = result.val1;

            // Return to user
            return 0;
        }
        _ => {
            panic!(
                "Unhandled RISC-V Trap: {:?} scause={:#x} sepc={:#x} stval={:#x}",
                kind, scause, ctx.sepc, ctx.stval
            );
        }
    }
}
