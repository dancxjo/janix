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
            match mode {
                CpuMode::User => {
                    (*frame).cs = 0x2b; // User Code 64 (Index 5 | 3)
                    (*frame).ss = 0x23; // User Data (Index 4 | 3)
                    (*frame).rsp = arg0;  // User stack from arg0
                    (*frame).rflags = 0x202; // IF=1
                    (*frame).rip = entry;
                }
                CpuMode::Kernel => {
                    (*frame).cs = 0x08;   // Kernel code segment
                    (*frame).ss = 0x10;   // Kernel data segment
                    (*frame).rsp = aligned_top; // Kernel stack top
                    (*frame).rflags = 0x202;
                    (*frame).rip = entry;
                }
            }
            
            // Set first argument in RDI (NOT needed for TrapFrame return, but for consistency)
            (*frame).rdi = arg0;
            
            // --- CONTEXT SWITCH FRAME SETUP ---
            // x86_switch_context expects the stack to allow returning to 'task_entry'.
            // task_entry expects [dispatch_ptr, entry_point] on stack.
            // Stack layout (growing down from frame_ptr):
            //   [x86_return_from_trap] (rsi / entry for task_entry)
            //   [frame_ptr]            (rdi / dispatch_ptr for task_entry)
            //   [task_entry]           (Return address for switch_to)
            //   [r15]..[rbx]           (Callee saved registers for switch_to)
            
            let mut sp = frame_ptr as u64;
            
            extern "C" {
                fn task_entry();
                fn x86_return_from_trap(tf: *const TrapFrame) -> !;
            }
            
            // Push arguments for task_entry
            sp -= 8;
            *(sp as *mut u64) = x86_return_from_trap as usize as u64; // rsi
            sp -= 8;
            *(sp as *mut u64) = frame_ptr as u64; // rdi
            
            // Push Return Address for switch_to
            sp -= 8;
            *(sp as *mut u64) = task_entry as usize as u64;
            
            // Push Callee Saved Registers (rbx, rbp, r12, r13, r14, r15) - 6 registers
            // x86_switch_context: pop r15..r12, pop rbp, pop rbx.
            // Order on stack (bottom up): r15, r14, r13, r12, rbp, rbx.
            // Since we push downwards: rbx first? No, pop reverses push.
            // switch_to pops r15 first. So r15 is at top of stack (lowest address)?
            // "pop r15" -> rsp increments. So r15 is at current RSP.
            // So we just need to reserve space for 6 regs.
            sp -= 48; 
            
            // Padding for 16-byte alignment
            sp -= 8;
            
            ctx.sp = sp;
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
