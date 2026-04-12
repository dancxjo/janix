//! Stem wrappers for POSIX signal syscalls.
//!
//! These are thin wrappers around the raw syscall numbers defined in
//! `abi::syscall`.  They follow the same conventions as other stem syscall
//! wrappers: errors are returned as `Err(Errno)`, success as `Ok(value)`.

use abi::errors::Errno;
use abi::signal::{SigAction, SigSet};
use abi::syscall::{
    SYS_ALARM, SYS_KILL, SYS_PAUSE, SYS_SIGACTION, SYS_SIGPENDING, SYS_SIGPROCMASK,
    SYS_SIGSUSPEND,
};

use super::arch::raw_syscall6;

// ─── kill ────────────────────────────────────────────────────────────────────

/// Send signal `sig` to process `pid`.
///
/// Use `sig == 0` for an existence/permission check.
pub fn kill(pid: i32, sig: u32) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_KILL, pid as usize, sig as usize, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

/// Send signal `sig` to the calling process.
pub fn raise(sig: u32) -> Result<(), Errno> {
    // getpid() equivalent: we call kill with our own PID.
    // Stem doesn't have getpid() here, but kill(0, sig) is not guaranteed to
    // target self on all kernels; use the pidprocmask trick instead.
    // For now, just do kill(-1 as i32, sig) which on ThingOS targets self.
    // TODO: use getpid() once it's available in stem.
    let ret = unsafe { raw_syscall6(SYS_KILL, (-1isize) as usize, sig as usize, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

// ─── sigaction ───────────────────────────────────────────────────────────────

/// Examine and/or change the action for signal `sig`.
///
/// Pass `None` for `act` to query without changing; pass `None` for `oldact`
/// to change without retrieving the old action.
pub fn sigaction(
    sig: u32,
    act: Option<&SigAction>,
    oldact: Option<&mut SigAction>,
) -> Result<(), Errno> {
    let act_ptr = act.map(|a| a as *const _ as usize).unwrap_or(0);
    let oldact_ptr = oldact.map(|a| a as *mut _ as usize).unwrap_or(0);
    let ret = unsafe { raw_syscall6(SYS_SIGACTION, sig as usize, act_ptr, oldact_ptr, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

// ─── sigprocmask ─────────────────────────────────────────────────────────────

/// Block additional signals (`SIG_BLOCK`).
pub fn sigblock(set: SigSet) -> Result<SigSet, Errno> {
    sigprocmask(abi::signal::sig_how::SIG_BLOCK, Some(set))
}

/// Unblock signals (`SIG_UNBLOCK`).
pub fn sigunblock(set: SigSet) -> Result<SigSet, Errno> {
    sigprocmask(abi::signal::sig_how::SIG_UNBLOCK, Some(set))
}

/// Replace the signal mask (`SIG_SETMASK`).
pub fn sigsetmask(set: SigSet) -> Result<SigSet, Errno> {
    sigprocmask(abi::signal::sig_how::SIG_SETMASK, Some(set))
}

/// Low-level `sigprocmask` wrapper.
pub fn sigprocmask(how: u32, new_set: Option<SigSet>) -> Result<SigSet, Errno> {
    let mut old = SigSet::EMPTY;
    let set_ptr = match new_set {
        Some(ref s) => s as *const SigSet as usize,
        None => 0,
    };
    let old_ptr = &mut old as *mut SigSet as usize;
    let ret = unsafe { raw_syscall6(SYS_SIGPROCMASK, how as usize, set_ptr, old_ptr, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| old)
}

// ─── sigpending ──────────────────────────────────────────────────────────────

/// Return the set of blocked pending signals.
pub fn sigpending() -> Result<SigSet, Errno> {
    let mut set = SigSet::EMPTY;
    let ret =
        unsafe { raw_syscall6(SYS_SIGPENDING, &mut set as *mut SigSet as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| set)
}

// ─── sigsuspend ──────────────────────────────────────────────────────────────

/// Atomically replace the signal mask with `mask` and sleep until a signal
/// arrives.  Always returns `Err(EINTR)`.
pub fn sigsuspend(mask: SigSet) -> Errno {
    let ret = unsafe { raw_syscall6(SYS_SIGSUSPEND, &mask as *const SigSet as usize, 0, 0, 0, 0, 0) };
    // sigsuspend always returns EINTR.
    match abi::errors::errno(ret) {
        Err(e) => e,
        Ok(_) => Errno::EINTR,
    }
}

// ─── alarm ───────────────────────────────────────────────────────────────────

/// Schedule `SIGALRM` to be delivered after `seconds` seconds.
///
/// Returns the number of seconds remaining on any previously set alarm, or 0.
pub fn alarm(seconds: u32) -> u32 {
    let ret = unsafe { raw_syscall6(SYS_ALARM, seconds as usize, 0, 0, 0, 0, 0) };
    if ret < 0 {
        0
    } else {
        ret as u32
    }
}

// ─── pause ───────────────────────────────────────────────────────────────────

/// Wait for a signal.  Always returns `Err(EINTR)`.
pub fn pause() -> Errno {
    let ret = unsafe { raw_syscall6(SYS_PAUSE, 0, 0, 0, 0, 0, 0) };
    match abi::errors::errno(ret) {
        Err(e) => e,
        Ok(_) => Errno::EINTR,
    }
}
