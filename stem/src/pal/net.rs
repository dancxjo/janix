//! Network platform abstraction.
//!
//! Provides access to:
//! - The primary NIC via raw syscalls (legacy path).
//! - The `/net/` VFS TCP socket API used by the libc socket shim (issue #542).

use crate::syscall;
use crate::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_write};
use abi::errors::{errno, Errno, SysResult};
use abi::syscall::vfs_flags::{O_NONBLOCK, O_RDONLY, O_RDWR, O_WRONLY};

/// A VFS file descriptor returned by [`vfs_open`].
pub type Fd = u32;

// ── poll intervals ────────────────────────────────────────────────────────────

/// Nanoseconds to wait between connection-state polls.
const CONNECT_POLL_NS: u64 = 5_000_000;

// ── TcpHandle ─────────────────────────────────────────────────────────────────

/// An open TCP connection backed by the `/net/tcp/<id>/` VFS subtree.
///
/// Obtained via [`tcp_connect`].  I/O goes through `data_fd`; control
/// commands (`"connect …"`, `"close"`) go through `ctl_fd`.
pub struct TcpHandle {
    /// Socket ID assigned by netd (index under `/net/tcp/`).
    pub id: u32,
    /// Read-write data channel: `/net/tcp/<id>/data`.
    pub data_fd: Fd,
    /// Write-only control channel: `/net/tcp/<id>/ctl`.
    pub ctl_fd: Fd,
}

impl TcpHandle {
    /// Read up to `buf.len()` bytes from the TCP data stream.
    pub fn read(&self, buf: &mut [u8]) -> SysResult<usize> {
        vfs_read(self.data_fd, buf)
    }

    /// Write `buf` to the TCP data stream.
    pub fn write(&self, buf: &[u8]) -> SysResult<usize> {
        vfs_write(self.data_fd, buf)
    }

    /// Send a control command to the TCP socket (e.g. `"close"`).
    pub fn control(&self, cmd: &[u8]) -> SysResult<usize> {
        vfs_write(self.ctl_fd, cmd)
    }

    /// Close the connection and release VFS file descriptors.
    pub fn close(self) {
        let _ = vfs_write(self.ctl_fd, b"close");
        let _ = vfs_close(self.data_fd);
        let _ = vfs_close(self.ctl_fd);
    }
}

// ── connection state ──────────────────────────────────────────────────────────

/// Parsed connection state from `/net/tcp/<id>/status`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TcpState {
    Created,
    Connecting,
    Connected,
    Closed,
    Other,
}

fn read_tcp_state(id: u32) -> TcpState {
    use alloc::format;
    let path = format!("/net/tcp/{id}/status");
    let Ok(fd) = vfs_open(&path, O_RDONLY) else {
        return TcpState::Other;
    };
    let mut buf = [0u8; 256];
    let n = vfs_read(fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(fd);

    let text = core::str::from_utf8(&buf[..n]).unwrap_or("");
    for line in text.lines() {
        if let Some(state) = line.strip_prefix("state: ") {
            return match state.trim() {
                "created" => TcpState::Created,
                "bound" | "syn-sent" | "syn-received" => TcpState::Connecting,
                "connected" | "established" | "fin-wait-1" | "fin-wait-2" | "close-wait" => {
                    TcpState::Connected
                }
                "closed" | "time-wait" | "closing" | "last-ack" => TcpState::Closed,
                _ => TcpState::Other,
            };
        }
    }
    TcpState::Other
}

// ── public API ────────────────────────────────────────────────────────────────

/// Open a TCP connection to `addr` (dotted-decimal IPv4) on `port`.
///
/// This maps to the following `/net/` operations:
/// 1. `open("/net/tcp/new", O_RDONLY)` → read socket id as text
/// 2. `open("/net/tcp/<id>/ctl", O_WRONLY)`
/// 3. `write("connect <addr> <port>")` to ctl fd
/// 4. `open("/net/tcp/<id>/data", O_RDWR | O_NONBLOCK)`
/// 5. Poll `/net/tcp/<id>/status` until state is `"connected"` or `"closed"`
/// 6. Return `TcpHandle { id, data_fd, ctl_fd }`
///
/// Returns `Err(Errno::ETIMEDOUT)` if the deadline (specified as nanoseconds
/// from boot, or `0` for no deadline) passes before the connection succeeds.
pub fn tcp_connect(addr: &str, port: u16, deadline_ns: u64) -> SysResult<TcpHandle> {
    use alloc::format;

    // Step 1: allocate socket
    let new_fd = vfs_open("/net/tcp/new", O_RDONLY)?;
    let mut id_buf = [0u8; 32];
    let n = vfs_read(new_fd, &mut id_buf)?;
    vfs_close(new_fd)?;
    let id_str = core::str::from_utf8(&id_buf[..n])
        .map_err(|_| Errno::EINVAL)?
        .trim();
    let id: u32 = id_str.parse().map_err(|_| Errno::EINVAL)?;

    // Step 2: open ctl fd
    let ctl_path = format!("/net/tcp/{id}/ctl");
    let ctl_fd = vfs_open(&ctl_path, O_WRONLY)?;

    // Step 3: send connect command
    let cmd = format!("connect {addr} {port}");
    if let Err(e) = vfs_write(ctl_fd, cmd.as_bytes()) {
        let _ = vfs_close(ctl_fd);
        return Err(e);
    }

    // Step 4: open data fd (non-blocking so the caller controls blocking)
    let data_path = format!("/net/tcp/{id}/data");
    let data_fd = match vfs_open(&data_path, O_RDWR | O_NONBLOCK) {
        Ok(fd) => fd,
        Err(e) => {
            let _ = vfs_write(ctl_fd, b"close");
            let _ = vfs_close(ctl_fd);
            return Err(e);
        }
    };

    // Step 5: poll until connected or closed/timed-out
    loop {
        match read_tcp_state(id) {
            TcpState::Connected => break,
            TcpState::Closed => {
                let _ = vfs_close(data_fd);
                let _ = vfs_write(ctl_fd, b"close");
                let _ = vfs_close(ctl_fd);
                return Err(Errno::ECONNREFUSED);
            }
            _ => {}
        }
        if deadline_ns != 0 && crate::syscall::monotonic_ns() >= deadline_ns {
            let _ = vfs_close(data_fd);
            let _ = vfs_write(ctl_fd, b"close");
            let _ = vfs_close(ctl_fd);
            return Err(Errno::ETIMEDOUT);
        }
        crate::syscall::sleep_ns(CONNECT_POLL_NS);
    }

    Ok(TcpHandle { id, data_fd, ctl_fd })
}

/// Get the MAC address of the primary NIC.
pub fn nic_mac(out: &mut [u8; 6]) -> SysResult<()> {
    let result = unsafe {
        syscall::syscall6(
            abi::syscall::SYS_NIC_MAC,
            out.as_mut_ptr() as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };

    errno(result).map(|_| ())
}

/// Check if the link is up.
pub fn nic_link_up() -> SysResult<bool> {
    let result = unsafe { syscall::syscall6(abi::syscall::SYS_NIC_LINK_UP, 0, 0, 0, 0, 0, 0) };

    errno(result).map(|v| v != 0)
}

/// Poll for a received frame.
///
/// Returns the frame length if available, or Ok(0) if no frame.
pub fn nic_poll_rx(buffer: &mut [u8]) -> SysResult<usize> {
    let result = unsafe {
        syscall::syscall6(
            abi::syscall::SYS_NIC_POLL_RX,
            buffer.as_mut_ptr() as usize,
            buffer.len(),
            0,
            0,
            0,
            0,
        )
    };

    errno(result)
}

/// Transmit a frame.
pub fn nic_tx(frame: &[u8]) -> SysResult<()> {
    let result = unsafe {
        syscall::syscall6(
            abi::syscall::SYS_NIC_TX,
            frame.as_ptr() as usize,
            frame.len(),
            0,
            0,
            0,
            0,
        )
    };

    errno(result).map(|_| ())
}
