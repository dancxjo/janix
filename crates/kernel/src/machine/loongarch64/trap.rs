use crate::trap::{self, Arch, FaultKind, TrapRecord};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapContext {
    pub regs: [u64; 31],
    pub era: u64,
    pub estat: u64,
    pub badv: u64,
    pub prmd: u64,
}

fn decode_ecode(e: u64) -> &'static str {
    match e {
        0 => "INT",
        1 => "PIL (Page Invalid Load)",
        2 => "PIS (Page Invalid Store)",
        3 => "PIF (Page Invalid Fetch)",
        4 => "PME (Page Modification)",
        7 => "ADE (Address Error)",
        11 => "SYS (Syscall)",
        12 => "BRK (Breakpoint)",
        13 => "INE (Instruction Non-Existent)",
        14 => "IPE (Privilege Error)",
        _ => "Unknown/Reserved",
    }
}

fn dump_regs(ctx: &TrapContext) {
    use crate::serial::{write, write_hex};
    write(b"\n--- Registers ---\n");
    for i in 0..31 {
        write(b"r");
        if i + 1 < 10 {
            write(&[b'0' + (i + 1) as u8]);
        } else {
            write(&[b'0' + ((i + 1) / 10) as u8, b'0' + ((i + 1) % 10) as u8]);
        }
        write(b"=");
        write_hex(ctx.regs[i]);
        if (i + 1) % 4 == 0 {
            write(b"\n");
        } else {
            write(b" ");
        }
    }
    write(b"\n");
}

fn panic_trap(ctx: &TrapContext, kind: FaultKind) -> ! {
    use crate::serial::{write, write_hex};
    let ecode = (ctx.estat >> 16) & 0x3f;

    write(b"\n\n========== LoongArch64 FATAL EXCEPTION ==========\n");
    write(b"Kind: ");
    match kind {
        FaultKind::PageFault => write(b"PageFault"),
        FaultKind::AccessFault => write(b"AccessFault"),
        FaultKind::IllegalInstruction => write(b"IllegalInstruction"),
        _ => write(b"Unknown"),
    }
    write(b"\n");

    write(b"ESTAT: ");
    write_hex(ctx.estat);
    write(b"\n");
    write(b"  ECODE: ");
    write_hex(ecode);
    write(b" -> ");
    write(decode_ecode(ecode).as_bytes());
    write(b"\n");

    write(b"\nERA: ");
    write_hex(ctx.era);
    write(b"\nBADV: ");
    write_hex(ctx.badv);
    write(b"\nPRMD: ");
    write_hex(ctx.prmd);
    let pplv = ctx.prmd & 0x3;
    write(b" (PPLV=");
    write_hex(pplv);
    if pplv == 0 {
        write(b" Kernel)\n");
    } else {
        write(b" User)\n");
    }

    dump_regs(ctx);
    write(b"==================================================\n");
    panic!("LoongArch64 Exception");
}

#[no_mangle]
pub unsafe extern "C" fn loongarch64_handle_trap(ctx: &mut TrapContext) -> u64 {
    let estat = ctx.estat;
    let ecode = (estat >> 16) & 0x3f;
    let is_int = (estat & 0x1fff) != 0;

    let mut kind;
    let mut addr = None;

    if is_int {
        kind = FaultKind::Irq;
        if (estat & (1 << 11)) != 0 {
            kind = FaultKind::Timer;
        }
    } else {
        match ecode {
            0 => kind = FaultKind::Irq,
            1 | 2 | 3 | 4 => {
                kind = FaultKind::PageFault;
                addr = Some(ctx.badv);
            }
            7 => {
                kind = FaultKind::AccessFault;
                addr = Some(ctx.badv);
            }
            11 => kind = FaultKind::Syscall,
            12 => kind = FaultKind::Breakpoint,
            13 => kind = FaultKind::IllegalInstruction,
            _ => kind = FaultKind::Unknown,
        }
    }

    let pplv = ctx.prmd & 0x3;
    let in_kernel = pplv == 0;

    graph::store::with_store(|store| {
        let rec = TrapRecord {
            arch: Arch::LoongArch64,
            kind,
            ip: ctx.era,
            sp: ctx.regs[2],
            addr,
            code: estat,
            vector: ecode as u32,
            cpu: 0,
            in_kernel,
            task: None,
        };
        trap::record_fault(store, &rec);
    });

    match kind {
        FaultKind::Breakpoint => {
            ctx.era += 4;
            return 0;
        }
        FaultKind::Timer => {
            super::timer::ack();
            let sp = ctx as *mut TrapContext as u64;
            let new_sp = crate::sched::tick(sp);
            if new_sp != 0 {
                return new_sp;
            }
        }
        FaultKind::Syscall => {
            ctx.era += 4;
            let nr = ctx.regs[10];
            let a0 = ctx.regs[3];
            let a1 = ctx.regs[4];
            let a2 = ctx.regs[5];
            let a3 = ctx.regs[6];
            let a4 = ctx.regs[7];
            let a5 = ctx.regs[8];
            let res = crate::syscall::dispatch::dispatch(nr as u32, a0, a1, a2, a3, a4, a5);
            ctx.regs[3] = res.val0;
            ctx.regs[4] = res.val1;
            return 0;
        }
        FaultKind::Irq => {
            return 0;
        }
        _ => {
            panic_trap(ctx, kind);
        }
    }
    0
}
