//! VFS node wrapper for IPC ports.
//!
//! This allows IPC ports to be treated as VFS nodes, enabling them to be
//! passed across channels using the standard handle-passing mechanism.

use alloc::sync::Arc;
use abi::errors::SysResult;
use crate::ipc::Port;
use super::{VfsNode, VfsStat};

/// A VFS node that wraps an IPC port.
pub struct PortNode {
    port: Arc<Port>,
}

impl PortNode {
    pub fn new(port: Arc<Port>) -> Self {
        Self { port }
    }

    pub fn port(&self) -> &Arc<Port> {
        &self.port
    }
}

impl VfsNode for PortNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Ports don't support standard VFS offset-based reads via this node.
        // They should be interacted with via the IPC channel syscalls.
        Ok(self.port.try_recv(buf))
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        Ok(self.port.send(buf))
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFIFO | 0o666,
            size: self.port.len() as u64,
            ino: 0, // Not applicable
        })
    }

    fn poll(&self) -> u16 {
        let mut revents = 0;
        if !self.port.is_empty() {
            revents |= abi::syscall::poll_flags::POLLIN;
        }
        if !self.port.is_full() {
            revents |= abi::syscall::poll_flags::POLLOUT;
        }
        revents
    }

    fn as_port(&self) -> Option<Arc<Port>> {
        Some(self.port.clone())
    }
}
