# Memfd: the Bulk-Data Path

Thing-OS uses a **two-tier IPC model**: small control messages flow over
channels, and large or zero-copy data travels via memory-mapped file descriptors
(**memfd**).

## Doctrine: control over channels, bulk over memfd

| Concern | Mechanism | When to use |
|---|---|---|
| Commands, events, ACKs | `channel_send` / `channel_recv` | latency-sensitive; fits in a few hundred bytes |
| Pixel buffers, audio rings, large blobs | `memfd_create` + `vm_map` + `channel_send_handle` | throughput-sensitive; larger than a channel ring |

Never embed multi-kilobyte payloads in channel messages.  Instead, put the data
in a memfd and send the file descriptor across the channel with
`channel_send_handle`.  The control message then carries only a small
[`abi::memfd::MemFdRef`] descriptor (16 bytes) that names the fd, and the byte
length of the valid window.

## Memfd lifetime and reference counting

The kernel maintains a reference count on each memfd region:

* A `memfd_create` call allocates contiguous physical frames and returns an
  open file descriptor (`fd`).
* Every `vm_map` call that backs itself on the fd increments the count.
* Calling `vfs_close(fd)` decrements the fd's reference.
* Calling `vm_unmap` decrements the mapping's reference.
* **Physical memory is freed only when the last fd and the last mapping are
  both gone.**

This means a receiver can safely hold onto its mapping even after the sender
has closed its own copy of the fd.

## Sharing a memfd across processes

```
Creator                          Receiver
───────                          ────────
memfd_create("frame", size)  →  fd

vm_map(fd, READ|WRITE|USER)  →  ptr
[fill pixel data at ptr]

channel_send_handle(chan, fd)  ───────────►  channel_recv_handle(chan) → new_fd
                                              vm_map(new_fd, READ|USER) → ptr
                                              [read pixel data at ptr]
                                              vm_unmap(ptr, size)
                                              vfs_close(new_fd)

vm_unmap(ptr, size)
vfs_close(fd)
           ╰── last reference dropped → physical pages freed
```

### Step-by-step (Rust / stem wrappers)

**Sender side**

```rust
use stem::syscall::{memfd_create, vm_map, channel_send_handle};
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use abi::memfd::MemFdRef;

let fd = memfd_create("frame", width * height * 4)?;

let req = VmMapReq {
    addr_hint: 0,
    len: width * height * 4,
    prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
    flags: VmMapFlags::empty(),
    backing: VmBacking::File { fd, offset: 0 },
};
let mapped = vm_map(&req)?;
let pixels: &mut [u32] = unsafe {
    core::slice::from_raw_parts_mut(mapped.addr as *mut u32, width * height)
};

// … fill pixels …

// Build the descriptor that will travel in the control message.
let desc = MemFdRef::new(fd, (width * height * 4) as u64);

// Transfer ownership of the fd to the receiver.
channel_send_handle(channel, fd as usize)?;

// Encode the descriptor into the control message.
let mut ctrl = [0u8; 17];
ctrl[0] = MSG_PRESENT;
desc.encode_le(&mut ctrl[1..]).unwrap();
channel_send(channel, &ctrl)?;
```

**Receiver side**

```rust
use stem::syscall::{channel_recv_handle, channel_recv, vm_map};
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use abi::memfd::MemFdRef;

// First receive the fd itself.
let new_fd = channel_recv_handle(channel)?;

// Then receive the control message with the descriptor.
let mut ctrl = [0u8; 17];
channel_recv(channel, &mut ctrl)?;
let desc = MemFdRef::decode_le(&ctrl[1..]).unwrap();

let req = VmMapReq {
    addr_hint: 0,
    len: desc.length as usize,
    prot: VmProt::READ | VmProt::USER,
    flags: VmMapFlags::empty(),
    backing: VmBacking::File { fd: new_fd, offset: 0 },
};
let mapped = vm_map(&req)?;
let pixels: &[u32] = unsafe {
    core::slice::from_raw_parts(mapped.addr as *const u32, desc.length as usize / 4)
};

// … consume pixels …

vm_unmap(mapped.addr, desc.length as usize)?;
vfs_close(new_fd)?;
```

## Revocation

There is no forced-unmap primitive.  A sender signals "I am done with this
buffer" by closing its own fd and sending a companion control message (e.g.
`MSG_BUFFER_RELEASED`).  The receiver is responsible for unmapping promptly
when it receives that signal.  As long as the receiver holds its fd open, the
physical memory remains allocated — leaking is possible, so protocol designers
must include an explicit release signal.

## Persistent shared rings

For streaming paths (audio, input, network) that need ultra-low latency without
repeated transfer handshakes, allocate the memfd once at setup and share it for
the lifetime of the session:

1. Server calls `memfd_create("ring", RING_BYTES)` and maps it read-write.
2. Server sends the fd to the client at connection time via
   `channel_send_handle`.
3. Client maps it and keeps the mapping open.
4. Both sides use an atomic sequence-number protocol (in the shared memory) to
   signal produce/consume positions — no fd transfers on the hot path.
5. When the session ends, both sides close their fd and unmap.

## Wire descriptor: `abi::memfd::MemFdRef`

All control-plane messages that accompany a memfd transfer **must** embed a
[`MemFdRef`] so the receiver can verify the expected length:

| Byte offset | Size | Field | Notes |
|---|---|---|---|
| 0 | 4 | `fd` | Sender-local fd (receiver obtains its own via `channel_recv_handle`) |
| 4 | 4 | `_pad` | Reserved, must be zero |
| 8 | 8 | `length` | Byte length of the valid data window |

For pixel buffers, use [`abi::display::types::BufferHandle`] which extends this
with `width`, `height`, `stride`, `format`, and `modifier` fields.

## Existing in-tree users

| Crate | Pattern |
|---|---|
| `display_bootfb` | Bootstrap memfd passed as argv fd to deliver channel handles |
| `display_virtio_gpu` | Frame-pool memfd created at startup; textures uploaded via per-frame memfds |
| `petals` | `Texture` type wraps a memfd + `vm_map` pointer for CPU-side pixel writes |
| `sprout` | Bootstrap memfd used to deliver driver channel handles at spawn time |
| `blossom` SVG | `SvgSource::MemFd(fd)` transfers SVG bytes without copying into the IPC ring |
