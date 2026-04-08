//! Syscall handler implementations
//!
//! Organized into focused modules by function category.

mod device;
mod futex;
mod logging;
mod memory;
mod net;
pub mod pipe;
mod port;
mod process;
mod random;
mod stdio;
pub mod stream;
mod time;
mod trace;
pub mod vfs;
mod wait;

// Re-export all syscall handlers
pub use device::*;
pub use futex::*;
pub use logging::*;
pub use memory::*;
pub use net::*;
pub use pipe::*;
pub use port::*;
pub use process::*;
pub use random::*;
pub use stdio::*;
pub use time::*;
pub use trace::*;
pub use wait::*;

// Shared utilities used by multiple handlers
use crate::syscall::validate::{copyin, copyout};
use abi::errors::{Errno, SysResult};
use alloc::string::String;
use core::sync::atomic::Ordering;

/// Blocking call to Root service (REMOVED)
pub(crate) fn root_call(_op: usize) -> SysResult<usize> {
    Err(Errno::ENOSYS)
}
