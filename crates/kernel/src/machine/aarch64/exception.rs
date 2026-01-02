//! AArch64 Exception Handlers
//!
//! Handles exceptions from EL1 (kernel) and EL0 (user).
//! Decodes ESR_EL1 to classify faults.

use crate::trap::{self, TrapRecord, FaultKind, Arch};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExceptionContext {
    pub x0: u64, pub x1: u64, pub x2: u64, pub x3: u64,
    pub x4: u64, pub x5: u64, pub x6: u64, pub x7: u64,
    pub x8: u64, pub x9: u64, pub x10: u64, pub x11: u64,
    pub x12: u64, pub x13: u64, pub x14: u64, pub x15: u64,
    pub x16: u64, pub x17: u64, pub x18: u64, pub x19: u64,
    pub x20: u64, pub x21: u64, pub x22: u64, pub x23: u64,
    pub x24: u64, pub x25: u64, pub x26: u64, pub x27: u64,
    pub x28: u64, pub x29: u64, pub x30: u64,
    pub sp_el0: u64,
    pub elr_el1: u64,
    pub spsr_el1: u64,
    pub esr_el1: u64,
    pub far_el1: u64,
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_handle_exception(ctx: &mut ExceptionContext, vector: u64) -> u64 {
    let esr = ctx.esr_el1;
    let ec = (esr >> 26) & 0x3f;
    
    // Default mappings
    let mut kind = FaultKind::Unknown;
    let mut addr = None;
    
    match ec {
        0x15 => { // SVC instruction execution in AArch64 state
            kind = FaultKind::Syscall;
        }
        0x20 | 0x21 => { // Instruction Abort (Lower EL | Current EL)
            kind = FaultKind::InstructionAbort;
            addr = Some(ctx.far_el1);
        }
        0x24 | 0x25 => { // Data Abort (Lower EL | Current EL)
            kind = FaultKind::DataAbort;
            addr = Some(ctx.far_el1);
        }
        0x3C => { // BRK instruction execution in AArch64 state
            kind = FaultKind::Breakpoint;
        }
        _ => {
            let offset = vector & 0x3;
            if vector == 5 || vector == 9 {
                kind = FaultKind::Irq;
            } else if offset == 2 {
                kind = FaultKind::ExternalInterrupt;
            }
        }
    }

    let in_kernel = (ctx.spsr_el1 & 0xF) == 0x4 || (ctx.spsr_el1 & 0xF) == 0x5;

    // Handle IRQs first
    if kind == FaultKind::Irq {
        let irq_id = super::gic::ack_irq();
        if irq_id == 1023 { return 0; }
        
        if irq_id == super::timer::TIMER_IRQ {
            super::timer::ack(); 
            let current_sp = ctx as *mut ExceptionContext as u64;
            let new_sp = crate::sched::tick(current_sp);
            if new_sp != 0 {
                return new_sp;
            }
        } else {
            super::gic::eoi(irq_id);
        }
        return 0;
    }

    // Handle Lazy Mapping (Data Abort)
    if kind == FaultKind::DataAbort {
        if let Some(fault_addr) = addr {
             if in_kernel && fault_addr >= 0xffffffff80000000 {
                 use crate::machine::MmioFlags;
                 let aligned = fault_addr & !0xfff;
                 let flags = MmioFlags::READ | MmioFlags::WRITE;
                 if unsafe { super::ARCH_MACHINE_IMPL.map_kernel_region(aligned, 0x1000, flags) } {
                     return 0; 
                 }
             }
        }
    }

    match kind {
        FaultKind::Breakpoint => {
             crate::serial::write(b"BREAKPOINT HIT\n");
             return 0; 
        }
        FaultKind::Syscall => {
            let nr = ctx.x8 as u32;
            let res = crate::syscall::dispatch(nr, ctx.x0, ctx.x1, ctx.x2, ctx.x3, ctx.x4, ctx.x5);
            ctx.x0 = res.status as u64;
            ctx.x1 = res.val0;
            ctx.x2 = res.val1;
            ctx.elr_el1 += 4;
            return 0;
        }
        _ => {
            crate::serial::write(b"AARCH64 FAULT: V=");
            crate::serial::write_hex(vector);
            crate::serial::write(b" ESR=");
            crate::serial::write_hex(esr);
            crate::serial::write(b"\n");
            panic!("Unhandled AArch64 Exception\n{:#?}", ctx);
        }
    }
}
