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

/// Close a port handle
pub fn port_close(handle: PortHandle) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_CLOSE, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

/// Wait for any of the given port handles to become readable.
/// Returns the handle that became readable.
pub fn port_wait(handles: &[PortHandle]) -> Result<PortHandle, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_WAIT,
            handles.as_ptr() as usize,
            handles.len(),
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as PortHandle)
}
