# Channels vs Pipes — When to Use Which

Both channels and pipes move bytes between processes, but they serve
fundamentally different purposes.  Mixing them up leads to awkward protocol
code or unnecessary complexity.

---

## 1. Side-by-Side Comparison

| Property | Channel | Pipe |
|----------|---------|------|
| **Byte model** | Discrete messages | Continuous byte stream |
| **Message boundaries** | Preserved | Not preserved |
| **Direction** | Bidirectional pair (one write thing + one read thing) | One-way (read thing + write thing) |
| **Capacity** | Configurable ring, 64 B – 64 KiB | Fixed kernel ring (4 KiB default) |
| **Thing passing** | Yes — `channel_send_msg` / `channel_recv_msg` | No |
| **Typical use** | Service requests, events, RPC, capability transfer | stdio, process output pipelines |
| **Poll integration** | Yes — bridge with `SYS_FD_FROM_HANDLE` | Yes — read/write ends are VFS things |
| **Syscall family** | `SYS_CHANNEL_*` | `SYS_PIPE` + `SYS_FS_*` |

---

## 2. Decision Rules

### Use a **channel** when

- You are sending discrete protocol messages (e.g. commands, ACKs, event
  notifications).
- You need to pass a capability (memfd, provider thing) to another process.
- You are implementing request/reply RPC with a service.
- You need bidirectional communication without creating two separate pipes.
- You want to wait on the channel together with other VFS things in one `poll`
  call.

### Use a **pipe** when

- You are wiring up stdio (thing 0, thing 1, thing 2) for a child process.
- You have a shell pipeline: the output of one process feeds the input of
  another.
- The data has no message boundaries (pure byte stream: text, binary output).
- You just need a one-way data stream with no metadata or capability transfer.

---

## 3. Poll Semantics Differences

| Event | Pipe read thing | Channel read thing (bridged) |
|-------|-----------------|------------------------------|
| Data available | `POLLIN` | `POLLIN` |
| Writer closed (EOF) | `POLLIN \| POLLHUP` | `POLLIN \| POLLHUP` |

| Event | Pipe write thing | Channel write thing (bridged) |
|-------|------------------|-------------------------------|
| Space available | `POLLOUT` | `POLLOUT` |
| Reader closed | `POLLERR \| POLLHUP` | `POLLERR \| POLLHUP` |

The semantics are deliberately identical so that the same event-loop code can
handle both without special-casing.

---

## 4. Common Misconceptions

### "I'll use a pipe because it's simpler"

Pipes do not support thing passing.  If your protocol ever needs to transfer
a capability thing, you must use a channel.  Starting with a channel avoids a
later refactor.

### "I'll use a channel for stdio"

Channels are not byte streams.  Each `channel_send` is a discrete message.  A
`cat` reading from a channel thing would receive one message per `channel_send`
call, not a continuous stream.  Use a pipe for stdio.

### "Pipes are lower overhead"

For small kernel-resident workloads both are comparable.  Channels add the
overhead of the thing table lookup; pipes add a VFS open-flags check.  Neither
difference is meaningful at application level.

---

## 5. Migration Guide

If you have existing code that uses a pipe for a message protocol:

1. Replace `SYS_PIPE` with `SYS_CHANNEL_CREATE`.
2. Replace `SYS_FS_WRITE(write_thing, …)` with `SYS_CHANNEL_SEND_ALL(write_thing, …)`.
3. Replace `SYS_FS_READ(read_thing, …)` with `SYS_CHANNEL_RECV(read_thing, …)`.
4. If you bridge the things to VFS things for poll, call `SYS_FD_FROM_HANDLE` on
   each thing after creation.

---

## 6. See Also

- `docs/concepts/ipc.md` — full primitive overview and decision matrix
- `docs/concepts/channel_semantics.md` — channel specification
- `docs/concepts/readiness.md` — unified poll/wait model
- `abi/src/numbers.rs` — `SYS_CHANNEL_*` and `SYS_PIPE` syscall numbers
- `kernel/src/ipc/pipe.rs` — pipe implementation
- `kernel/src/ipc/port.rs` — channel (port) implementation
