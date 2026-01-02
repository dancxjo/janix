//! loongarch64 ArchContext implementation (stub).

use super::TrapFrame;
use crate::machine::context::{ArchTask, ArchTrap, CpuMode, TrapInfo, ResumeSpec};

pub struct LoongArchArch;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TaskContext {
    pub sp: u64,
}

impl ArchTask for LoongArchArch {
    type TaskContext = TaskContext;

    fn init_task_context(
        ctx: &mut Self::TaskContext,
        entry: u64,
        stack_top: u64,
        mode: CpuMode,
        _arg0: u64,
    ) {
        let aligned_top = stack_top & !0xf;
        let layout = core::alloc::Layout::new::<TrapFrame>();
        let frame_ptr = (aligned_top - layout.size() as u64) as *mut TrapFrame;
        
        unsafe {
            core::ptr::write_bytes(frame_ptr as *mut u8, 0, layout.size());
            let frame = &mut *frame_ptr;
            frame.era = entry;
            // PRMD: PPLV (0-1) = privilege, PIE (bit 2) = interrupt enable
            frame.prmd = match mode {
                CpuMode::Kernel => 0x4, // PLV0, PIE=1
                CpuMode::User => 0x3 | 0x4, // PLV3, PIE=1
            };
        }
        
        ctx.sp = frame_ptr as u64;
    }
}

impl ArchTrap for LoongArchArch {
    type TrapFrame = TrapFrame;
    type TaskContext = TaskContext;

    fn summarize(tf: &Self::TrapFrame) -> TrapInfo {
        TrapInfo {
            vector: ((tf.estat >> 16) & 0x3f) as u32,
            mode: Self::mode(tf),
            pc: tf.era,
            sp: tf.regs[2], // r3 = sp
            flags: tf.prmd,
        }
    }

    fn mode(tf: &Self::TrapFrame) -> CpuMode {
        // PRMD.PPLV (bits 0-1): 0 = Kernel, 3 = User
        let pplv = tf.prmd & 0x3;
        if pplv == 0 { CpuMode::Kernel } else { CpuMode::User }
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
            CpuMode::Kernel => tf.prmd = (tf.prmd & !0x3) | 0x0,
            CpuMode::User => tf.prmd = (tf.prmd & !0x3) | 0x3,
        }
        if spec.interrupts_enabled {
            tf.prmd |= 0x4; // PIE
        } else {
            tf.prmd &= !0x4;
        }
    }


    unsafe fn return_from_trap(tf: *const Self::TrapFrame) -> ! {
        extern "C" {
            fn loongarch64_trap_restore(tf: *const TrapFrame) -> !;
        }
        loongarch64_trap_restore(tf)
    }
}
