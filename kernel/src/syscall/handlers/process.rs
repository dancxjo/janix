//! Process lifecycle and task management syscalls

use crate::syscall::validate::validate_user_range;
use super::copyin;
use abi::errors::{Errno, SysResult};

pub fn sys_exit(code: i32) -> SysResult<usize> {
    crate::kprintln!("SYSCALL EXIT: code={}", code);
    unsafe {
        crate::task::scheduler::exit_current(code);
    }
    Ok(0)
}

pub fn sys_get_tid() -> SysResult<usize> {
    unsafe { Ok(crate::task::scheduler::current_tid_current() as usize) }
}

pub fn sys_spawn_thread(entry: usize, stack: usize) -> SysResult<usize> {
    validate_user_range(entry, 1, false)?;
    let stack_top = if stack == 0 {
        unsafe { crate::task::scheduler::alloc_user_stack_current(0) }.ok_or(Errno::ENOMEM)?
    } else {
        validate_user_range(stack, 1, true)?;
        stack
    };

    let tid = unsafe { crate::task::scheduler::spawn_user_thread_current(entry, stack_top, 0) };
    if let Some(tid) = tid {
        Ok(tid as usize)
    } else {
        Err(Errno::EAGAIN)
    }
}

pub fn sys_spawn_process(name_ptr: usize, name_len: usize, arg: usize) -> SysResult<usize> {
    if name_len > 128 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(name_ptr, name_len, false)?;
    let mut buf = [0u8; 128];
    unsafe {
        copyin(&mut buf[..name_len], name_ptr)?;
    }
    let name = core::str::from_utf8(&buf[..name_len]).map_err(|_| Errno::EINVAL)?;
    let tid = unsafe { crate::task::scheduler::spawn_process_current(name, arg) };
    if let Some(tid) = tid {
        Ok(tid as usize)
    } else {
        Err(Errno::ENOENT)
    }
}

pub fn sys_task_poll(pid: usize) -> SysResult<usize> {
    use abi::types::TaskStatus;

    let status_opt = unsafe { crate::task::scheduler::task_status_current(pid as u64) };

    if let Some((state, exit_code)) = status_opt {
        let (st, code) = match state {
            crate::task::TaskState::Runnable => (TaskStatus::Runnable, 0),
            crate::task::TaskState::Running => (TaskStatus::Running, 0),
            crate::task::TaskState::Blocked => (TaskStatus::Blocked, 0),
            crate::task::TaskState::Dead => (TaskStatus::Dead, exit_code.unwrap_or(0)),
        };

        let val = (st as u64) | ((code as u32 as u64) << 32);
        Ok(val as usize)
    } else {
        Err(Errno::ESRCH)
    }
}

pub fn sys_task_wait(tid: usize) -> SysResult<usize> {
    loop {
        let status_opt = unsafe { crate::task::scheduler::task_status_current(tid as u64) };
        
        match status_opt {
            Some((state, exit_code)) => {
                if state == crate::task::TaskState::Dead {
                    return Ok(exit_code.unwrap_or(0) as usize);
                }
                unsafe {
                    crate::task::scheduler::yield_now_current();
                }
            }
            None => {
                return Err(Errno::ECHILD);
            }
        }
    }
}
