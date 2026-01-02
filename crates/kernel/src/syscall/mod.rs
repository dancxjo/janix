//! Syscall dispatch
//!
//! Provides the kernel's system call interface. Syscalls are dispatched
//! by number and return (status, val0, val1).

pub mod dispatch;
pub mod cap;
pub mod graph;
pub mod log;
pub mod memory;
pub mod watch;
pub mod input; // Added input module

pub use dispatch::dispatch;

/// Initialize syscall dispatch (if needed)
pub fn init() {
    // dispatch table is static
}

// Re-export specific symbols if needed by other crates or legacy code
pub use abi::wire::SyscallResult;
pub use abi::syscall::nr;
