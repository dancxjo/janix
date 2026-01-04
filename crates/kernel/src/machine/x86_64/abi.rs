use crate::machine::machine;
use crate::machine::Context;

/// Setup a new task's stack and context
///
/// Prepares the kernel stack for a new task so that `switch_to` can
/// successfully switch to it and execute `task_entry_stub` -> `entry_point`.
pub unsafe fn setup_new_task_stack(
    stack_top: *mut u8,
    entry_point: u64,
    dispatch_ptr: u64,
    task_ctx: &mut Context,
) {
    // x86_64 stack layout for task_entry:
    // Top:
    // [ dispatch_ptr (rdi) ] -> popped by task_entry stub
    // [ entry_point (rax) ]  -> popped by task_entry stub
    // [ ret_addr ]           -> popped by x86_switch_context (ret)
    // [ regs ]               -> popped by x86_switch_context
    // -> SP

    let mut sp = stack_top as *mut u64;

    // "pop rax" gets this (entry_point)
    sp = sp.sub(1);
    *sp = entry_point;

    // "pop rdi" gets this (dispatch_ptr)
    sp = sp.sub(1);
    *sp = dispatch_ptr;

    // Return address for `switch_to` -> `task_entry_stub`
    sp = sp.sub(1);
    *sp = machine().task_entry_stub();

    // 6 callee-saved registers (rbx, rbp, r12-r15)
    sp = sp.sub(6);
    core::ptr::write_bytes(sp as *mut u8, 0, 6 * 8);

    task_ctx.sp = sp as u64;
}
