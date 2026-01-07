#![allow(unused)]

use crate::machine::context::{ArchTask, ArchTrap, CpuMode, ResumeSpec, TrapInfo};
use crate::memory::map::{MapPerms, MapResult};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TaskContext {
    pub sp: u64,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TrapFrame {
    pub ip: u64,
    pub sp: u64,
    pub flags: u64,
    pub rdi: u64,
}

pub struct TestArch;

impl ArchTask for TestArch {
    type TaskContext = TaskContext;

    fn init_task_context(
        ctx: &mut Self::TaskContext,
        entry: u64,
        stack_top: u64,
        _mode: CpuMode,
        _arg0: u64,
    ) {
        ctx.sp = stack_top;
    }
}

impl ArchTrap for TestArch {
    type TrapFrame = TrapFrame;
    type TaskContext = TaskContext;

    fn summarize(tf: &Self::TrapFrame) -> TrapInfo {
        TrapInfo {
            vector: 0,
            mode: CpuMode::Kernel,
            pc: tf.ip,
            sp: tf.sp,
            flags: tf.flags,
        }
    }

    fn mode(_tf: &Self::TrapFrame) -> CpuMode {
        CpuMode::Kernel
    }

    fn save_from_trap(tf: &Self::TrapFrame, out: &mut Self::TaskContext) {
        out.sp = tf.sp;
    }

    fn load_into_trap(ctx: &Self::TaskContext, tf: &mut Self::TrapFrame) {
        tf.sp = ctx.sp;
    }

    fn apply_resume_spec(_tf: &mut Self::TrapFrame, _spec: ResumeSpec) {}

    unsafe fn return_from_trap(_tf: *const Self::TrapFrame) -> ! {
        unreachable!("return_from_trap called in test harness")
    }
}

pub struct AddressSpace {
    // Mapping from Virt -> (Size, Perms)
    pub mappings: BTreeMap<u64, (usize, MapPerms)>,
}

impl AddressSpace {
    pub fn new() -> MapResult<Self> {
        Ok(Self { mappings: BTreeMap::new() })
    }

    pub fn from_existing(_pml4: u64) -> Self {
         Self { mappings: BTreeMap::new() }
    }

    pub fn activate(&self) {}

    pub fn map(&mut self, virt: u64, _phys: u64, len: usize, perms: MapPerms) -> MapResult<()> {
        self.mappings.insert(virt, (len, perms));
        Ok(())
    }

    pub fn user_range_end(&self) -> u64 {
        u64::MAX
    }

    pub fn probe_user_range(&self, start: u64, len: usize, perms: MapPerms) -> bool {
        // Naive check: does the start address fall into any mapped region with correct perms?
        // And does that region cover the whole length?

        for (base, (size, map_perms)) in self.mappings.iter() {
            let end = base + *size as u64;
            if start >= *base && (start + len as u64) <= end {
                if map_perms.contains(perms) {
                    return true;
                }
            }
        }
        false
    }
}
