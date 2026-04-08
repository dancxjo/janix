//! Channel IPC syscall wrappers for userspace

use crate::syscall::arch::raw_syscall6;
use abi::errors::Errno;
use abi::syscall::*;

/// A handle to a channel endpoint for IPC.
pub type ChannelHandle = u32;

/// Create a new channel pair (returns packed read/write handles).
/// Result: (write_handle << 16) | read_handle
pub fn channel_create(capacity: usize) -> Result<(ChannelHandle, ChannelHandle), Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_CREATE, capacity, 0, 0, 0, 0, 0) };
    let val = abi::errors::errno(ret)?;
    let write_handle = ((val >> 16) & 0xFFFF) as ChannelHandle;
    let read_handle = (val & 0xFFFF) as ChannelHandle;
    Ok((write_handle, read_handle))
}

pub fn channel_send(handle: ChannelHandle, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_SEND,
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

pub fn channel_send_all(handle: ChannelHandle, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_SEND_ALL,
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

pub fn channel_recv(handle: ChannelHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_RECV,
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

pub fn channel_try_recv(handle: ChannelHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_TRY_RECV,
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

pub fn channel_close(handle: ChannelHandle) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_CLOSE, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

pub fn channel_wait(handles: &[ChannelHandle], flags: u32) -> Result<ChannelHandle, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_WAIT,
            handles.as_ptr() as usize,
            handles.len(),
            flags as usize,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|v| v as ChannelHandle)
}

pub fn channel_len(handle: ChannelHandle) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_INFO, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v & 0xFFFFFFFF) as usize)
}

pub fn channel_capacity(handle: ChannelHandle) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_INFO, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v >> 32) as usize)
}
