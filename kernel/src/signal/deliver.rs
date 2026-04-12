//! Signal delivery logic.
//!
//! The main entry point [`kernel_signal_check`] is called from the
//! architecture-specific syscall assembly stub (via `call kernel_signal_check`)
//! immediately before returning to user mode.  It is type-erased: all
//! access to scheduler internals goes through the hook table in
//! `crate::sched::hooks` (same pattern used by the rest of the kernel).

use abi::signal::{
    sa_flags, sig_handler, SigAction, SigSet, SIGCONT, SIGKILL, SIGSTOP, SIGTSTP, SIGTTIN,
    SIGTTOU,
};
use alloc::sync::Arc;
use spin::Mutex;

use crate::task::{Process, TaskId};

// ─── Default action classification ──────────────────────────────────────────

/// The POSIX default action for a signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultAction {
    /// Terminate the process.
    Terminate,
    /// Terminate the process and create a core-dump placeholder.
    CoreDump,
    /// Ignore the signal entirely.
    Ignore,
    /// Stop all threads in the process.
    Stop,
    /// Continue a stopped process.
    Continue,
}

/// Return the POSIX default action for signal `sig` (1-based).
pub fn default_action(sig: u32) -> DefaultAction {
    use abi::signal::*;
    match sig {
        SIGHUP => DefaultAction::Terminate,
        SIGINT => DefaultAction::Terminate,
        SIGQUIT => DefaultAction::CoreDump,
        SIGILL => DefaultAction::CoreDump,
        SIGTRAP => DefaultAction::CoreDump,
        SIGABRT => DefaultAction::CoreDump,
        SIGBUS => DefaultAction::CoreDump,
        SIGFPE => DefaultAction::CoreDump,
        SIGKILL => DefaultAction::Terminate,
        SIGUSR1 => DefaultAction::Terminate,
        SIGSEGV => DefaultAction::CoreDump,
        SIGUSR2 => DefaultAction::Terminate,
        SIGPIPE => DefaultAction::Terminate,
        SIGALRM => DefaultAction::Terminate,
        SIGTERM => DefaultAction::Terminate,
        SIGSTKFLT => DefaultAction::Terminate,
        SIGCHLD => DefaultAction::Ignore,
        SIGCONT => DefaultAction::Continue,
        SIGSTOP => DefaultAction::Stop,
        SIGTSTP => DefaultAction::Stop,
        SIGTTIN => DefaultAction::Stop,
        SIGTTOU => DefaultAction::Stop,
        SIGURG => DefaultAction::Ignore,
        SIGXCPU => DefaultAction::CoreDump,
        SIGXFSZ => DefaultAction::CoreDump,
        SIGVTALRM => DefaultAction::Terminate,
        SIGPROF => DefaultAction::Terminate,
        SIGWINCH => DefaultAction::Ignore,
        SIGIO => DefaultAction::Terminate,
        SIGPWR => DefaultAction::Terminate,
        SIGSYS => DefaultAction::CoreDump,
        _ => DefaultAction::Terminate,
    }
}

// ─── Architecture-specific frame injection hook ──────────────────────────────

/// Architecture hook: inject a signal frame onto the user stack and redirect
/// execution to the signal handler.
///
/// Set by the `bran` crate for each supported architecture.
///
/// # Parameters
/// - `frame_ptr`  — raw pointer to the arch-specific `UserTrapFrame`.
/// - `signum`     — signal number being delivered.
/// - `handler`    — user-space handler address.
/// - `saved_mask` — the blocked-signal mask to save in the frame.
///
/// Returns `true` on success, `false` if the user stack is inaccessible.
pub static mut INJECT_FRAME_HOOK: Option<
    unsafe fn(frame_ptr: *mut u8, signum: u32, handler: usize, saved_mask: SigSet) -> bool,
> = None;

// ─── Type-erased signal-state hooks (set by sched::init) ────────────────────

/// Get the (blocked, pending) signal sets for the current thread.
pub(crate) static mut GET_THREAD_SIGNAL_HOOK: Option<fn() -> (SigSet, SigSet)> = None;

/// Atomically set the blocked mask for the current thread and return the old mask.
pub(crate) static mut SET_THREAD_BLOCKED_HOOK: Option<fn(SigSet) -> SigSet> = None;

/// Remove a signal from the current thread's thread-level pending set and
/// return whether it was there (vs. in the process-level pending set).
pub(crate) static mut CLEAR_THREAD_PENDING_HOOK: Option<fn(u32) -> bool> = None;

/// Return whether the current thread is stopped.
pub(crate) static mut GET_THREAD_STOPPED_HOOK: Option<fn() -> bool> = None;

/// Set the stopped flag on the current thread.
pub(crate) static mut SET_THREAD_STOPPED_HOOK: Option<fn(bool)> = None;

/// Get the sigsuspend saved mask for the current thread (Some if in sigsuspend).
pub(crate) static mut GET_SIGSUSPEND_MASK_HOOK: Option<fn() -> Option<SigSet>> = None;

/// Clear the sigsuspend saved mask and restore the saved blocked mask.
pub(crate) static mut CLEAR_SIGSUSPEND_HOOK: Option<fn(SigSet)> = None;

// ─── Sending signals ─────────────────────────────────────────────────────────

/// Add signal `sig` to the process-level pending set.
pub fn send_to_process(proc: &Arc<Mutex<Process>>, sig: u32) {
    if sig == 0 || sig >= abi::signal::NSIG {
        return;
    }
    let mut p = proc.lock();
    // SIGCONT clears stop-related pending signals.
    if sig == SIGCONT {
        p.pending_signals.remove(SIGSTOP);
        p.pending_signals.remove(SIGTSTP);
        p.pending_signals.remove(SIGTTIN);
        p.pending_signals.remove(SIGTTOU);
    }
    // SIG_IGN disposition: don't queue (uncatchable signals still get through).
    let disp = p.signal_dispositions[(sig - 1) as usize];
    if disp.handler == sig_handler::SIG_IGN && sig != SIGKILL && sig != SIGSTOP {
        return;
    }
    p.pending_signals.add(sig);
    let tids: alloc::vec::Vec<TaskId> = p.thread_ids.clone();
    drop(p);
    for tid in tids {
        unsafe { crate::sched::wake_task_erased(tid) };
    }
}

// ─── SIGCHLD helper ──────────────────────────────────────────────────────────

/// Deliver `SIGCHLD` to the parent process identified by `ppid`.
pub fn signal_parent_sigchld(ppid: u32) {
    if ppid == 0 {
        return;
    }
    if let Some(pi) = crate::sched::process_info_for_tid_current(ppid as u64) {
        send_to_process(&pi, abi::signal::SIGCHLD);
    }
}

// ─── Alarm tick check ────────────────────────────────────────────────────────

/// Called once per timer tick on CPU 0.  Fires SIGALRM for any process whose
/// alarm deadline has elapsed.
pub fn check_alarms() {
    let tick = crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);
    let procs = crate::sched::list_processes_current();
    for snap in procs {
        if let Some(pi) = crate::sched::process_info_for_tid_current(snap.pid as u64) {
            let fire = {
                let p = pi.lock();
                p.alarm_deadline > 0 && tick >= p.alarm_deadline
            };
            if fire {
                pi.lock().alarm_deadline = 0;
                send_to_process(&pi, abi::signal::SIGALRM);
            }
        }
    }
}

// ─── Main signal-check entry point ──────────────────────────────────────────

/// Called by the x86_64 (and other arch) syscall stub before returning to
/// user mode.  May not return if the signal's default action is Terminate.
///
/// `frame_ptr` — raw pointer to the `UserTrapFrame` currently on the kernel stack.
/// `syscall_ret` — the value the syscall was about to return.
///
/// Returns the (possibly unchanged) return value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_signal_check(frame_ptr: *mut u8, syscall_ret: isize) -> isize {
    let tid = unsafe { crate::sched::current_tid_current() };
    if tid == 0 {
        return syscall_ret;
    }

    let proc_arc = crate::sched::process_info_current();
    let proc = match proc_arc {
        Some(ref p) => p,
        None => return syscall_ret,
    };

    // Deliver one signal per syscall return (loop past SIG_IGN signals).
    loop {
        // --- pick the lowest deliverable signal ---
        let (blocked, thread_pending) = match unsafe { GET_THREAD_SIGNAL_HOOK } {
            Some(f) => f(),
            None => return syscall_ret,
        };

        let proc_pending = proc.lock().pending_signals;
        let combined = thread_pending.union(proc_pending);
        // SIGKILL and SIGSTOP bypass the mask.
        let killstop = SigSet::from_signal(SIGKILL).union(SigSet::from_signal(SIGSTOP));
        let deliverable = combined.difference(blocked).union(combined.intersect(killstop));
        let sig = deliverable.lowest_pending();
        if sig == 0 {
            return syscall_ret;
        }

        // --- dequeue it ---
        let was_thread_level = unsafe { CLEAR_THREAD_PENDING_HOOK }
            .map(|f| f(sig))
            .unwrap_or(false);
        if !was_thread_level {
            proc.lock().pending_signals.remove(sig);
        }

        // --- SIGKILL is always fatal ---
        if sig == SIGKILL {
            unsafe { crate::sched::exit_current(128 + SIGKILL as i32) };
        }

        let disp: SigAction = {
            let p = proc.lock();
            p.signal_dispositions[(sig - 1) as usize]
        };

        match disp.handler {
            h if h == sig_handler::SIG_IGN => {
                continue; // discard and check next
            }
            h if h == sig_handler::SIG_DFL => {
                match default_action(sig) {
                    DefaultAction::Ignore => return syscall_ret,
                    DefaultAction::Continue => return syscall_ret,
                    DefaultAction::Stop => {
                        // Mark thread stopped, block it, then continue loop on wakeup.
                        if let Some(f) = unsafe { SET_THREAD_STOPPED_HOOK } {
                            f(true);
                        }
                        let ppid = proc.lock().ppid;
                        let our_pid = proc.lock().pid;
                        // Send SIGCHLD to parent.
                        if ppid > 0 {
                            signal_parent_sigchld(ppid);
                        }
                        let _ = our_pid;
                        unsafe { crate::sched::block_current_erased() };
                        if let Some(f) = unsafe { SET_THREAD_STOPPED_HOOK } {
                            f(false);
                        }
                        continue;
                    }
                    DefaultAction::Terminate | DefaultAction::CoreDump => {
                        unsafe { crate::sched::exit_current(128 + sig as i32) };
                    }
                }
            }
            handler_addr => {
                // User-space handler.
                match unsafe { INJECT_FRAME_HOOK } {
                    Some(inject) => {
                        // Build the new blocked mask for the duration of the handler.
                        let mut extra_mask = disp.mask;
                        if (disp.flags & sa_flags::SA_NODEFER) == 0 {
                            extra_mask.add(sig);
                        }
                        let saved_mask = match unsafe { SET_THREAD_BLOCKED_HOOK } {
                            Some(f) => {
                                let cur_blocked = unsafe { GET_THREAD_SIGNAL_HOOK }
                                    .map(|g| g().0)
                                    .unwrap_or(SigSet::EMPTY);
                                let old = f(extra_mask.union(cur_blocked));
                                old
                            }
                            None => SigSet::EMPTY,
                        };

                        let ok = unsafe { inject(frame_ptr, sig, handler_addr, saved_mask) };
                        if !ok {
                            unsafe { crate::sched::exit_current(128 + sig as i32) };
                        }
                        if (disp.flags & sa_flags::SA_RESETHAND) != 0 {
                            proc.lock().signal_dispositions[(sig - 1) as usize] =
                                SigAction::default();
                        }
                        return 0;
                    }
                    None => {
                        // No arch hook — fall back to default.
                        match default_action(sig) {
                            DefaultAction::Terminate | DefaultAction::CoreDump => {
                                unsafe { crate::sched::exit_current(128 + sig as i32) };
                            }
                            _ => return syscall_ret,
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::signal::*;

    #[test]
    fn default_actions_match_posix() {
        assert_eq!(default_action(SIGKILL), DefaultAction::Terminate);
        assert_eq!(default_action(SIGSTOP), DefaultAction::Stop);
        assert_eq!(default_action(SIGCONT), DefaultAction::Continue);
        assert_eq!(default_action(SIGCHLD), DefaultAction::Ignore);
        assert_eq!(default_action(SIGTERM), DefaultAction::Terminate);
        assert_eq!(default_action(SIGQUIT), DefaultAction::CoreDump);
        assert_eq!(default_action(SIGSEGV), DefaultAction::CoreDump);
        assert_eq!(default_action(SIGWINCH), DefaultAction::Ignore);
        assert_eq!(default_action(SIGTSTP), DefaultAction::Stop);
    }

    #[test]
    fn unknown_signal_defaults_to_terminate() {
        assert_eq!(default_action(0), DefaultAction::Terminate);
        assert_eq!(default_action(255), DefaultAction::Terminate);
    }
}
