# VFS RPC — Provider Lifecycle and Protocol Reference

This document describes the full lifecycle of a userland VFS provider in
Thing-OS, from registration through normal operation to clean shutdown.

Implementation references:
- `kernel/src/vfs/provider.rs` — kernel-side `ProviderFs`
- `abi/src/vfs_rpc.rs` — wire format and op codes
- `userspace/iso9660d/` — reference provider implementation
- `libs/ipc_helpers/` — provider server-loop helper library

---

## 1. Overview

A **VFS provider** is a userland process that handles filesystem operations for
a mounted subtree.  The kernel forwards every VFS operation (open, read, write,
stat, readdir, …) to the provider as a serialised message, waits for a reply,
and then returns the result to the original caller.

```
User process           Kernel (ProviderFs)          Provider process
─────────────          ─────────────────────         ────────────────
vfs_open("/mnt/foo")
  │                   parse path → in mounted tree
  │                   serialise Lookup(path) →────── channel_recv_all()
  │                                                  handle_lookup(path)
  │                   ◄─────────────────────────── channel_send_all(resp)
  │                   insert fd ─────────────────────────────────────────
  ◄── fd
```

---

## 2. Provider Lifecycle

### Phase 1: Create the provider channel

The provider creates a channel pair that the kernel will use to send requests:

```rust
let (vfs_write, vfs_read) = channel_create(VFS_RPC_MAX_REQ * 8)?;
```

- `vfs_write` — the kernel writes requests to this handle.
- `vfs_read` — the provider reads requests from this handle.

### Phase 2: Register with the supervisor

The provider sends `MSG_BIND_READY` to the supervisor with `vfs_write` attached
(see `docs/concepts/supervisor_protocol.md`).

The supervisor verifies the registration and calls `SYS_FS_MOUNT(vfs_write,
path)` on the provider's behalf.

### Phase 3: Mount

`SYS_FS_MOUNT(write_handle, path)` — the kernel registers a `ProviderFs` at
`path` in the global VFS tree.  The `ProviderFs` holds the write handle and a
private response channel.

From this point on, all VFS operations under `path` are forwarded to the
provider.

### Phase 4: Serve requests

The provider enters a loop:

```
loop {
    recv request
    dispatch to handler
    send response
}
```

The `libs/ipc_helpers` crate provides a ready-made `ProviderLoop` that handles
framing, dispatch, and error replies.

### Phase 5: Shutdown

**Clean shutdown:**

1. Provider sends `MSG_SERVICE_EXITING` to the supervisor.
2. Provider calls `vfs_close(vfs_read)` to drop the read end.
3. The kernel detects that the write handle's read peer is gone and marks the
   `ProviderFs` as dead.

**Unexpected crash:**

The kernel detects the dead provider because all handles to the provider's
request port are dropped.  The `ProviderFs` enters the dead state immediately.

---

## 3. Dead Provider Behaviour

When the kernel's `ProviderFs` detects that its provider is dead:

- All in-flight RPC calls return `EIO` to their callers.
- Subsequent VFS operations on the mounted subtree return `EIO`.
- The mount point remains in the VFS tree until explicitly unmounted.  Clients
  will receive `EIO` on every operation until the path is unmounted or a new
  provider is mounted.

---

## 4. Wire Protocol

### Request header (7 bytes)

Every request starts with a `VfsRpcReqHeader`:

```text
[resp_port: u32 LE][op: u8][_pad: u8][_pad: u8]
```

| Field | Type | Description |
|-------|------|-------------|
| `resp_port` | `u32 LE` | Write handle of the kernel's private response port |
| `op` | `u8` | Operation code (see §5) |
| `_pad` | `[u8; 2]` | Reserved, must be zero |

The provider **must** send the response to `resp_port` using `channel_send_all`.

### Response format

```text
[status: u8][payload bytes...]
```

| `status` | Meaning |
|----------|---------|
| `0` | OK — payload follows |
| non-zero | Errno value — no payload |

---

## 5. Operation Codes

| Code | Name | Request payload | Response payload (on OK) |
|------|------|-----------------|--------------------------|
| 1 | `Lookup` | `[path_len: u32][path UTF-8]` | `[handle: u64]` |
| 2 | `Read` | `[handle: u64][offset: u64][len: u32]` | `[bytes_read: u32][data...]` |
| 3 | `Write` | `[handle: u64][offset: u64][data_len: u32][data...]` | `[bytes_written: u32]` |
| 4 | `Readdir` | `[handle: u64][offset: u64][len: u32]` | `[bytes_read: u32][dirent data...]` |
| 5 | `Stat` | `[handle: u64]` | `[mode: u32][size: u64][ino: u64]` |
| 6 | `Close` | `[handle: u64]` | (empty) |
| 7 | `Poll` | `[handle: u64][events: u32]` | `[revents: u32]` |
| 8 | `DeviceCall` | `[handle: u64][DeviceCall struct]` | `[u32 return value]` |
| 9 | `SubscribeReady` | `[handle: u64][events: u32]` | (empty) |
| 10 | `UnsubscribeReady` | `[handle: u64]` | (empty) |
| 11 | `Rename` | `[old_len: u32][old_path][new_len: u32][new_path]` | (empty) |

### Dirent wire encoding

Each directory entry in a `Readdir` response is a packed
`DirentWire` followed immediately by `name_len` UTF-8 bytes (no NUL):

```text
[ino: u64][file_type: u8][name_len: u8][name bytes...]
```

---

## 6. Size Limits

| Constant | Value | Description |
|----------|-------|-------------|
| `VFS_RPC_MAX_PATH` | 4096 | Maximum path length in a `Lookup` request |
| `VFS_RPC_MAX_DATA` | 65536 | Maximum data in a `Read`/`Write` payload |
| `VFS_RPC_MAX_RESP` | 65600 | Maximum response buffer the provider should allocate |
| `VFS_RPC_MAX_REQ` | 65607 | Maximum request buffer size |

---

## 7. Required Responses

A provider **must** reply to every request.  Failing to reply causes the
calling thread to block indefinitely.

For operations the provider does not support, reply with `errno::ENOSYS` (38).

For unknown op codes, reply with `errno::EINVAL` (22).

---

## 8. Timeout and Cancellation

There is no per-RPC timeout in the current kernel implementation.  A provider
that stalls on a request will stall the calling user thread indefinitely.
Providers **must** not block indefinitely inside a request handler.

Future work may add a per-mount timeout and automatic dead-provider detection
on the first stalled RPC.

---

## 9. Using the `ipc_helpers` Provider Loop

The `libs/ipc_helpers` crate provides `ProviderLoop` which handles all framing:

```rust
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use stem::syscall::channel_recv_all;

let mut loop_ = ProviderLoop::new(vfs_read);
loop {
    let req = loop_.next_request()?;  // blocks until a request arrives
    let resp = match req.op {
        VfsRpcOp::Lookup => handle_lookup(&req.payload),
        VfsRpcOp::Read   => handle_read(&req.payload),
        // …
        _ => ProviderResponse::err(abi::errors::Errno::ENOSYS),
    };
    loop_.send_response(req.resp_port, resp)?;
}
```

---

## 10. See Also

- `abi/src/vfs_rpc.rs` — wire types
- `kernel/src/vfs/provider.rs` — kernel `ProviderFs`
- `kernel/src/syscall/handlers/vfs.rs` — `SYS_FS_MOUNT` handler
- `userspace/iso9660d/src/main.rs` — reference implementation
- `libs/ipc_helpers/src/provider.rs` — provider server-loop helper
- `docs/concepts/supervisor_protocol.md` — registration handshake
- `docs/concepts/ipc.md` — IPC overview
