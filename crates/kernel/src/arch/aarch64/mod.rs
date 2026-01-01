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

    // AArch64 task entry stack frame simulation
    // switch_to pops 6 pairs (12 regs): x19-x30
    // Layout (growing down):
    // [ x29, x30 ] (Top)
    // [ x27, x28 ]
    // ...
    // [ x19, x20 ] (Bottom)

    // Allocate space (96 bytes)
    sp = sp.sub(12);
    
    // Clear area (0 init)
    core::ptr::write_bytes(sp as *mut u8, 0, 12 * 8);

    // Set x30 (LR) to entry stub (at sp+11)
    let lr_ptr = sp.add(11);
    *lr_ptr = machine().task_entry_stub();
    
    // Stub expects dispatch_ptr and entry_point on the stack ABOVE the frame popped by switch_to?
    // Let's check task_entry_stub:
    // ldr x0, [sp], #8 -> pops dispatch
    // ldr x1, [sp], #8 -> pops entry
    
    // So when switch_to returns, SP is at "Top" (original stack_top).
    // The stub executes. It pops from SP.
    // So we need to ensure dispatch/entry are at stack_top (before frame allocation).
    
    // Wait, `sp = stack_top as *mut u64`.
    // We need to push dispatch/entry FIRST.
    // "Top"
    // [ entry_point ] (Top - 8)
    // [ dispatch_ptr ] (Top - 16)
    // [ saved_regs ] (Top - 16 - 96)
    
    // Reset sp to stack_top
    sp = stack_top as *mut u64;
    
    // Push entry_point
    sp = sp.sub(1);
    *sp = entry_point;
    
    // Push dispatch_ptr
    sp = sp.sub(1);
    *sp = dispatch_ptr;
    
    // Now push saved regs frame (12 regs)
    sp = sp.sub(12);
    core::ptr::write_bytes(sp as *mut u8, 0, 12 * 8); // Clear regs
    
    // Set LR (x30 is at index 11 in the frame)
    *sp.add(11) = machine().task_entry_stub();
    
    task_ctx.sp = sp as u64;
}
