//! Port IPC syscalls

use super::{copyin, copyout};
use crate::syscall::validate::validate_user_range;
use abi::errors::{Errno, SysResult};

// Lines 7-9 are duplicates of 3-5

pub fn sys_channel_create(capacity: usize) -> SysResult<usize> {
    let capacity = capacity.min(65536).max(64);
    let port_id = crate::ipc::create_port(capacity);

    let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
    let write_handle = table
        .alloc(port_id, crate::ipc::HandleMode::Write)
        .ok_or(Errno::ENOMEM)?;
    let read_handle = table
        .alloc(port_id, crate::ipc::HandleMode::Read)
        .ok_or(Errno::ENOMEM)?;

    let packed = ((write_handle.0 as usize) << 16) | (read_handle.0 as usize);
    Ok(packed)
}

pub fn sys_channel_send(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, false)?;

    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, crate::ipc::HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    if !port.has_readers() {
        return Err(Errno::EPIPE);
    }

    let mut buf = [0u8; 4096];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    let written = port.send(&buf[..len]);
    Ok(written)
}

pub fn sys_channel_send_all(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, false)?;

    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, crate::ipc::HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    if !port.has_readers() {
        return Err(Errno::EPIPE);
    }

    let mut buf = [0u8; 4096];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    if port.send_all(&buf[..len]) {
        crate::ktrace!(
            "sys_channel_send_all: wrote {} bytes to port {}",
            len,
            entry.port_id.0
        );
        Ok(len)
    } else {
        crate::ktrace!(
            "sys_channel_send_all: port {} FULL, returning EAGAIN",
            entry.port_id.0
        );
        Err(Errno::EAGAIN)
    }
}

fn sys_channel_recv_impl(
    handle: usize,
    ptr: usize,
    len: usize,
    blocking: bool,
) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, true)?;

    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, crate::ipc::HandleMode::Read)
            .copied()
            .ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    let mut buf = [0u8; 4096];
    let tid = unsafe { crate::sched::current_tid_current() };

    loop {
        let read = port.try_recv(&mut buf[..len]);
        if read > 0 {
            unsafe {
                copyout(ptr, &buf[..read])?;
            }
            crate::ktrace!(
                "sys_channel_recv: read {} bytes from port {}",
                read,
                entry.port_id.0
            );
            return Ok(read);
        }

        if !port.has_writers() {
            return Err(Errno::EPIPE);
        }

        if !blocking {
            return Err(Errno::EAGAIN);
        }

        port.add_waiter_read(tid);

        let read = port.try_recv(&mut buf[..len]);
        if read > 0 {
            port.remove_waiter_read(tid);
            unsafe {
                copyout(ptr, &buf[..read])?;
            }
            crate::ktrace!(
                "sys_channel_recv: read {} bytes from port {} after wait registration",
                read,
                entry.port_id.0
            );
            return Ok(read);
        }

        if !port.has_writers() {
            port.remove_waiter_read(tid);
            return Err(Errno::EPIPE);
        }

        unsafe {
            crate::sched::block_current_erased();
        }
    }
}

pub fn sys_channel_recv(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    sys_channel_recv_impl(handle, ptr, len, true)
}

pub fn sys_channel_try_recv(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    sys_channel_recv_impl(handle, ptr, len, false)
}

pub fn sys_channel_close(handle: usize) -> SysResult<usize> {
    let handle = crate::ipc::Handle(handle as u32);
    let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
    if let Some(entry) = table.close(handle) {
        drop(table);

        let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
        let destroy = match entry.mode {
            crate::ipc::HandleMode::Read => port.close_reader(),
            crate::ipc::HandleMode::Write => port.close_writer(),
        };
        if destroy {
            crate::ipc::close_port(entry.port_id);
        }
        Ok(0)
    } else {
        Err(Errno::EBADF)
    }
}

pub fn sys_channel_wait(handles_ptr: usize, count: usize, flags: usize) -> SysResult<usize> {
    if count == 0 || count > 64 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(handles_ptr, count * 4, false)?;

    let mut handles = [0u32; 64];
    unsafe {
        let dest = core::slice::from_raw_parts_mut(handles.as_mut_ptr() as *mut u8, count * 4);
        copyin(dest, handles_ptr)?;
    }

    let tid = unsafe { crate::sched::current_tid_current() };
    let flags = flags as u32;

    // Cleanup helper to ensure we don't leave stale entries in any port's wait queue
    let cleanup = |handles: &[u32], table: &mut crate::ipc::HandleTable| {
        for &h in handles {
            let h_ipc = crate::ipc::Handle(h);
            if let Some(entry) = table.get(h_ipc, crate::ipc::HandleMode::Read) {
                if let Some(port) = crate::ipc::get_port(entry.port_id) {
                    port.remove_waiter_read(tid);
                }
            }
            if let Some(entry) = table.get(h_ipc, crate::ipc::HandleMode::Write) {
                if let Some(port) = crate::ipc::get_port(entry.port_id) {
                    port.remove_waiter_write(tid);
                }
            }
        }
    };

    loop {
        // 1. Register as waiter BEFORE checking (prevents race)
        {
            let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
            for i in 0..count {
                let h = handles[i];
                let h_ipc = crate::ipc::Handle(h);
                if (flags & abi::syscall::channel_wait::READABLE) != 0 {
                    if let Some(entry) = table.get(h_ipc, crate::ipc::HandleMode::Read) {
                        if let Some(port) = crate::ipc::get_port(entry.port_id) {
                            port.add_waiter_read(tid);
                        }
                    }
                }
                if (flags & abi::syscall::channel_wait::WRITABLE) != 0 {
                    if let Some(entry) = table.get(h_ipc, crate::ipc::HandleMode::Write) {
                        if let Some(port) = crate::ipc::get_port(entry.port_id) {
                            port.add_waiter_write(tid);
                        }
                    }
                }
            }
        }

        // 2. Check if any port is ready
        {
            let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
            for i in 0..count {
                let h = handles[i];
                let h_ipc = crate::ipc::Handle(h);
                if let Some(entry) = table.get(h_ipc, crate::ipc::HandleMode::Read) {
                    if (flags & abi::syscall::channel_wait::READABLE) != 0 {
                        if let Some(port) = crate::ipc::get_port(entry.port_id) {
                            if !port.is_empty() {
                                crate::ktrace!(
                                    "sys_port_wait: port {} is NOT empty, returning Ok({})",
                                    entry.port_id.0,
                                    h
                                );
                                cleanup(&handles[..count], &mut table);
                                return Ok(h as usize);
                            }
                        }
                    }
                }
                if let Some(entry) = table.get(h_ipc, crate::ipc::HandleMode::Write) {
                    if (flags & abi::syscall::channel_wait::WRITABLE) != 0 {
                        if let Some(port) = crate::ipc::get_port(entry.port_id) {
                            if !port.is_full() {
                                cleanup(&handles[..count], &mut table);
                                return Ok(h as usize);
                            }
                        }
                    }
                }
            }
        }

        // 3. Block current task
        unsafe {
            crate::ktrace!("sys_port_wait: blocking task {}", tid);
            crate::sched::block_current_erased();
            crate::ktrace!("sys_port_wait: unblocked task {}", tid);
        }
    }
}

pub fn sys_channel_info(handle: usize) -> SysResult<usize> {
    let handle = crate::ipc::Handle(handle as u32);
    let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
    let entry = table
        .get(handle, crate::ipc::HandleMode::Read)
        .or_else(|| table.get(handle, crate::ipc::HandleMode::Write))
        .ok_or(Errno::EBADF)?;

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;

    let len = port.len();
    let cap = port.capacity(); // Need to expose capacity

    // Return packed: top 32 bits capacity, bottom 32 bits length
    Ok((cap << 32) | (len & 0xFFFFFFFF))
}

pub fn sys_channel_send_handle(handle: usize, fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let vfs_node = {
        let lock = pinfo_arc.lock();
        let file = lock.fd_table.get(fd as u32)?;
        file.node.clone()
    };

    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, crate::ipc::HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    port.send_cap(vfs_node);
    Ok(0)
}

pub fn sys_channel_recv_handle(handle: usize, out_fd_ptr: usize) -> SysResult<usize> {
    validate_user_range(out_fd_ptr, 4, true)?;

    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, crate::ipc::HandleMode::Read)
            .copied()
            .ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    let cap = port.try_recv_cap().ok_or(Errno::EAGAIN)?;

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let new_fd = pinfo_arc
        .lock()
        .fd_table
        .open(cap, crate::vfs::OpenFlags::read_write(), "port".into())?;

    let new_fd_bytes = new_fd.to_ne_bytes();
    unsafe { super::copyout(out_fd_ptr, &new_fd_bytes)? };

    Ok(0)
}
