//! Static hook system for type-erased scheduler access.

use crate::task::{TaskId, TaskState};
use super::types::StackFaultResult;

pub(crate) static mut YIELD_HOOK: Option<fn()> = None;
pub(crate) static mut EXIT_HOOK: Option<fn(i32)> = None;
pub(crate) static mut SPAWN_USER_HOOK: Option<unsafe fn(usize, usize, usize, abi::types::StackInfo) -> TaskId> = None;
pub(crate) static mut SPAWN_PROCESS_HOOK: Option<unsafe fn(&str, usize) -> Option<TaskId>> = None;
pub(crate) static mut CURRENT_TID_HOOK: Option<fn() -> u64> = None;
pub(crate) static mut TASK_STATUS_HOOK: Option<fn(TaskId) -> Option<(TaskState, Option<i32>)>> = None;
pub(crate) static mut ALLOC_USER_STACK_HOOK: Option<fn(usize) -> Option<usize>> = None;
pub(crate) static mut STACK_FAULT_HOOK: Option<unsafe fn(u64) -> StackFaultResult> = None;

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
) -> Option<TaskId> {
    if let Some(hook) = unsafe { SPAWN_USER_HOOK } {
        Some(unsafe { hook(entry, stack, arg, stack_info) })
    } else {
        None
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
