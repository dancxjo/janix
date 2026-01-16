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
use crate::syscall::user_mem;

pub fn check(op: CapOp, target: Option<ThingId>) -> Result<(), SyscallResult> {
    sched::with_current_task(|task| user_mem::require_cap(task, op, target))
        .unwrap_or(Err(err::EFAULT))
        .map_err(|code| SyscallResult::new(code, 0, 0))
}

pub fn sys_cap_grant(target_low: u64, target_high: u64, cap_ptr: u64) -> SyscallResult {
    // 1. Check if caller has GrantCaps
    if let Err(code) = user_mem::require_current_cap(CapOp::GrantCaps, None) {
        return SyscallResult::new(code, 0, 0);
    }

    // 2. Decode Arguments
    let target_id = ThingId::from_parts(target_high, target_low);
    if cap_ptr == 0 {
        return SyscallResult::new(err::EFAULT, 0, 0);
    }
    
    let mut cap = Cap {
        op: CapOp::Log,
        scope: CapScope::Global,
    };
    let cap_bytes = unsafe {
        core::slice::from_raw_parts_mut(
            &mut cap as *mut Cap as *mut u8,
            core::mem::size_of::<Cap>(),
        )
    };
    if let Err(code) = user_mem::copy_from_user(cap_bytes, cap_ptr, cap_bytes.len()) {
        return SyscallResult::new(code, 0, 0);
    }

    // 3. Apply to Target Task
    // We need to find the task by ThingId. Scheduler tracks TaskId, but Task struct has ThingId.
    // We need a helper to find task by ThingId.
    // Optimization: If target_id == current_task.thing (Self grant), easy.
    // Else scan.
    
    let found = sched::with_sched(|sched| {
        // Iterate over values since BTreeMap is keyed by TaskId, not ThingId
        if let Some(task) = sched.tasks.values_mut().find(|t| t.thing == target_id) {
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

/// Grant creator ownership capabilities on a newly created Thing.
///
/// This implements the default policy: when a task creates a Thing, it automatically
/// receives read and write capabilities scoped to that Thing.
///
/// This is called from object creation syscalls (e.g., surface_create) to grant
/// the creator appropriate capabilities without needing boot grants.
pub fn grant_creator_ownership(thing_id: ThingId) {
    use abi::cap::{Cap, CapOp, CapScope};
    
    crate::sched::with_current_task(|task| {
        task.caps.push(Cap {
            op: CapOp::GraphRead,
            scope: CapScope::Thing(thing_id),
        });
        task.caps.push(Cap {
            op: CapOp::GraphWrite,
            scope: CapScope::Thing(thing_id),
        });
    });
}
