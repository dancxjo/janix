# IPC Cookbook — Practical Recipes for Driver and Service Authors

This cookbook shows how to use the Thing-OS IPC primitives for the most common
patterns.  Each recipe is self-contained.  Prerequisites: read
`docs/concepts/ipc.md` first.

---

## Recipe 1 — Parent/child stdio via pipe

**Problem**: spawn a child process and capture its stdout.

```rust
use stem::syscall::vfs::{pipe, vfs_close, vfs_read};
use stem::syscall::process::spawn_process;

fn capture_child_stdout() {
    let mut pipefds = [0u32; 2];
    pipe(&mut pipefds).expect("pipe");
    let (pr, pw) = (pipefds[0], pipefds[1]);

    // Spawn child; pass write end as its stdout (fd 1).
    spawn_process("child", &[], &[("STDOUT_FD", &pw.to_string())])
        .expect("spawn");

    // Close our copy of the write end so we see EOF when child exits.
    vfs_close(pw).expect("close write");

    // Read child output.
    let mut buf = [0u8; 256];
    loop {
        match vfs_read(pr, &mut buf) {
            Ok(0) | Err(_) => break,   // EOF or error
            Ok(n) => {
                let s = core::str::from_utf8(&buf[..n]).unwrap_or("?");
                stem::print!("{}", s);
            }
        }
    }
    vfs_close(pr).unwrap();
}
```

---

## Recipe 2 — Request/reply service over a channel

**Problem**: expose a service that answers typed requests and sends typed
replies.

### Server side

```rust
use stem::syscall::channel::{channel_create, channel_recv, channel_send_all};
use abi::rpc::{RpcHeader, RPC_FLAG_REPLY};

fn run_service(capacity: usize) {
    let (write_h, read_h) = channel_create(capacity).expect("channel_create");
    // Publish write_h to clients (e.g. via supervisor registration).

    let mut buf = [0u8; 512];
    loop {
        let n = channel_recv(read_h, &mut buf).expect("recv");
        if n < RpcHeader::WIRE_SIZE {
            continue; // too short, ignore
        }
        let hdr = RpcHeader::decode_le(&buf[..RpcHeader::WIRE_SIZE]).unwrap();
        let payload = &buf[RpcHeader::WIRE_SIZE..n];

        // Dispatch …
        let reply_payload: &[u8] = b"ok";

        let reply_hdr = RpcHeader {
            request_id: hdr.request_id,
            flags: RPC_FLAG_REPLY,
            _pad: [0; 5],
        };
        let mut out = [0u8; 512];
        reply_hdr.encode_le(&mut out[..RpcHeader::WIRE_SIZE]).unwrap();
        out[RpcHeader::WIRE_SIZE..RpcHeader::WIRE_SIZE + reply_payload.len()]
            .copy_from_slice(reply_payload);
        channel_send_all(
            write_h,    // use the client's reply thing if passed separately
            &out[..RpcHeader::WIRE_SIZE + reply_payload.len()],
        ).ok();
    }
}
```

### Client side

```rust
use stem::syscall::channel::{channel_recv, channel_send_all};
use abi::rpc::{RpcHeader, RPC_FLAG_REQUEST};

fn call_service(svc_write: u32, svc_read: u32) {
    let request_id: u64 = 42;
    let hdr = RpcHeader {
        request_id,
        flags: RPC_FLAG_REQUEST,
        _pad: [0; 5],
    };
    let mut msg = [0u8; 16];
    hdr.encode_le(&mut msg[..RpcHeader::WIRE_SIZE]).unwrap();
    msg[RpcHeader::WIRE_SIZE..].copy_from_slice(b"hello\0\0\0");
    channel_send_all(svc_write, &msg).expect("send");

    // Wait for reply.
    let mut reply = [0u8; 512];
    let n = channel_recv(svc_read, &mut reply).expect("recv");
    let reply_hdr = RpcHeader::decode_le(&reply[..RpcHeader::WIRE_SIZE]).unwrap();
    assert_eq!(reply_hdr.request_id, request_id);
    let reply_payload = &reply[RpcHeader::WIRE_SIZE..n];
    stem::println!("reply: {:?}", reply_payload);
}
```

---

## Recipe 3 — Handle passing (first-class message API)

**Problem**: transfer one or more things (capabilities) from one
process to another as part of a message.

### New API (preferred): `channel_send_msg` / `channel_recv_msg`

Handles are **first-class properties of a message** — they travel atomically
alongside the data bytes in the same message unit.

```rust
use stem::syscall::channel::{channel_send_msg, channel_recv_msg};

// Sender: send data bytes + two fds in one atomic operation.
fn send_fds(channel: u32, fd1: u32, fd2: u32) {
    let handles = [fd1, fd2];
    channel_send_msg(channel, b"two-fds", &handles).expect("send_msg");
}

// Receiver: receive data + fds together.
fn recv_fds(channel: u32) -> (u32, u32) {
    let mut data = [0u8; 16];
    let mut new_fds = [0u32; 2];
    let (data_len, handles_count) =
        channel_recv_msg(channel, &mut data, &mut new_fds).expect("recv_msg");
    assert_eq!(&data[..data_len], b"two-fds");
    assert_eq!(handles_count, 2);
    (new_fds[0], new_fds[1])
}
```

The kernel re-numbers each thing in the receiver's thing table.  Duplicate
semantics: the sender retains its own thing.

### Legacy API (compatibility): `channel_send_handle` / `channel_recv_handle`

The old single-thing API is still supported as a compatibility wrapper:

```rust
use stem::syscall::channel::{channel_send_handle, channel_recv_handle, channel_send_all, channel_recv};

// Sender:
fn send_fd(channel: u32, fd: u32) {
    channel_send_handle(channel, fd).expect("send_handle");
    channel_send_all(channel, b"fd-ready").expect("send");
}

// Receiver:
fn recv_fd(channel: u32) -> u32 {
    let new_fd = channel_recv_handle(channel).expect("recv_handle");
    let mut tag = [0u8; 8];
    channel_recv(channel, &mut tag).expect("recv");
    assert_eq!(&tag, b"fd-ready");
    new_fd
}
```

The kernel re-numbers the fd in the receiver's fd table.  The receiver can
use `new_fd` with any `SYS_FS_*` syscall immediately.

> **Note**: The new `channel_send_msg` / `channel_recv_msg` API is preferred
> for all new code.  The old `send_handle` / `recv_handle` pair is kept for
> backward compatibility and is implemented as a thin wrapper.

---

## Recipe 4 — VFS provider implementation

**Problem**: expose a virtual directory tree under a mount point.

See `libs/ipc_helpers/src/provider.rs` for the `ProviderLoop` helper, and
`userspace/iso9660d/` for a complete reference implementation.

Minimal skeleton:

```rust
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use abi::vfs_rpc::VfsRpcOp;
use abi::errors::Errno;

fn run_provider(vfs_read: u32) {
    let mut lp = ProviderLoop::new(vfs_read);
    loop {
        let req = match lp.next_request() {
            Ok(r) => r,
            Err(_) => break, // channel closed, shutdown
        };
        let resp = match req.op {
            VfsRpcOp::Lookup => {
                let path = core::str::from_utf8(&req.payload[4..]).unwrap_or("");
                if path == "hello.txt" {
                    ProviderResponse::ok_u64(1) // handle = 1
                } else {
                    ProviderResponse::err(Errno::ENOENT)
                }
            }
            VfsRpcOp::Read => ProviderResponse::ok_bytes(b"Hello, world!\n"),
            VfsRpcOp::Stat => ProviderResponse::ok_stat(0o100644, 14, 1),
            VfsRpcOp::Close => ProviderResponse::ok_empty(),
            _ => ProviderResponse::err(Errno::ENOSYS),
        };
        lp.send_response(req.resp_port, resp).ok();
    }
}
```

---

## Recipe 5 — Memfd-backed shared buffer exchange

**Problem**: transfer a large pixel buffer from a display driver to a compositor
without copying.

```rust
use stem::syscall::memory::{memfd_create, vm_map, vm_unmap};
use stem::syscall::channel::{channel_send_handle, channel_recv_handle, channel_send_all, channel_recv};
use abi::memfd::MemFdRef;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};

const W: usize = 1920;
const H: usize = 1080;
const BPP: usize = 4;
const SIZE: usize = W * H * BPP;

// Sender (display driver):
fn send_frame(channel: u32) {
    let fd = memfd_create("frame", SIZE as u64).unwrap();
    let req = VmMapReq {
        addr_hint: 0,
        len: SIZE,
        prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
        flags: VmMapFlags::empty(),
        backing: VmBacking::File { fd, offset: 0 },
    };
    let mapped = vm_map(&req).unwrap();
    let pixels = unsafe {
        core::slice::from_raw_parts_mut(mapped.addr as *mut u32, W * H)
    };
    // … render into pixels …

    let desc = MemFdRef::new(fd, SIZE as u64);
    channel_send_handle(channel, fd).unwrap();
    let mut ctrl = [0u8; 1 + abi::memfd::MEMFD_REF_WIRE_SIZE];
    ctrl[0] = 0x01; // MSG_PRESENT
    desc.encode_le(&mut ctrl[1..]).unwrap();
    channel_send_all(channel, &ctrl).unwrap();

    vm_unmap(mapped.addr, SIZE).unwrap();
    // vfs_close(fd) would free physical pages once receiver also closes
}

// Receiver (compositor):
fn recv_frame(channel: u32) {
    let new_fd = channel_recv_handle(channel).unwrap();
    let mut ctrl = [0u8; 1 + abi::memfd::MEMFD_REF_WIRE_SIZE];
    channel_recv(channel, &mut ctrl).unwrap();
    let desc = MemFdRef::decode_le(&ctrl[1..]).unwrap();

    let req = VmMapReq {
        addr_hint: 0,
        len: desc.length as usize,
        prot: VmProt::READ | VmProt::USER,
        flags: VmMapFlags::empty(),
        backing: VmBacking::File { fd: new_fd, offset: 0 },
    };
    let mapped = vm_map(&req).unwrap();
    let pixels = unsafe {
        core::slice::from_raw_parts(mapped.addr as *const u32, W * H)
    };
    // … consume pixels …
    vm_unmap(mapped.addr, desc.length as usize).unwrap();
}
```

---

## Recipe 6 — Poll-based event loop

**Problem**: wait on a channel, a pipe read end, and a device file all at once.

```rust
use stem::syscall::vfs::{vfs_poll, vfs_fd_from_handle};
use abi::syscall::{PollFd, poll_flags};

fn event_loop(pipe_read: u32, channel_write_h: u32, channel_read_h: u32, dev_fd: u32) {
    // Bridge the channel read thing into a VFS thing for poll.
    let channel_fd = vfs_fd_from_handle(channel_read_h).expect("bridge");

    let mut fds = [
        PollFd { fd: pipe_read as i32,  events: poll_flags::POLLIN, revents: 0 },
        PollFd { fd: channel_fd as i32, events: poll_flags::POLLIN, revents: 0 },
        PollFd { fd: dev_fd as i32,     events: poll_flags::POLLIN, revents: 0 },
    ];

    loop {
        let n = vfs_poll(&mut fds, u64::MAX).expect("poll");
        if n == 0 { continue; }

        for entry in &fds {
            if entry.revents == 0 { continue; }
            if entry.revents & poll_flags::POLLHUP != 0 {
                // Peer closed — clean up and exit.
                return;
            }
            if entry.revents & poll_flags::POLLIN != 0 {
                // Read available data.
                let mut buf = [0u8; 256];
                stem::syscall::vfs::vfs_read(entry.fd as u32, &mut buf).ok();
            }
        }

        for entry in &mut fds { entry.revents = 0; }
    }
}
```

---

## Recipe 7 — Supervisor startup/registration handshake

**Problem**: register a new driver with the supervisor so it gets a path under
`/dev`.

See `docs/concepts/supervisor_protocol.md` for the full specification and
`drivers/display_bootfb/src/main.rs` for a complete reference implementation.

Minimal sketch:

```rust
use stem::syscall::channel::{channel_create, channel_send_handle, channel_send_all, channel_try_recv};
use abi::supervisor_protocol::{self, classes};
use abi::vfs_rpc::VFS_RPC_MAX_REQ;

fn register_driver(drv_req_read: u32, drv_resp_write: u32, bind_instance_id: u64) {
    // 1. Create provider channel.
    let (vfs_write, vfs_read) = channel_create(VFS_RPC_MAX_REQ * 8).unwrap();

    // 2. Send provider thing + BIND_READY.
    let payload = supervisor_protocol::BindReadyPayload {
        bind_instance_id,
        class_mask: classes::DISPLAY_CARD | classes::FRAMEBUFFER,
        _reserved: 0,
    };
    let mut payload_bytes = [0u8; supervisor_protocol::BIND_READY_PAYLOAD_SIZE];
    supervisor_protocol::encode_bind_ready_le(&payload, &mut payload_bytes);

    channel_send_handle(drv_resp_write, vfs_write).unwrap();
    // encode into a framed message…
    channel_send_all(drv_resp_write, &payload_bytes).unwrap();

    // 3. Wait for BIND_ASSIGNED.
    let mut buf = [0u8; 256];
    loop {
        if let Ok(n) = channel_try_recv(drv_req_read, &mut buf) {
            // parse msg_type, check for MSG_BIND_ASSIGNED …
            break;
        }
        stem::syscall::process::yield_now();
    }

    // 4. Serve VFS requests on vfs_read.
    run_provider(vfs_read);
}
```

---

## See Also

- `docs/concepts/ipc.md` — primitive overview
- `docs/concepts/channel_semantics.md` — channel specification
- `docs/concepts/channels_vs_pipes.md` — channel vs pipe
- `docs/concepts/vfs_rpc_provider.md` — VFS provider lifecycle
- `docs/concepts/memfd.md` — memfd bulk-data path
- `docs/concepts/readiness.md` — poll/wait model
- `docs/concepts/supervisor_protocol.md` — registration protocol
- `libs/ipc_helpers/` — userspace helper library
- `abi/src/rpc.rs` — request/reply header types
