use crate::errors::Errno;
use crate::stack::Stack;

pub type ThreadId = u64;

pub fn spawn(entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    let stack = Stack::alloc_default()?;
    spawn_on(stack, entry)
}

pub fn spawn_on(stack: Stack, entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    crate::syscall::spawn_thread(entry, stack.top()).map(|id| id as ThreadId)
}

pub fn spawn_with_stack(stack: Stack, entry: extern "C" fn() -> !) -> Result<ThreadId, Errno> {
    spawn_on(stack, entry)
}

pub fn yield_now() {
    crate::syscall::yield_now();
}
