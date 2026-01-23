use crate::errors::Errno;
use crate::stack::{Stack, StackSpec};

pub type ThreadId = u64;

pub fn spawn(entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    let stack = Stack::alloc_growing_stack(StackSpec::default())?;
    spawn_on(stack, entry)
}

pub fn spawn_on(stack: Stack, entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    crate::syscall::spawn_thread(entry, &stack).map(|id| id as ThreadId)
}

pub fn spawn_with_arg(
    entry: extern "C" fn(usize) -> !,
    arg: usize,
) -> Result<ThreadId, Errno> {
    let stack = Stack::alloc_growing_stack(StackSpec::default())?;
    spawn_on_with_arg(stack, entry, arg)
}

pub fn spawn_on_with_arg(
    stack: Stack,
    entry: extern "C" fn(usize) -> !,
    arg: usize,
) -> Result<ThreadId, Errno> {
    crate::syscall::spawn_thread_with_arg(entry, arg, &stack).map(|id| id as ThreadId)
}

pub fn spawn_with_stack(stack: Stack, entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    spawn_on(stack, entry)
}

pub fn yield_now() {
    crate::syscall::yield_now();
}

pub fn current_id() -> Result<ThreadId, Errno> {
    crate::syscall::get_tid().map(|id| id as ThreadId)
}

/// Block until the specified task exits, returning its exit code.
pub fn wait(tid: ThreadId) -> Result<i32, Errno> {
    crate::syscall::task_wait(tid)
}

pub fn set_priority(tid: ThreadId, priority: usize) -> Result<(), Errno> {
    crate::syscall::set_priority(tid, priority)
}
