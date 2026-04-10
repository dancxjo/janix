# IPC Doctrine — Thing-OS Canonical Primitive Model

Thing-OS inter-process communication is built on five primitives.  Each has
one job.  Use the right primitive for the job; do not stretch one to cover
another.

---

## 1. Primitive Overview

| Primitive | Syscall family | When to use |
|-----------|---------------|-------------|
| **Channel** | `SYS_CHANNEL_*` | Discrete messages: commands, ACKs, events, handles, RPC |
| **Pipe** | `SYS_PIPE` / `SYS_FS_*` | Sequential byte streams: stdio, process output pipelines |
| **Memfd** | `SYS_MEMFD_CREATE` / `SYS_VM_MAP` | Bulk data, zero-copy buffers, shared rings |
| **Futex** | `SYS_FUTEX_WAIT` / `SYS_FUTEX_WAKE` | Low-level in-process synchronisation, mutex/condvar building blocks |
| **VFS RPC** | `SYS_FS_MOUNT` + channel protocol | Structured request/reply filesystem provider interface |

---

## 2. Channels

### What channels are

A **channel** is a bounded, message-oriented, bidirectional-by-pair IPC
primitive.  `SYS_CHANNEL_CREATE` returns a write handle and a read handle that
share one ring buffer.

### Message model

- Messages are discrete: each `channel_send` / `channel_recv` pair transfers
  one logical unit.
- Maximum message size: 4 KiB (the ring capacity).  Larger payloads must use
  memfd (see §5).
- `channel_send_all` is atomic: it either writes the entire payload or fails
  with `EAGAIN`.  Prefer it over `channel_send` for protocol messages.

### Handle passing

A channel message may carry one attached **capability handle** per
`channel_send_handle` call.  The kernel re-numbers the handle in the
receiver's fd table when the receiver calls `channel_recv_handle`.  Supported
capability types: VFS file descriptors, memfds, channel endpoints, VFS
provider ports.

See `docs/concepts/channel_semantics.md` for the full specification.

### When to use channels

- Any control-plane exchange: service requests, device commands, event
  notifications, registration handshakes.
- Request/reply RPC that is not naturally file-shaped.
- Passing capabilities (file descriptors, provider handles) between processes.

### When **not** to use channels

- Large binary blobs (images, audio, network payloads) — use memfd.
- Sequential byte streams without message boundaries — use a pipe.
- Shared-state synchronisation within a single address space — use a futex.

---

## 3. Pipes

### What pipes are

A **pipe** is a one-way, anonymous byte stream.  `SYS_PIPE` returns a read fd
and a write fd backed by a kernel ring buffer.

### When to use pipes

- Parent–child stdio (`fd 0`, `fd 1`, `fd 2`).
- Any producer–consumer relationship where the data has no message boundaries
  and both ends run in a direct parent–child relationship.
- Shell pipelines.

### When **not** to use pipes

- RPC or structured message exchange — use a channel.
- Cross-service communication with capability passing — use a channel.
- Bulk one-shot data transfer — consider memfd.

See `docs/concepts/channels_vs_pipes.md` for a detailed comparison.

---

## 4. Memfd

### What memfd is

A **memfd** (`SYS_MEMFD_CREATE`) is an anonymous, resizable, in-kernel memory
object exposed as a file descriptor.  The caller maps it into its address space
with `SYS_VM_MAP`.

### When to use memfd

- Pixel buffers, audio rings, network receive windows — any bulk-data path.
- Zero-copy exchange: sender writes into the mapped region, sends the fd over a
  channel; receiver maps it read-only.
- Persistent shared rings for audio or network I/O that avoids repeated fd
  transfers on the hot path.

### Doctrine

> **Control plane over channels; bulk data over memfd.**

Never embed multi-kilobyte payloads in channel messages.  Pass a
`abi::memfd::MemFdRef` (16 bytes) in the control message and the actual fd
via `channel_send_handle`.

See `docs/concepts/memfd.md` for the full lifecycle and wire format.

---

## 5. Futex

### What futex is

A **futex** is a 32-bit user-space integer at a known virtual address.  The
kernel provides `SYS_FUTEX_WAIT` and `SYS_FUTEX_WAKE` to park or wake threads
based on the value of that integer.

### When to use futex

- Building mutex / condvar / semaphore primitives in user space.
- Spinning-then-blocking patterns in lock-free data structures.
- Any in-process synchronisation where `stem::sync` primitives already wrap
  futexes correctly.

### When **not** to use futex

- Cross-process synchronisation that does not share a memory mapping — use
  channels.
- Any IPC that carries data, not just a signal — use a channel or pipe.

---

## 6. VFS RPC

### What VFS RPC is

VFS RPC allows a userland process to export a subtree of the filesystem
namespace.  The kernel serialises every filesystem operation (open, read,
write, stat, readdir …) into a typed message and delivers it to the provider's
channel.  The provider answers synchronously.

### When to use VFS RPC

- Implementing filesystem drivers (iso9660d, network FS, synthetic /proc entries).
- Exposing a device as a set of files under `/dev`.
- Any service that looks naturally file-shaped to its clients.

### When **not** to use VFS RPC

- Low-latency event sources — use a channel directly.
- Bulk streaming — combine VFS RPC with memfd for the data path.

See `docs/concepts/vfs_rpc_provider.md` for the provider lifecycle and wire
protocol.

---

## 7. Decision Matrix

| I need to … | Use |
|-------------|-----|
| Send a command / event to a service | Channel |
| Do synchronous request/reply RPC | Channel + `abi::rpc::RpcHeader` |
| Pass a file descriptor to another process | Channel + `channel_send_handle` |
| Transfer a large buffer zero-copy | Memfd + channel (for the descriptor) |
| Stream bytes parent→child | Pipe |
| Expose a subtree as a filesystem | VFS RPC |
| Wait on multiple I/O sources at once | `SYS_FS_POLL` (works for all VFS fds) |
| Implement a mutex or condvar | Futex (or `stem::sync` wrappers) |

---

## 8. Readiness and Waiting

All VFS-backed fds — pipes, channels bridged via `SYS_FD_FROM_HANDLE`, and
mounted provider files — participate in the same `SYS_FS_POLL` readiness model.
An event loop does not need to special-case object types.

See `docs/concepts/readiness.md` for the full specification.

---

## 9. See Also

- `docs/concepts/channel_semantics.md` — capacity, atomicity, blocking, peer death
- `docs/concepts/channels_vs_pipes.md` — when to use which
- `docs/concepts/vfs_rpc_provider.md` — provider lifecycle
- `docs/concepts/memfd.md` — bulk-data path
- `docs/concepts/readiness.md` — poll/wait model
- `docs/concepts/supervisor_protocol.md` — service registration over channels
- `docs/concepts/ipc_cookbook.md` — practical recipes
- `abi/src/rpc.rs` — structured request/reply header types
- `libs/ipc_helpers/` — userspace helper library
