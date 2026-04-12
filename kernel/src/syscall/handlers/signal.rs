//! POSIX signal syscall handlers.
//!
//! Implements:
//!   SYS_KILL, SYS_SIGACTION, SYS_SIGPROCMASK, SYS_SIGPENDING,
//!   SYS_SIGSUSPEND, SYS_SIGRETURN, SYS_ALARM, SYS_PAUSE.

use abi::errors::{Errno, SysResult};
use abi::signal::{
    sa_flags, sig_handler, sig_how, SigAction, SigSet, SIGKILL, SIGSTOP,
};

use crate::syscall::validate::{copyin, validate_user_range};

// ─── kill ────────────────────────────────────────────────────────────────────

/// `kill(pid, sig)` — send signal `sig` to process `pid`.
///
/// Permission model (simplified):
/// - Any process may send signals to itself.
/// - Any process may send signals to any other process it could have spawned
///   (currently: any process, since uid tracking is not yet implemented).
/// - `sig == 0` is an existence check; no signal is delivered.
pub fn sys_kill(pid: usize, sig: usize) -> SysResult<usize> {
    let sig = sig as u32;
    let pid = pid as i32;

    if sig >= abi::signal::NSIG {
        return Err(Errno::EINVAL);
    }

    if pid <= 0 {
        // Process-group and broadcast kill are not yet implemented.
        return Err(Errno::ESRCH);
    }

    let target_pid = pid as u32;

    // Find the target process.
    let procs = crate::sched::list_processes_current();
    let found = procs.iter().any(|p| p.pid == target_pid);
    if !found {
        return Err(Errno::ESRCH);
    }

    // Existence check only.
    if sig == 0 {
        return Ok(0);
    }

    // Deliver the signal.
    if let Some(pi) = crate::sched::process_info_for_tid_current(target_pid as u64) {
        crate::signal::deliver::send_to_process(&pi, sig);
        Ok(0)
    } else {
        Err(Errno::ESRCH)
    }
}

// ─── sigaction ───────────────────────────────────────────────────────────────

/// `sigaction(sig, act, oldact)` — examine and change a signal action.
pub fn sys_sigaction(sig: usize, act_ptr: usize, oldact_ptr: usize) -> SysResult<usize> {
    let sig = sig as u32;

    if sig == 0 || sig >= abi::signal::NSIG {
        return Err(Errno::EINVAL);
    }
    // SIGKILL and SIGSTOP cannot be caught or ignored.
    if sig == SIGKILL || sig == SIGSTOP {
        return Err(Errno::EINVAL);
    }

    let pi = crate::sched::process_info_current().ok_or(Errno::ESRCH)?;

    // Read old disposition first.
    let old: SigAction = {
        let p = pi.lock();
        p.signal_dispositions[(sig - 1) as usize]
    };

    // Write old disposition to userspace if requested.
    if oldact_ptr != 0 {
        let size = core::mem::size_of::<SigAction>();
        validate_user_range(oldact_ptr, size, true)?;
        unsafe {
            let dst = oldact_ptr as *mut SigAction;
            dst.write_volatile(old);
        }
    }

    // Install new disposition if provided.
    if act_ptr != 0 {
        let size = core::mem::size_of::<SigAction>();
        validate_user_range(act_ptr, size, false)?;
        let mut new_act: SigAction = unsafe { core::mem::zeroed() };
        let slice = unsafe {
            core::slice::from_raw_parts_mut(&mut new_act as *mut _ as *mut u8, size)
        };
        unsafe { copyin(slice, act_ptr)? };

        // Validate handler value.
        if new_act.handler != sig_handler::SIG_DFL
            && new_act.handler != sig_handler::SIG_IGN
        {
            validate_user_range(new_act.handler, 1, false)?;
        }

        // Mask out SIGKILL / SIGSTOP from the sa_mask.
        new_act.mask.remove(SIGKILL);
        new_act.mask.remove(SIGSTOP);

        let mut p = pi.lock();
        p.signal_dispositions[(sig - 1) as usize] = new_act;
    }

    Ok(0)
}

// ─── sigprocmask ─────────────────────────────────────────────────────────────

/// `sigprocmask(how, set, oldset)` — examine and change the signal mask.
pub fn sys_sigprocmask(how: usize, set_ptr: usize, oldset_ptr: usize) -> SysResult<usize> {
    let how = how as u32;

    // Read current mask.
    let (cur_blocked, _) = crate::sched::hooks::get_thread_signal_current();

    // Write old mask to userspace if requested.
    if oldset_ptr != 0 {
        validate_user_range(oldset_ptr, core::mem::size_of::<SigSet>(), true)?;
        unsafe {
            (oldset_ptr as *mut SigSet).write_volatile(cur_blocked);
        }
    }

    if set_ptr != 0 {
        validate_user_range(set_ptr, core::mem::size_of::<SigSet>(), false)?;
        let mut new_set: SigSet = unsafe { core::mem::zeroed() };
        unsafe {
            copyin(
                core::slice::from_raw_parts_mut(
                    &mut new_set as *mut _ as *mut u8,
                    core::mem::size_of::<SigSet>(),
                ),
                set_ptr,
            )?
        };

        // SIGKILL and SIGSTOP can never be blocked.
        new_set.remove(SIGKILL);
        new_set.remove(SIGSTOP);

        let new_blocked = match how {
            sig_how::SIG_BLOCK => cur_blocked.union(new_set),
            sig_how::SIG_UNBLOCK => cur_blocked.difference(new_set),
            sig_how::SIG_SETMASK => new_set,
            _ => return Err(Errno::EINVAL),
        };

        crate::sched::hooks::set_thread_blocked_current(new_blocked);
    }

    Ok(0)
}

// ─── sigpending ──────────────────────────────────────────────────────────────

/// `sigpending(set)` — return the set of pending signals.
pub fn sys_sigpending(set_ptr: usize) -> SysResult<usize> {
    validate_user_range(set_ptr, core::mem::size_of::<SigSet>(), true)?;

    let (blocked, thread_pending) = crate::sched::hooks::get_thread_signal_current();
    let proc_pending = crate::sched::process_info_current()
        .map(|pi| pi.lock().pending_signals)
        .unwrap_or(SigSet::EMPTY);

    // Pending signals are those that are blocked (would be delivered if unblocked).
    let pending = thread_pending.union(proc_pending).intersect(blocked);

    unsafe {
        (set_ptr as *mut SigSet).write_volatile(pending);
    }
    Ok(0)
}

// ─── sigsuspend ──────────────────────────────────────────────────────────────

/// `sigsuspend(mask)` — atomically replace the mask and wait for a signal.
pub fn sys_sigsuspend(mask_ptr: usize) -> SysResult<usize> {
    validate_user_range(mask_ptr, core::mem::size_of::<SigSet>(), false)?;
    let mut mask: SigSet = unsafe { core::mem::zeroed() };
    unsafe {
        copyin(
            core::slice::from_raw_parts_mut(
                &mut mask as *mut _ as *mut u8,
                core::mem::size_of::<SigSet>(),
            ),
            mask_ptr,
        )?
    };
    mask.remove(SIGKILL);
    mask.remove(SIGSTOP);

    // Save old mask and install temporary one.
    crate::sched::hooks::save_sigsuspend_mask_current();
    crate::sched::hooks::set_thread_blocked_current(mask);

    // Block until a signal arrives.
    unsafe { crate::sched::block_current_erased() };

    // Mask is restored by kernel_signal_check when it delivers the signal.
    Err(Errno::EINTR)
}

// ─── sigreturn ───────────────────────────────────────────────────────────────

/// `sigreturn()` — return from a signal handler.
///
/// This syscall does not use the normal return path; the architecture-specific
/// assembly stub calls [`crate::syscall::signal::sigreturn_impl`] directly with
/// a pointer to the trap frame so that the frame can be overwritten with the
/// saved register state from `SigFrame`.
///
/// The stub should call `kernel_sigreturn(frame_ptr)` instead of going through
/// the normal dispatch.  We still provide a dispatch entry for completeness.
pub fn sys_sigreturn() -> SysResult<usize> {
    // The real work is done by the arch stub.  If we get here without the
    // frame pointer being available, just return -EINTR.
    Err(Errno::EINTR)
}

// ─── alarm ───────────────────────────────────────────────────────────────────

/// `alarm(seconds)` — schedule SIGALRM.
///
/// Returns the number of seconds remaining on any previous alarm.
pub fn sys_alarm(seconds: usize) -> SysResult<usize> {
    const TICKS_PER_SEC: u64 = 1000; // 1 kHz timer assumed

    let pi = crate::sched::process_info_current().ok_or(Errno::ESRCH)?;
    let now = crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);

    let mut p = pi.lock();
    let old_deadline = p.alarm_deadline;
    let remaining = if old_deadline > now {
        (old_deadline - now + TICKS_PER_SEC - 1) / TICKS_PER_SEC
    } else {
        0
    };

    if seconds == 0 {
        p.alarm_deadline = 0;
    } else {
        p.alarm_deadline = now + (seconds as u64) * TICKS_PER_SEC;
    }

    Ok(remaining as usize)
}

// ─── pause ───────────────────────────────────────────────────────────────────

/// `pause()` — wait for a signal.
pub fn sys_pause() -> SysResult<usize> {
    // Block until kernel_signal_check wakes us.
    unsafe { crate::sched::block_current_erased() };
    Err(Errno::EINTR)
}
