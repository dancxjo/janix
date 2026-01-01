//! Watch Syscalls

use abi::wire::SyscallResult;
use abi::syscall::err;

pub fn sys_watch_create(_target_low: u64, _flags: u64) -> SyscallResult {
    // Stub
    SyscallResult::new(err::ENOSYS, 0, 0)
}

pub fn sys_watch_poll(_watch_id: u64, _out_ptr: u64, _len: u64) -> SyscallResult {
    SyscallResult::new(err::ENOSYS, 0, 0)
}
