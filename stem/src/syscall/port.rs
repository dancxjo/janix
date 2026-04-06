//! Port IPC syscall wrappers for userspace

use crate::syscall::arch::raw_syscall6;
use abi::errors::Errno;
use abi::syscall::*;

/// A handle to a port for IPC
pub type PortHandle = u32;

/// Create a new port (returns packed read/write handles)
/// Result: (write_handle << 16) | read_handle
pub fn port_create(capacity: usize) -> Result<(PortHandle, PortHandle), Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_CREATE, capacity, 0, 0, 0, 0, 0) };
    let val = abi::errors::errno(ret)?;
    let write_handle = ((val >> 16) & 0xFFFF) as PortHandle;
    let read_handle = (val & 0xFFFF) as PortHandle;
    Ok((write_handle, read_handle))
}

/// Send bytes to a port via handle
/// Returns number of bytes written
pub fn port_send(handle: PortHandle, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_SEND,
            handle as usize,
            data.as_ptr() as usize,
            data.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Send bytes atomically: either all bytes are written or EAGAIN is returned.
pub fn port_send_all(handle: PortHandle, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_SEND_ALL,
            handle as usize,
            data.as_ptr() as usize,
            data.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Receive bytes from a port via handle
/// Returns number of bytes read
pub fn port_recv(handle: PortHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_RECV,
            handle as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Try to receive bytes from a port without blocking.
/// Returns `EAGAIN` when no bytes are queued.
pub fn port_try_recv(handle: PortHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_TRY_RECV,
            handle as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Close a port handle
pub fn port_close(handle: PortHandle) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_CLOSE, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

/// Wait for any of the given port handles to meet the criteria in `flags`.
/// Returns the handle that met the criteria.
pub fn port_wait(handles: &[PortHandle], flags: u32) -> Result<PortHandle, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_WAIT,
            handles.as_ptr() as usize,
            handles.len(),
            flags as usize,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as PortHandle)
}

/// Returns the current number of bytes in the port buffer
pub fn port_len(handle: PortHandle) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_INFO, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v & 0xFFFFFFFF) as usize)
}

/// Returns the maximum capacity of the port buffer
pub fn port_capacity(handle: PortHandle) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_INFO, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v >> 32) as usize)
}

/// Create a broadcast topic and return its topic id.
pub fn topic_create() -> Result<u32, Errno> {
    let ret = unsafe { raw_syscall6(SYS_TOPIC_CREATE, 0, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| v as u32)
}

/// Subscribe a write port handle to a topic.
pub fn topic_subscribe(topic_id: u32, write_handle: PortHandle) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_TOPIC_SUBSCRIBE,
            topic_id as usize,
            write_handle as usize,
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

/// Publish a payload to all subscribers of a topic.
/// Returns number of subscribers that accepted the payload.
pub fn topic_publish(topic_id: u32, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_TOPIC_PUBLISH,
            topic_id as usize,
            data.as_ptr() as usize,
            data.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}
