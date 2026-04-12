//! POSIX signal constants, types, and ABI structures.
//!
//! These definitions are shared between the kernel, `stem`, and userspace.
//! Signal numbers follow the standard Linux/POSIX numbering on all
//! architectures supported by Thing-OS.

// ─── Signal numbers ──────────────────────────────────────────────────────────

pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
/// Bus error — unaligned or inaccessible physical address.
pub const SIGBUS: u32 = 7;
pub const SIGFPE: u32 = 8;
/// Kill — cannot be caught, blocked, or ignored.
pub const SIGKILL: u32 = 9;
pub const SIGUSR1: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGUSR2: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGSTKFLT: u32 = 16;
pub const SIGCHLD: u32 = 17;
pub const SIGCONT: u32 = 18;
/// Stop — cannot be caught, blocked, or ignored.
pub const SIGSTOP: u32 = 19;
/// Terminal stop (Ctrl-Z).
pub const SIGTSTP: u32 = 20;
/// Background read from terminal.
pub const SIGTTIN: u32 = 21;
/// Background write to terminal.
pub const SIGTTOU: u32 = 22;
pub const SIGURG: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGIO: u32 = 29;
pub const SIGPWR: u32 = 30;
pub const SIGSYS: u32 = 31;

/// One past the last supported real-time signal.  Signals are numbered 1–31
/// (non-realtime); future work may extend this range to 64.
pub const NSIG: u32 = 32;

// ─── SigSet ──────────────────────────────────────────────────────────────────

/// A bitmask of pending or blocked signals.  Bit *n − 1* corresponds to signal
/// number *n* (POSIX convention).  Only bits 0–30 (signals 1–31) are used by
/// the non-realtime signal model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SigSet(pub u64);

impl SigSet {
    /// The empty set.
    pub const EMPTY: Self = Self(0);

    /// A set with all non-realtime signals *except* SIGKILL and SIGSTOP.
    pub const ALL_CATCHABLE: Self = {
        let mut bits = 0u64;
        let mut s = 1u32;
        while s < NSIG {
            if s != SIGKILL && s != SIGSTOP {
                bits |= 1u64 << (s - 1);
            }
            s += 1;
        }
        Self(bits)
    };

    /// Create a set containing a single signal.
    #[inline]
    pub const fn from_signal(sig: u32) -> Self {
        if sig == 0 || sig >= NSIG {
            Self(0)
        } else {
            Self(1u64 << (sig - 1))
        }
    }

    /// Add a signal to the set.
    #[inline]
    pub fn add(&mut self, sig: u32) {
        if sig > 0 && sig < NSIG {
            self.0 |= 1u64 << (sig - 1);
        }
    }

    /// Remove a signal from the set.
    #[inline]
    pub fn remove(&mut self, sig: u32) {
        if sig > 0 && sig < NSIG {
            self.0 &= !(1u64 << (sig - 1));
        }
    }

    /// Test whether a signal is in the set.
    #[inline]
    pub const fn contains(&self, sig: u32) -> bool {
        if sig == 0 || sig >= NSIG {
            false
        } else {
            (self.0 >> (sig - 1)) & 1 != 0
        }
    }

    /// Return the lowest-numbered pending signal, or 0 if empty.
    #[inline]
    pub fn lowest_pending(&self) -> u32 {
        if self.0 == 0 {
            0
        } else {
            self.0.trailing_zeros() + 1
        }
    }

    /// Return `true` if the set is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Union: `self | other`.
    #[inline]
    pub fn union(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Intersection: `self & other`.
    #[inline]
    pub fn intersect(&self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Difference: `self & !other`.
    #[inline]
    pub fn difference(&self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Complement: `~self` (restricted to signal range).
    #[inline]
    pub fn complement(&self) -> Self {
        // Only bits 0..31 are meaningful.
        Self(!self.0 & ((1u64 << (NSIG - 1)) - 1))
    }
}

// ─── sigprocmask `how` values ────────────────────────────────────────────────

/// Values for the `how` argument to `sigprocmask`.
pub mod sig_how {
    /// `blocked |= set`
    pub const SIG_BLOCK: u32 = 0;
    /// `blocked &= ~set`
    pub const SIG_UNBLOCK: u32 = 1;
    /// `blocked = set`
    pub const SIG_SETMASK: u32 = 2;
}

// ─── SigAction ───────────────────────────────────────────────────────────────

/// Special handler values (matching POSIX SIG_DFL / SIG_IGN).
pub mod sig_handler {
    /// Perform the default action for the signal.
    pub const SIG_DFL: usize = 0;
    /// Ignore the signal.
    pub const SIG_IGN: usize = 1;
}

/// `SA_*` flags for [`SigAction`].
pub mod sa_flags {
    /// Resume a blocked syscall instead of returning `EINTR`.
    pub const SA_RESTART: u32 = 0x10000000;
    /// Restore the old signal mask after the handler returns (automatically
    /// added by the kernel; userspace may also pass it).
    pub const SA_RESETHAND: u32 = 0x80000000;
    /// Prevent recursive delivery of the same signal within a handler.
    pub const SA_NODEFER: u32 = 0x40000000;
    /// Deliver on an alternate signal stack (not yet supported; reserved).
    pub const SA_ONSTACK: u32 = 0x08000000;
}

/// Kernel/userspace interface for `sigaction(2)`.
///
/// Layout must stay stable across kernel upgrades; add new fields at the end.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct SigAction {
    /// Handler function pointer, or one of the [`sig_handler`] constants.
    pub handler: usize,
    /// Signals to additionally block while the handler is running.
    pub mask: SigSet,
    /// [`sa_flags`] bitset.
    pub flags: u32,
    pub _pad: u32,
}

// ─── SigFrame (x86_64 user-mode signal delivery frame) ───────────────────────

/// Magic sentinel written at the start of every [`SigFrame`].
/// Validated by `sigreturn` to detect stack corruption.
pub const SIGFRAME_MAGIC: u64 = 0x5349_4752_414D_4546; // "SIGFRAME"

/// Number of bytes in the trampoline embedded at the end of [`SigFrame`].
///
/// The trampoline executes `mov rax, SYS_SIGRETURN; syscall` (9 bytes) plus one
/// alignment byte.
pub const SIGFRAME_TRAMPOLINE_LEN: usize = 10;

/// Structure pushed onto the user stack when a signal handler is invoked.
///
/// The signal handler is called with RSP pointing at this structure and RIP
/// pointing at the `handler` field of the matching [`SigAction`].  The return
/// address pushed by the `call` is `&trampoline[0]`, which runs a two-instruction
/// `sigreturn` sequence.
///
/// Layout is **architecture-specific** (this version is x86_64).  On other
/// architectures a similar structure must be defined in their respective arch
/// crates; the kernel signal subsystem references this type only through the
/// architecture-specific `inject_signal_frame` hook.
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct SigFrame {
    /// Validation sentinel.
    pub magic: u64,
    /// Saved user-mode instruction pointer (the address the thread should
    /// return to after the handler finishes).
    pub saved_rip: u64,
    /// Saved user-mode stack pointer before signal delivery.
    pub saved_rsp: u64,
    /// Saved RFLAGS.
    pub saved_rflags: u64,
    /// Saved general-purpose registers in the order stored by the kernel's
    /// `UserTrapFrame` (r15 … rax).
    pub saved_r15: u64,
    pub saved_r14: u64,
    pub saved_r13: u64,
    pub saved_r12: u64,
    pub saved_r11: u64,
    pub saved_r10: u64,
    pub saved_r9: u64,
    pub saved_r8: u64,
    pub saved_rbp: u64,
    pub saved_rdi: u64,
    pub saved_rsi: u64,
    pub saved_rdx: u64,
    pub saved_rcx: u64,
    pub saved_rbx: u64,
    pub saved_rax: u64,
    /// The signal mask in effect *before* signal delivery (restored by
    /// `sigreturn`).
    pub saved_mask: SigSet,
    /// The signal number delivered to the handler.
    pub signum: u32,
    pub _pad: u32,
    /// Inline `sigreturn` trampoline: `mov rax, SYS_SIGRETURN; syscall; nop`
    /// (10 bytes padded to 16 for alignment).
    pub trampoline: [u8; 16],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sigset_basic_operations() {
        let mut s = SigSet::EMPTY;
        assert!(s.is_empty());

        s.add(SIGINT);
        assert!(s.contains(SIGINT));
        assert!(!s.contains(SIGTERM));
        assert_eq!(s.lowest_pending(), SIGINT);

        s.add(SIGTERM);
        assert_eq!(s.lowest_pending(), SIGINT); // lower number wins

        s.remove(SIGINT);
        assert!(!s.contains(SIGINT));
        assert_eq!(s.lowest_pending(), SIGTERM);
    }

    #[test]
    fn sigset_from_signal() {
        let s = SigSet::from_signal(SIGUSR1);
        assert!(s.contains(SIGUSR1));
        assert!(!s.contains(SIGUSR2));
    }

    #[test]
    fn sigset_all_catchable_excludes_kill_stop() {
        let s = SigSet::ALL_CATCHABLE;
        assert!(!s.contains(SIGKILL));
        assert!(!s.contains(SIGSTOP));
        assert!(s.contains(SIGTERM));
        assert!(s.contains(SIGINT));
    }

    #[test]
    fn sigset_union_intersect_difference() {
        let a = SigSet::from_signal(SIGINT).union(SigSet::from_signal(SIGTERM));
        let b = SigSet::from_signal(SIGTERM).union(SigSet::from_signal(SIGUSR1));

        assert_eq!(a.intersect(b), SigSet::from_signal(SIGTERM));
        assert_eq!(a.difference(b), SigSet::from_signal(SIGINT));
    }

    #[test]
    fn sigset_out_of_range_is_noop() {
        let mut s = SigSet::EMPTY;
        s.add(0);
        s.add(NSIG);
        s.add(64);
        assert!(s.is_empty());
        assert!(!s.contains(0));
        assert!(!s.contains(NSIG));
    }
}

#[cfg(test)]
mod signal_number_tests {
    use super::*;

    #[test]
    fn signal_numbers_match_posix() {
        assert_eq!(SIGHUP, 1);
        assert_eq!(SIGINT, 2);
        assert_eq!(SIGKILL, 9);
        assert_eq!(SIGTERM, 15);
        assert_eq!(SIGCHLD, 17);
        assert_eq!(SIGSTOP, 19);
    }

    #[test]
    fn sigset_complement_excludes_sigkill_sigstop() {
        // ALL_CATCHABLE should not contain SIGKILL or SIGSTOP.
        let all = SigSet::ALL_CATCHABLE;
        assert!(!all.contains(SIGKILL));
        assert!(!all.contains(SIGSTOP));
        // But it should contain everything else in range.
        assert!(all.contains(SIGTERM));
        assert!(all.contains(SIGUSR1));
        assert!(all.contains(SIGUSR2));
    }

    #[test]
    fn sig_how_constants_correct() {
        assert_eq!(sig_how::SIG_BLOCK, 0);
        assert_eq!(sig_how::SIG_UNBLOCK, 1);
        assert_eq!(sig_how::SIG_SETMASK, 2);
    }
}
