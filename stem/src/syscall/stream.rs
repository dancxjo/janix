use crate::syscall::arch::raw_syscall6;
use abi::errors::Errno;
use abi::syscall::*;

pub fn stream_listen(node_id: usize, port_handle: u32) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_STREAM_LISTEN, node_id, port_handle as usize, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

pub fn stream_open(node_id: usize) -> Result<(u32, u32), Errno> {
    let ret = unsafe { raw_syscall6(SYS_STREAM_OPEN, node_id, 0, 0, 0, 0, 0) };
    let val = abi::errors::errno(ret)?;
    let write = ((val >> 16) & 0xFFFF) as u32;
    let read = (val & 0xFFFF) as u32;
    Ok((write, read))
}

pub fn stream_read(handle: u32, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_STREAM_READ,
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

pub fn stream_poll(handle: u32) -> Result<bool, Errno> {
    let ret = unsafe { raw_syscall6(SYS_STREAM_POLL, handle as usize, 0, 0, 0, 0, 0) };
    // Should return 1 if readable, 0 if not
    abi::errors::errno(ret).map(|v| v > 0)
}
