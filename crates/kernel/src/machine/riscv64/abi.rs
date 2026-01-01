use crate::machine::Context;
use crate::machine::machine;

/// Setup a new task's stack and context
///
/// Prepares the kernel stack for a new task so that `switch_to` can
/// successfully switch to it and execute `task_entry_stub` -> `entry_point`.
pub unsafe fn setup_new_task_stack(
    stack_top: *mut u8,
    entry_point: u64,
    dispatch_ptr: u64,
    task_ctx: &mut Context
) {
    let mut sp = stack_top as *mut u64;

    // Push args needed by task_entry_stub
    // It will pop dispatch_ptr (a0) and entry_point (a1)
    sp = sp.sub(1);
    *sp = entry_point;
    sp = sp.sub(1);
    *sp = dispatch_ptr;

    // Push context registers for switch_to (ra, s0-s11 = 13 regs)
    // Layout assumes standard switch_to:
    // sd ra,  0(sp)
    // sd s0,  8(sp)
    // ...
    // sd s11, 96(sp)
    
    sp = sp.sub(13);
    core::ptr::write_bytes(sp as *mut u8, 0, 13 * 8);

    // Set ra (at top of frame, index 0) to task_entry_stub
    *sp = machine().task_entry_stub();

    task_ctx.sp = sp as u64;
}
