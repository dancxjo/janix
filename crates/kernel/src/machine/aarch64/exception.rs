//! AArch64 Exception Handlers
//!
//! Handles exceptions from EL1 (kernel) and EL0 (user).
//! Decodes ESR_EL1 to classify faults.

use crate::trap::{self, TrapRecord, FaultKind, Arch};
use crate::machine::aarch64::serial; // For panic logging if needed
use core::arch::asm;
use graph::store;

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
    let iss = esr & 0x1ffffff;
    
    // Default mappings
    let mut kind = FaultKind::Unknown;
    let mut addr = None;
    
    match ec {
        0x01 => { // WFI/WFE
            kind = FaultKind::Unknown; 
        }
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
            if offset == 1 {
                kind = FaultKind::Irq;
            } else if offset == 2 {
                kind = FaultKind::ExternalInterrupt; // FIQ treated as external?
            } else if offset == 3 {
                kind = FaultKind::Unknown; // SError
            }
        }
    }

    let in_kernel = (ctx.spsr_el1 & 0xF) == 0x4 || (ctx.spsr_el1 & 0xF) == 0x5;

    // We only record serious faults, not IRQs (spam) unless for debug
    if kind != FaultKind::Irq {
        graph::store::with_store(|store| {
             let rec = TrapRecord {
                arch: Arch::Aarch64,
                kind,
                ip: ctx.elr_el1,
                sp: ctx.sp_el0,
                addr,
                code: esr,
                vector: vector as u32,
                cpu: 0,
                in_kernel,
                task: None,
            };
            trap::record_fault(store, &rec);
        });
    }

    match kind {
        FaultKind::Breakpoint => {
             // For smoke test, we just continue.
             return 0; 
        }
        FaultKind::Irq => {
            // ACK GIC
            let irq_id = super::gic::ack_irq();
            // 1023 = Spurious
            if irq_id == 1023 {
                return 0;
            }
            
            if irq_id == super::timer::TIMER_IRQ {
                super::timer::ack(); // Rearm (and EOI internal?)
                // Actually my timer::ack calls eoi(30).
                // So GIC is happy.
                
                // Tick!
                let current_sp = ctx as *mut ExceptionContext as u64;
                // Wait, ctx is POINTER to stack struct. `current_sp` IS `ctx`.
                
                // Call sched::tick
                if let Some(new_sp) = crate::sched::tick(current_sp) {
                    return new_sp;
                }
            } else {
                // EOI unknown IRQ
                super::gic::eoi(irq_id);
            }
            return 0;
        }
        FaultKind::Syscall => {
            panic!("Syscall not implemented yet");
        }
        _ => {
            panic!("Unhandled AArch64 Exception: Limit reached.\n{:#?}", ctx);
        }
    }
}
