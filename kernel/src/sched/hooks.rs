//! Static hook system for type-erased scheduler access.

use super::types::StackFaultResult;
use crate::sched::spawn::{SpawnExResult, StdioSpec};
use crate::task::{ProcessInfo, TaskId, TaskState};
use abi::errors::Errno;
use abi::vm::{VmProt, VmRegionInfo};
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

// ── ProcessSnapshot ───────────────────────────────────────────────────────────

/// A type-erased snapshot of per-process state, used by procfs to render
/// `/proc/<pid>/status` and similar files without needing the `R` type parameter.
#[derive(Clone)]
pub struct ProcessSnapshot {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub state: TaskState,
    pub argv: Vec<Vec<u8>>,
}

pub(crate) static mut YIELD_HOOK: Option<fn() -> bool> = None;
pub(crate) static mut EXIT_HOOK: Option<fn(i32)> = None;
pub(crate) static mut SPAWN_USER_HOOK: Option<
    unsafe fn(
        usize,
        usize,
        crate::task::StartupArg,
        abi::types::StackInfo,
        crate::task::TaskPriority,
        u64,
        bool,
    ) -> TaskId,
> = None;
pub(crate) static mut SPAWN_PROCESS_HOOK: Option<
    unsafe fn(&str, crate::task::StartupArg) -> Option<TaskId>,
> = None;
pub(crate) static mut CURRENT_TID_HOOK: Option<fn() -> u64> = None;
pub(crate) static mut INTERRUPT_TASK_HOOK: Option<fn(TaskId) -> Result<(), Errno>> = None;
pub(crate) static mut TAKE_PENDING_INTERRUPT_HOOK: Option<fn() -> bool> = None;
pub(crate) static mut TASK_STATUS_HOOK: Option<fn(TaskId) -> Option<(TaskState, Option<i32>)>> =
    None;
pub(crate) static mut TASK_WAIT_HOOK: Option<fn(TaskId) -> Result<i32, Errno>> = None;
pub(crate) static mut SET_PRIORITY_HOOK: Option<fn(TaskId, crate::task::TaskPriority)> = None;
pub(crate) static mut CURRENT_PRIORITY_HOOK: Option<fn() -> crate::task::TaskPriority> = None;
pub(crate) static mut ALLOC_USER_STACK_HOOK: Option<fn(usize) -> Option<usize>> = None;
pub(crate) static mut STACK_FAULT_HOOK: Option<unsafe fn(u64) -> StackFaultResult> = None;
pub(crate) static mut SLEEP_TICKS_HOOK: Option<fn(u64)> = None;
pub(crate) static mut ADD_USER_MAPPING_HOOK: Option<fn(VmRegionInfo) -> Result<(), Errno>> = None;
pub(crate) static mut REMOVE_USER_MAPPINGS_HOOK: Option<
    fn(usize, usize) -> Result<Vec<(usize, usize)>, Errno>,
> = None;
pub(crate) static mut CHECK_USER_MAPPING_HOOK: Option<fn(usize, usize, bool) -> bool> = None;
pub(crate) static mut GET_USER_MAPPING_AT_HOOK: Option<fn(usize) -> Option<VmRegionInfo>> = None;
pub(crate) static mut PROTECT_USER_RANGE_HOOK: Option<
    unsafe fn(u64, usize, VmProt) -> Result<(), Errno>,
> = None;
pub(crate) static mut RUN_SCHEDULER_HOOK: Option<fn() -> !> = None;
pub(crate) static mut KILL_BY_TID_HOOK: Option<fn(u64) -> bool> = None;
pub(crate) static mut DUMP_STATS_HOOK: Option<fn()> = None;
pub(crate) static mut PROCESS_INFO_HOOK: Option<fn() -> Option<Arc<Mutex<ProcessInfo>>>> = None;
pub(crate) static mut PROCESS_INFO_FOR_TID_HOOK: Option<
    fn(u64) -> Option<Arc<Mutex<ProcessInfo>>>,
> = None;
pub(crate) static mut GRAPH_THING_FOR_CURRENT_HOOK: Option<fn() -> Option<u64>> = None;
pub(crate) static mut POLL_TASK_EXIT_HOOK: Option<fn(TaskId) -> Result<Option<i32>, Errno>> = None;
pub(crate) static mut REGISTER_TASK_EXIT_WAITER_HOOK: Option<
    fn(TaskId, TaskId) -> Result<Option<i32>, Errno>,
> = None;
pub(crate) static mut UNREGISTER_TASK_EXIT_WAITER_HOOK: Option<
    fn(TaskId, TaskId) -> Result<(), Errno>,
> = None;
pub(crate) static mut REGISTER_TIMEOUT_WAKE_HOOK: Option<fn(TaskId, u64)> = None;
pub(crate) static mut UNREGISTER_TIMEOUT_WAKE_HOOK: Option<fn(TaskId)> = None;
/// Return a snapshot of all live processes (those that have process_info set).
pub(crate) static mut LIST_PROCESSES_HOOK: Option<fn() -> Vec<ProcessSnapshot>> = None;
pub(crate) static mut CURRENT_TASK_NAME_HOOK: Option<fn() -> [u8; 32]> = None;
pub(crate) static mut TASK_EXEC_HOOK: Option<
    fn(u32, Vec<Vec<u8>>, BTreeMap<Vec<u8>, Vec<u8>>) -> Result<(), Errno>,
> = None;
/// Updates the current task's stored `user_fs_base` field without touching hardware.
pub(crate) static mut SET_CURRENT_USER_FS_BASE_HOOK: Option<fn(u64)> = None;

/// Wait for a child process to exit, returning (child_pid, exit_code).
pub(crate) static mut WAITPID_HOOK: Option<fn(i64, u32) -> Result<(u64, i32), Errno>> = None;

pub unsafe fn yield_now_current() {
    if let Some(hook) = unsafe { YIELD_HOOK } {
        let _ = hook();
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

pub fn interrupt_task_current(id: TaskId) -> Result<(), Errno> {
    if let Some(hook) = unsafe { INTERRUPT_TASK_HOOK } {
        hook(id)
    } else {
        Err(Errno::ENOSYS)
    }
}

pub fn take_pending_interrupt_current() -> bool {
    if let Some(hook) = unsafe { TAKE_PENDING_INTERRUPT_HOOK } {
        hook()
    } else {
        false
    }
}

pub unsafe fn task_status_current(id: TaskId) -> Option<(TaskState, Option<i32>)> {
    if let Some(hook) = unsafe { TASK_STATUS_HOOK } {
        hook(id)
    } else {
        None
    }
}

pub unsafe fn task_wait_current(id: TaskId) -> Result<i32, Errno> {
    if let Some(hook) = unsafe { TASK_WAIT_HOOK } {
        hook(id)
    } else {
        Err(Errno::ENOSYS)
    }
}

pub unsafe fn poll_task_exit_current(id: TaskId) -> Result<Option<i32>, Errno> {
    if let Some(hook) = unsafe { POLL_TASK_EXIT_HOOK } {
        hook(id)
    } else {
        Err(Errno::ENOSYS)
    }
}

pub unsafe fn register_task_exit_waiter_current(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<Option<i32>, Errno> {
    if let Some(hook) = unsafe { REGISTER_TASK_EXIT_WAITER_HOOK } {
        hook(target_tid, waiter_tid)
    } else {
        Err(Errno::ENOSYS)
    }
}

pub unsafe fn unregister_task_exit_waiter_current(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<(), Errno> {
    if let Some(hook) = unsafe { UNREGISTER_TASK_EXIT_WAITER_HOOK } {
        hook(target_tid, waiter_tid)
    } else {
        Err(Errno::ENOSYS)
    }
}

pub fn register_timeout_wake_current(tid: TaskId, wake_tick: u64) {
    if let Some(hook) = unsafe { REGISTER_TIMEOUT_WAKE_HOOK } {
        hook(tid, wake_tick);
    }
}

pub fn unregister_timeout_wake_current(tid: TaskId) {
    if let Some(hook) = unsafe { UNREGISTER_TIMEOUT_WAKE_HOOK } {
        hook(tid);
    }
}

pub unsafe fn kill_by_tid_current(tid: u64) -> bool {
    if let Some(hook) = unsafe { KILL_BY_TID_HOOK } {
        hook(tid)
    } else {
        false
    }
}

pub fn dump_stats_current() {
    if let Some(hook) = unsafe { DUMP_STATS_HOOK } {
        hook();
    }
}

pub unsafe fn get_user_mapping_at_current(addr: usize) -> Option<VmRegionInfo> {
    if let Some(hook) = unsafe { GET_USER_MAPPING_AT_HOOK } {
        hook(addr)
    } else {
        None
    }
}

pub fn protect_user_range_current(addr: u64, len: usize, prot: VmProt) -> Result<(), Errno> {
    if let Some(hook) = unsafe { PROTECT_USER_RANGE_HOOK } {
        unsafe { hook(addr, len, prot) }
    } else {
        Err(Errno::ENOSYS)
    }
}

pub unsafe fn spawn_process_current(name: &str, arg: crate::task::StartupArg) -> Option<TaskId> {
    if let Some(hook) = unsafe { SPAWN_PROCESS_HOOK } {
        unsafe { hook(name, arg) }
    } else {
        None
    }
}

pub unsafe fn spawn_user_thread_current(
    entry: usize,
    stack: usize,
    arg: crate::task::StartupArg,
    stack_info: abi::types::StackInfo,
    priority: crate::task::TaskPriority,
    tls_base: u64,
    detached: bool,
) -> Option<TaskId> {
    if let Some(hook) = unsafe { SPAWN_USER_HOOK } {
        Some(unsafe { hook(entry, stack, arg, stack_info, priority, tls_base, detached) })
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

pub unsafe fn remove_user_mappings_current(
    addr: usize,
    len: usize,
) -> Result<Vec<(usize, usize)>, Errno> {
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

pub fn process_info_current() -> Option<Arc<Mutex<ProcessInfo>>> {
    if let Some(hook) = unsafe { PROCESS_INFO_HOOK } {
        hook()
    } else {
        None
    }
}

pub fn process_info_for_tid_current(tid: u64) -> Option<Arc<Mutex<ProcessInfo>>> {
    if let Some(hook) = unsafe { PROCESS_INFO_FOR_TID_HOOK } {
        hook(tid)
    } else {
        None
    }
}

pub(crate) static mut SPAWN_PROCESS_EX_HOOK: Option<
    unsafe fn(
        &str,
        Vec<Vec<u8>>,
        BTreeMap<Vec<u8>, Vec<u8>>,
        StdioSpec,
        StdioSpec,
        StdioSpec,
        u64,
        Vec<u64>,
        Option<alloc::string::String>,
    ) -> Result<SpawnExResult, abi::errors::Errno>,
> = None;

pub unsafe fn spawn_process_ex_current(
    name: &str,
    argv: Vec<Vec<u8>>,
    env: BTreeMap<Vec<u8>, Vec<u8>>,
    stdin_spec: StdioSpec,
    stdout_spec: StdioSpec,
    stderr_spec: StdioSpec,
    boot_arg: u64,
    inherited_handles: Vec<u64>,
    cwd: Option<alloc::string::String>,
) -> Result<SpawnExResult, abi::errors::Errno> {
    if let Some(hook) = SPAWN_PROCESS_EX_HOOK {
        hook(
            name,
            argv,
            env,
            stdin_spec,
            stdout_spec,
            stderr_spec,
            boot_arg,
            inherited_handles,
            cwd,
        )
    } else {
        Err(abi::errors::Errno::ENOSYS)
    }
}

pub unsafe fn graph_thing_for_current() -> Option<u64> {
    if let Some(hook) = unsafe { GRAPH_THING_FOR_CURRENT_HOOK } {
        hook()
    } else {
        None
    }
}

/// Return a snapshot of all live processes.
pub fn list_processes_current() -> Vec<ProcessSnapshot> {
    if let Some(hook) = unsafe { LIST_PROCESSES_HOOK } {
        hook()
    } else {
        Vec::new()
    }
}

pub unsafe fn current_task_name_current() -> [u8; 32] {
    if let Some(hook) = unsafe { CURRENT_TASK_NAME_HOOK } {
        hook()
    } else {
        let mut n = [0u8; 32];
        n[0..7].copy_from_slice(b"unknown");
        n
    }
}

pub unsafe fn task_exec_current(
    fd: u32,
    argv: Vec<Vec<u8>>,
    env: BTreeMap<Vec<u8>, Vec<u8>>,
) -> Result<(), Errno> {
    if let Some(hook) = unsafe { TASK_EXEC_HOOK } {
        hook(fd, argv, env)
    } else {
        Err(Errno::ENOSYS)
    }
}

/// Update the current task's stored `user_fs_base` field (without touching hardware).
///
/// This should be called alongside a hardware write whenever the TLS base changes
/// via syscall, so that the value is preserved correctly on the next context switch.
pub unsafe fn set_current_user_fs_base_current(base: u64) {
    if let Some(hook) = unsafe { SET_CURRENT_USER_FS_BASE_HOOK } {
        hook(base)
    }
}

/// Wait for a child process to exit, returning `(child_pid, exit_code)`.
///
/// `pid > 0`: wait for the specific child with that PID.
/// `pid <= 0`: wait for any child.
/// `flags & WNOHANG`: return `Ok((0, 0))` immediately if no child has exited.
/// Returns `Err(ECHILD)` when no matching children exist at all.
pub unsafe fn waitpid_current(pid: i64, flags: u32) -> Result<(u64, i32), Errno> {
    if let Some(hook) = unsafe { WAITPID_HOOK } {
        hook(pid, flags)
    } else {
        Err(Errno::ENOSYS)
    }
}
