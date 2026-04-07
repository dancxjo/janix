//! Userspace VFS syscall wrappers.
//!
//! These are thin wrappers around the raw VFS syscalls introduced in the
//! janix de-graphing migration (Act III – Birth of the VFS, Act IV – Kernel
//! Filesystems).

use abi::errors::{Errno, SysResult};
use abi::syscall::{SYS_VFS_CLOSE, SYS_VFS_MKDIR, SYS_VFS_MOUNT, SYS_VFS_OPEN, SYS_VFS_READ, SYS_VFS_UMOUNT, SYS_VFS_UNLINK, SYS_VFS_WRITE};

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

/// Remove a file or empty directory at `path`.
///
/// Returns `Ok(())` on success, or an [`Errno`] on failure.
pub fn vfs_unlink(path: &str) -> SysResult<()> {
    let ret = unsafe {
        raw_syscall6(
            SYS_VFS_UNLINK,
            path.as_ptr() as usize,
            path.len(),
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

/// Create a directory at `path`.
///
/// Returns `Ok(())` on success, or an [`Errno`] on failure.
pub fn vfs_mkdir(path: &str) -> SysResult<()> {
    let ret = unsafe {
        raw_syscall6(
            SYS_VFS_MKDIR,
            path.as_ptr() as usize,
            path.len(),
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

/// Mount a userland VFS provider at `path`.
///
/// `provider_write_handle` is the write end of a port pair that the provider
/// owns.  The kernel will send [`abi::vfs_rpc`] messages to that port whenever
/// a VFS operation touches a path under `path`.
///
/// Returns `Ok(())` on success, or an [`Errno`] on failure.
pub fn vfs_mount(provider_write_handle: u32, path: &str) -> SysResult<()> {
    let ret = unsafe {
        raw_syscall6(
            SYS_VFS_MOUNT,
            provider_write_handle as usize,
            path.as_ptr() as usize,
            path.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

/// Unmount the userland VFS provider previously mounted at `path`.
///
/// Returns `Ok(())` on success, or an [`Errno`] on failure.
pub fn vfs_umount(path: &str) -> SysResult<()> {
    let ret = unsafe {
        raw_syscall6(
            SYS_VFS_UMOUNT,
            path.as_ptr() as usize,
            path.len(),
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}
