//! Capability Enforcement
//!
//! Checks if the current task has the required capability to perform an operation.
//! 
//! Semantic Rules:
//! - Checks `current_task().caps` (in-memory fast path).
//! - Graph reflection is optional/best-effort (not implemented in check path).

pub use abi::cap::{Cap, CapOp, CapScope};
use abi::ids::ThingId;
use abi::syscall::err;
use abi::wire::SyscallResult;
use crate::sched;

pub fn check(op: CapOp, target: Option<ThingId>) -> Result<(), SyscallResult> {
    sched::with_current_task(|task| {
        // Superuser bypass for kernel tasks? 
        // For now, explicit grants only.
        
        for cap in &task.caps {
            if cap.op == op {
                match cap.scope {
                    CapScope::Global => return Ok(()),
                    CapScope::Thing(id) => {
                        if let Some(req_target) = target {
                            if req_target == id {
                                return Ok(());
                            }
                        } else {
                            // If operation doesn't require target but cap is scoped...
                            // Usually this means "Access to Thing X", but if op is Global-like...
                            // This branch means user asked check(Op, None) but cap is Op(Thing).
                            // This is a mismatch unless Op implies wildcard.
                            // Strict match for now.
                        }
                    }
                }
            }
        }
        
        // No matching cap found
        Err(SyscallResult::new(err::EPERM, 0, 0))
    }).unwrap_or(Err(SyscallResult::new(err::EFAULT, 0, 0)))
}

pub fn sys_cap_grant(target_low: u64, target_high: u64, cap_ptr: u64) -> SyscallResult {
    // 1. Check if caller has GrantCaps
    if let Err(e) = check(CapOp::GrantCaps, None) {
        return e;
    }

    // 2. Decode Arguments
    let target_id = ThingId::from_parts(target_high, target_low);
    if cap_ptr == 0 {
        return SyscallResult::new(err::EFAULT, 0, 0);
    }
    
    // Safety: Userspace pointer dereference.
    // In a real kernel we must copy_from_user / verify bounds.
    // For now assuming identity map / valid user ptr in same address space (linear model)
    // or manually reconstructing.
    // Let's copy it safely.
    let cap = unsafe {
        *(cap_ptr as *const Cap)
    };

    // 3. Apply to Target Task
    // We need to find the task by ThingId. Scheduler tracks TaskId, but Task struct has ThingId.
    // We need a helper to find task by ThingId.
    // Optimization: If target_id == current_task.thing (Self grant), easy.
    // Else scan.
    
    let found = sched::with_sched(|sched| {
        if let Some(task) = sched.tasks.iter_mut().find(|t| t.thing == target_id) {
            task.caps.push(cap);
            true
        } else {
            false
        }
    });

    if found {
        SyscallResult::new(0, 0, 0)
    } else {
        SyscallResult::new(err::ENOENT, 0, 0)
    }
}

// Helper for boot.rs and internal grants to inject caps
pub fn inject_cap(task_id: ThingId, cap: Cap) {
    sched::with_sched(|sched| {
        if let Some(task) = sched.tasks.iter_mut().find(|t| t.thing == task_id) {
            task.caps.push(cap);
        }
    });
}
