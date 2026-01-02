//! riscv64 ArchContext implementation.

use super::TrapFrame;
use crate::machine::context::{ArchTask, ArchTrap, CpuMode, TrapInfo, ResumeSpec};

pub struct Riscv64Arch;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TaskContext {
    pub sp: u64,
}

impl ArchTask for Riscv64Arch {
    type TaskContext = TaskContext;

    fn init_task_context(
        ctx: &mut Self::TaskContext,
        entry: u64,
        stack_top: u64,
        mode: CpuMode,
        arg0: u64,
    ) {
        let aligned_top = stack_top & !0xf;
        let layout = core::alloc::Layout::new::<TrapFrame>();
        let frame_ptr = (aligned_top - layout.size() as u64) as *mut TrapFrame;
        
        unsafe {
            core::ptr::write_bytes(frame_ptr as *mut u8, 0, layout.size());
            let frame = &mut *frame_ptr;
            
            // Set program counter
            frame.sepc = entry;
            
            // Set stack pointer: x2 is regs[1] (x1-x31 so x2 is at index 1)
            // For user mode, this is the user stack
            // For kernel mode, use the kernel stack
            frame.regs[1] = match mode {
                CpuMode::User => arg0, // User stack passed via arg0
                CpuMode::Kernel => aligned_top,
            };
            
            // Set a0 (x10 = regs[9]) for first argument
            frame.regs[9] = arg0;
            
            // sstatus: SPP (bit 8) = privilege mode, SPIE (bit 5) = enable interrupts on sret
            frame.sstatus = match mode {
                CpuMode::Kernel => (1 << 8) | (1 << 5), // SPP=1 (S-mode), SPIE=1
                CpuMode::User => (1 << 5),              // SPP=0 (U-mode), SPIE=1
            };
        }
        
        ctx.sp = frame_ptr as u64;
    }
}

impl ArchTrap for Riscv64Arch {
    type TrapFrame = TrapFrame;
    type TaskContext = TaskContext;

    fn summarize(tf: &Self::TrapFrame) -> TrapInfo {
        TrapInfo {
            vector: tf.scause as u32,
            mode: Self::mode(tf),
            pc: tf.sepc,
            sp: tf.regs[1], // x2 = sp
            flags: tf.sstatus,
        }
    }

    fn mode(tf: &Self::TrapFrame) -> CpuMode {
        // sstatus.SPP (bit 8): 1 = Supervisor, 0 = User
        if tf.sstatus & (1 << 8) != 0 { CpuMode::Kernel } else { CpuMode::User }
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
            CpuMode::Kernel => tf.sstatus |= 1 << 8,
            CpuMode::User => tf.sstatus &= !(1 << 8),
        }
        if spec.interrupts_enabled {
            tf.sstatus |= 1 << 5; // SPIE
        } else {
            tf.sstatus &= !(1 << 5);
        }
    }

    unsafe fn return_from_trap(tf: *const Self::TrapFrame) -> ! {
        extern "C" {
            fn riscv64_return_from_trap(tf: *const TrapFrame) -> !;
        }
        riscv64_return_from_trap(tf)
    }
}
