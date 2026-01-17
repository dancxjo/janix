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

pub fn spawn_with_stack(stack: Stack, entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    spawn_on(stack, entry)
}

pub fn yield_now() {
    crate::syscall::yield_now();
}

/// Block until the specified task exits, returning its exit code.
pub fn wait(tid: ThreadId) -> Result<i32, Errno> {
    crate::syscall::task_wait(tid)
}

pub fn set_priority(tid: ThreadId, priority: usize) -> Result<(), Errno> {
    crate::syscall::set_priority(tid, priority)
}
