//! VFS syscall handlers: open, close, read, write, stat, readdir,
//!                       unlink, mkdir, dup, dup2, pipe, poll.
//!
//! These handlers implement the janix VFS syscall interface:
//!
//! - [`SYS_FS_open`]   — open (or create) a path, return a file descriptor
//! - [`SYS_FS_close`]  — release a file descriptor (all fds, including 0-2)
//! - [`SYS_FS_read`]   — read from a file descriptor into a user buffer
//! - [`SYS_FS_write`]  — write from a user buffer to a file descriptor
//! - [`SYS_FS_unlink`] — remove a file or empty directory
//! - [`SYS_FS_mkdir`]  — create a directory
//! - [`SYS_FS_DUP`]        — duplicate a file descriptor to the lowest free slot
//! - [`SYS_FS_DUP2`]       — duplicate a file descriptor to a specific slot
//! - [`sys_pipe`]       — create an anonymous pipe, allocating two fds
//! - [`SYS_FS_poll`]   — poll a set of fds for readiness (POSIX-style)

use alloc::vec;
use alloc::sync::Arc;

use abi::errors::{Errno, SysResult};
use abi::syscall::{PollFd, poll_flags, vfs_flags};

use crate::syscall::validate::{copyin, copyout, validate_user_range};
use crate::vfs::{self, OpenFlags};

// ── open ────────────────────────────────────────────────────────────────────

pub fn SYS_FS_open(path_ptr: usize, path_len: usize, flags: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }

    // Copy path from userspace.
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    if path == "/dev/fb0" {
        crate::kinfo!(
            "SYS_FS_open: path='{}' len={} flags=0x{:x}",
            path,
            path_len,
            flags
        );
    }

    let open_flags = OpenFlags(flags as u32);
    let want_creat = (flags as u32) & vfs_flags::O_CREAT != 0;
    let want_trunc = (flags as u32) & vfs_flags::O_TRUNC != 0;

    // Resolve path through the mount table, creating the file if O_CREAT is set.
    let node = if want_creat {
        // Try lookup first; fall back to create if the file doesn't exist.
        match vfs::mount::lookup(path) {
            Ok(existing) => {
                if want_trunc {
                    // Truncate the file to zero length.
                    let _ = existing.truncate(0);
                }
                existing
            }
            Err(Errno::ENOENT) => vfs::mount::create(path)?,
            Err(e) => return Err(e),
        }
    } else {
        vfs::mount::lookup(path)?
    };

    if path == "/dev/fb0" {
        match node.stat() {
            Ok(stat) => crate::kinfo!(
                "SYS_FS_open: resolved node for /dev/fb0 mode=0o{:o} size={} ino={}",
                stat.mode,
                stat.size,
                stat.ino
            ),
            Err(err) => crate::kwarn!("SYS_FS_open: resolved /dev/fb0 but stat failed: {:?}", err),
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
                    "SYS_FS_open: no process info for /dev/fb0 current_tid={} direct_lookup={}",
                    tid,
                    direct
                );
            }
            return Err(Errno::ENOENT);
        }
    };
    if path == "/dev/fb0" {
        crate::kinfo!("SYS_FS_open: process info present for /dev/fb0");
    }
    let fd = pinfo_arc.lock().fd_table.open(node, open_flags)?;

    if path == "/dev/fb0" {
        crate::kinfo!("SYS_FS_open: fd_table.open('/dev/fb0') -> {}", fd);
    }

    Ok(fd as usize)
}

// ── close ───────────────────────────────────────────────────────────────────

pub fn SYS_FS_close(fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    pinfo_arc.lock().fd_table.close(fd as u32)?;
    Ok(0)
}

// ── read ────────────────────────────────────────────────────────────────────

pub fn SYS_FS_read(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
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

pub fn SYS_FS_write(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
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

    if n > 0 {
        *offset_cell.lock() = offset.saturating_add(n as u64);
        // Emit MODIFY event
        crate::vfs::watch::emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0);
    }

    Ok(n)
}

pub fn SYS_FS_unlink(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    
    // Resolve parent to emit event
    let (parent_path, name) = split_parent(path);
    let parent_node = vfs::mount::lookup(parent_path).ok();

    vfs::mount::unlink(path)?;

    if let Some(parent) = parent_node {
        crate::vfs::watch::emit_event(&*parent, abi::vfs_watch::mask::REMOVE, Some(name), 0);
    }

    Ok(0)
}

// ── mkdir ───────────────────────────────────────────────────────────────────

/// Create a directory at `path`.
pub fn SYS_FS_mkdir(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    
    // Resolve parent to emit event
    let (parent_path, name) = split_parent(path);
    let parent_node = vfs::mount::lookup(parent_path).ok();

    vfs::mount::mkdir(path)?;

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

    let (read_node, write_node) = crate::ipc::pipe::create_fd_pair(4096, false);
    let (read_fd, write_fd) = {
        let mut lock = pinfo_arc.lock();
        let read_fd = lock
            .fd_table
            .open(read_node, crate::vfs::OpenFlags::read_only())?;
        match lock
            .fd_table
            .open(write_node, crate::vfs::OpenFlags::write_only())
        {
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

// ── mount ───────────────────────────────────────────────────────────────────

/// Mount a userland VFS provider at the given path prefix.
///
/// `provider_write_handle` is the *write* end of a port pair owned by the
/// calling process.  The kernel will send VFS RPC messages (see
/// [`abi::vfs_rpc`]) to that port whenever a path under `path` is accessed.
///
/// The kernel creates a private response port and registers its write-handle
/// in the global handle table so the provider can call `SYS_PORT_SEND` to
/// deliver replies.
pub fn SYS_FS_mount(
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

    // Resolve the provider's write handle to a port Arc.
    let prov_handle = crate::ipc::Handle(provider_write_handle as u32);
    let prov_entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(prov_handle, crate::ipc::HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };
    let req_port = crate::ipc::get_port(prov_entry.port_id).ok_or(Errno::EBADF)?;

    // Create the kernel response port.
    // Large capacity to hold multiple concurrent responses (though we serialise
    // requests, responses may vary in size).
    let resp_port_id = crate::ipc::create_port(abi::vfs_rpc::VFS_RPC_MAX_RESP * 4);
    let resp_port = crate::ipc::get_port(resp_port_id).ok_or(Errno::ENOMEM)?;

    // Register the write end of the response port in the global handle table
    // so the provider process can call SYS_PORT_SEND on it.
    let resp_write_handle = {
        let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .alloc(resp_port_id, crate::ipc::HandleMode::Write)
            .ok_or(Errno::ENOMEM)?
    };

    // Build and mount the provider filesystem.
    let provider_fs = alloc::sync::Arc::new(vfs::provider::ProviderFs::new(
        req_port,
        resp_port,
        resp_write_handle.0,
    ));
    vfs::mount::mount(path, provider_fs);

    crate::kinfo!("vfs: mounted userland provider at {}", path);
    Ok(0)
}

// ── umount ──────────────────────────────────────────────────────────────────

/// Unmount the VFS provider at the given path prefix.
pub fn SYS_FS_umount(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    vfs::mount::umount(path)?;
    crate::kinfo!("vfs: unmounted userland provider at {}", path);
    Ok(0)
}

// ── poll ────────────────────────────────────────────────────────────────────

/// POSIX-style poll over VFS file descriptors.
///
/// Examines each entry in the `pollfds` array and sets `revents` on those
/// that are immediately ready.  `timeout_ms` is currently **ignored** — the
/// syscall returns immediately (non-blocking poll).  A blocking variant that
/// parks the calling task will be added once a generic fd-readiness wait
/// mechanism is in place.
///
/// # Arguments
/// - `pollfds_ptr` — pointer to a `[PollFd; nfds]` in user memory (read/write)
/// - `nfds`         — number of entries in the array
/// - `_timeout_ms`  — milliseconds to wait (currently unused; returns immediately)
///
/// # Returns
/// The number of entries with non-zero `revents`, or an errno on error.
pub fn SYS_FS_poll(pollfds_ptr: usize, nfds: usize, _timeout_ms: usize) -> SysResult<usize> {
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
    let mut ready_count = 0usize;

    for kfd in kfds.iter_mut() {
        kfd.revents = 0;

        if kfd.fd < 0 {
            // Negative fd → skip (POSIX says ignore these).
            continue;
        }

        let fd = kfd.fd as u32;

        // Try to get the node; POLLNVAL if the fd is not open.
        let node_opt = {
            let lock = pinfo_arc.lock();
            lock.fd_table.get(fd).ok().map(|f| f.node.clone())
        };

        let node = match node_opt {
            Some(n) => n,
            None => {
                kfd.revents |= poll_flags::POLLNVAL;
                ready_count += 1;
                continue;
            }
        };

        // Determine readiness by checking the node's stat.  For now we use a
        // simple heuristic: regular files and character devices are always
        // ready; pipes report readiness based on whether any data is available.
        let stat = node.stat().unwrap_or_default();
        let is_pipe = stat.is_fifo();
        let is_chr = stat.is_chr();
        let is_reg = stat.is_reg();
        let is_dir = stat.is_dir();

        let mut revents: u16 = 0;

        if (kfd.events & poll_flags::POLLIN) != 0 {
            // Regular files and char devices are always readable.
            if is_reg || is_dir {
                revents |= poll_flags::POLLIN;
            } else if is_chr {
                // Character device (e.g. /dev/console): probe by attempting a
                // non-blocking zero-byte-sized read — the node signals
                // readiness by succeeding rather than returning EAGAIN.
                let mut probe = [0u8; 0];
                match node.read(0, &mut probe) {
                    Err(Errno::EAGAIN) => {} // not ready
                    _ => revents |= poll_flags::POLLIN,
                }
            } else if is_pipe {
                // Attempt a zero-byte read; EAGAIN means no data available.
                let mut probe = [0u8; 0];
                match node.read(0, &mut probe) {
                    Err(Errno::EAGAIN) => {}
                    _ => revents |= poll_flags::POLLIN,
                }
            }
        }

        if (kfd.events & poll_flags::POLLOUT) != 0 {
            // Files and character devices (including pipes write end) are
            // assumed always writable for this initial implementation.
            if is_reg || is_chr || is_pipe {
                revents |= poll_flags::POLLOUT;
            }
        }

        kfd.revents = revents;
        if revents != 0 {
            ready_count += 1;
        }
    }

    // Write the updated PollFd array back to userspace.
    unsafe {
        copyout(
            pollfds_ptr,
            core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
        )?
    };

    Ok(ready_count)
}

// ── seek ────────────────────────────────────────────────────────────────────
pub fn SYS_FS_seek(fd: usize, offset: usize, whence: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let mut lock = pinfo_arc.lock();
    let file = lock.fd_table.get_mut(fd as u32)?;
    
    let node = file.node.clone();
    let stat = node.stat()?;
    let size = stat.size;
    
    let mut current_offset = *file.offset.lock();
    let new_offset = match whence {
        0 => offset as u64, // SEEK_SET
        1 => current_offset.saturating_add(offset as u64), // SEEK_CUR
        2 => size.saturating_add(offset as u64), // SEEK_END
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
    let watch_fd = pinfo_arc.lock().fd_table.open(watch, crate::vfs::OpenFlags::read_only())?;
    Ok(watch_fd as usize)
}

pub fn sys_watch_path(path_ptr: usize, path_len: usize, mask: usize, flags: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    crate::kinfo!("WATCH_PATH: looking up '{}' mask=0x{:x}", path, mask);
    let node = vfs::mount::lookup(path)?;
    crate::kinfo!("WATCH_PATH: resolved node, registering watch");

    let watch = Arc::new(crate::vfs::watch::Watch::new(mask as u32, flags as u32));
    crate::vfs::watch::register_watch(&node, watch.clone())?;
    crate::kinfo!("WATCH_PATH: registered watch, opening fd");

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let watch_fd = pinfo_arc.lock().fd_table.open(watch, crate::vfs::OpenFlags::read_only())?;
    crate::kinfo!("WATCH_PATH: success fd={}", watch_fd);
    Ok(watch_fd as usize)
}

// ── rename ──────────────────────────────────────────────────────────────────

static NEXT_COOKIE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(1);

pub fn SYS_FS_rename(
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

    // Resolve parents for events
    let (old_parent_path, old_name) = split_parent(old_path);
    let (new_parent_path, new_name) = split_parent(new_path);
    let old_parent_node = vfs::mount::lookup(old_parent_path).ok();
    let new_parent_node = vfs::mount::lookup(new_parent_path).ok();

    // Perform rename (VFS mount layer needs a rename method too, which redirects to driver)
    vfs::mount::rename(old_path, new_path)?;

    // Emit MOVE events
    let cookie = NEXT_COOKIE.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    if let Some(parent) = old_parent_node {
        crate::vfs::watch::emit_event(&*parent, abi::vfs_watch::mask::MOVE_FROM, Some(old_name), cookie);
    }
    if let Some(parent) = new_parent_node {
        crate::vfs::watch::emit_event(&*parent, abi::vfs_watch::mask::MOVE_TO, Some(new_name), cookie);
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
