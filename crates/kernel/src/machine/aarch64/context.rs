//! aarch64 ArchContext implementation (stub).

use super::TrapFrame;
use crate::machine::context::{ArchTask, ArchTrap, CpuMode, TrapInfo, ResumeSpec};

pub struct AArch64Arch;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TaskContext {
    pub sp: u64,
}

impl ArchTask for AArch64Arch {
    type TaskContext = TaskContext;

    fn init_task_context(
        ctx: &mut Self::TaskContext,
        entry: u64,
        stack_top: u64,
        mode: CpuMode,
        arg0: u64,
    ) {
        // Build a TrapFrame on the stack that aarch64_return_from_trap will restore.
        // Stack layout (growing down):
        //   [TrapFrame]
        //   ...
        
        let aligned_top = stack_top & !0xf; // 16-byte align
        let layout = core::alloc::Layout::new::<TrapFrame>();
        let frame_ptr = (aligned_top - layout.size() as u64) as *mut TrapFrame;
        
        unsafe {
            core::ptr::write_bytes(frame_ptr as *mut u8, 0, layout.size());
            let frame = &mut *frame_ptr;
            
            frame.elr_el1 = entry;
            frame.x[0] = arg0; // Pass arg0 to entry point
            
            match mode {
                CpuMode::Kernel => {
                    frame.spsr_el1 = 0x005; // EL1h, DAIF unmasked
                    frame.sp = 0;      
                }
                CpuMode::User => {
                    frame.spsr_el1 = 0x0;   // EL0t
                    frame.sp = arg0;    // If arg0 is user stack
                }
            }
        }
        
        ctx.sp = frame_ptr as u64;
    }
}

impl ArchTrap for AArch64Arch {
    type TrapFrame = TrapFrame;
    type TaskContext = TaskContext;

    fn summarize(tf: &Self::TrapFrame) -> TrapInfo {
        TrapInfo {
            vector: 0,
            mode: Self::mode(tf),
            pc: tf.elr_el1,
            sp: if Self::mode(tf) == CpuMode::User { tf.sp } else { tf as *const _ as u64 + core::mem::size_of::<TrapFrame>() as u64 },
            flags: tf.spsr_el1,
        }
    }

    fn mode(tf: &Self::TrapFrame) -> CpuMode {
        // SPSR_EL1.M[3:0]: 0x0 = EL0, 0x4/0x5 = EL1
        let m = tf.spsr_el1 & 0xf;
        if m == 0 { CpuMode::User } else { CpuMode::Kernel }
    }

    fn save_from_trap(tf: &Self::TrapFrame, out: &mut Self::TaskContext) {
        out.sp = tf as *const TrapFrame as u64;
    }

    fn load_into_trap(ctx: &Self::TaskContext, tf: &mut Self::TrapFrame) {
        let saved = ctx.sp as *const TrapFrame;
        unsafe { *tf = *saved; }
    }

    fn apply_resume_spec(tf: &mut Self::TrapFrame, spec: ResumeSpec) {
        match spec.mode {
            CpuMode::User => tf.spsr_el1 = (tf.spsr_el1 & !0xf) | 0x0,
            CpuMode::Kernel => tf.spsr_el1 = (tf.spsr_el1 & !0xf) | 0x5,
        }
        // DAIF bits for interrupts (bits 6-9)
        if spec.interrupts_enabled {
            tf.spsr_el1 &= !(0xf << 6);
        } else {
            tf.spsr_el1 |= 0xf << 6;
        }
    }

    unsafe fn return_from_trap(tf: *const Self::TrapFrame) -> ! {
        extern "C" {
            fn aarch64_return_from_trap(tf: *const TrapFrame) -> !;
        }
        aarch64_return_from_trap(tf)
    }
}
