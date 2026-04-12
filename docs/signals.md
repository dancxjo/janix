# POSIX Signal Semantics in ThingOS

## Overview

ThingOS implements a classic POSIX-compatible non-realtime signal model centered
on the process/thread-group abstraction.  This document describes the current
semantics, the data-model, the syscall surface, and intentional gaps.

---

## Signal numbers

Signals 1–31 are defined in `abi/src/signal.rs` and match the standard
Linux/POSIX numbering on all supported architectures:

| Number | Name     | Default action   |
|--------|----------|------------------|
| 1      | SIGHUP   | terminate        |
| 2      | SIGINT   | terminate        |
| 3      | SIGQUIT  | core dump        |
| 4      | SIGILL   | core dump        |
| 5      | SIGTRAP  | core dump        |
| 6      | SIGABRT  | core dump        |
| 7      | SIGBUS   | core dump        |
| 8      | SIGFPE   | core dump        |
| 9      | SIGKILL  | terminate †      |
| 10     | SIGUSR1  | terminate        |
| 11     | SIGSEGV  | core dump        |
| 12     | SIGUSR2  | terminate        |
| 13     | SIGPIPE  | terminate        |
| 14     | SIGALRM  | terminate        |
| 15     | SIGTERM  | terminate        |
| 17     | SIGCHLD  | ignore           |
| 18     | SIGCONT  | continue         |
| 19     | SIGSTOP  | stop †           |
| 20     | SIGTSTP  | stop             |
| 21     | SIGTTIN  | stop             |
| 22     | SIGTTOU  | stop             |
| 28     | SIGWINCH | ignore           |

† **SIGKILL** and **SIGSTOP** cannot be caught, blocked, or ignored.

---

## Data model

### Per-process state (`ProcessInfo`)

```
signal_dispositions: [SigAction; 32]   // per-signal handler/flags
pending_signals:     SigSet            // process-level pending bitset
is_stopped:          bool              // true while stopped by SIGSTOP/SIGTSTP
alarm_deadline:      u64               // kernel tick for SIGALRM (0 = none)
```

### Per-thread state (`Thread`)

```
blocked_signals:   SigSet             // current signal mask
pending_signals:   SigSet             // thread-targeted pending set
sigsuspend_mask:   Option<SigSet>     // saved mask during sigsuspend
is_stopped:        bool               // thread-level stop flag
```

### `SigAction`

```c
struct SigAction {
    usize    handler;    // SIG_DFL=0, SIG_IGN=1, or user function pointer
    SigSet   mask;       // signals to block during handler
    u32      flags;      // SA_RESTART | SA_RESETHAND | SA_NODEFER | SA_ONSTACK
    u32      _pad;
};
```

### `SigSet`

A `u64` bitmask: bit `n-1` corresponds to signal `n`.  Signals 1–31 are
supported; bits 32–63 are reserved for future realtime signals.

---

## Syscall surface

| Syscall number | Name           | Signature                                       |
|----------------|----------------|------------------------------------------------|
| 0x1500         | SYS_KILL       | `(pid: i32, sig: u32) → i32`                  |
| 0x1501         | SYS_SIGACTION  | `(sig, act_ptr, oldact_ptr) → i32`             |
| 0x1502         | SYS_SIGPROCMASK| `(how, set_ptr, oldset_ptr) → i32`             |
| 0x1503         | SYS_SIGPENDING | `(set_ptr) → i32`                              |
| 0x1504         | SYS_SIGSUSPEND | `(mask_ptr) → EINTR`                           |
| 0x1505         | SYS_SIGRETURN  | `() → does not return normally`                |
| 0x1506         | SYS_ALARM      | `(seconds: u32) → u32` (remaining seconds)    |
| 0x1507         | SYS_PAUSE      | `() → EINTR`                                   |

Stem wrappers live in `stem/src/syscall/signal.rs`:
`kill`, `raise`, `sigaction`, `sigblock`, `sigunblock`, `sigsetmask`,
`sigprocmask`, `sigpending`, `sigsuspend`, `alarm`, `pause`.

---

## Delivery model

### When signals are checked

Signal delivery is checked **on every return to user mode** — i.e. immediately
after the kernel dispatches a syscall and before `sysretq` executes.  The
x86_64 stub calls `kernel_signal_check(frame_ptr, syscall_ret)` which:

1. Picks the lowest-numbered deliverable signal.
2. Removes it from the pending set.
3. If `SIG_IGN`: discards and loops.
4. If `SIG_DFL`: executes the default action (may not return for terminate/stop).
5. If user handler: calls `INJECT_FRAME_HOOK` to set up a `SigFrame` on the
   user stack and redirects execution to the handler.

### Signal frame (x86_64)

When a user handler is invoked the kernel:
1. Subtracts `sizeof(SigFrame)` from the user RSP.
2. Writes a `SigFrame` containing the saved register state and a `sigreturn`
   trampoline into the user stack.
3. Sets RSP to point at the return-address slot (just above the frame).
4. Redirects RIP to the handler; sets RDI = signal number.

On return from the handler the trampoline (`movq $SYS_SIGRETURN, %rax; syscall`)
triggers `kernel_sigreturn(frame_ptr)` which:
1. Validates the `SIGFRAME_MAGIC` sentinel.
2. Restores all general-purpose registers from the `SigFrame`.
3. Restores the pre-signal blocked mask.
4. Clears any `sigsuspend` state.

### SIGKILL / SIGSTOP

These signals bypass the disposition table entirely.  `SIGKILL` calls
`exit_current(128+9)` immediately.  `SIGSTOP` blocks the thread and sends
`SIGCHLD` to the parent; it is unblocked when `SIGCONT` arrives.

---

## `alarm(seconds)`

The alarm deadline is stored as a tick count in `ProcessInfo.alarm_deadline`.
On every 100th tick (≈100 ms at 1 kHz), `signal::deliver::check_alarms()` is
called by `sched::on_tick`.  When the deadline elapses, `SIGALRM` is queued
for the process and the deadline is cleared.

---

## SIGCHLD

`SIGCHLD` is delivered to the parent process when the thread-group leader
exits.  The delivery happens inside `mark_task_exited` in `sched/mod.rs` before
the leader's `thread_ids` list is drained.

---

## Intentional gaps (deferred)

* **Realtime signals** (34–64): data structure space is reserved in `SigSet`
  (u64, bits 32–63) but the kernel does not yet process them.
* **`siginfo_t` / queued payloads**: pending signals use a bitset, not a queue.
  A second delivery of the same signal before delivery is lost (standard
  non-realtime POSIX behaviour).
* **`sigaltstack`**: alternate signal stacks are not yet supported.
* **Full process-group / session / TTY job control**: `kill(0, sig)` and
  `kill(-1, sig)` return `ESRCH`.  `SIGTTIN`, `SIGTTOU` are defined but not
  automatically raised by the TTY layer.
* **`ptrace` signal mediation**: no `ptrace` support.
* **Core dump files**: the "core dump" default action currently terminates the
  process; no actual dump file is written.
* **`SA_ONSTACK`**: alternate signal stacks are reserved for future use.
* **`waitpid(WUNTRACED | WCONTINUED)`**: stop/continue events are not yet
  surfaced through `waitpid` status codes (a follow-up task).

---

## Utilities

* `userspace/kill` — sends a signal to one or more processes.
  Usage: `kill [-<SIG>] <pid> [<pid>...]`
* `userspace/sig_test` — regression test suite covering handler installation,
  blocking/pending, uncatchable signals, `alarm`, and `SIG_IGN`.

---

## Architecture notes

Other architectures (aarch64, riscv64, loongarch64) currently lack the
`INJECT_FRAME_HOOK` implementation.  `kernel_signal_check` is called but
falls through to the default action when no hook is registered.  Each
architecture must implement a `kernel_sigreturn` function and call
`signal::deliver::INJECT_FRAME_HOOK = Some(arch_inject_frame)` during
its boot initialisation.
