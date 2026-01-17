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

pub fn sys_spawn_thread(req_ptr: usize, _unused: usize) -> SysResult<usize> {
    use abi::types::SpawnThreadReq;

    let size = core::mem::size_of::<SpawnThreadReq>();
    validate_user_range(req_ptr, size, false)?;

    let mut req: SpawnThreadReq = unsafe { core::mem::zeroed() };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut req as *mut _ as *mut u8, size) };
    unsafe {
        copyin(slice, req_ptr)?;
    }

    validate_user_range(req.entry, 1, false)?;
    validate_user_range(req.sp, 1, true)?;
    if req.stack.guard_end != req.stack.reserve_start {
        return Err(Errno::EINVAL);
    }
    if req.stack.reserve_start >= req.stack.reserve_end {
        return Err(Errno::EINVAL);
    }
    if req.stack.committed_start < req.stack.reserve_start
        || req.stack.committed_start > req.stack.reserve_end
    {
        return Err(Errno::EINVAL);
    }
    if req.sp > req.stack.reserve_end || req.sp < req.stack.committed_start {
        return Err(Errno::EINVAL);
    }
    if req.stack.grow_chunk_bytes == 0 {
        return Err(Errno::EINVAL);
    }

    let current_p = unsafe { crate::task::scheduler::current_priority_current() };

    let tid = unsafe {
        crate::task::scheduler::spawn_user_thread_current(
            req.entry,
            req.sp,
            0,
            req.stack,
            current_p,
        )
    };
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

pub fn sys_set_priority(tid: usize, priority: usize) -> SysResult<usize> {
    if priority > 4 {
        return Err(Errno::EINVAL);
    }
    let p = match priority {
        0 => crate::task::TaskPriority::Idle,
        1 => crate::task::TaskPriority::Low,
        2 => crate::task::TaskPriority::Normal,
        3 => crate::task::TaskPriority::High,
        4 => crate::task::TaskPriority::Realtime,
        _ => unreachable!(),
    };
    unsafe {
        crate::task::scheduler::set_priority_current(tid as u64, p);
    }
    Ok(0)
}
