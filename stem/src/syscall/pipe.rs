//! Stem syscall wrappers for anonymous pipes.

use abi::errors::Errno;
use abi::syscall::*;

use super::arch::raw_syscall6;

/// Create an anonymous pipe.
///
/// Returns `(pipe_id_read, pipe_id_write)` — currently both are the same ID;
/// the kernel distinguishes read vs write by which syscall is used.
pub fn pipe_create(capacity: u32, flags: u32) -> Result<(u64, u64), Errno> {
    let ret =
        unsafe { raw_syscall6(SYS_PIPE_CREATE, capacity as usize, flags as usize, 0, 0, 0, 0) };
    let id = abi::errors::errno(ret)? as u64;
    Ok((id, id))
}

/// Read from a pipe. Blocks until data is available or EOF.
pub fn pipe_read(pipe_id: u64, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PIPE_READ,
            pipe_id as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Write to a pipe. Blocks until space is available or pipe is broken.
pub fn pipe_write(pipe_id: u64, buf: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PIPE_WRITE,
            pipe_id as usize,
            buf.as_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Close one end of a pipe.
///
/// `end`: 0 = read end, 1 = write end.
pub fn pipe_close(pipe_id: u64, end: u32) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PIPE_CLOSE,
            pipe_id as usize,
            end as usize,
            0,
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}
