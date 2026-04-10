# Channel Semantics Specification

This document is the authoritative specification for the channel primitive in
Thing-OS.  Every kernel implementation detail referenced here is in
`kernel/src/ipc/port.rs` and `kernel/src/syscall/handlers/port.rs`.

---

## 1. What Is a Channel

A **channel** is a bounded, FIFO, byte-oriented ring buffer shared between
exactly one writer handle and one reader handle.

`SYS_CHANNEL_CREATE(capacity) -> (write_handle, read_handle)`

- `capacity` is clamped to `[64, 65536]` bytes and rounded up to the next
  power of two.
- The kernel returns a packed `usize`: `(write_handle << 16) | read_handle`.
- Handle `0` is reserved and always invalid.

---

## 2. Capacity and Message Size

| Limit | Value | Notes |
|-------|-------|-------|
| Minimum ring capacity | 64 bytes | |
| Maximum ring capacity | 65536 bytes (64 KiB) | Requested via `SYS_CHANNEL_CREATE` |
| Maximum single message | 4096 bytes (4 KiB) | Enforced by `SYS_CHANNEL_SEND` / `SYS_CHANNEL_SEND_ALL` |
| Maximum attached handles | 1 per `SYS_CHANNEL_SEND_HANDLE` call | Queued independently from byte data |

> **Practical guideline**: protocol messages should fit in a few hundred bytes.
> For anything larger, embed a `abi::memfd::MemFdRef` and transfer the data
> via memfd.

---

## 3. Atomicity

### `SYS_CHANNEL_SEND` (partial write allowed)

- Writes as many bytes as the available ring space allows.
- Returns the number of bytes actually written (may be less than `len`).
- Returns `EPIPE` if the read end is closed.

### `SYS_CHANNEL_SEND_ALL` (all-or-nothing)

- If `len` bytes fit in the ring, all are written atomically.
- If `len` bytes do **not** fit, **no bytes** are written.
- Returns `EAGAIN` when the ring is full (not a partial failure).
- Use `SYS_CHANNEL_SEND_ALL` for protocol messages that must not be split.

### `SYS_CHANNEL_RECV` (blocking)

- Reads up to `len` bytes from the ring.
- Blocks until at least 1 byte is available.
- Returns `EPIPE` if the write end is closed and the ring is empty.

### `SYS_CHANNEL_TRY_RECV` (non-blocking)

- Like `SYS_CHANNEL_RECV` but returns `EAGAIN` immediately if the ring is
  empty instead of blocking.

---

## 4. FIFO Guarantee

Bytes arrive at the receiver in the same order they were written by the sender.
There is no reordering.

---

## 5. Blocking Behaviour

### Sender blocks / returns early

`SYS_CHANNEL_SEND` does **not** block; it writes what fits and returns early.

`SYS_CHANNEL_SEND_ALL` does **not** block; it fails with `EAGAIN` if the ring
is full.  The caller is responsible for retrying (or using poll/wait to wait
for `POLLOUT` on the bridged VFS fd).

### Receiver blocks

`SYS_CHANNEL_RECV` parks the calling task in the port's read wait queue until
data arrives or the write end is closed.

`SYS_CHANNEL_TRY_RECV` never blocks.

### `SYS_CHANNEL_WAIT`

Wait on one or more handles simultaneously.  The caller supplies an array of
handle values and a flags word (`READABLE | WRITABLE`).  Returns the first
handle that becomes ready.  Blocks indefinitely until at least one handle is
ready.

---

## 6. Peer Death Semantics

### Write end closes (sender exits or calls `channel_close`)

1. Any threads blocked in `SYS_CHANNEL_RECV` are woken immediately.
2. `channel_recv` drains remaining buffered bytes normally.
3. After the buffer is empty, `channel_recv` returns `EPIPE` to signal
   end-of-stream.
4. Polling the read fd reports `POLLIN | POLLHUP`.

### Read end closes (receiver exits or calls `channel_close`)

1. Any threads blocked in `SYS_CHANNEL_RECV` are woken.
2. All further `SYS_CHANNEL_SEND` and `SYS_CHANNEL_SEND_ALL` calls return
   `EPIPE`.
3. Any threads blocked in `SYS_CHANNEL_SEND` are woken immediately.
4. Polling the write fd reports `POLLERR | POLLHUP`.

### Process crash / unexpected exit

The kernel closes all handles owned by a process on exit, triggering the same
peer-death sequences above.  A service that holds a channel read handle will
observe `POLLERR | POLLHUP` on its write end within the same scheduling
quantum that the sender process exits.

---

## 7. Cleanup Semantics for Queued Handles

When a channel is closed while capability handles are still queued (i.e.
`channel_send_handle` was called but `channel_recv_handle` was not):

- The kernel drops the `Arc` reference it holds to each queued node.
- If no other references exist, the underlying VFS node is closed.
- No handles are silently leaked into any process's fd table.

---

## 8. Error Reference

| Error | Condition |
|-------|-----------|
| `EBADF` | Handle value does not exist or has the wrong mode |
| `EINVAL` | `count == 0` or `count > 64` in `channel_wait` |
| `EAGAIN` | Ring is full (`send_all`) or empty (`try_recv`) |
| `EPIPE` | The peer endpoint is closed |
| `ENOMEM` | Handle table is full (`MAX_HANDLES = 1024`) |
| `EIO` | Internal ring write shorter than expected (provider send error) |

---

## 9. Bridging Channels to VFS Poll

`SYS_FD_FROM_HANDLE(handle) -> fd`

Wraps a channel handle in a VFS file descriptor so it can participate in
`SYS_FS_POLL`.  The fd inherits the handle's mode (read or write).

Once bridged:
- `POLLIN` fires when the ring has bytes (read end).
- `POLLOUT` fires when the ring has free space (write end).
- `POLLHUP` fires when the peer has closed.
- `POLLERR` fires when the peer has closed the read end (write-end perspective).

---

## 10. Diagnostics

The kernel maintains per-port counters exposed under `/proc/ipc/channels`:

| Counter | Description |
|---------|-------------|
| `sends` | Total `channel_send` calls that wrote ≥1 byte |
| `recvs` | Total `channel_recv` calls that read ≥1 byte |
| `bytes_sent` | Cumulative bytes written |
| `bytes_recv` | Cumulative bytes read |
| `handles_sent` | Total capability handles enqueued |
| `handles_recv` | Total capability handles dequeued |
| `full_events` | Times `channel_send_all` returned `EAGAIN` due to full ring |
| `peer_deaths` | Times a peer closure was observed |

See `docs/concepts/ipc_diag.md` for how to read and interpret these counters.

---

## 11. See Also

- `kernel/src/ipc/port.rs` — ring buffer implementation
- `kernel/src/syscall/handlers/port.rs` — syscall handlers
- `abi/src/numbers.rs` — syscall numbers (`SYS_CHANNEL_*`)
- `stem/src/syscall/channel.rs` — userspace wrappers
- `docs/concepts/readiness.md` — poll/wait model
- `docs/concepts/ipc.md` — primitive overview and decision matrix
