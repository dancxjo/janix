//! Static hook system for type-erased scheduler access.

use crate::task::{TaskId, TaskState};
use super::types::StackFaultResult;
use abi::errors::Errno;
use abi::vm::VmRegionInfo;
use alloc::vec::Vec;

pub(crate) static mut YIELD_HOOK: Option<fn()> = None;
pub(crate) static mut EXIT_HOOK: Option<fn(i32)> = None;
pub(crate) static mut SPAWN_USER_HOOK: Option<unsafe fn(usize, usize, usize, abi::types::StackInfo, crate::task::TaskPriority) -> TaskId> = None;
pub(crate) static mut SPAWN_PROCESS_HOOK: Option<unsafe fn(&str, usize) -> Option<TaskId>> = None;
pub(crate) static mut CURRENT_TID_HOOK: Option<fn() -> u64> = None;
pub(crate) static mut TASK_STATUS_HOOK: Option<fn(TaskId) -> Option<(TaskState, Option<i32>)>> = None;
pub(crate) static mut SET_PRIORITY_HOOK: Option<fn(TaskId, crate::task::TaskPriority)> = None;
pub(crate) static mut CURRENT_PRIORITY_HOOK: Option<fn() -> crate::task::TaskPriority> = None;
pub(crate) static mut ALLOC_USER_STACK_HOOK: Option<fn(usize) -> Option<usize>> = None;
pub(crate) static mut STACK_FAULT_HOOK: Option<unsafe fn(u64) -> StackFaultResult> = None;
pub(crate) static mut SLEEP_TICKS_HOOK: Option<fn(u64)> = None;
pub(crate) static mut ADD_USER_MAPPING_HOOK: Option<fn(VmRegionInfo) -> Result<(), Errno>> = None;
pub(crate) static mut REMOVE_USER_MAPPINGS_HOOK: Option<fn(usize, usize) -> Result<Vec<(usize, usize)>, Errno>> = None;
pub(crate) static mut CHECK_USER_MAPPING_HOOK: Option<fn(usize, usize, bool) -> bool> = None;
pub(crate) static mut GET_USER_MAPPING_AT_HOOK: Option<fn(usize) -> Option<VmRegionInfo>> = None;

pub unsafe fn yield_now_current() {
    if let Some(hook) = unsafe { YIELD_HOOK } {
        hook();
    }
}

pub unsafe fn exit_current(code: i32) {
    if let Some(hook) = unsafe { EXIT_HOOK } {
        hook(code);
    } else {
        // Fallback if no scheduler
        crate::kprintln!("exit_current called without scheduler!");
    }
}

pub unsafe fn current_tid_current() -> u64 {
    if let Some(hook) = unsafe { CURRENT_TID_HOOK } {
        hook()
    } else {
        0
    }
}

pub unsafe fn task_status_current(id: TaskId) -> Option<(TaskState, Option<i32>)> {
    if let Some(hook) = unsafe { TASK_STATUS_HOOK } {
        hook(id)
    } else {
        None
    }
}

pub unsafe fn get_user_mapping_at_current(addr: usize) -> Option<VmRegionInfo> {
    if let Some(hook) = unsafe { GET_USER_MAPPING_AT_HOOK } {
        hook(addr)
    } else {
        None
    }
}

pub unsafe fn spawn_process_current(name: &str, arg: usize) -> Option<TaskId> {
    if let Some(hook) = unsafe { SPAWN_PROCESS_HOOK } {
        unsafe { hook(name, arg) }
    } else {
        None
    }
}

pub unsafe fn spawn_user_thread_current(
    entry: usize,
    stack: usize,
    arg: usize,
    stack_info: abi::types::StackInfo,
    priority: crate::task::TaskPriority,
) -> Option<TaskId> {
    if let Some(hook) = unsafe { SPAWN_USER_HOOK } {
        Some(unsafe { hook(entry, stack, arg, stack_info, priority) })
    } else {
        None
    }
}

pub unsafe fn set_priority_current(id: TaskId, priority: crate::task::TaskPriority) {
    if let Some(hook) = unsafe { SET_PRIORITY_HOOK } {
        hook(id, priority);
    }
}

pub unsafe fn current_priority_current() -> crate::task::TaskPriority {
    if let Some(hook) = unsafe { CURRENT_PRIORITY_HOOK } {
        hook()
    } else {
        crate::task::TaskPriority::Normal
    }
}

pub unsafe fn handle_user_stack_fault_current(addr: u64) -> StackFaultResult {
    if let Some(hook) = unsafe { STACK_FAULT_HOOK } {
        unsafe { hook(addr) }
    } else {
        StackFaultResult::NotStack
    }
}

pub unsafe fn alloc_user_stack_current(pages: usize) -> Option<usize> {
    unsafe { ALLOC_USER_STACK_HOOK }.and_then(|hook| hook(pages))
}

/// True blocking sleep for the specified number of ticks
pub fn sleep_ticks_current(ticks: u64) {
    if let Some(hook) = unsafe { SLEEP_TICKS_HOOK } {
        hook(ticks);
    }
}

pub unsafe fn add_user_mapping_current(region: VmRegionInfo) -> Result<(), Errno> {
    if let Some(hook) = unsafe { ADD_USER_MAPPING_HOOK } {
        hook(region)
    } else {
        // If hook not present, we can't track.
        // For early boot, maybe ignore? But sys_vm_map calls this.
        // sys_vm_map shouldn't be called before scheduler init.
        Err(Errno::ENOSYS)
    }
}

pub unsafe fn remove_user_mappings_current(addr: usize, len: usize) -> Result<Vec<(usize, usize)>, Errno> {
    if let Some(hook) = unsafe { REMOVE_USER_MAPPINGS_HOOK } {
        hook(addr, len)
    } else {
        Err(Errno::ENOSYS)
    }
}

pub unsafe fn check_user_mapping_current(addr: usize, len: usize, write: bool) -> Option<bool> {
    if let Some(hook) = unsafe { CHECK_USER_MAPPING_HOOK } {
        Some(hook(addr, len, write))
    } else {
        None
    }
}
