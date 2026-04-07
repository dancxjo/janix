//! Userspace VFS syscall wrappers.
//!
//! These are thin wrappers around the raw VFS syscalls introduced in the
//! janix de-graphing migration (Act III – Birth of the VFS).

use abi::errors::{Errno, SysResult};
use abi::syscall::{SYS_VFS_CLOSE, SYS_VFS_OPEN, SYS_VFS_READ, SYS_VFS_WRITE};

use super::arch::raw_syscall6;

/// Open a file at the given absolute path.
///
/// `flags` follows the same encoding as [`abi::syscall::vfs_flags`].
/// Returns a file descriptor on success, or an [`Errno`] on failure.
pub fn vfs_open(path: &str, flags: u32) -> SysResult<u32> {
    let ret = unsafe {
        raw_syscall6(
            SYS_VFS_OPEN,
            path.as_ptr() as usize,
            path.len(),
            flags as usize,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as u32)
}

/// Close a VFS file descriptor previously returned by [`vfs_open`].
pub fn vfs_close(fd: u32) -> SysResult<()> {
    let ret = unsafe { raw_syscall6(SYS_VFS_CLOSE, fd as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

/// Read up to `buf.len()` bytes from `fd` into `buf`.
///
/// Returns the number of bytes actually read (may be 0 at EOF).
pub fn vfs_read(fd: u32, buf: &mut [u8]) -> SysResult<usize> {
    let ret = unsafe {
        raw_syscall6(
            SYS_VFS_READ,
            fd as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Write `buf` to `fd`.
///
/// Returns the number of bytes written.
pub fn vfs_write(fd: u32, buf: &[u8]) -> SysResult<usize> {
    let ret = unsafe {
        raw_syscall6(
            SYS_VFS_WRITE,
            fd as usize,
            buf.as_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}
