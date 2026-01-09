use crate::simd::SimdState;
use crate::BootRuntime;

pub struct Task {
    pub simd_state: SimdState,
    // Future fields: stack, cpu_context, etc.
}

impl Task {
    pub fn new(rt: &dyn BootRuntime) -> Self {
        Self {
            simd_state: SimdState::new(rt),
        }
    }
}

/// A stub for the context switch logic to demonstrate where SIMD save/restore lives.
///
/// In a real scheduler, this would be part of `switch_to` or similar.
///
/// # Safety
/// This function is unsafe because it performs raw context switching (stubbed).
pub unsafe fn context_switch(
    _old_task: Option<&mut Task>,
    _new_task: &Task,
    rt: &dyn BootRuntime,
) {
    // 1. Save old task's SIMD state eagerly
    if let Some(old) = _old_task {
        old.simd_state.save(rt);
    }

    // 2. Platform specific context switch (save registers, switch stack)
    // ... asm block would go here ...

    // 3. Restore new task's SIMD state eagerly
    _new_task.simd_state.restore(rt);
}
