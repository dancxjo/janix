use crate::errors::Errno;
use crate::stack::{Stack, StackSpec};
use abi::wait::{WaitKind, WaitSpec};

pub type ThreadId = u64;

pub fn spawn(entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    let stack = Stack::alloc_growing_stack(StackSpec::default())?;
    spawn_on(stack, entry)
}

pub fn spawn_on(stack: Stack, entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    crate::syscall::spawn_thread(entry as usize, 0, &stack).map(|id| id as ThreadId)
}

pub fn spawn_with_stack(stack: Stack, entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    spawn_on(stack, entry)
}

pub fn spawn_with_arg(entry: extern "C" fn(usize) -> !, arg: usize) -> Result<ThreadId, Errno> {
    let stack = Stack::alloc_growing_stack(StackSpec::default())?;
    crate::syscall::spawn_thread(entry as usize, arg, &stack).map(|id| id as ThreadId)
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

/// A first-class handle for observing a task's exit via `wait_many`.
///
/// Placing a `TaskExitWatch` in a `wait_many` call eliminates the need for
/// a dedicated polling loop around [`wait()`]. The result's `value` field
/// carries the exit code of the exited task.
///
/// # Example
///
/// ```ignore
/// let tid = stem::thread::spawn_with_arg(worker, arg)?;
/// let watcher = TaskExitWatch::new(tid);
///
/// let specs = [watcher.wait_spec(0)];
/// let mut results = [WaitResult::default()];
/// stem::syscall::wait_many(&specs, &mut results, None)?;
/// let exit_code = results[0].value as i32;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskExitWatch {
    tid: ThreadId,
}

impl TaskExitWatch {
    /// Create a watch for the given thread ID.
    pub fn new(tid: ThreadId) -> Self {
        Self { tid }
    }

    /// Returns the thread ID being watched.
    pub fn tid(&self) -> ThreadId {
        self.tid
    }

    /// Returns a [`WaitSpec`] that fires when the watched task exits.
    ///
    /// Pass the returned spec to `stem::syscall::wait_many` alongside any other
    /// specs (ports, graph watches, timeouts, etc.) to multiplex without blocking.
    /// When the spec fires, `WaitResult::value` contains the task's exit code and
    /// `WaitResult::flags` has [`abi::wait::ready::EXITED`](abi::wait::ready::EXITED) set.
    pub fn wait_spec(&self, token: u64) -> WaitSpec {
        WaitSpec {
            kind: WaitKind::TaskExit as u32,
            flags: 0,
            object: self.tid,
            token,
        }
    }
}
