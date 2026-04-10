# Unified Readiness Model

Thing-OS uses a single, coherent readiness model for all waitable kernel
objects.  The same set of flags, the same poll contract, and the same
blocking pattern apply whether you are waiting on a pipe, a channel (IPC
port), a VFS-backed device file, or a mix of all three.

---

## 1. Readiness Flags

All readiness is expressed as a bitmask using the constants in
`abi::syscall::poll_flags`:

| Flag       | Value  | Meaning |
|------------|--------|---------|
| `POLLIN`   | 0x0001 | Data is available to read (or EOF on a pipe/channel). |
| `POLLOUT`  | 0x0004 | Space is available to write without blocking. |
| `POLLERR`  | 0x0008 | An error condition is present; applicable to write ends when the peer has closed. |
| `POLLHUP`  | 0x0010 | The peer has closed its end (hangup). Always checked regardless of `events`. |
| `POLLNVAL` | 0x0020 | The file descriptor is not open or is not valid. |

`POLLERR` and `POLLHUP` are always reported in `revents` if they occur,
even when not listed in `events`.

---

## 2. Per-Object-Class Semantics

### 2.1 Pipes (`PipeReadNode` / `PipeWriteNode`)

Pipes are anonymous byte streams created by `SYS_FS_PIPE`.  Each pipe has a
read end and a write end.

**Read end (`PipeReadNode`):**

| Condition | POLLIN | POLLHUP |
|-----------|--------|---------|
| Buffer has bytes | ✓ | — |
| Buffer empty, writer alive | — | — |
| Buffer has bytes, writer closed | ✓ | ✓ |
| Buffer empty, writer closed (EOF) | ✓ | ✓ |

A consumer detects EOF by receiving `POLLIN` with no readable bytes
(`read` returns 0) **or** by observing `POLLHUP` directly.

**Write end (`PipeWriteNode`):**

| Condition | POLLOUT | POLLERR | POLLHUP |
|-----------|---------|---------|---------|
| Buffer has free space, reader alive | ✓ | — | — |
| Buffer full, reader alive | — | — | — |
| Reader closed (broken pipe) | — | ✓ | ✓ |

A writer detects that the read end is gone by observing `POLLERR | POLLHUP`
on its write end.  A subsequent `write` call will return `EPIPE`.

### 2.2 Channels / IPC Ports (`PortNode`)

Channels are bounded byte queues created by `SYS_CHANNEL_CREATE`.  Each
channel exposes a read handle and a write handle, each of which can be
bridged to a VFS file descriptor via `SYS_FS_FD_FROM_HANDLE`.

**Read handle:**

| Condition | POLLIN | POLLHUP |
|-----------|--------|---------|
| Queue has bytes | ✓ | — |
| Queue empty, writer alive | — | — |
| Queue empty, writer closed | ✓ | ✓ |
| Queue has bytes, writer closed | ✓ | ✓ |

**Write handle:**

| Condition | POLLOUT | POLLERR | POLLHUP |
|-----------|---------|---------|---------|
| Queue has free space, reader alive | ✓ | — | — |
| Queue full, reader alive | — | — | — |
| Reader closed | — | ✓ | ✓ |

The semantics mirror pipes exactly, which means the same event-loop code
can handle both without special-casing.

### 2.3 VFS-Backed Files

Regular files and device nodes opened via `SYS_FS_OPEN` implement
`VfsNode::poll`.  The default implementation returns `POLLIN | POLLOUT`
unconditionally, matching POSIX semantics for non-socket file descriptors:
**regular files are always ready**.

Device-specific implementations (e.g. a framebuffer driver, a terminal)
may override this to reflect actual buffer state.

---

## 3. The `SYS_FS_POLL` Syscall

```
SYS_FS_POLL(pollfds_ptr: usize, nfds: usize, timeout_ms: usize) -> SysResult<usize>
```

- `pollfds_ptr` — pointer to a `[PollFd; nfds]` array in user memory
- `nfds`        — number of entries (max 256)
- `timeout_ms`  — `0` = non-blocking; `usize::MAX` = block indefinitely;
                  any other value = timeout in milliseconds

Returns the number of entries with a non-zero `revents`, or an errno.

### Algorithm (no-lost-wakeup)

The kernel handler uses a three-phase algorithm to guarantee that a wakeup
produced between registration and the actual sleep is never lost:

1. **Probe** — call `VfsNode::poll()` on every entry.  If anything is
   ready or the timeout is 0, copy `revents` back and return immediately.
2. **Register** — call `VfsNode::add_waiter(tid)` on every node, then
   optionally arm a scheduler timeout via `register_timeout_wake_current`.
3. **Re-probe** — repeat the probe after registration.  If still nothing
   is ready, call `block_current_erased()` to park the task.
4. On wake, **unregister** from all nodes and re-probe to gather results.

This means the loop exits correctly whether the wakeup arrives before step 3
completes or after the task is parked.

---

## 4. Sample Event Loop

```rust
use stem::syscall::vfs::*;
use abi::syscall::{PollFd, poll_flags};

fn run_event_loop(pipe_read: u32, channel_fd: u32, file_fd: u32) {
    let mut fds = [
        PollFd { fd: pipe_read as i32,  events: poll_flags::POLLIN, revents: 0 },
        PollFd { fd: channel_fd as i32, events: poll_flags::POLLIN, revents: 0 },
        PollFd { fd: file_fd as i32,    events: poll_flags::POLLIN | poll_flags::POLLOUT, revents: 0 },
    ];

    loop {
        let n = vfs_poll(&mut fds, u64::MAX).expect("poll failed");
        if n == 0 { continue; } // spurious wakeup

        for entry in &fds {
            if entry.revents == 0 { continue; }

            if entry.revents & poll_flags::POLLHUP != 0 {
                // Peer closed — handle EOF / broken pipe.
                break;
            }
            if entry.revents & poll_flags::POLLIN != 0 {
                // Read available data.
            }
            if entry.revents & poll_flags::POLLOUT != 0 {
                // Write space available.
            }
        }

        // Reset revents before next iteration.
        for entry in &mut fds { entry.revents = 0; }
    }
}
```

---

## 5. Peer-Death / Hangup Consistency

Hangup behaviour is uniform across all object classes:

- Closing the **write** end of a pipe or channel causes `POLLIN | POLLHUP`
  on the read end (the EOF signal).
- Closing the **read** end of a pipe or channel causes `POLLERR | POLLHUP`
  on the write end (broken pipe signal).
- These bits are always reported in `revents` regardless of the `events`
  mask, so an event loop does not need to subscribe to them explicitly.

---

## 6. Future Extensions

The same `VfsNode::poll` / `add_waiter` / `remove_waiter` contract is the
extension point for timer FDs, process-exit notification FDs, IRQ FDs, and
any other kernel waitable that needs to participate in `SYS_FS_POLL`.

The higher-level `SYS_WAIT_MANY` syscall (see `docs/wait_many.md`) uses a
parallel but orthogonal mechanism based on typed `WaitSpec` entries.  For
VFS file descriptors, `SYS_WAIT_MANY` supports `WaitKind::Fd` which routes
through the same `VfsNode::poll` / waiter API.
