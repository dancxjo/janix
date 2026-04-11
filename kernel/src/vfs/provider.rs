//! Userland VFS provider channel — kernel side.
//!
//! When a userland process calls `SYS_FS_MOUNT`, the kernel instantiates a
//! [`ProviderFs`] and registers it in the global mount table.  From that point
//! on every VFS operation whose path falls under the mount point is serialised
//! into a [`VfsRpcOp`] message and forwarded to the provider process via the
//! IPC port it supplied.

use alloc::collections::BTreeMap;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use spin::Mutex;

use crate::sched::wait_queue::WaitQueue;
use crate::syscall::validate::{copyin, copyout};
use abi::{
    errors::{Errno, SysResult},
    vfs_rpc::{VFS_RPC_MAX_DATA, VFS_RPC_MAX_RESP, VfsRpcOp, VfsRpcReqHeader},
};

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
    /// Map of provider-assigned handles to kernel-side wait queues.
    waiters: BTreeMap<u64, Arc<WaitQueue>>,
}

impl ProviderChannel {
    /// Perform a blocking round-trip RPC with the provider.
    fn rpc(&self, op: VfsRpcOp, payload: &[u8]) -> SysResult<alloc::vec::Vec<u8>> {
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

        crate::ipc::diag::VFS_RPC_REQUESTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

        let written = self.req.send(&msg);
        if written < msg.len() {
            crate::ipc::diag::record_dead_provider_error();
            return Err(Errno::EIO);
        }

        let mut resp_buf = vec![0u8; VFS_RPC_MAX_RESP];
        let n = self.recv_response(&mut resp_buf)?;
        resp_buf.truncate(n);
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
                crate::ipc::diag::record_dead_provider_error();
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
                crate::ipc::diag::record_dead_provider_error();
                return Err(Errno::EPIPE);
            }
            unsafe {
                crate::sched::block_current_erased();
            }
        }
    }

    fn get_wait_queue(&mut self, handle: u64) -> Arc<WaitQueue> {
        self.waiters
            .entry(handle)
            .or_insert_with(|| Arc::new(WaitQueue::new()))
            .clone()
    }
}

// ── ProviderFs ───────────────────────────────────────────────────────────────

/// A [`VfsDriver`] that forwards all operations to a userland provider via IPC.
pub struct ProviderFs {
    channel: Mutex<ProviderChannel>,
}

static PROVIDER_MAP: Mutex<BTreeMap<u32, Weak<ProviderFs>>> = Mutex::new(BTreeMap::new());

impl ProviderFs {
    pub fn new(
        req_port: Arc<crate::ipc::Port>,
        resp_port: Arc<crate::ipc::Port>,
        resp_write_handle: u32,
        req_port_id: u32,
    ) -> Arc<Self> {
        let this = Arc::new(Self {
            channel: Mutex::new(ProviderChannel {
                req: req_port,
                resp: resp_port,
                resp_write_handle,
                waiters: BTreeMap::new(),
            }),
        });
        PROVIDER_MAP
            .lock()
            .insert(req_port_id, Arc::downgrade(&this));
        this
    }

    pub fn notify(&self, handle: u64, _revents: u16) {
        let mut chan = self.channel.lock();
        if let Some(wq) = chan.waiters.get(&handle) {
            wq.wake_all();
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
            let wq = self.channel.lock().get_wait_queue(handle);
            Arc::new(ProviderNode {
                handle,
                channel: Arc::new(Mutex::new(ProviderChannelRef {
                    req: self.channel.lock().req.clone(),
                    resp: self.channel.lock().resp.clone(),
                    resp_write_handle: self.channel.lock().resp_write_handle,
                })),
                wait_queue: wq,
            }) as Arc<dyn VfsNode>
        })
    }

    fn rename(&self, old_path: &str, new_path: &str) -> SysResult<()> {
        let old_bytes = old_path.as_bytes();
        let new_bytes = new_path.as_bytes();
        let mut payload = vec![0u8; 4 + old_bytes.len() + 4 + new_bytes.len()];
        payload[0..4].copy_from_slice(&(old_bytes.len() as u32).to_le_bytes());
        payload[4..4 + old_bytes.len()].copy_from_slice(old_bytes);
        let off = 4 + old_bytes.len();
        payload[off..off + 4].copy_from_slice(&(new_bytes.len() as u32).to_le_bytes());
        payload[off + 4..].copy_from_slice(new_bytes);

        let resp = self.channel.lock().rpc(VfsRpcOp::Rename, &payload)?;
        if resp.is_empty() {
            return Err(Errno::EIO);
        }
        if resp[0] != 0 {
            return Err(errno_from_u8(resp[0]));
        }
        Ok(())
    }
}

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

struct ProviderChannelRef {
    req: Arc<crate::ipc::Port>,
    resp: Arc<crate::ipc::Port>,
    resp_write_handle: u32,
}

impl ProviderChannelRef {
    fn rpc(&self, op: VfsRpcOp, payload: &[u8]) -> SysResult<alloc::vec::Vec<u8>> {
        let hdr = VfsRpcReqHeader {
            resp_port: self.resp_write_handle,
            op: op as u8,
            _pad: [0, 0],
        };
        let hdr_size = core::mem::size_of::<VfsRpcReqHeader>();
        let mut msg = vec![0u8; hdr_size + payload.len()];
        unsafe {
            core::ptr::copy_nonoverlapping(
                &hdr as *const _ as *const u8,
                msg.as_mut_ptr(),
                hdr_size,
            );
        }
        msg[hdr_size..].copy_from_slice(payload);

        crate::ipc::diag::VFS_RPC_REQUESTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

        let written = self.req.send(&msg);
        if written < msg.len() {
            crate::ipc::diag::record_dead_provider_error();
            return Err(Errno::EIO);
        }

        let mut resp_buf = vec![0u8; VFS_RPC_MAX_RESP];
        let n = self.recv_response(&mut resp_buf)?;
        resp_buf.truncate(n);
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
                crate::ipc::diag::record_dead_provider_error();
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
                crate::ipc::diag::record_dead_provider_error();
                return Err(Errno::EPIPE);
            }
            unsafe {
                crate::sched::block_current_erased();
            }
        }
    }
}

// ── ProviderNode ─────────────────────────────────────────────────────────────

pub struct ProviderNode {
    handle: u64,
    channel: Arc<Mutex<ProviderChannelRef>>,
    wait_queue: Arc<WaitQueue>,
}

unsafe impl Send for ProviderNode {}
unsafe impl Sync for ProviderNode {}

impl VfsNode for ProviderNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let len = buf.len().min(abi::vfs_rpc::VFS_RPC_MAX_DATA) as u32;
        let mut payload = [0u8; 20];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..16].copy_from_slice(&offset.to_le_bytes());
        payload[16..20].copy_from_slice(&len.to_le_bytes());
        let resp = self.channel.lock().rpc(VfsRpcOp::Read, &payload)?;
        parse_response_read(&resp, buf)
    }

    fn write(&self, offset: u64, data: &[u8]) -> SysResult<usize> {
        let data_len = data.len().min(abi::vfs_rpc::VFS_RPC_MAX_DATA) as u32;
        let mut payload = vec![0u8; 20 + data_len as usize];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..16].copy_from_slice(&offset.to_le_bytes());
        payload[16..20].copy_from_slice(&data_len.to_le_bytes());
        payload[20..].copy_from_slice(&data[..data_len as usize]);
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
        let mut payload = [0u8; 20];
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
        unsafe {
            core::ptr::copy_nonoverlapping(
                call as *const _ as *const u8,
                payload[8..].as_mut_ptr(),
                core::mem::size_of::<abi::device::DeviceCall>(),
            );
        }
        if in_len > 0 {
            unsafe {
                copyin(
                    &mut payload[8 + core::mem::size_of::<abi::device::DeviceCall>()..],
                    call.in_ptr as usize,
                )?;
            }
        }
        let resp = self.channel.lock().rpc(VfsRpcOp::DeviceCall, &payload)?;
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

    fn poll(&self) -> u16 {
        let mut payload = [0u8; 12];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        let events = abi::syscall::poll_flags::POLLIN | abi::syscall::poll_flags::POLLOUT;
        payload[8..12].copy_from_slice(&(events as u32).to_le_bytes());
        let resp = self.channel.lock().rpc(VfsRpcOp::Poll, &payload);
        match resp {
            Ok(r) => parse_response_u32(&r).unwrap_or(0) as u16,
            Err(_) => abi::syscall::poll_flags::POLLERR,
        }
    }

    fn add_waiter(&self, tid: u64) {
        self.wait_queue.push_back(tid);
        let mut payload = [0u8; 12];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        let events = abi::syscall::poll_flags::POLLIN | abi::syscall::poll_flags::POLLOUT;
        payload[8..12].copy_from_slice(&(events as u32).to_le_bytes());
        let _ = self.channel.lock().rpc(VfsRpcOp::SubscribeReady, &payload);
    }

    fn remove_waiter(&self, tid: u64) {
        self.wait_queue.remove(tid);
        if self.wait_queue.is_empty() {
            let payload = self.handle.to_le_bytes();
            let _ = self
                .channel
                .lock()
                .rpc(VfsRpcOp::UnsubscribeReady, &payload);
        }
    }
}

pub fn notify_by_port(port_id: u32, handle: u64, revents: u16) -> SysResult<()> {
    let weak = PROVIDER_MAP
        .lock()
        .get(&port_id)
        .cloned()
        .ok_or(Errno::ENOENT)?;
    if let Some(fs) = weak.upgrade() {
        fs.notify(handle, revents);
        Ok(())
    } else {
        Err(Errno::ENOENT)
    }
}

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
    Ok(VfsStat {
        mode,
        size,
        ino,
        ..Default::default()
    })
}

fn errno_from_u8(v: u8) -> Errno {
    match v {
        1 => Errno::EPERM,
        2 => Errno::ENOENT,
        5 => Errno::EIO,
        9 => Errno::EBADF,
        11 => Errno::EAGAIN,
        12 => Errno::ENOMEM,
        13 => Errno::EACCES,
        17 => Errno::EEXIST,
        20 => Errno::ENOTDIR,
        21 => Errno::EISDIR,
        22 => Errno::EINVAL,
        28 => Errno::ENOSPC,
        32 => Errno::EPIPE,
        38 => Errno::ENOSYS,
        _ => Errno::EIO,
    }
}
