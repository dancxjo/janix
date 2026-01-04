//! x86_64 ArchContext implementation.
//!
//! Implements the cross-arch preemption contract for x86_64.

use super::TrapFrame;
use crate::machine::context::{ArchTask, ArchTrap, CpuMode, ResumeSpec, TrapInfo};
use core::mem;

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
        // Build a TrapFrame on the stack that the interrupt return path can restore.
        // Stack layout (growing down):
        //   [SS]
        //   [RSP]
        //   [RFLAGS]
        //   [CS]
        //   [RIP]
        //   [r15] .. [rax]
        // ctx.sp points at [rax] so the timer trampoline can pop/iretq directly.
        let aligned_top = stack_top & !0xf; // 16-byte align
        let frame_size = mem::size_of::<TrapFrame>() as u64;
        let frame_ptr = (aligned_top - frame_size) as *mut TrapFrame;

        unsafe {
            // Zero the frame first
            core::ptr::write_bytes(frame_ptr as *mut u8, 0, frame_size as usize);

            // Set up iretq frame (5 words)
            match mode {
                CpuMode::User => {
                    (*frame_ptr).cs = 0x2b; // User Code 64 (Index 5 | 3)
                    (*frame_ptr).ss = 0x23; // User Data (Index 4 | 3)
                    (*frame_ptr).rsp = arg0; // User stack from arg0
                    (*frame_ptr).rflags = 0x202; // IF=1
                    (*frame_ptr).rip = entry;
                }
                CpuMode::Kernel => {
                    (*frame_ptr).cs = 0x08; // Kernel code segment
                    (*frame_ptr).ss = 0x10; // Kernel data segment
                    (*frame_ptr).rsp = aligned_top; // Kernel stack top
                    (*frame_ptr).rflags = 0x202;
                    (*frame_ptr).rip = entry;
                }
            }

            // Set first argument in RDI (NOT needed for TrapFrame return, but for consistency)
            (*frame_ptr).rdi = arg0;

            // Return directly through the interrupt return path.
            ctx.sp = frame_ptr as u64;
        }
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
                tf.cs = 0x2b; // User Code 64 (Index 5 | 3)
                tf.ss = 0x23; // User Data (Index 4 | 3)
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
