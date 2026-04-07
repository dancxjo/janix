//! VFS syscall handlers: open, close, read, write.
//!
//! These handlers implement the first working cut of the janix VFS
//! ("Hello from VFS" milestone):
//!
//! - [`sys_vfs_open`]  — open a path and return a file descriptor
//! - [`sys_vfs_close`] — release a file descriptor
//! - [`sys_vfs_read`]  — read from a file descriptor into a user buffer
//! - [`sys_vfs_write`] — write from a user buffer to a file descriptor

use alloc::vec;

use abi::errors::{Errno, SysResult};

use crate::syscall::validate::{copyin, copyout, validate_user_range};
use crate::vfs::{self, OpenFlags};

// ── open ────────────────────────────────────────────────────────────────────

pub fn sys_vfs_open(path_ptr: usize, path_len: usize, flags: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }

    // Copy path from userspace.
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let open_flags = OpenFlags(flags as u32);

    // Resolve path through the mount table.
    let node = vfs::mount::lookup(path)?;

    // Insert into the per-process fd table.
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let fd = pinfo_arc.lock().fd_table.open(node, open_flags)?;

    Ok(fd as usize)
}

// ── close ───────────────────────────────────────────────────────────────────

pub fn sys_vfs_close(fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    pinfo_arc.lock().fd_table.close(fd as u32)?;
    Ok(0)
}

// ── read ────────────────────────────────────────────────────────────────────

pub fn sys_vfs_read(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    validate_user_range(buf_ptr, buf_len, true)?;
    if buf_len == 0 {
        return Ok(0);
    }

    // Clone the node Arc so we don't hold the process lock during the read.
    let (node, offset, flags) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        if !file.flags.is_readable() {
            return Err(Errno::EBADF);
        }
        (file.node.clone(), file.offset, file.flags)
    };

    let mut kbuf = vec![0u8; buf_len];
    let n = node.read(offset, &mut kbuf)?;

    // Advance offset (non-blocking/device nodes ignore this, but it is
    // correct for regular files).
    if n > 0 {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let mut lock = pinfo_arc.lock();
        if let Ok(file) = lock.fd_table.get_mut(fd as u32) {
            if !flags.is_append() {
                file.offset = file.offset.saturating_add(n as u64);
            }
        }
    }

    unsafe { copyout(buf_ptr, &kbuf[..n])? };
    Ok(n)
}

// ── write ───────────────────────────────────────────────────────────────────

pub fn sys_vfs_write(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    validate_user_range(buf_ptr, buf_len, false)?;
    if buf_len == 0 {
        return Ok(0);
    }

    let mut kbuf = vec![0u8; buf_len];
    unsafe { copyin(&mut kbuf, buf_ptr)? };

    let (node, offset, flags) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        if !file.flags.is_writable() {
            return Err(Errno::EBADF);
        }
        (file.node.clone(), file.offset, file.flags)
    };

    let n = node.write(offset, &kbuf)?;

    if n > 0 {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let mut lock = pinfo_arc.lock();
        if let Ok(file) = lock.fd_table.get_mut(fd as u32) {
            if !flags.is_append() {
                file.offset = file.offset.saturating_add(n as u64);
            }
        }
    }

    Ok(n)
}
