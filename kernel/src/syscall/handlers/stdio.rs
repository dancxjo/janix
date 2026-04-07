use crate::sched;
use crate::syscall::validate::{copyout, validate_user_range};
use crate::task::{StdioBinding, StdioPipeMode};
use abi::errors::{Errno, SysResult};
use alloc::vec;

fn current_stdio_binding(fd: usize) -> Result<StdioBinding, Errno> {
    if fd > 2 {
        return Err(Errno::EBADF);
    }
    let pinfo = sched::process_info_current().ok_or(Errno::ENOENT)?;
    Ok(pinfo.lock().stdio[fd])
}

pub fn sys_read(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    validate_user_range(buf_ptr, buf_len, true)?;
    if buf_len == 0 {
        return Ok(0);
    }

    match current_stdio_binding(fd)? {
        StdioBinding::Null => Ok(0),
        StdioBinding::Console => {
            let pinfo = sched::process_info_current().ok_or(Errno::ENOENT)?;
            let mut out = vec![0u8; buf_len];

            loop {
                let read = {
                    let mut lock = pinfo.lock();
                    let mut count = 0usize;
                    while count < out.len() {
                        match lock.console_stdin.pop_front() {
                            Some(byte) => {
                                out[count] = byte;
                                count += 1;
                            }
                            None => break,
                        }
                    }
                    count
                };

                if read > 0 {
                    unsafe {
                        copyout(buf_ptr, &out[..read])?;
                    }
                    return Ok(read);
                }

                unsafe {
                    crate::sched::yield_now_current();
                }
            }
        }
        StdioBinding::Pipe { pipe_id, mode } => match mode {
            StdioPipeMode::Read => {
                let mut out = vec![0u8; buf_len];
                let read = crate::ipc::pipe::read(pipe_id, &mut out)?;
                unsafe {
                    copyout(buf_ptr, &out[..read])?;
                }
                Ok(read)
            }
            StdioPipeMode::Write => Err(Errno::EBADF),
        },
    }
}

pub fn sys_write(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    use crate::syscall::validate::copyin;

    validate_user_range(buf_ptr, buf_len, false)?;
    if buf_len == 0 {
        return Ok(0);
    }

    let mut data = vec![0u8; buf_len];
    unsafe {
        copyin(&mut data, buf_ptr)?;
    }

    match current_stdio_binding(fd)? {
        StdioBinding::Null => Ok(buf_len),
        StdioBinding::Console => {
            let rt = crate::runtime_base();
            for &b in &data {
                rt.putchar(b);
            }
            Ok(buf_len)
        }
        StdioBinding::Pipe { pipe_id, mode } => match mode {
            StdioPipeMode::Write => crate::ipc::pipe::write(pipe_id, &data),
            StdioPipeMode::Read => Err(Errno::EBADF),
        },
    }
}
