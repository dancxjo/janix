//! Process management (stub)
//!
//! Will provide process creation, termination, and management.
//! Currently returns ENOSYS for all operations.

use crate::syscall::err::ENOSYS;

/// Spawn a new process from an ELF
pub fn spawn(_elf_data: &[u8]) -> Result<u64, i32> {
    Err(ENOSYS)
}

/// Get the current process ID
pub fn current_pid() -> u64 {
    0
}

/// Exit the current process
pub fn exit(_code: i32) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
