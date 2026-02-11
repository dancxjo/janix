//! Pipe syscall handlers
//!
//! These are straight-line syscall handlers (not Root service messages).
//! Blocking uses the scheduler's thread parking primitives.

use crate::ipc::pipe;
use crate::syscall::validate::{copyin, copyout, validate_user_range};
use abi::errors::{Errno, SysResult};

/// SYS_PIPE_CREATE: Create an anonymous pipe.
///
/// Args: capacity (0 = default 4096), flags (NONBLOCK, CLOEXEC).
/// Returns: pipe_id (used for both read and write ends).
pub fn sys_pipe_create(capacity: usize, flags: usize) -> SysResult<usize> {
    let id = pipe::create(capacity as u32, flags as u32);
    Ok(id as usize)
}

/// SYS_PIPE_READ: Read from a pipe.
///
/// Args: pipe_id, buf_ptr, buf_len.
/// Returns: number of bytes read (0 = EOF).
pub fn sys_pipe_read(pipe_id: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    if buf_len == 0 {
        return Ok(0);
    }
    validate_user_range(buf_ptr, buf_len, true)?;

    // Use a kernel-side buffer to avoid holding pipe lock across user memory ops
    let read_len = buf_len.min(4096);
    let mut kbuf = [0u8; 4096];
    let n = pipe::read(pipe_id as u64, &mut kbuf[..read_len])?;
    if n > 0 {
        unsafe {
            copyout(buf_ptr, &kbuf[..n])?;
        }
    }
    Ok(n)
}

/// SYS_PIPE_WRITE: Write to a pipe.
///
/// Args: pipe_id, buf_ptr, buf_len.
/// Returns: number of bytes written.
pub fn sys_pipe_write(pipe_id: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    if buf_len == 0 {
        return Ok(0);
    }
    validate_user_range(buf_ptr, buf_len, false)?;

    let write_len = buf_len.min(4096);
    let mut kbuf = [0u8; 4096];
    unsafe {
        copyin(&mut kbuf[..write_len], buf_ptr)?;
    }
    let n = pipe::write(pipe_id as u64, &kbuf[..write_len])?;
    Ok(n)
}

/// SYS_PIPE_CLOSE: Close one end of a pipe.
///
/// Args: pipe_id, end (0 = read end, 1 = write end).
pub fn sys_pipe_close(pipe_id: usize, end: usize) -> SysResult<usize> {
    match end {
        0 => pipe::close_read(pipe_id as u64).map(|_| 0),
        1 => pipe::close_write(pipe_id as u64).map(|_| 0),
        _ => Err(Errno::EINVAL),
    }
}
