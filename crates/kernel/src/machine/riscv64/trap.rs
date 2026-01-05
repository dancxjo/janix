use crate::trap::{self, Arch, FaultKind, TrapRecord};
use core::arch::asm;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct TrapContext {
    pub regs: [u64; 31],
    pub sstatus: u64,
    pub sepc: u64,
    pub scause: u64,
    pub stval: u64,
}

fn decode_scause(code: u64, is_int: bool) -> &'static str {
    if is_int {
        match code {
            1 => "Supervisor SW Int",
            5 => "Supervisor Timer",
            9 => "Supervisor Ext Int",
            _ => "Unknown Interrupt",
        }
    } else {
        match code {
            0 => "Instr Addr Misalign",
            1 => "Instr Access Fault",
            2 => "Illegal Instruction",
            3 => "Breakpoint",
            4 => "Load Addr Misalign",
            5 => "Load Access Fault",
            6 => "Store Addr Misalign",
            7 => "Store Access Fault",
            8 => "ECall U-mode",
            9 => "ECall S-mode",
            12 => "Instr Page Fault",
            13 => "Load Page Fault",
            15 => "Store Page Fault",
            _ => "Unknown Exception",
        }
    }
}

fn dump_regs(ctx: &TrapContext) {
    use crate::serial::{write, write_hex};
    write(b"\n--- Registers ---\n");
    let names: [&[u8]; 31] = [
        b"ra", b"sp", b"gp", b"tp", b"t0", b"t1", b"t2", b"s0", b"s1", b"a0", b"a1", b"a2", b"a3",
        b"a4", b"a5", b"a6", b"a7", b"s2", b"s3", b"s4", b"s5", b"s6", b"s7", b"s8", b"s9", b"sA",
        b"sB", b"t3", b"t4", b"t5", b"t6",
    ];
    for i in 0..31 {
        write(b"x");
        if i + 1 < 10 {
            write(&[b'0' + (i + 1) as u8]);
        } else {
            write(&[b'0' + ((i + 1) / 10) as u8, b'0' + ((i + 1) % 10) as u8]);
        }
        write(b"(");
        write(names[i]);
        write(b")=");
        write_hex(ctx.regs[i]);
        if (i + 1) % 3 == 0 {
            write(b"\n");
        } else {
            write(b" ");
        }
    }
    write(b"\n");
}

fn panic_trap(ctx: &TrapContext, kind: FaultKind) -> ! {
    use crate::serial::{write, write_hex};
    let scause = ctx.scause;
    let is_int = (scause >> 63) != 0;
    let code = scause & 0xfff;

    write(b"\n\n========== RISC-V 64 FATAL EXCEPTION ==========\n");
    write(b"Kind: ");
    match kind {
        FaultKind::PageFault => write(b"PageFault"),
        FaultKind::AccessFault => write(b"AccessFault"),
        FaultKind::IllegalInstruction => write(b"IllegalInstruction"),
        FaultKind::InstructionAbort => write(b"InstructionAbort"),
        _ => write(b"Unknown"),
    }
    write(b"\n");

    write(b"SCAUSE: ");
    write_hex(scause);
    write(b"\n");
    write(b"  Interrupt: ");
    if is_int {
        write(b"Yes\n");
    } else {
        write(b"No\n");
    }
    write(b"  Code: ");
    write_hex(code);
    write(b" -> ");
    write(decode_scause(code, is_int).as_bytes());
    write(b"\n");

    write(b"\nSEPC: ");
    write_hex(ctx.sepc);
    write(b"\nSTVAL: ");
    write_hex(ctx.stval);
    write(b"\nSSTATUS: ");
    write_hex(ctx.sstatus);
    let spp = (ctx.sstatus >> 8) & 1;
    write(b" (SPP=");
    if spp != 0 {
        write(b"S)\n");
    } else {
        write(b"U)\n");
    }

    dump_regs(ctx);
    write(b"================================================\n");
    panic!("RISC-V Exception");
}

#[no_mangle]
pub unsafe extern "C" fn riscv64_handle_trap(ctx: &mut TrapContext) -> u64 {
    let scause = ctx.scause;
    let is_int = (scause >> 63) != 0;
    let code = scause & 0xfff;

    let mut kind = FaultKind::Unknown;
    let mut addr = None;

    if is_int {
        match code {
            1 => kind = FaultKind::SoftwareInterrupt,
            5 => kind = FaultKind::Timer,
            9 => kind = FaultKind::ExternalInterrupt,
            _ => kind = FaultKind::Irq,
        }
    } else {
        match code {
            0 => kind = FaultKind::InstructionAbort,
            1 => {
                kind = FaultKind::InstructionAbort;
                addr = Some(ctx.stval);
            }
            2 => kind = FaultKind::IllegalInstruction,
            3 => kind = FaultKind::Breakpoint,
            5 | 6 => kind = FaultKind::AccessFault,
            7 => {
                kind = FaultKind::AccessFault;
                addr = Some(ctx.stval);
            }
            8 | 9 => kind = FaultKind::Syscall,
            12 => kind = FaultKind::PageFault,
            13 => {
                kind = FaultKind::PageFault;
                addr = Some(ctx.stval);
            }
            15 => {
                kind = FaultKind::PageFault;
                addr = Some(ctx.stval);
            }
            _ => kind = FaultKind::Unknown,
        }
    }

    let in_kernel = (ctx.sstatus & (1 << 8)) != 0;

    if is_int {
        match code {
            5 => {
                super::timer::ack();
                let sp = ctx as *mut TrapContext as u64;
                let new_sp = crate::sched::tick(sp);
                return if new_sp != 0 { new_sp } else { 0 };
            }
            1 => {
                asm!("csrc sip, {}", in(reg) 2, options(nomem, preserves_flags));
                return 0;
            }
            9 => {
                crate::serial::write(b"WARN: External Int\n");
                return 0;
            }
            _ => {
                crate::serial::write(b"WARN: Unknown Int\n");
                return 0;
            }
        }
    }

    if kind == FaultKind::Breakpoint {
        ctx.sepc += 4;
        return 0;
    }

    ::graph::store::with_store(|store| {
        let rec = TrapRecord {
            arch: Arch::Riscv64,
            kind,
            ip: ctx.sepc,
            sp: ctx.regs[1],
            addr,
            code: scause,
            vector: code as u32,
            cpu: 0,
            in_kernel,
            task: None,
        };
        trap::record_fault(store, &rec);
    });

    match kind {
        FaultKind::Syscall => {
            let nr = ctx.regs[16];
            let a0 = ctx.regs[9];
            let a1 = ctx.regs[10];
            let a2 = ctx.regs[11];
            let a3 = ctx.regs[12];
            let a4 = ctx.regs[13];
            let a5 = ctx.regs[14];
            ctx.sepc += 4;
            let res = crate::syscall::dispatch::dispatch(nr as u32, a0, a1, a2, a3, a4, a5);
            ctx.regs[9] = res.status; // a0 = status
            ctx.regs[10] = res.val0; // a1 = val0
            ctx.regs[11] = res.val1; // a2 = val1
            return 0;
        }
        _ => {
            panic_trap(ctx, kind);
        }
    }
}
