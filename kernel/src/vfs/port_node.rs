//! VFS node wrapper for IPC ports.
//!
//! This allows IPC ports to be treated as VFS nodes, enabling them to be
//! passed across channels using the standard handle-passing mechanism.

use alloc::sync::Arc;
use abi::errors::SysResult;
use crate::ipc::{Port, HandleMode};
use super::{VfsNode, VfsStat};

/// A VFS node that wraps an IPC port.
pub struct PortNode {
    port: Arc<Port>,
    mode: HandleMode,
}

impl PortNode {
    pub fn new(port: Arc<Port>, mode: HandleMode) -> Self {
        Self { port, mode }
    }

    pub fn port(&self) -> &Arc<Port> {
        &self.port
    }
}

impl VfsNode for PortNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if self.mode != HandleMode::Read {
            return Err(abi::errors::Errno::EBADF);
        }
        Ok(self.port.try_recv(buf))
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        if self.mode != HandleMode::Write {
            return Err(abi::errors::Errno::EBADF);
        }
        Ok(self.port.send(buf))
    }

    fn stat(&self) -> SysResult<VfsStat> {
        let (r, w) = if self.mode == HandleMode::Read {
            (0o400, 0)
        } else {
            (0, 0o200)
        };
        Ok(VfsStat {
            mode: VfsStat::S_IFIFO | r | w,
            size: self.port.len() as u64,
            ino: 0,
        })
    }

    fn poll(&self) -> u16 {
        use abi::syscall::poll_flags::{POLLIN, POLLOUT, POLLHUP, POLLERR};
        let mut revents = 0;
        match self.mode {
            HandleMode::Read => {
                if !self.port.is_empty() || !self.port.has_writers() {
                    revents |= POLLIN;
                }
                if !self.port.has_writers() {
                    revents |= POLLHUP;
                }
            }
            HandleMode::Write => {
                if !self.port.has_readers() {
                    revents |= POLLHUP | POLLERR;
                } else if !self.port.is_full() {
                    revents |= POLLOUT;
                }
            }
        }
        revents
    }

    fn add_waiter(&self, tid: u64) {
        match self.mode {
            HandleMode::Read => self.port.add_waiter_read(tid),
            HandleMode::Write => self.port.add_waiter_write(tid),
        }
    }

    fn remove_waiter(&self, tid: u64) {
        match self.mode {
            HandleMode::Read => self.port.remove_waiter_read(tid),
            HandleMode::Write => self.port.remove_waiter_write(tid),
        }
    }

    fn as_port(&self) -> Option<Arc<Port>> {
        Some(self.port.clone())
    }
}
