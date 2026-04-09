//! Userland VFS provider channel — kernel side.
//!
//! When a userland process calls `SYS_FS_MOUNT`, the kernel instantiates a
//! [`ProviderFs`] and registers it in the global mount table.  From that point
//! on every VFS operation whose path falls under the mount point is serialised
//! into a [`VfsRpcOp`] message and forwarded to the provider process via the
//! IPC port it supplied.
//!
//! ## Transport
//!
//! At mount time the kernel:
//! 1. Takes the *write* handle supplied by `SYS_FS_MOUNT` — this is the port
//!    the kernel sends requests **to** the provider on.
//! 2. Creates a fresh response port.  The kernel holds `Arc<Port>` for the
//!    read side; the write handle of that port is embedded in every request so
//!    the provider can send its reply back.
//! 3. Stores the response-port write handle as a raw `u32` (`resp_write_handle`)
//!    which is placed in the first 4 bytes of every request.
//!
//! Concurrent requests to the **same** provider are serialised by
//! `ProviderFs::lock` (a spin-lock).  This is sufficient for the initial
//! implementation — true parallelism per provider can be added later.
//!
//! ## Request framing
//!
//! ```text
//! [resp_port: u32 LE] [op: u8] [_pad: u8 × 2] [payload...]
//! ```
//!
//! `op` is one of the [`VfsRpcOp`] discriminants.
//!
//! ## Response framing
//!
//! ```text
//! [status: u8]   — 0 = OK, otherwise the raw `Errno` discriminant
//! [payload...]   — present only when status == 0
//! ```

use alloc::sync::Arc;
use alloc::vec;
use spin::Mutex;

use abi::{
    errors::{Errno, SysResult},
    vfs_rpc::{VFS_RPC_MAX_DATA, VFS_RPC_MAX_RESP, VfsRpcOp, VfsRpcReqHeader},
};
use crate::syscall::validate::{copyin, copyout};

use super::{VfsDriver, VfsNode, VfsStat};

// ── ProviderChannel ──────────────────────────────────────────────────────────

/// Inner state protected by the serialisation lock.
struct ProviderChannel {
    /// The provider's request port (kernel → provider).
    req: Arc<crate::ipc::Port>,
    /// The kernel's private response port (provider → kernel).
    resp: Arc<crate::ipc::Port>,
    /// Write handle for the response port.  Sent to the provider in every
    /// request header so the provider knows where to send its reply.
    resp_write_handle: u32,
}

impl ProviderChannel {
    /// Perform a blocking round-trip RPC with the provider.
    ///
    /// Sends `req_payload` to the provider's request port and blocks until a
    /// response arrives on the response port.  Returns the raw response bytes
    /// (starting with the 1-byte status).
    fn rpc(&self, op: VfsRpcOp, payload: &[u8]) -> SysResult<alloc::vec::Vec<u8>> {
        crate::kinfo!(
            "providerfs: rpc send op={:?} payload_len={} resp_handle={}",
            op,
            payload.len(),
            self.resp_write_handle
        );
        // Build header + payload in a single contiguous buffer.
        let hdr = VfsRpcReqHeader {
            resp_port: self.resp_write_handle,
            op: op as u8,
            _pad: [0, 0],
        };
        let hdr_size = core::mem::size_of::<VfsRpcReqHeader>();
        let mut msg = vec![0u8; hdr_size + payload.len()];
        // SAFETY: VfsRpcReqHeader is repr(C, packed) and fully initialised.
        unsafe {
            core::ptr::copy_nonoverlapping(
                &hdr as *const VfsRpcReqHeader as *const u8,
                msg.as_mut_ptr(),
                hdr_size,
            );
        }
        msg[hdr_size..].copy_from_slice(payload);

        // Send the request.
        let written = self.req.send(&msg);
        if written < msg.len() {
            return Err(Errno::EIO);
        }

        // Block-wait for the response on the private response port.
        let mut resp_buf = vec![0u8; VFS_RPC_MAX_RESP];
        let n = self.recv_response(&mut resp_buf)?;
        resp_buf.truncate(n);
        crate::kinfo!("providerfs: rpc recv op={:?} resp_len={}", op, resp_buf.len());
        Ok(resp_buf)
    }

    /// Blocking recv from the response port without going through the handle
    /// table (no copy to/from user memory required).
    fn recv_response(&self, buf: &mut [u8]) -> SysResult<usize> {
        let tid = unsafe { crate::sched::current_tid_current() };
        loop {
            let n = self.resp.try_recv(buf);
            if n > 0 {
                return Ok(n);
            }

            if !self.resp.has_writers() {
                return Err(Errno::EPIPE);
            }

            self.resp.add_waiter_read(tid);
            let n = self.resp.try_recv(buf);
            if n > 0 {
                self.resp.remove_waiter_read(tid);
                return Ok(n);
            }
            if !self.resp.has_writers() {
                self.resp.remove_waiter_read(tid);
                return Err(Errno::EPIPE);
            }

            // Park this task; the provider will wake us when the response arrives.
            unsafe {
                crate::sched::block_current_erased();
            }
        }
    }
}

// ── ProviderFs ───────────────────────────────────────────────────────────────

/// A [`VfsDriver`] that forwards all operations to a userland provider via IPC.
pub struct ProviderFs {
    channel: Mutex<ProviderChannel>,
}

impl ProviderFs {
    /// Create a new `ProviderFs` from the kernel-facing side of an IPC port.
    ///
    /// * `req_port`          — the port the kernel sends requests *to* (the
    ///                         provider reads from its own end of this port).
    /// * `resp_port`         — the kernel's response port (provider writes
    ///                         replies here; kernel reads replies from here).
    /// * `resp_write_handle` — the *write* handle id for `resp_port` that was
    ///                         registered in the global handle table so the
    ///                         provider can call `SYS_channel_send(resp_write_handle, …)`.
    pub fn new(
        req_port: Arc<crate::ipc::Port>,
        resp_port: Arc<crate::ipc::Port>,
        resp_write_handle: u32,
    ) -> Self {
        Self {
            channel: Mutex::new(ProviderChannel {
                req: req_port,
                resp: resp_port,
                resp_write_handle,
            }),
        }
    }
}

impl VfsDriver for ProviderFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        let path_bytes = path.as_bytes();
        let path_len = path_bytes.len() as u32;

        let mut payload = vec![0u8; 4 + path_bytes.len()];
        payload[..4].copy_from_slice(&path_len.to_le_bytes());
        payload[4..].copy_from_slice(path_bytes);

        let resp = self.channel.lock().rpc(VfsRpcOp::Lookup, &payload)?;
        parse_response_handle(&resp).map(|handle| {
            Arc::new(ProviderNode {
                handle,
                // Share the ProviderFs channel via a fresh Arc so we don't
                // need to clone the whole ProviderFs.
                channel: Arc::new(Mutex::new(ProviderChannelRef {
                    req: self.channel.lock().req.clone(),
                    resp: self.channel.lock().resp.clone(),
                    resp_write_handle: self.channel.lock().resp_write_handle,
                })),
            }) as Arc<dyn VfsNode>
        })
    }
}

// Helper: parse a `[status: u8][handle: u64 LE]` response.
fn parse_response_handle(resp: &[u8]) -> SysResult<u64> {
    if resp.is_empty() {
        return Err(Errno::EIO);
    }
    if resp[0] != 0 {
        return Err(errno_from_u8(resp[0]));
    }
    if resp.len() < 9 {
        return Err(Errno::EIO);
    }
    Ok(u64::from_le_bytes([
        resp[1], resp[2], resp[3], resp[4], resp[5], resp[6], resp[7], resp[8],
    ]))
}

// ── ProviderChannelRef ────────────────────────────────────────────────────────

/// A reference to the channel that can be cloned independently of the full
/// `ProviderFs` (used inside `ProviderNode`).
struct ProviderChannelRef {
    req: Arc<crate::ipc::Port>,
    resp: Arc<crate::ipc::Port>,
    resp_write_handle: u32,
}

impl ProviderChannelRef {
    fn rpc(&self, op: VfsRpcOp, payload: &[u8]) -> SysResult<alloc::vec::Vec<u8>> {
        crate::kinfo!(
            "providernode: rpc send op={:?} payload_len={} resp_handle={}",
            op,
            payload.len(),
            self.resp_write_handle
        );
        let hdr = VfsRpcReqHeader {
            resp_port: self.resp_write_handle,
            op: op as u8,
            _pad: [0, 0],
        };
        let hdr_size = core::mem::size_of::<VfsRpcReqHeader>();
        let mut msg = vec![0u8; hdr_size + payload.len()];
        unsafe {
            core::ptr::copy_nonoverlapping(
                &hdr as *const VfsRpcReqHeader as *const u8,
                msg.as_mut_ptr(),
                hdr_size,
            );
        }
        msg[hdr_size..].copy_from_slice(payload);

        let written = self.req.send(&msg);
        if written < msg.len() {
            return Err(Errno::EIO);
        }

        let mut resp_buf = vec![0u8; VFS_RPC_MAX_RESP];
        let n = self.recv_response(&mut resp_buf)?;
        resp_buf.truncate(n);
        crate::kinfo!("providernode: rpc recv op={:?} resp_len={}", op, resp_buf.len());
        Ok(resp_buf)
    }

    fn recv_response(&self, buf: &mut [u8]) -> SysResult<usize> {
        let tid = unsafe { crate::sched::current_tid_current() };
        loop {
            let n = self.resp.try_recv(buf);
            if n > 0 {
                return Ok(n);
            }
            if !self.resp.has_writers() {
                return Err(Errno::EPIPE);
            }
            self.resp.add_waiter_read(tid);
            let n = self.resp.try_recv(buf);
            if n > 0 {
                self.resp.remove_waiter_read(tid);
                return Ok(n);
            }
            if !self.resp.has_writers() {
                self.resp.remove_waiter_read(tid);
                return Err(Errno::EPIPE);
            }
            unsafe {
                crate::sched::block_current_erased();
            }
        }
    }
}

// ── ProviderNode ─────────────────────────────────────────────────────────────

/// An open file or directory handle backed by a userland provider.
pub struct ProviderNode {
    /// Provider-assigned opaque handle.
    handle: u64,
    /// Shared channel reference (shared with the parent `ProviderFs`).
    channel: Arc<Mutex<ProviderChannelRef>>,
}

// SAFETY: `ProviderNode` is `Send + Sync` because the `Mutex` provides
// interior-mutability safety and `Arc` is thread-safe.
unsafe impl Send for ProviderNode {}
unsafe impl Sync for ProviderNode {}

impl VfsNode for ProviderNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let len = buf.len().min(abi::vfs_rpc::VFS_RPC_MAX_DATA) as u32;
        let mut payload = [0u8; 8 + 8 + 4];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..16].copy_from_slice(&offset.to_le_bytes());
        payload[16..20].copy_from_slice(&len.to_le_bytes());

        let resp = self.channel.lock().rpc(VfsRpcOp::Read, &payload)?;
        parse_response_read(&resp, buf)
    }

    fn write(&self, offset: u64, data: &[u8]) -> SysResult<usize> {
        let data_len = data.len().min(abi::vfs_rpc::VFS_RPC_MAX_DATA) as u32;
        let actual_data = &data[..data_len as usize];

        let mut payload = vec![0u8; 8 + 8 + 4 + actual_data.len()];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..16].copy_from_slice(&offset.to_le_bytes());
        payload[16..20].copy_from_slice(&data_len.to_le_bytes());
        payload[20..].copy_from_slice(actual_data);

        let resp = self.channel.lock().rpc(VfsRpcOp::Write, &payload)?;
        parse_response_u32(&resp).map(|n| n as usize)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        let payload = self.handle.to_le_bytes();
        let resp = self.channel.lock().rpc(VfsRpcOp::Stat, &payload)?;
        parse_response_stat(&resp)
    }

    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let len = buf.len().min(abi::vfs_rpc::VFS_RPC_MAX_DATA) as u32;
        let mut payload = [0u8; 8 + 8 + 4];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..16].copy_from_slice(&offset.to_le_bytes());
        payload[16..20].copy_from_slice(&len.to_le_bytes());

        let resp = self.channel.lock().rpc(VfsRpcOp::Readdir, &payload)?;
        parse_response_read(&resp, buf)
    }

    fn close(&self) {
        let payload = self.handle.to_le_bytes();
        let _ = self.channel.lock().rpc(VfsRpcOp::Close, &payload);
    }

    fn device_call(&self, call: &abi::device::DeviceCall) -> SysResult<usize> {
        let in_len = call.in_len as usize;
        let out_len = call.out_len as usize;

        if in_len > VFS_RPC_MAX_DATA || out_len > VFS_RPC_MAX_DATA {
            return Err(Errno::EINVAL);
        }

        let mut payload = vec![0u8; 8 + core::mem::size_of::<abi::device::DeviceCall>() + in_len];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());

        // Copy DeviceCall struct
        unsafe {
            core::ptr::copy_nonoverlapping(
                call as *const _ as *const u8,
                payload[8..].as_mut_ptr(),
                core::mem::size_of::<abi::device::DeviceCall>(),
            );
        }

        // Copy in_data if present
        if in_len > 0 {
            unsafe {
                copyin(&mut payload[8 + core::mem::size_of::<abi::device::DeviceCall>()..], call.in_ptr as usize)?;
            }
        }

        let resp = self.channel.lock().rpc(VfsRpcOp::DeviceCall, &payload)?;

        // Parse response: [status][ret_val: u32][actual_out_len: u32][out_data...]
        if resp.is_empty() {
            return Err(Errno::EIO);
        }
        if resp[0] != 0 {
            return Err(errno_from_u8(resp[0]));
        }
        if resp.len() < 9 {
            return Err(Errno::EIO);
        }

        let ret_val = u32::from_le_bytes([resp[1], resp[2], resp[3], resp[4]]);
        let actual_out_len = u32::from_le_bytes([resp[5], resp[6], resp[7], resp[8]]) as usize;

        if actual_out_len > 0 && out_len > 0 {
            let copy_n = actual_out_len.min(out_len).min(resp.len() - 9);
            unsafe {
                copyout(call.out_ptr as usize, &resp[9..9 + copy_n])?;
            }
        }

        Ok(ret_val as usize)
    }
}

// ── Response parsers ─────────────────────────────────────────────────────────

fn parse_response_read(resp: &[u8], buf: &mut [u8]) -> SysResult<usize> {
    if resp.is_empty() {
        return Err(Errno::EIO);
    }
    if resp[0] != 0 {
        return Err(errno_from_u8(resp[0]));
    }
    if resp.len() < 5 {
        return Err(Errno::EIO);
    }
    let n = u32::from_le_bytes([resp[1], resp[2], resp[3], resp[4]]) as usize;
    let data = &resp[5..];
    let copy_n = n.min(data.len()).min(buf.len());
    buf[..copy_n].copy_from_slice(&data[..copy_n]);
    Ok(copy_n)
}

fn parse_response_u32(resp: &[u8]) -> SysResult<u32> {
    if resp.is_empty() {
        return Err(Errno::EIO);
    }
    if resp[0] != 0 {
        return Err(errno_from_u8(resp[0]));
    }
    if resp.len() < 5 {
        return Err(Errno::EIO);
    }
    Ok(u32::from_le_bytes([resp[1], resp[2], resp[3], resp[4]]))
}

fn parse_response_stat(resp: &[u8]) -> SysResult<VfsStat> {
    if resp.is_empty() {
        return Err(Errno::EIO);
    }
    if resp[0] != 0 {
        return Err(errno_from_u8(resp[0]));
    }
    // mode: u32, size: u64, ino: u64  →  4 + 8 + 8 = 20 bytes
    if resp.len() < 21 {
        return Err(Errno::EIO);
    }
    let mode = u32::from_le_bytes([resp[1], resp[2], resp[3], resp[4]]);
    let size = u64::from_le_bytes([
        resp[5], resp[6], resp[7], resp[8], resp[9], resp[10], resp[11], resp[12],
    ]);
    let ino = u64::from_le_bytes([
        resp[13], resp[14], resp[15], resp[16], resp[17], resp[18], resp[19], resp[20],
    ]);
    Ok(VfsStat { mode, size, ino })
}

/// Convert a raw status byte (Errno discriminant) back to `Errno`.
fn errno_from_u8(v: u8) -> Errno {
    // Try to reconstruct a known Errno from its discriminant.  Fall back to
    // EIO for unrecognised values — callers can't do better anyway.
    match v as u32 {
        1 => Errno::EPERM,
        2 => Errno::ENOENT,
        5 => Errno::EIO,
        9 => Errno::EBADF,
        11 => Errno::EAGAIN,
        12 => Errno::ENOMEM,
        13 => Errno::EACCES,
        20 => Errno::ENOTDIR,
        21 => Errno::EISDIR,
        22 => Errno::EINVAL,
        28 => Errno::ENOSPC,
        32 => Errno::EPIPE,
        38 => Errno::ENOSYS,
        _ => Errno::EIO,
    }
}
