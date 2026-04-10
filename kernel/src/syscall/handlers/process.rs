//! Process lifecycle and task management syscalls

use super::copyin;
use crate::sched as scheduler;
use crate::sched::StdioSpec;
use crate::syscall::validate::validate_user_range;
use crate::task::StartupArg;
use abi::errors::{Errno, SysResult};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub fn sys_exit(code: i32) -> SysResult<usize> {
    if code != 0 {
        crate::kprintln!("SYSCALL EXIT: code={}", code);
    }
    unsafe {
        crate::sched::exit_current(code);
    }
    Ok(0)
}

pub fn sys_reboot() -> SysResult<usize> {
    crate::kprintln!("SYSCALL REBOOT: system reboot requested");
    crate::runtime_base().reboot();
}

pub fn sys_get_tid() -> SysResult<usize> {
    unsafe { Ok(crate::sched::current_tid_current() as usize) }
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

    let current_p = unsafe { crate::sched::current_priority_current() };

    let tid = unsafe {
        crate::sched::spawn_user_thread_current(
            req.entry,
            req.sp,
            StartupArg::Raw(req.arg),
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
    let tid = unsafe { crate::sched::spawn_process_current(name, StartupArg::Raw(arg)) };
    if let Some(tid) = tid {
        Ok(tid as usize)
    } else {
        Err(Errno::ENOENT)
    }
}

pub fn sys_task_poll(pid: usize) -> SysResult<usize> {
    use abi::types::TaskStatus;

    let status_opt = unsafe { crate::sched::task_status_current(pid as u64) };

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
    unsafe { crate::sched::task_wait_current(tid as u64) }.map(|code| code as usize)
}

/// `SYS_WAITPID`: Wait for a child process to exit and retrieve its exit status.
///
/// Arguments:
/// - `pid`:  `i64` cast as `usize`.  `pid > 0` waits for the specific child;
///           `pid == -1` (or `0`) waits for any child.
/// - `status_ptr`: optional pointer to an `i32` that receives the exit code.
///                 Pass `0` to discard.
/// - `flags`: `waitpid_flags::WNOHANG` (1) for non-blocking poll.
///
/// Returns the child PID on success, `0` when `WNOHANG` and no child exited,
/// or a negative errno on error.
pub fn sys_waitpid(pid: usize, status_ptr: usize, flags: usize) -> SysResult<usize> {
    let pid = pid as isize as i64;
    let flags = flags as u32;

    if status_ptr != 0 {
        validate_user_range(status_ptr, core::mem::size_of::<i32>(), true)?;
    }

    let (child_pid, exit_code) = unsafe { crate::sched::waitpid_current(pid, flags) }?;

    if status_ptr != 0 {
        unsafe {
            super::copyout(status_ptr, &exit_code.to_le_bytes())?;
        }
    }

    Ok(child_pid as usize)
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
        crate::sched::set_priority_current(tid as u64, p);
    }
    Ok(0)
}

pub fn sys_task_kill(tid: usize) -> SysResult<usize> {
    let killed = unsafe { crate::sched::kill_by_tid_current(tid as u64) };
    if killed { Ok(0) } else { Err(Errno::ESRCH) }
}

pub fn sys_task_interrupt(tid: usize) -> SysResult<usize> {
    crate::sched::interrupt_task_current(tid as u64)?;
    Ok(0)
}

pub fn sys_task_dump() -> SysResult<usize> {
    crate::sched::dump_stats_current();
    Ok(0)
}

pub fn sys_getpid() -> SysResult<usize> {
    let pinfo = crate::sched::process_info_current();
    let pid = pinfo.map(|p| p.lock().pid).unwrap_or(0);
    Ok(pid as usize)
}

pub fn sys_getppid() -> SysResult<usize> {
    let pinfo = crate::sched::process_info_current();
    let ppid = pinfo.map(|p| p.lock().ppid).unwrap_or(0);
    Ok(ppid as usize)
}

/// Serialize argv into a userspace buffer.
/// Format: count: u32, then for each arg: len: u32, bytes...
/// Returns: total bytes needed (caller can retry with bigger buffer if it was too small).
pub fn sys_argv_get(buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    let pinfo = crate::sched::process_info_current();
    let pinfo = pinfo.ok_or(Errno::ENOENT)?;
    let lock = pinfo.lock();

    // Calculate total size needed
    let mut total = 4u32; // count
    for arg in &lock.argv {
        total += 4 + arg.len() as u32; // len + bytes
    }

    if buf_ptr == 0 || buf_len == 0 {
        // Just return the needed size
        return Ok(total as usize);
    }

    // Serialize into a kernel buffer, then copyout
    let copy_len = (buf_len as u32).min(total) as usize;
    let mut out = alloc::vec![0u8; copy_len];
    let mut pos = 0usize;

    // Write count
    let count = lock.argv.len() as u32;
    if pos + 4 <= copy_len {
        out[pos..pos + 4].copy_from_slice(&count.to_le_bytes());
    }
    pos += 4;

    for arg in &lock.argv {
        let len = arg.len() as u32;
        if pos + 4 <= copy_len {
            out[pos..pos + 4].copy_from_slice(&len.to_le_bytes());
        }
        pos += 4;
        let arg_copy = (arg.len()).min(copy_len.saturating_sub(pos));
        if arg_copy > 0 && pos < copy_len {
            out[pos..pos + arg_copy].copy_from_slice(&arg[..arg_copy]);
        }
        pos += arg.len();
    }

    drop(lock);

    validate_user_range(buf_ptr, copy_len, true)?;
    unsafe {
        super::copyout(buf_ptr, &out[..copy_len])?;
    }
    Ok(total as usize)
}

pub fn sys_env_get(
    key_ptr: usize,
    key_len: usize,
    val_ptr: usize,
    val_len: usize,
) -> SysResult<usize> {
    if key_len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(key_ptr, key_len, false)?;
    let mut key = alloc::vec![0u8; key_len];
    unsafe {
        copyin(&mut key, key_ptr)?;
    }

    let pinfo = crate::sched::process_info_current();
    let pinfo = pinfo.ok_or(Errno::ENOENT)?;
    let lock = pinfo.lock();

    let val = lock.env.get(&key).ok_or(Errno::ENOENT)?;
    let needed = val.len();

    if val_ptr != 0 && val_len > 0 {
        let copy_len = val_len.min(needed);
        validate_user_range(val_ptr, copy_len, true)?;
        unsafe {
            super::copyout(val_ptr, &val[..copy_len])?;
        }
    }
    Ok(needed)
}

pub fn sys_env_set(
    key_ptr: usize,
    key_len: usize,
    val_ptr: usize,
    val_len: usize,
) -> SysResult<usize> {
    if key_len > 4096 || val_len > 65536 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(key_ptr, key_len, false)?;
    validate_user_range(val_ptr, val_len, false)?;

    let mut key = alloc::vec![0u8; key_len];
    let mut val = alloc::vec![0u8; val_len];
    unsafe {
        copyin(&mut key, key_ptr)?;
        copyin(&mut val, val_ptr)?;
    }

    let pinfo = crate::sched::process_info_current();
    let pinfo = pinfo.ok_or(Errno::ENOENT)?;
    pinfo.lock().env.insert(key, val);
    Ok(0)
}

pub fn sys_env_unset(key_ptr: usize, key_len: usize) -> SysResult<usize> {
    if key_len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(key_ptr, key_len, false)?;
    let mut key = alloc::vec![0u8; key_len];
    unsafe {
        copyin(&mut key, key_ptr)?;
    }

    let pinfo = crate::sched::process_info_current();
    let pinfo = pinfo.ok_or(Errno::ENOENT)?;
    pinfo.lock().env.remove(&key);
    Ok(0)
}

/// Serialize all env vars into a userspace buffer.
/// Format: count: u32, then for each: key_len: u32, key_bytes, val_len: u32, val_bytes
/// Returns: total bytes needed.
pub fn sys_env_list(buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    let info = scheduler::process_info_current().ok_or(Errno::ENOENT)?;
    let pi = info.lock();

    let mut blob = Vec::new();
    let count = pi.env.len() as u32;
    blob.extend_from_slice(&count.to_le_bytes());
    for (k, v) in pi.env.iter() {
        blob.extend_from_slice(&(k.len() as u32).to_le_bytes());
        blob.extend_from_slice(k);
        blob.extend_from_slice(&(v.len() as u32).to_le_bytes());
        blob.extend_from_slice(v);
    }

    let needed = blob.len();
    if buf_len > 0 && buf_ptr != 0 {
        let copy_len = needed.min(buf_len);
        validate_user_range(buf_ptr, copy_len, true)?;
        unsafe {
            super::copyout(buf_ptr, &blob[..copy_len])?;
        }
    }
    Ok(needed)
}

/// SYS_SPAWN_PROCESS_EX handler.
/// Args: req_ptr = pointer to SpawnProcessExReq, resp_ptr = pointer to SpawnProcessExResp.
pub fn sys_spawn_process_ex(req_ptr: usize, resp_ptr: usize) -> SysResult<usize> {
    use abi::types::{SpawnProcessExReq, SpawnProcessExResp, stdio_mode};

    // Copy in the request struct
    validate_user_range(req_ptr, core::mem::size_of::<SpawnProcessExReq>(), false)?;
    let mut req = SpawnProcessExReq::default();
    unsafe {
        core::ptr::copy_nonoverlapping(
            req_ptr as *const u8,
            &mut req as *mut SpawnProcessExReq as *mut u8,
            core::mem::size_of::<SpawnProcessExReq>(),
        );
    }

    // Copy in the program name
    let name_len = req.name_len as usize;
    if name_len == 0 || name_len > 256 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(req.name_ptr as usize, name_len, false)?;
    let mut name_bytes = alloc::vec![0u8; name_len];
    unsafe {
        copyin(&mut name_bytes, req.name_ptr as usize)?;
    }
    let name = core::str::from_utf8(&name_bytes).map_err(|_| Errno::EINVAL)?;

    // Deserialize argv blob
    let argv = if req.argv_len > 0 && req.argv_ptr != 0 {
        let alen = req.argv_len as usize;
        validate_user_range(req.argv_ptr as usize, alen, false)?;
        let mut blob = alloc::vec![0u8; alen];
        unsafe {
            copyin(&mut blob, req.argv_ptr as usize)?;
        }
        deserialize_argv(&blob)?
    } else {
        Vec::new()
    };

    // Deserialize env blob
    let env = if req.env_len > 0 && req.env_ptr != 0 {
        let elen = req.env_len as usize;
        validate_user_range(req.env_ptr as usize, elen, false)?;
        let mut blob = alloc::vec![0u8; elen];
        unsafe {
            copyin(&mut blob, req.env_ptr as usize)?;
        }
        deserialize_env(&blob)?
    } else {
        BTreeMap::new()
    };

    // Translate stdio modes
    let stdin_spec = mode_to_spec(req.stdin_mode)?;
    let stdout_spec = mode_to_spec(req.stdout_mode)?;
    let stderr_spec = match req.stderr_mode {
        1 => scheduler::StdioSpec::Inherit,
        _ => scheduler::StdioSpec::Null,
    };

    let boot_arg = req.boot_arg;

    let mut inherited_handles = Vec::with_capacity(req.num_inherited_handles as usize);
    for i in 0..req.num_inherited_handles as usize {
        if i < 8 {
            inherited_handles.push(req.handles_to_inherit[i]);
        }
    }

    let result = unsafe {
        scheduler::spawn_process_ex_current(
            name,
            argv,
            env,
            stdin_spec,
            stdout_spec,
            stderr_spec,
            boot_arg,
            inherited_handles,
        )
    }?;

    // Write the response
    unsafe {
        *(resp_ptr as *mut SpawnProcessExResp) = SpawnProcessExResp {
            child_tid: result.child_tid as u64,
            stdin_pipe: result.stdin_pipe,
            stdout_pipe: result.stdout_pipe,
            stderr_pipe: result.stderr_pipe,
        };
    }

    Ok(result.child_tid as usize)
}

fn mode_to_spec(mode: u32) -> Result<StdioSpec, Errno> {
    use abi::types::stdio_mode;
    match mode {
        stdio_mode::INHERIT => Ok(StdioSpec::Inherit),
        stdio_mode::NULL => Ok(StdioSpec::Null),
        stdio_mode::PIPE => Ok(StdioSpec::Pipe),
        _ => Err(Errno::EINVAL),
    }
}

/// Deserialize an argv blob: count:u32, then (len:u32, bytes...) per arg.
fn deserialize_argv(blob: &[u8]) -> Result<Vec<Vec<u8>>, Errno> {
    if blob.len() < 4 {
        return Err(Errno::EINVAL);
    }
    let count = u32::from_le_bytes(blob[..4].try_into().unwrap()) as usize;
    let mut offset = 4;
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        if offset + 4 > blob.len() {
            return Err(Errno::EINVAL);
        }
        let len = u32::from_le_bytes(blob[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        if offset + len > blob.len() {
            return Err(Errno::EINVAL);
        }
        result.push(blob[offset..offset + len].to_vec());
        offset += len;
    }
    Ok(result)
}

/// Deserialize an env blob: count:u32, then (klen:u32, key, vlen:u32, val) per entry.
fn deserialize_env(blob: &[u8]) -> Result<BTreeMap<Vec<u8>, Vec<u8>>, Errno> {
    if blob.len() < 4 {
        return Err(Errno::EINVAL);
    }
    let count = u32::from_le_bytes(blob[..4].try_into().unwrap()) as usize;
    let mut offset = 4;
    let mut result = BTreeMap::new();
    for _ in 0..count {
        if offset + 4 > blob.len() {
            return Err(Errno::EINVAL);
        }
        let klen = u32::from_le_bytes(blob[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        if offset + klen > blob.len() {
            return Err(Errno::EINVAL);
        }
        let key = blob[offset..offset + klen].to_vec();
        offset += klen;
        if offset + 4 > blob.len() {
            return Err(Errno::EINVAL);
        }
        let vlen = u32::from_le_bytes(blob[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        if offset + vlen > blob.len() {
            return Err(Errno::EINVAL);
        }
        let val = blob[offset..offset + vlen].to_vec();
        offset += vlen;
        result.insert(key, val);
    }
    Ok(result)
}

/// `SYS_TASK_SET_TLS_BASE`: Set the calling thread's user TLS base (FS_BASE on x86_64).
///
/// The new base takes effect immediately in hardware and is preserved across
/// context switches.  On architectures without a dedicated user TLS register
/// this call succeeds silently (no-op).
///
/// Returns `EINVAL` if `base` is a non-canonical address on x86_64 (bits
/// 63:47 must all be identical — all 0 or all 1).
pub fn sys_task_set_tls_base(base: usize) -> SysResult<usize> {
    let base = base as u64;

    // Validate canonical address: shift right by 47 to get bits [63:47] (17 bits).
    // A canonical address has them all 0 (user low half) or all 1 (kernel high half).
    // Non-canonical addresses trigger a #GP on the first FS-relative user access.
    let sign_bits = base >> 47;
    if sign_bits != 0 && sign_bits != 0x1_FFFF {
        return Err(abi::errors::Errno::EINVAL);
    }

    // Update the hardware register immediately.
    crate::runtime_base().set_user_tls_base_dyn(base);

    // Also persist into the current task's record so context-switch save/restore
    // starts with the correct value even if the task has never been switched out.
    unsafe {
        crate::sched::set_current_user_fs_base_current(base);
    }

    Ok(0)
}

/// `SYS_TASK_GET_TLS_BASE`: Return the calling thread's user TLS base.
///
/// Reads the live hardware register so the value is always current,
/// regardless of whether `task_set_tls_base` was used.
pub fn sys_task_get_tls_base() -> SysResult<usize> {
    let base = crate::runtime_base().get_user_tls_base_dyn();
    Ok(base as usize)
}

pub fn sys_task_exec(
    fd: u32,
    argv_ptr: usize,
    argv_len: usize,
    envp_ptr: usize,
    envp_len: usize,
) -> SysResult<usize> {
    // Deserialize argv
    let argv = if argv_ptr != 0 && argv_len > 0 {
        validate_user_range(argv_ptr, argv_len, false)?;
        let mut blob = alloc::vec![0u8; argv_len];
        unsafe {
            copyin(&mut blob, argv_ptr)?;
        }
        deserialize_argv(&blob)?
    } else {
        Vec::new()
    };

    // Deserialize env
    let env = if envp_ptr != 0 && envp_len > 0 {
        validate_user_range(envp_ptr, envp_len, false)?;
        let mut blob = alloc::vec![0u8; envp_len];
        unsafe {
            copyin(&mut blob, envp_ptr)?;
        }
        deserialize_env(&blob)?
    } else {
        BTreeMap::new()
    };

    // Call the internal exec helper via erased hook
    unsafe {
        scheduler::task_exec_current(fd, argv, env)?;
    }

    // If task_exec_current returns, it means it failed.
    // However, on success it should NEVER return.
    unreachable!()
}
