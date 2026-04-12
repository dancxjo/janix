//! Kernel POSIX signal subsystem.
//!
//! # Architecture
//!
//! Signal *dispositions* belong to the process (`Process.signal_dispositions`).
//! Signal *masks* belong to individual threads (`Thread.blocked_signals`).
//! Pending signals exist at both levels:
//! - `Process.pending_signals` — sent by `kill(pid, …)`.
//! - `Thread.pending_signals`  — sent to a specific thread (e.g. hardware fault).
//!
//! Delivery happens at the boundary between kernel mode and user mode — i.e.
//! just before `sysretq` / `eret` / `sret` returns to the calling thread.
//! The architecture-specific assembly stub calls [`check_and_deliver`] with
//! a pointer to the saved user register frame; that function may redirect
//! execution to a registered signal handler or perform an immediate default
//! action (terminate, stop, …).
//!
//! # Signal frame (x86_64)
//!
//! When a user-space handler needs to run the kernel:
//! 1. Lowers the user-mode RSP by `sizeof(SigFrame)`.
//! 2. Fills in the [`SigFrame`] with the pre-signal register state.
//! 3. Writes a tiny `sigreturn` trampoline into `SigFrame::trampoline`.
//! 4. Sets the saved RIP to the handler entry point and RDI to the signal number.
//! 5. Sets the saved return address (pushed `call` slot below RSP) to
//!    `&SigFrame::trampoline[0]` so that `ret` in the handler enters `sigreturn`.
//!
//! On `SYS_SIGRETURN` the kernel validates the magic sentinel and restores
//! the register state from the frame, then continues normal return-to-user
//! processing.

pub mod deliver;
