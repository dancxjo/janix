//! VFS syscall handlers: open, close, read, write, dup, dup2, pipe.
//!
//! These handlers implement the janix VFS syscall interface:
//!
//! - [`sys_vfs_open`]  — open a path and return a file descriptor
//! - [`sys_vfs_close`] — release a file descriptor (all fds, including 0-2)
//! - [`sys_vfs_read`]  — read from a file descriptor into a user buffer
//! - [`sys_vfs_write`] — write from a user buffer to a file descriptor
//! - [`sys_dup`]       — duplicate a file descriptor to the lowest free slot
//! - [`sys_dup2`]      — duplicate a file descriptor to a specific slot
//! - [`sys_pipe`]      — create an anonymous pipe, allocating two fds

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

    // Clone the node Arc and the shared offset so we don't hold the process lock
    // during the read.
    let (node, offset_cell) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        if !file.flags.is_readable() {
            return Err(Errno::EBADF);
        }
        (file.node.clone(), file.offset.clone())
    };

    let offset = *offset_cell.lock();
    let mut kbuf = vec![0u8; buf_len];
    let n = node.read(offset, &mut kbuf)?;

    if n > 0 {
        *offset_cell.lock() = offset.saturating_add(n as u64);
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

    let (node, offset_cell) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        if !file.flags.is_writable() {
            return Err(Errno::EBADF);
        }
        (file.node.clone(), file.offset.clone())
    };

    let offset = *offset_cell.lock();
    let n = node.write(offset, &kbuf)?;

    // O_APPEND semantics (positioning at EOF before write) are handled by the
    // node implementation; the offset is still updated here to track position.
    if n > 0 {
        *offset_cell.lock() = offset.saturating_add(n as u64);
    }

    Ok(n)
}

// ── dup ─────────────────────────────────────────────────────────────────────

/// Duplicate `old_fd` to the lowest available file descriptor.
///
/// Returns the new file descriptor, or an error if `old_fd` is not open.
pub fn sys_dup(old_fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let new_fd = pinfo_arc.lock().fd_table.dup(old_fd as u32)?;
    Ok(new_fd as usize)
}

// ── dup2 ────────────────────────────────────────────────────────────────────

/// Duplicate `old_fd` to `new_fd`.
///
/// If `new_fd` is already open it is closed first.  If `old_fd == new_fd`
/// this is a no-op.  Returns `new_fd` on success.
pub fn sys_dup2(old_fd: usize, new_fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let result = pinfo_arc.lock().fd_table.dup2(old_fd as u32, new_fd as u32)?;
    Ok(result as usize)
}

// ── pipe ────────────────────────────────────────────────────────────────────

/// Create an anonymous pipe and allocate two file descriptors.
///
/// Writes the read-end fd and write-end fd into the user buffer pointed to by
/// `pipefd_ptr` (which must point to a `[u32; 2]`).  Returns 0 on success.
pub fn sys_pipe(pipefd_ptr: usize) -> SysResult<usize> {
    // Validate: we need to write 8 bytes (two u32s).
    validate_user_range(pipefd_ptr, 8, true)?;

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;

    let (read_node, write_node) = crate::ipc::pipe::create_fd_pair(4096, false);
    let (read_fd, write_fd) = {
        let mut lock = pinfo_arc.lock();
        let read_fd = lock
            .fd_table
            .open(read_node, crate::vfs::OpenFlags::read_only())?;
        match lock.fd_table.open(write_node, crate::vfs::OpenFlags::write_only()) {
            Ok(wfd) => (read_fd, wfd),
            Err(e) => {
                let _ = lock.fd_table.close(read_fd);
                return Err(e);
            }
        }
    };

    // Write [read_fd, write_fd] to userspace as two consecutive u32 values (8 bytes).
    let read_fd_bytes = read_fd.to_ne_bytes();
    let write_fd_bytes = write_fd.to_ne_bytes();
    let mut fds_bytes = [0u8; 8];
    fds_bytes[..4].copy_from_slice(&read_fd_bytes);
    fds_bytes[4..].copy_from_slice(&write_fd_bytes);
    unsafe { copyout(pipefd_ptr, &fds_bytes)? };

    Ok(0)
}

