//! Port IPC syscalls

use crate::syscall::validate::validate_user_range;
use super::{copyin, copyout};
use abi::errors::{Errno, SysResult};

/// Global handle tables per-process (simplified: single global table for v0)
static PORT_HANDLE_TABLE: spin::Mutex<crate::ipc::HandleTable> = 
    spin::Mutex::new(crate::ipc::HandleTable::new());

pub fn sys_port_create(capacity: usize) -> SysResult<usize> {
    let capacity = capacity.min(65536).max(64);
    let port_id = crate::ipc::create_port(capacity);
    
    let mut table = PORT_HANDLE_TABLE.lock();
    let write_handle = table.alloc(port_id, crate::ipc::HandleMode::Write)
        .ok_or(Errno::ENOMEM)?;
    let read_handle = table.alloc(port_id, crate::ipc::HandleMode::Read)
        .ok_or(Errno::ENOMEM)?;
    
    let packed = ((write_handle.0 as usize) << 16) | (read_handle.0 as usize);
    Ok(packed)
}

pub fn sys_port_send(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }
    
    validate_user_range(ptr, len, false)?;
    
    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = PORT_HANDLE_TABLE.lock();
        table.get(handle, crate::ipc::HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };
    
    let port = crate::ipc::get_port(entry.port_id)
        .ok_or(Errno::EBADF)?;
    
    let mut buf = [0u8; 4096];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }
    
    let written = port.send(&buf[..len]);
    Ok(written)
}

pub fn sys_port_recv(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }
    
    validate_user_range(ptr, len, true)?;
    
    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = PORT_HANDLE_TABLE.lock();
        table.get(handle, crate::ipc::HandleMode::Read)
            .copied()
            .ok_or(Errno::EBADF)?
    };
    
    let port = crate::ipc::get_port(entry.port_id)
        .ok_or(Errno::EBADF)?;
    
    let mut buf = [0u8; 4096];
    let read = port.recv(&mut buf[..len]);
    
    if read > 0 {
        unsafe {
            copyout(ptr, &buf[..read])?;
        }
    }
    
    Ok(read)
}

pub fn sys_port_close(handle: usize) -> SysResult<usize> {
    let handle = crate::ipc::Handle(handle as u32);
    let mut table = PORT_HANDLE_TABLE.lock();
    if table.close(handle) {
        Ok(0)
    } else {
        Err(Errno::EBADF)
    }
}
