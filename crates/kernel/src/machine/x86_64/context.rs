//! x86_64 ArchContext implementation.
//!
//! Implements the cross-arch preemption contract for x86_64.

use super::TrapFrame;
use crate::machine::context::{ArchTask, ArchTrap, CpuMode, ResumeSpec, TrapInfo};
use core::arch::asm;
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

/// Initialize FPU/SSE state to a clean default.
/// This MUST be called before any new thread runs code that uses SSE/AVX.
#[inline(always)]
pub unsafe fn init_fpu_state() {
    // Initialize x87 FPU to clean state
    asm!("fninit", options(nomem, nostack, preserves_flags));
    
    // Zero all SSE registers
    asm!(
        "xorps xmm0, xmm0",
        "xorps xmm1, xmm1",
        "xorps xmm2, xmm2",
        "xorps xmm3, xmm3",
        "xorps xmm4, xmm4",
        "xorps xmm5, xmm5",
        "xorps xmm6, xmm6",
        "xorps xmm7, xmm7",
        "xorps xmm8, xmm8",
        "xorps xmm9, xmm9",
        "xorps xmm10, xmm10",
        "xorps xmm11, xmm11",
        "xorps xmm12, xmm12",
        "xorps xmm13, xmm13",
        "xorps xmm14, xmm14",
        "xorps xmm15, xmm15",
        options(nomem, nostack, preserves_flags)
    );
    
    // Set MXCSR to default (mask all SSE exceptions)
    let default_mxcsr: u32 = 0x1F80;
    asm!("ldmxcsr [{}]", in(reg) &default_mxcsr, options(nostack));
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

            // Set first argument in RDI
            (*frame_ptr).rdi = arg0;

            // Return directly through the interrupt return path.
            ctx.sp = frame_ptr as u64;
            
            // Initialize FPU/SSE state for this new thread
            // This ensures the thread starts with clean SIMD state
            init_fpu_state();
        }
    }
}

impl ArchTrap for X86Arch {
    type TrapFrame = TrapFrame;
    type TaskContext = TaskContext;

    fn summarize(tf: &Self::TrapFrame) -> TrapInfo {
        TrapInfo {
            vector: 0,
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
        out.sp = tf as *const TrapFrame as u64;
    }

    fn load_into_trap(ctx: &Self::TaskContext, tf: &mut Self::TrapFrame) {
        let saved = ctx.sp as *const TrapFrame;
        unsafe {
            *tf = *saved;
        }
    }

    fn apply_resume_spec(tf: &mut Self::TrapFrame, spec: ResumeSpec) {
        match spec.mode {
            CpuMode::User => {
                tf.cs = 0x2b;
                tf.ss = 0x23;
            }
            CpuMode::Kernel => {
                tf.cs = 0x08;
                tf.ss = 0x10;
            }
        }

        if spec.interrupts_enabled {
            tf.rflags |= 0x200;
        } else {
            tf.rflags &= !0x200;
        }
    }

    unsafe fn return_from_trap(tf: *const Self::TrapFrame) -> ! {
        extern "C" {
            fn x86_return_from_trap(tf: *const TrapFrame) -> !;
        }
        x86_return_from_trap(tf)
    }
}
