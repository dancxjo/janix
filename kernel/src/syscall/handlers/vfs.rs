//! VFS syscall handlers: open, close, read, write, stat, readdir,
//!                       unlink, mkdir, dup, dup2, pipe, poll.
//!
//! These handlers implement the janix VFS syscall interface:
//!
//! - [`sys_fs_open`]   — open (or create) a path, return a file descriptor
//! - [`sys_fs_close`]  — release a file descriptor (all fds, including 0-2)
//! - [`sys_fs_read`]   — read from a file descriptor into a user buffer
//! - [`sys_fs_write`]  — write from a user buffer to a file descriptor
//! - [`sys_fs_unlink`] — remove a file or empty directory
//! - [`sys_fs_mkdir`]  — create a directory
//! - [`SYS_FS_DUP`]        — duplicate a file descriptor to the lowest free slot
//! - [`SYS_FS_DUP2`]       — duplicate a file descriptor to a specific slot
//! - [`sys_pipe`]       — create an anonymous pipe, allocating two fds
//! - [`sys_fs_poll`]   — poll a set of fds for readiness (POSIX-style)

use alloc::sync::Arc;
use alloc::vec;

use abi::errors::{Errno, SysResult};
use abi::syscall::{PollFd, fcntl_cmd, fd_flags, poll_flags, vfs_flags};

use crate::syscall::validate::{copyin, copyout, validate_user_range};
use crate::vfs::{self, OpenFlags};

// ── open ────────────────────────────────────────────────────────────────────

pub fn sys_fs_open(path_ptr: usize, path_len: usize, flags: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }

    // Copy path from userspace.
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let tid = unsafe { crate::sched::current_tid_current() };
    crate::ktrace!("VFS: sys_fs_open path='{}' tid={}", path, tid);

    if path == "/dev/fb0" {
        crate::kdebug!(
            "sys_fs_open: path='{}' len={} flags=0x{:x}",
            path,
            path_len,
            flags
        );
    }

    let open_flags = OpenFlags::from_open_call(flags as u32);
    let want_creat = (flags as u32) & vfs_flags::O_CREAT != 0;
    let want_trunc = (flags as u32) & vfs_flags::O_TRUNC != 0;

    let abs_path = resolve_path(path)?;

    // Resolve path through the mount table, creating the file if O_CREAT is set.
    let node = if want_creat {
        // Try lookup first; fall back to create if the file doesn't exist.
        match vfs::mount::lookup(&abs_path) {
            Ok(existing) => {
                if want_trunc {
                    // Truncate the file to zero length.
                    let _ = existing.truncate(0);
                }
                existing
            }
            Err(Errno::ENOENT) => vfs::mount::create(&abs_path)?,
            Err(e) => return Err(e),
        }
    } else {
        vfs::mount::lookup(&abs_path)?
    };

    if path == "/dev/fb0" {
        match node.stat() {
            Ok(stat) => crate::kdebug!(
                "sys_fs_open: resolved node for /dev/fb0 mode=0o{:o} size={} ino={}",
                stat.mode,
                stat.size,
                stat.ino
            ),
            Err(err) => crate::kwarn!("sys_fs_open: resolved /dev/fb0 but stat failed: {:?}", err),
        }
    }

    // Insert into the per-process fd table.
    let pinfo_arc = match crate::sched::process_info_current() {
        Some(pinfo_arc) => pinfo_arc,
        None => {
            if path == "/dev/fb0" {
                let tid = unsafe { crate::sched::current_tid_current() };
                let direct = crate::sched::process_info_for_tid_current(tid).is_some();
                crate::kwarn!(
                    "sys_fs_open: no process info for /dev/fb0 current_tid={} direct_lookup={}",
                    tid,
                    direct
                );
            }
            return Err(Errno::ENOENT);
        }
    };
    if path == "/dev/fb0" {
        crate::kdebug!("sys_fs_open: process info present for /dev/fb0");
    }
    let fd = pinfo_arc.lock().fd_table.open(node, open_flags, abs_path)?;

    if path == "/dev/fb0" {
        crate::kdebug!("sys_fs_open: fd_table.open('/dev/fb0') -> {}", fd);
    }

    Ok(fd as usize)
}

// ── close ───────────────────────────────────────────────────────────────────

pub fn sys_fs_close(fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    pinfo_arc.lock().fd_table.close(fd as u32)?;
    Ok(0)
}

// ── sync (fsync) ─────────────────────────────────────────────────────────────

/// Flush the VFS node associated with `fd` to its backing store.
///
/// For RAM-backed filesystems this is a no-op that always succeeds.
/// Returns `Ok(0)` on success, or an errno on failure.
pub fn sys_fs_sync(fd: usize) -> SysResult<usize> {
    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.fd_table.get(fd as u32)?.node.clone()
    };
    node.sync()?;
    Ok(0)
}

/// File-descriptor control.
pub fn sys_fs_fcntl(fd: usize, cmd: usize, arg: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let mut lock = pinfo_arc.lock();

    match cmd as u32 {
        fcntl_cmd::F_GETFD => Ok(lock.fd_table.get_fd_flags(fd as u32)? as usize),
        fcntl_cmd::F_SETFD => {
            lock.fd_table
                .set_fd_flags(fd as u32, (arg as u32) & fd_flags::FD_CLOEXEC)?;
            Ok(0)
        }
        fcntl_cmd::F_GETFL => {
            let file = lock.fd_table.get(fd as u32)?;
            Ok(file.status_flags.lock().0 as usize)
        }
        fcntl_cmd::F_SETFL => {
            let file = lock.fd_table.get(fd as u32)?;
            let mut status_flags = file.status_flags.lock();
            *status_flags = status_flags.with_mutable_status(arg as u32);
            Ok(0)
        }
        _ => Err(Errno::EINVAL),
    }
}

// ── read ────────────────────────────────────────────────────────────────────

pub fn sys_fs_read(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    validate_user_range(buf_ptr, buf_len, true)?;
    if buf_len == 0 {
        return Ok(0);
    }

    // Clone the node Arc and the shared offset so we don't hold the process lock
    // during the read.
    let (node, offset_cell, status_flags) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        let status_flags = *file.status_flags.lock();
        if !status_flags.is_readable() {
            return Err(Errno::EBADF);
        }
        (file.node.clone(), file.offset.clone(), status_flags)
    };

    if status_flags.read_would_block(node.poll()) {
        return Err(Errno::EAGAIN);
    }

    let offset = *offset_cell.lock();
    let mut kbuf = vec![0u8; buf_len];
    let n = node.read(offset, &mut kbuf)?;

    if n > 0 {
        *offset_cell.lock() = offset.saturating_add(n as u64);
    }

    unsafe { copyout(buf_ptr, &kbuf[..n])? };
    Ok(n)
}

pub fn sys_fs_stat(fd: usize, stat_ptr: usize, _a2: usize, _a3: usize) -> SysResult<usize> {
    let stat_size = core::mem::size_of::<abi::fs::FileStat>();
    validate_user_range(stat_ptr, stat_size, true)?;

    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        file.node.clone()
    };

    let stat = node.stat()?;
    let file_stat = stat.to_abi_stat();
    // SAFETY: `file_stat` is a plain repr(C) struct on the stack; we read it as bytes.
    let bytes = unsafe {
        core::slice::from_raw_parts(
            &file_stat as *const abi::fs::FileStat as *const u8,
            stat_size,
        )
    };
    unsafe { copyout(stat_ptr, bytes)? };

    Ok(0)
}

pub fn sys_fs_isatty(fd: usize) -> SysResult<usize> {
    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        file.node.clone()
    };
    Ok(if node.is_tty() { 1 } else { 0 })
}

pub fn sys_fs_readdir(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    validate_user_range(buf_ptr, buf_len, true)?;
    if buf_len == 0 {
        return Ok(0);
    }

    let (node, offset_cell, path) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        (file.node.clone(), file.offset.clone(), file.path.clone())
    };

    let mut kbuf = vec![0u8; buf_len];
    let offset: u64 = *offset_cell.lock();

    // 1. First, call the node's own readdir.
    // This fills the buffer using the filesystem's own entries.
    let mut n = node.readdir(offset, &mut kbuf)?;

    // 2. Then, supplement with mount points if there is space and we've reached
    // the "end" of the node's natural entries (heuristic: n < buf_len).
    if n < buf_len {
        let mounts = crate::vfs::mount::get_mounts_under(&path);
        if !mounts.is_empty() {
            // Write more entries if we have space.
            // For now, we only supplement if n == 0 to avoid complex deduplication
            // and offset management. This is sufficient for /dev/display and /sys.
            if n == 0 {
                let m_n = crate::vfs::write_readdir_entries(
                    mounts.iter().map(|s| s.as_str()),
                    offset,
                    &mut kbuf,
                )?;
                n = m_n;
            }
        }
    }

    if n > 0 {
        *offset_cell.lock() = offset.saturating_add(n as u64);
        unsafe { copyout(buf_ptr, &kbuf[..n])? };
    }

    Ok(n)
}

// ── write ───────────────────────────────────────────────────────────────────

pub fn sys_fs_write(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    validate_user_range(buf_ptr, buf_len, false)?;
    if buf_len == 0 {
        return Ok(0);
    }

    let mut kbuf = vec![0u8; buf_len];
    unsafe { copyin(&mut kbuf, buf_ptr)? };

    let (node, offset_cell, status_flags) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        let status_flags = *file.status_flags.lock();
        if !status_flags.is_writable() {
            return Err(Errno::EBADF);
        }
        (file.node.clone(), file.offset.clone(), status_flags)
    };

    if status_flags.write_would_block(node.poll()) {
        return Err(Errno::EAGAIN);
    }

    let write_offset = status_flags.effective_write_offset(*offset_cell.lock(), node.stat()?.size);
    let n = node.write(write_offset, &kbuf)?;

    if n > 0 {
        *offset_cell.lock() = write_offset.saturating_add(n as u64);
        // Emit MODIFY event
        crate::vfs::watch::emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0);
    }

    Ok(n)
}

pub fn sys_fs_unlink(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;

    // Resolve parent to emit event
    let (parent_path, name) = split_parent(&abs_path);
    let parent_node = vfs::mount::lookup(parent_path).ok();

    vfs::mount::unlink(&abs_path)?;

    if let Some(parent) = parent_node {
        crate::vfs::watch::emit_event(&*parent, abi::vfs_watch::mask::REMOVE, Some(name), 0);
    }

    Ok(0)
}

// ── mkdir ───────────────────────────────────────────────────────────────────

/// Create a directory at `path`.
pub fn sys_fs_mkdir(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;

    // Resolve parent to emit event
    let (parent_path, name) = split_parent(&abs_path);
    let parent_node = vfs::mount::lookup(parent_path).ok();

    vfs::mount::mkdir(&abs_path)?;

    if let Some(parent) = parent_node {
        crate::vfs::watch::emit_event(&*parent, abi::vfs_watch::mask::CREATE, Some(name), 0);
    }

    Ok(0)
}

// ── dup ─────────────────────────────────────────────────────────────────────

/// Duplicate `old_fd` to the lowest available file descriptor.
///
/// Returns the new file descriptor, or an error if `old_fd` is not open.
pub fn SYS_FS_DUP(old_fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let new_fd = pinfo_arc.lock().fd_table.dup(old_fd as u32)?;
    Ok(new_fd as usize)
}

// ── dup2 ────────────────────────────────────────────────────────────────────

/// Duplicate `old_fd` to `new_fd`.
///
/// If `new_fd` is already open it is closed first.  If `old_fd == new_fd`
/// this is a no-op.  Returns `new_fd` on success.
pub fn SYS_FS_DUP2(old_fd: usize, new_fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let result = pinfo_arc
        .lock()
        .fd_table
        .dup2(old_fd as u32, new_fd as u32)?;
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

    let (pipe_id, read_node, write_node) = crate::ipc::pipe::create_fd_pair_with_id(4096, false);
    let (read_fd, write_fd) = {
        let mut lock = pinfo_arc.lock();
        let read_fd = lock.fd_table.open(
            read_node,
            crate::vfs::OpenFlags::read_only(),
            alloc::format!("pipe:{}", pipe_id),
        )?;
        match lock.fd_table.open(
            write_node,
            crate::vfs::OpenFlags::write_only(),
            alloc::format!("pipe:{}", pipe_id),
        ) {
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

/// Explicitly bridge an IPC handle into the VFS world as a file descriptor.
///
/// This allows standard `poll()` to be used across both files and channels.
pub fn sys_fd_from_handle(handle_val: usize) -> SysResult<usize> {
    let handle = crate::ipc::Handle(handle_val as u32);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;

    // Resolve handle through current process's handle table
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table.get_any(handle).copied().ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;

    // Create a PortNode wrapper
    let node = Arc::new(crate::vfs::port_node::PortNode::new(port, entry.mode));

    // Insert into FD table
    let fd = {
        let mut lock = pinfo_arc.lock();
        lock.fd_table.open(
            node,
            match entry.mode {
                crate::ipc::HandleMode::Read => crate::vfs::OpenFlags::read_only(),
                crate::ipc::HandleMode::Write => crate::vfs::OpenFlags::write_only(),
            },
            alloc::format!("handle:{}", handle_val),
        )?
    };

    Ok(fd as usize)
}

// ── mount ───────────────────────────────────────────────────────────────────

/// Mount a userland VFS provider at the given path prefix.
///
/// `provider_write_handle` is the *write* end of a port pair owned by the
/// calling process.  The kernel will send VFS RPC messages (see
/// [`abi::vfs_rpc`]) to that port whenever a path under `path` is accessed.
///
/// The kernel creates a private response port and registers its write-handle
/// in the global handle table so the provider can call `SYS_channel_send` to
/// deliver replies.
pub fn sys_fs_mount(
    provider_write_handle: usize,
    path_ptr: usize,
    path_len: usize,
) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }

    // Copy path from userspace.
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    let abs_path = resolve_path(path)?;

    // Resolve the provider's write handle to a port Arc.
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;

    // Attempt 1: Check if it's a VFS FD pointing to a PortNode
    let req_port = {
        let lock = pinfo_arc.lock();
        if let Ok(file) = lock.fd_table.get(provider_write_handle as u32) {
            // Try to downcast or check if it's a PortNode
            // Since we don't have easy downcasting for traits in no_std without more machinery,
            // we'll use a hack or update the trait.
            // Actually, we can check the mode and if it's S_IFIFO (set in PortNode)
            // But the best way is to try to call a method.
            // For now, let's assume if it came from recv_handle and it's a PortNode, we can get it.
            // We'll add a helper to PortNode or use a well-known trick.

            // I'll add a method `as_port()` to VfsNode with default None.
            file.node.as_port()
        } else {
            None
        }
    };

    let req_port = if let Some(p) = req_port {
        p
    } else {
        // Attempt 2: IPC handle table
        let prov_handle = crate::ipc::Handle(provider_write_handle as u32);
        let prov_entry = {
            let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
            table
                .get(prov_handle, crate::ipc::HandleMode::Write)
                .copied()
                .ok_or(Errno::EBADF)?
        };
        crate::ipc::get_port(prov_entry.port_id).ok_or(Errno::EBADF)?
    };

    // Create the kernel response port.
    // Large capacity to hold multiple concurrent responses (though we serialise
    // requests, responses may vary in size).
    let resp_port_id = crate::ipc::create_port(abi::vfs_rpc::VFS_RPC_MAX_RESP * 4);
    let resp_port = crate::ipc::get_port(resp_port_id).ok_or(Errno::ENOMEM)?;

    // Register the write end of the response port in the global handle table
    // so the provider process can call SYS_channel_send on it.
    let resp_write_handle = {
        let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .alloc(resp_port_id, crate::ipc::HandleMode::Write)
            .ok_or(Errno::ENOMEM)?
    };

    let req_port_id = crate::ipc::find_port_id(&req_port).ok_or(Errno::EBADF)?;

    // Build and mount the provider filesystem.
    let provider_fs =
        vfs::provider::ProviderFs::new(req_port, resp_port, resp_write_handle.0, req_port_id.0);
    vfs::mount::mount(&abs_path, provider_fs);

    crate::kinfo!("vfs: mounted userland provider at {}", abs_path);
    Ok(0)
}

// ── umount ──────────────────────────────────────────────────────────────────

/// Unmount the VFS provider at the given path prefix.
pub fn sys_fs_umount(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    let abs_path = resolve_path(path)?;
    vfs::mount::umount(&abs_path)?;
    crate::kinfo!("vfs: unmounted userland provider at {}", abs_path);
    Ok(0)
}

// ── notify ──────────────────────────────────────────────────────────────────

/// Notify the kernel that a provider-backed node is ready.
///
/// `req_handle` is the handle to the request port of the provider.
pub fn sys_fs_notify(req_handle: usize, node_handle: usize, revents: usize) -> SysResult<usize> {
    let handle = crate::ipc::Handle(req_handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table.get_any(handle).copied().ok_or(Errno::EBADF)?
    };

    vfs::provider::notify_by_port(entry.port_id.0, node_handle as u64, revents as u16)?;
    Ok(0)
}

// ── poll ────────────────────────────────────────────────────────────────────

/// POSIX-style poll over VFS file descriptors.
///
/// Examines each entry in the `pollfds` array and sets `revents` on those
/// that are ready.  Uses a three-phase no-lost-wakeup algorithm:
///
/// 1. **Probe** — call `VfsNode::poll()` on every entry.  If anything is
///    ready or `timeout_ms == 0`, copy `revents` back and return immediately.
/// 2. **Register** — call `VfsNode::add_waiter(tid)` on every node, then
///    optionally arm a scheduler timeout.
/// 3. **Re-probe** — repeat the probe after registration.  If still nothing
///    is ready, call `block_current_erased()` to park the calling task until
///    a node wakes it or the timeout expires.
///
/// # Arguments
/// - `pollfds_ptr` — pointer to a `[PollFd; nfds]` in user memory (read/write)
/// - `nfds`         — number of entries in the array (max 256)
/// - `timeout_ms`   — `0` = non-blocking; `usize::MAX` = block indefinitely;
///                    any other value = maximum wait in milliseconds
///
/// # Returns
/// The number of entries with non-zero `revents`, or an errno on error.
/// Returns `Ok(0)` when the timeout expires with no events.
pub fn sys_fs_poll(pollfds_ptr: usize, nfds: usize, timeout_ms: usize) -> SysResult<usize> {
    const MAX_POLLFDS: usize = 256;
    if nfds == 0 {
        return Ok(0);
    }
    if nfds > MAX_POLLFDS {
        return Err(Errno::EINVAL);
    }

    let byte_len = nfds * core::mem::size_of::<PollFd>();
    validate_user_range(pollfds_ptr, byte_len, true)?;

    // Copy all PollFd entries from userspace.
    let mut kfds = vec![PollFd::default(); nfds];
    unsafe {
        copyin(
            core::slice::from_raw_parts_mut(kfds.as_mut_ptr() as *mut u8, byte_len),
            pollfds_ptr,
        )?
    };

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let tid = unsafe { crate::sched::current_tid_current() };

    // Resolve all entries once to avoid repeated FD table locking in the loop.
    #[derive(Clone)]
    struct Entry {
        node: Option<Arc<dyn vfs::VfsNode>>,
        events: u16,
    }
    let mut entries = vec![
        Entry {
            node: None,
            events: 0
        };
        nfds
    ];
    {
        let lock = pinfo_arc.lock();
        for (i, kfd) in kfds.iter().enumerate() {
            if kfd.fd >= 0 {
                entries[i].node = lock
                    .fd_table
                    .get(kfd.fd as u32)
                    .ok()
                    .map(|f| f.node.clone());
                entries[i].events = kfd.events;
            }
        }
    }

    let deadline = if timeout_ms == usize::MAX {
        None
    } else {
        let ticks = (timeout_ms as u64).saturating_add(9) / 10;
        Some(crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed) + ticks)
    };

    loop {
        if crate::sched::take_pending_interrupt_current() {
            unsafe {
                copyout(
                    pollfds_ptr,
                    core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                )?
            };
            return Err(Errno::EINTR);
        }

        // Pass 1: Probe current state
        let mut ready_count = 0;
        for (i, entry) in entries.iter().enumerate() {
            let revents = if let Some(ref node) = entry.node {
                node.poll() & (entry.events | poll_flags::POLLERR | poll_flags::POLLHUP)
            } else if kfds[i].fd >= 0 {
                poll_flags::POLLNVAL
            } else {
                0
            };

            kfds[i].revents = revents;
            if revents != 0 {
                ready_count += 1;
            }
        }

        // Immediate return if something is ready or if it's a non-blocking poll.
        if ready_count > 0 || timeout_ms == 0 {
            unsafe {
                copyout(
                    pollfds_ptr,
                    core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                )?
            };
            return Ok(ready_count);
        }

        // Check for timeout.
        if let Some(d) = deadline {
            if crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed) >= d {
                unsafe {
                    copyout(
                        pollfds_ptr,
                        core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                    )?
                };
                return Ok(0);
            }
        }

        // Pass 2: Register as waiter on all nodes.
        for entry in entries.iter() {
            if let Some(ref node) = entry.node {
                node.add_waiter(tid);
            }
        }

        if let Some(d) = deadline {
            crate::sched::register_timeout_wake_current(tid, d);
        }

        // Pass 3: Re-probe after registration to avoid the missed-wakeup race.
        let mut ready_count = 0;
        for (i, entry) in entries.iter().enumerate() {
            let revents = if let Some(ref node) = entry.node {
                node.poll() & (entry.events | poll_flags::POLLERR | poll_flags::POLLHUP)
            } else if kfds[i].fd >= 0 {
                poll_flags::POLLNVAL
            } else {
                0
            };

            kfds[i].revents = revents;
            if revents != 0 {
                ready_count += 1;
            }
        }

        if ready_count > 0
            || (deadline.is_some()
                && crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed)
                    >= deadline.unwrap())
        {
            for entry in entries.iter() {
                if let Some(ref node) = entry.node {
                    node.remove_waiter(tid);
                }
            }
            unsafe {
                copyout(
                    pollfds_ptr,
                    core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                )?
            };
            return Ok(ready_count);
        }

        // Wait for an event.
        unsafe { crate::sched::block_current_erased() };

        // Pass 4: Unregister waiters and repeat.
        for entry in entries.iter() {
            if let Some(ref node) = entry.node {
                node.remove_waiter(tid);
            }
        }

        if crate::sched::take_pending_interrupt_current() {
            unsafe {
                copyout(
                    pollfds_ptr,
                    core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                )?
            };
            return Err(Errno::EINTR);
        }
    }
}

// ── seek ────────────────────────────────────────────────────────────────────
pub fn sys_fs_seek(fd: usize, offset: usize, whence: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let mut lock = pinfo_arc.lock();
    let file = lock.fd_table.get_mut(fd as u32)?;

    let node = file.node.clone();
    let stat = node.stat()?;
    let size = stat.size;

    let mut current_offset = *file.offset.lock();
    let new_offset = match whence {
        0 => offset as u64,                                // SEEK_SET
        1 => current_offset.saturating_add(offset as u64), // SEEK_CUR
        2 => size.saturating_add(offset as u64),           // SEEK_END
        _ => return Err(Errno::EINVAL),
    };

    *file.offset.lock() = new_offset;
    Ok(new_offset as usize)
}

// ── watch ───────────────────────────────────────────────────────────────────

pub fn sys_watch_fd(fd: usize, mask: usize, flags: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;

    let node = {
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        file.node.clone()
    };

    let watch = Arc::new(crate::vfs::watch::Watch::new(mask as u32, flags as u32));
    crate::vfs::watch::register_watch(&node, watch.clone())?;

    // Return the watch as a new file descriptor
    let watch_fd = pinfo_arc.lock().fd_table.open(
        watch,
        crate::vfs::OpenFlags::read_only(),
        "watch:fd".into(),
    )?;
    Ok(watch_fd as usize)
}

pub fn sys_watch_path(
    path_ptr: usize,
    path_len: usize,
    mask: usize,
    flags: usize,
) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;
    let node = vfs::mount::lookup(&abs_path)?;

    let watch = Arc::new(crate::vfs::watch::Watch::new(mask as u32, flags as u32));
    crate::vfs::watch::register_watch(&node, watch.clone())?;

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let watch_fd = pinfo_arc.lock().fd_table.open(
        watch,
        crate::vfs::OpenFlags::read_only(),
        alloc::format!("watch:{}", abs_path),
    )?;
    Ok(watch_fd as usize)
}

pub fn sys_fs_device_call(fd: usize, call_ptr: usize) -> SysResult<usize> {
    let size = core::mem::size_of::<abi::device::DeviceCall>();
    validate_user_range(call_ptr, size, true)?;
    let mut call: abi::device::DeviceCall = unsafe { core::mem::zeroed() };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut call as *mut _ as *mut u8, size) };
    unsafe {
        copyin(slice, call_ptr)?;
    }

    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        file.node.clone()
    };

    node.device_call(&call)
}

// ── rename ──────────────────────────────────────────────────────────────────

static NEXT_COOKIE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(1);

pub fn sys_fs_rename(
    _old_dirfd: usize,
    old_const_ptr: usize,
    old_len: usize,
    _new_dirfd: usize,
    new_const_ptr: usize,
    new_len: usize,
) -> SysResult<usize> {
    validate_user_range(old_const_ptr, old_len, false)?;
    validate_user_range(new_const_ptr, new_len, false)?;

    let mut old_path_buf = vec![0u8; old_len];
    let mut new_path_buf = vec![0u8; new_len];
    unsafe {
        copyin(&mut old_path_buf, old_const_ptr)?;
        copyin(&mut new_path_buf, new_const_ptr)?;
    }
    let old_path = core::str::from_utf8(&old_path_buf).map_err(|_| Errno::EINVAL)?;
    let new_path = core::str::from_utf8(&new_path_buf).map_err(|_| Errno::EINVAL)?;

    let old_abs = resolve_path(old_path)?;
    let new_abs = resolve_path(new_path)?;

    // Resolve parents for events
    let (old_parent_path, old_name) = split_parent(&old_abs);
    let (new_parent_path, new_name) = split_parent(&new_abs);
    let old_parent_node = vfs::mount::lookup(old_parent_path).ok();
    let new_parent_node = vfs::mount::lookup(new_parent_path).ok();

    // Perform rename (VFS mount layer needs a rename method too, which redirects to driver)
    vfs::mount::rename(&old_abs, &new_abs)?;

    // Emit MOVE events
    let cookie = NEXT_COOKIE.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    if let Some(parent) = old_parent_node {
        crate::vfs::watch::emit_event(
            &*parent,
            abi::vfs_watch::mask::MOVE_FROM,
            Some(old_name),
            cookie,
        );
    }
    if let Some(parent) = new_parent_node {
        crate::vfs::watch::emit_event(
            &*parent,
            abi::vfs_watch::mask::MOVE_TO,
            Some(new_name),
            cookie,
        );
    }

    Ok(0)
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn split_parent(path: &str) -> (&str, &str) {
    let trimmed = path.trim_end_matches('/');
    match trimmed.rfind('/') {
        Some(0) => ("/", &trimmed[1..]),
        Some(idx) => (&trimmed[..idx], &trimmed[idx + 1..]),
        None => ("/", trimmed),
    }
}

pub fn resolve_path(path: &str) -> SysResult<alloc::string::String> {
    let abs = if path.starts_with('/') {
        alloc::string::String::from(path)
    } else {
        let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let cwd = pinfo.lock().cwd.clone();

        if cwd.ends_with('/') {
            alloc::format!("{}{}", cwd, path)
        } else {
            alloc::format!("{}/{}", cwd, path)
        }
    };

    // Ensure all paths are canonical (handle . and ..)
    vfs::path::normalise(&abs)
}

pub fn sys_fs_chdir(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;

    // Verify it exists and is a directory
    let node = vfs::mount::lookup(&abs_path)?;
    let stat = node.stat()?;
    if !stat.is_dir() {
        return Err(Errno::ENOTDIR);
    }

    // Normalise is now handled by resolve_path
    let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    pinfo.lock().cwd = abs_path;

    Ok(0)
}

pub fn sys_fs_getcwd(buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let cwd = pinfo.lock().cwd.clone();

    let needed = cwd.len();
    if buf_ptr != 0 && buf_len > 0 {
        let copy_len = buf_len.min(needed);
        validate_user_range(buf_ptr, copy_len, true)?;
        unsafe { copyout(buf_ptr, &cwd.as_bytes()[..copy_len])? };
    }
    Ok(needed)
}

/// Resolve `path` (relative or absolute) to its canonical absolute form,
/// writing the result into the caller-supplied buffer.
///
/// Signature: `SYS_FS_REALPATH(path_ptr, path_len, buf_ptr, buf_len) → len`
///
/// - On success returns the length of the canonical path (excluding NUL).
/// - If `buf_len` is smaller than the canonical path length, the output
///   buffer is not written; the needed length is still returned so the
///   caller can retry with a suitably sized buffer.
/// - `buf_ptr` may be `0` (null) to query the required size without
///   writing any output; `buf_len` is ignored in that case.
/// - Returns `EINVAL` if `path` is empty or too long.
/// - Returns `ENOENT` if no process context is available (relative path + no cwd).
pub fn sys_fs_realpath(
    path_ptr: usize,
    path_len: usize,
    buf_ptr: usize,
    buf_len: usize,
) -> SysResult<usize> {
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(path_ptr, path_len, false)?;

    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let canonical = resolve_path(path)?;
    let canonical_bytes = canonical.as_bytes();
    let needed = canonical_bytes.len();

    if buf_ptr != 0 && buf_len >= needed {
        validate_user_range(buf_ptr, needed, true)?;
        unsafe { copyout(buf_ptr, canonical_bytes)? };
    }

    Ok(needed)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sched::hooks::CURRENT_TID_HOOK;
    use crate::vfs::fd_table::FdTable;
    use crate::vfs::{OpenFlags, VfsNode, VfsStat};
    use abi::errors::SysResult;
    use abi::syscall::{poll_flags, PollFd};
    use alloc::sync::Arc;
    use spin::Mutex;

    // ── Test nodes ────────────────────────────────────────────────────────────

    /// A node that is always readable and writable (POLLIN | POLLOUT).
    struct AlwaysReadyNode;
    impl VfsNode for AlwaysReadyNode {
        fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
            Ok(0)
        }
        fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
            Ok(buf.len())
        }
        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat {
                mode: VfsStat::S_IFCHR | 0o666,
                ..Default::default()
            })
        }
        // poll() not overridden → returns POLLIN | POLLOUT (default)
    }

    /// A node whose poll() reports no readiness at all.
    struct NeverReadyNode;
    impl VfsNode for NeverReadyNode {
        fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
            Ok(0)
        }
        fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
            Ok(buf.len())
        }
        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat {
                mode: VfsStat::S_IFCHR | 0o666,
                ..Default::default()
            })
        }
        fn poll(&self) -> u16 {
            0 // never ready
        }
    }

    // ── Serialization guard ───────────────────────────────────────────────────
    //
    // The kernel hooks (`PROCESS_INFO_HOOK`, `CURRENT_TID_HOOK`) are global
    // mutable statics.  Tests run concurrently, so we serialize them with a
    // spin-mutex guard.  The critical sections are short (non-blocking poll)
    // so spinning is acceptable here.

    static TEST_POLL_GUARD: spin::Mutex<()> = spin::Mutex::new(());

    // Holds the current test's ProcessInfo while inside the critical section.
    static TEST_PROCESS_INFO: spin::Mutex<
        Option<Arc<Mutex<crate::task::ProcessInfo>>>,
    > = spin::Mutex::new(None);

    fn process_info_hook() -> Option<Arc<Mutex<crate::task::ProcessInfo>>> {
        TEST_PROCESS_INFO.lock().clone()
    }

    fn test_current_tid() -> u64 {
        42
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn make_process_info_with_nodes(
        nodes: &[(u32, Arc<dyn VfsNode>)],
    ) -> Arc<Mutex<crate::task::ProcessInfo>> {
        let mut fd_table = FdTable::new();
        for (fd, node) in nodes {
            fd_table
                .insert_at(*fd, node.clone(), OpenFlags::read_write(), "/test".into())
                .expect("insert_at");
        }
        Arc::new(Mutex::new(crate::task::ProcessInfo {
            pid: 1,
            ppid: 0,
            argv: alloc::vec![],
            env: alloc::collections::BTreeMap::new(),
            auxv: alloc::vec![],
            fd_table,
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            thread_ids: alloc::vec![1],
        }))
    }

    /// Run `sys_fs_poll` with `timeout_ms = 0` (non-blocking) over `fds`.
    ///
    /// Acquires `TEST_POLL_GUARD` to serialize concurrent tests that share
    /// the global `PROCESS_INFO_HOOK` and `CURRENT_TID_HOOK`.
    fn poll_nonblocking(
        pinfo: Arc<Mutex<crate::task::ProcessInfo>>,
        fds: &mut [PollFd],
    ) -> SysResult<usize> {
        let _guard = TEST_POLL_GUARD.lock();

        // Install hooks while holding the guard so no other test can clobber
        // them between setup and the actual sys_fs_poll call.
        unsafe {
            CURRENT_TID_HOOK = Some(test_current_tid);
            crate::sched::hooks::PROCESS_INFO_HOOK = Some(process_info_hook);
        }
        TEST_PROCESS_INFO.lock().replace(pinfo);

        // Use the slice's heap address as "user" memory.
        // validate_user_range only rejects NULL and kernel-high addresses.
        let ptr = fds.as_mut_ptr() as usize;
        let nfds = fds.len();
        let res = sys_fs_poll(ptr, nfds, 0 /* non-blocking */);

        // Clean up before releasing the guard.
        unsafe {
            crate::sched::hooks::PROCESS_INFO_HOOK = None;
            CURRENT_TID_HOOK = None;
        }
        TEST_PROCESS_INFO.lock().take();

        res
        // _guard released here
    }

    // ── Tests ─────────────────────────────────────────────────────────────────

    /// Passing nfds=0 must succeed immediately without touching any state.
    #[test]
    fn poll_zero_nfds_returns_ok_zero() {
        let res = sys_fs_poll(0, 0, 0);
        assert_eq!(res, Ok(0));
    }

    /// A single fd backed by an always-ready node → returns 1 with POLLIN set.
    #[test]
    fn poll_single_ready_fd_returns_one() {
        let node: Arc<dyn VfsNode> = Arc::new(AlwaysReadyNode);
        let pinfo = make_process_info_with_nodes(&[(3, node)]);

        let mut fds = [PollFd {
            fd: 3,
            events: poll_flags::POLLIN,
            revents: 0,
        }];

        let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
        assert_eq!(n, 1, "one fd should be ready");
        assert_ne!(
            fds[0].revents & poll_flags::POLLIN,
            0,
            "POLLIN should be set"
        );
    }

    /// Non-blocking poll over a node with poll() == 0 must return 0 immediately.
    #[test]
    fn poll_nonblocking_returns_zero_when_no_fd_ready() {
        let node: Arc<dyn VfsNode> = Arc::new(NeverReadyNode);
        let pinfo = make_process_info_with_nodes(&[(3, node)]);

        let mut fds = [PollFd {
            fd: 3,
            events: poll_flags::POLLIN,
            revents: 0,
        }];

        let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
        assert_eq!(n, 0, "no fds ready in non-blocking mode");
        assert_eq!(fds[0].revents, 0, "revents must remain 0");
    }

    /// Multiple ready fds: the return value is the count of fds with revents != 0.
    #[test]
    fn poll_multiple_ready_fds_returns_correct_count() {
        let ready: Arc<dyn VfsNode> = Arc::new(AlwaysReadyNode);
        let not_ready: Arc<dyn VfsNode> = Arc::new(NeverReadyNode);
        let pinfo = make_process_info_with_nodes(&[
            (3, ready.clone()),
            (4, not_ready.clone()),
            (5, ready.clone()),
        ]);

        let mut fds = [
            PollFd {
                fd: 3,
                events: poll_flags::POLLIN | poll_flags::POLLOUT,
                revents: 0,
            },
            PollFd {
                fd: 4,
                events: poll_flags::POLLIN,
                revents: 0,
            },
            PollFd {
                fd: 5,
                events: poll_flags::POLLOUT,
                revents: 0,
            },
        ];

        let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
        assert_eq!(n, 2, "two ready fds (indices 0 and 2)");
        assert_ne!(fds[0].revents, 0, "fd 3 should be ready");
        assert_eq!(fds[1].revents, 0, "fd 4 should not be ready");
        assert_ne!(fds[2].revents, 0, "fd 5 should be ready");
    }

    /// A fd number that doesn't exist in the fd table must yield POLLNVAL.
    #[test]
    fn poll_invalid_fd_reports_pollnval() {
        let pinfo = make_process_info_with_nodes(&[]);

        let mut fds = [PollFd {
            fd: 99, // no such fd
            events: poll_flags::POLLIN,
            revents: 0,
        }];

        let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
        assert_eq!(n, 1, "POLLNVAL counts as a ready entry");
        assert_ne!(
            fds[0].revents & poll_flags::POLLNVAL,
            0,
            "POLLNVAL should be set for missing fd"
        );
    }

    /// A negative fd must be silently skipped (revents stays 0).
    #[test]
    fn poll_negative_fd_is_skipped() {
        let pinfo = make_process_info_with_nodes(&[]);

        let mut fds = [PollFd {
            fd: -1,
            events: poll_flags::POLLIN,
            revents: 0,
        }];

        let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
        assert_eq!(n, 0, "negative fd is silently skipped");
        assert_eq!(fds[0].revents, 0, "revents must stay 0 for negative fd");
    }

    /// Mixed poll: pipe (ready), channel (not ready), VFS file (ready).
    /// Exercises the multi-fd path with heterogeneous node types.
    #[test]
    fn poll_mixed_pipe_channel_vfsfile() {
        let (pipe_r, pipe_w) = crate::ipc::pipe::create_fd_pair(0, false);
        // Write data so the read end is POLLIN-ready.
        pipe_w.write(0, b"hello").expect("write to pipe");

        let always: Arc<dyn VfsNode> = Arc::new(AlwaysReadyNode);
        let never: Arc<dyn VfsNode> = Arc::new(NeverReadyNode);

        let pinfo = make_process_info_with_nodes(&[
            (3, pipe_r.clone()),
            (4, never.clone()),
            (5, always.clone()),
        ]);

        let mut fds = [
            PollFd {
                fd: 3,
                events: poll_flags::POLLIN,
                revents: 0,
            },
            PollFd {
                fd: 4,
                events: poll_flags::POLLIN,
                revents: 0,
            },
            PollFd {
                fd: 5,
                events: poll_flags::POLLIN | poll_flags::POLLOUT,
                revents: 0,
            },
        ];

        let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
        assert_eq!(
            n, 2,
            "pipe-read and always-ready should fire; never-ready should not"
        );
        assert_ne!(
            fds[0].revents & poll_flags::POLLIN,
            0,
            "pipe read end has data → POLLIN"
        );
        assert_eq!(fds[1].revents, 0, "NeverReadyNode → no events");
        assert_ne!(
            fds[2].revents & (poll_flags::POLLIN | poll_flags::POLLOUT),
            0,
            "AlwaysReadyNode → ready"
        );
    }
}
