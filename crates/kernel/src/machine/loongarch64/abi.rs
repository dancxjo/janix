use crate::machine::machine;
use crate::machine::Context;

/// Setup a new task's stack and context
pub unsafe fn setup_new_task_stack(
    stack_top: *mut u8,
    entry_point: u64,
    dispatch_ptr: u64,
    task_ctx: &mut Context,
) {
    let mut sp = stack_top as *mut u64;

    // Push args needed by task_entry_stub (a0, a1 equivalent)
    sp = sp.sub(1);
    *sp = entry_point;
    sp = sp.sub(1);
    *sp = dispatch_ptr;

    // Push context registers for switch_to
    // LoongArch64 callee saved: ra, fp, s0..s8 (11 regs)
    sp = sp.sub(11);
    core::ptr::write_bytes(sp as *mut u8, 0, 11 * 8);

    // Set ra (at top of frame, index 0) to task_entry_stub
    *sp = machine().task_entry_stub();

    task_ctx.sp = sp as u64;
}
