
use crate::trap::{self, TrapRecord, FaultKind, Arch};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapContext {
    pub regs: [u64; 31], // r1-r31 (r0 is zero)
    pub era: u64,   // ERA (EPC)
    pub estat: u64, // ESTAT
    pub badv: u64,  // BADV
    pub prmd: u64,  // PRMD
}

#[no_mangle]
pub unsafe extern "C" fn loongarch64_handle_trap(ctx: &mut TrapContext) -> u64 {
    let estat = ctx.estat;
    let ecode = (estat >> 16) & 0x3f; // ECODE is bits 16-21
    let is_interrupt = (estat & 0x1fff) != 0; // IS bits 0-12
    
    let mut kind;
    let mut addr = None;
    
    if is_interrupt {
        // Simple map for now.
        // IS[11] is timer? IS[12] is IPI? Depends on config.
        // Assuming implementation standard:
        kind = FaultKind::Irq; 
        if (estat & (1<<11)) != 0 {
             kind = FaultKind::Timer;
        }
    } else {
        match ecode {
            0 => { // INT (Interrupt? Should be covered by IS check?)
                kind = FaultKind::Irq; 
            }
            1 => { // PIL (Page Invalid Load)
                kind = FaultKind::PageFault;
                addr = Some(ctx.badv);
            }
            2 => { // PIS (Page Invalid Store)
                kind = FaultKind::PageFault;
                addr = Some(ctx.badv);
            }
            3 => { // PIF (Page Invalid Fetch)
                kind = FaultKind::PageFault;
                addr = Some(ctx.badv);
            }
            4 => { // PME (Page Modification/Dirty)
                kind = FaultKind::PageFault;
                addr = Some(ctx.badv);
            }
            7 => { // ADE (Address Error)
                 kind = FaultKind::AccessFault;
                 addr = Some(ctx.badv);
            } 
            11 => kind = FaultKind::Syscall, // SYS
            12 => kind = FaultKind::Breakpoint, // BRK
            13 => kind = FaultKind::IllegalInstruction, // INE
            _ => kind = FaultKind::Unknown,
        }
    }
    
    // PRMD.PPLV (bits 0-1) = Previous Privilege Level. 0 = Highest (Kernel), 3 = User.
    // Actually LoongArch: 0=PLV0 (Kernel), 3=PLV3 (User).
    let prmd = ctx.prmd;
    let pplv = prmd & 0x3;
    let in_kernel = pplv == 0;

    graph::store::with_store(|store| {
         let rec = TrapRecord {
            arch: Arch::LoongArch64,
            kind,
            ip: ctx.era,
            sp: 0, // Need from regs if needed. r3 is sp. regs[2] (r3 is index 3 in architectural, index 2 in 0-based array sans r0?)
            // We save r1-r31. so r3 is at index 2.
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
            // Advance ERA by 4? 
            ctx.era += 4;
            return 0;
        }
        FaultKind::Timer => {
            super::timer::ack();
            let current_sp = ctx as *mut TrapContext as u64;
            if let Some(new_sp) = crate::sched::tick(current_sp) {
                return new_sp;
            }
        }
        _ => {}
    }
    0
}
