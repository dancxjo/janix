//! x86_64 ArchContext implementation.
//!
//! Implements the cross-arch preemption contract for x86_64.

use super::TrapFrame;
use crate::machine::context::{ArchTask, ArchTrap, CpuMode, TrapInfo, ResumeSpec};

/// x86_64 architecture context implementation.
pub struct X86Arch;

/// Task context stored in task struct.
/// Contains the stack pointer pointing to a prepared TrapFrame.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TaskContext {
    /// Stack pointer pointing to saved TrapFrame (or prepared initial frame).
    pub sp: u64,
}

impl ArchTask for X86Arch {
    type TaskContext = TaskContext;

    fn init_task_context(
        ctx: &mut Self::TaskContext,
        entry: u64,
        stack_top: u64,
        mode: CpuMode,
        arg0: u64,
    ) {
        // Build a TrapFrame on the stack that return_from_trap will restore.
        // Stack layout (growing down):
        //   [SS]      +160
        //   [RSP]     +152
        //   [RFLAGS]  +144
        //   [CS]      +136
        //   [RIP]     +128
        //   [r15]     +120
        //   ... GPRs ...
        //   [rax]     +0   <- ctx.sp points here
        
        let aligned_top = stack_top & !0xf; // 16-byte align
        
        // Frame size: 15 GPRs (120 bytes) + 5 iretq words (40 bytes) = 160 bytes
        let frame_ptr = (aligned_top - 160) as *mut u64;
        
        unsafe {
            // Zero the frame first
            core::ptr::write_bytes(frame_ptr as *mut u8, 0, 160);
            
            let frame = frame_ptr as *mut TrapFrame;
            
            // Set up iretq frame (5 words)
            // GDT: 0=Null, 8=KCode, 16=KData, 24=UCode32, 32=UData, 40=UCode64
            match mode {
                CpuMode::User => {
                    (*frame).ss = 0x23;   // User data segment (index 4 | RPL 3 = 35 = 0x23)
                    (*frame).rsp = arg0;  // User stack from arg0
                    (*frame).cs = 0x2B;   // User code segment (index 5 | RPL 3 = 43 = 0x2B)
                }
                CpuMode::Kernel => {
                    (*frame).ss = 0x10;   // Kernel data segment (index 2 = 16)
                    (*frame).rsp = aligned_top; // Kernel stack top
                    (*frame).cs = 0x08;   // Kernel code segment (index 1 = 8)
                }
            }
            (*frame).rflags = 0x202;  // IF=1 (interrupts enabled)
            (*frame).rip = entry;
            
            // Set first argument in RDI
            (*frame).rdi = arg0;
        }
        
        ctx.sp = frame_ptr as u64;
    }
}

impl ArchTrap for X86Arch {
    type TrapFrame = TrapFrame;
    type TaskContext = TaskContext;

    fn summarize(tf: &Self::TrapFrame) -> TrapInfo {
        TrapInfo {
            vector: 0, // Would need to be passed in or stored in frame
            mode: Self::mode(tf),
            pc: tf.rip,
            sp: tf.rsp,
            flags: tf.rflags,
        }
    }

    fn mode(tf: &Self::TrapFrame) -> CpuMode {
        if tf.cs & 3 != 0 {
            CpuMode::User
        } else {
            CpuMode::Kernel
        }
    }

    fn save_from_trap(tf: &Self::TrapFrame, out: &mut Self::TaskContext) {
        // The trap frame IS the saved state. Just record where it is.
        out.sp = tf as *const TrapFrame as u64;
    }

    fn load_into_trap(ctx: &Self::TaskContext, tf: &mut Self::TrapFrame) {
        // Copy saved context into the live trap frame
        let saved = ctx.sp as *const TrapFrame;
        unsafe {
            *tf = *saved;
        }
    }

    fn apply_resume_spec(tf: &mut Self::TrapFrame, spec: ResumeSpec) {
        // Set privilege level via CS
        match spec.mode {
            CpuMode::User => {
                tf.cs = 0x23;
                tf.ss = 0x1b;
            }
            CpuMode::Kernel => {
                tf.cs = 0x08;
                tf.ss = 0x10;
            }
        }
        
        // Set interrupt flag
        if spec.interrupts_enabled {
            tf.rflags |= 0x200;
        } else {
            tf.rflags &= !0x200;
        }
    }

    unsafe fn return_from_trap(tf: *const Self::TrapFrame) -> ! {
        // The trap frame must be on the stack in canonical layout.
        // We set RSP to point at the GPR portion, restore GPRs, then iretq.
        extern "C" {
            fn x86_return_from_trap(tf: *const TrapFrame) -> !;
        }
        x86_return_from_trap(tf)
    }
}
