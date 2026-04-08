//! VFS-backed smoltcp Device implementation
//!
//! Implements network device I/O over `/dev/net/virtio0/{rx,tx,events}`
//! using direct file reads/writes with length-prefixed frames.
//!
//! Frame wire format (same as virtio_netd VFS provider):
//! ```text
//! [4 bytes: frame_length_le] [frame_length bytes: raw Ethernet frame]
//! ```

use smoltcp::phy::{self, Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;
use stem::syscall::vfs::{vfs_read, vfs_write};

/// Maximum raw Ethernet frame size (1500 MTU + 14 byte header)
const MAX_FRAME_LEN: usize = 1514;

/// Network device that communicates via VFS file descriptors on
/// `/dev/net/virtio0/{rx,tx,events}`.
pub struct VfsNicDevice {
    /// File descriptor for receiving frames (opened O_RDONLY | O_NONBLOCK)
    rx_fd: u32,
    /// File descriptor for sending frames (opened O_WRONLY)
    tx_fd: u32,
    /// File descriptor for link-state event stream (opened O_RDONLY | O_NONBLOCK)
    events_fd: u32,
    /// MAC address of the NIC
    mac: [u8; 6],
    /// MTU in bytes
    mtu: usize,
    /// Current link state (updated by `poll_events`)
    link_up: bool,
}

impl VfsNicDevice {
    /// Create a new VFS NIC device.
    pub fn new(
        rx_fd: u32,
        tx_fd: u32,
        events_fd: u32,
        mac: [u8; 6],
        mtu: usize,
        link_up: bool,
    ) -> Self {
        Self {
            rx_fd,
            tx_fd,
            events_fd,
            mac,
            mtu,
            link_up,
        }
    }

    /// Return a smoltcp-compatible timestamp from the kernel clock.
    pub fn now() -> Instant {
        Instant::from_millis(stem::time::now().as_millis() as i64)
    }

    /// Return the MAC address.
    pub fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// Return the MTU.
    pub fn mtu(&self) -> usize {
        self.mtu
    }

    /// Return the current link state.
    pub fn link_up(&self) -> bool {
        self.link_up
    }

    /// Drain the events fd and update the internal link state.
    ///
    /// Event bytes: `0x01` = link up, `0x00` = link down.
    pub fn poll_events(&mut self) {
        let mut buf = [0u8; 16];
        // Non-blocking: ignore errors (EAGAIN is expected when no events)
        if let Ok(n) = vfs_read(self.events_fd, &mut buf) {
            for &byte in &buf[..n] {
                match byte {
                    0x01 => self.link_up = true,
                    0x00 => self.link_up = false,
                    _ => {}
                }
            }
        }
    }

    /// Try to read one length-prefixed frame from `rx_fd` (non-blocking).
    ///
    /// Returns `(frame_buf, frame_len)` on success, `None` if no frame is
    /// available or an error occurred.
    fn try_recv_frame(&mut self) -> Option<([u8; MAX_FRAME_LEN], usize)> {
        // Read 4-byte little-endian length prefix.
        let mut len_buf = [0u8; 4];
        match vfs_read(self.rx_fd, &mut len_buf) {
            Ok(4) => {}
            Ok(0) | Err(_) => return None, // No data (EAGAIN) or EOF
            Ok(n) => {
                stem::warn!("VfsNicDevice: short length prefix read: {} bytes", n);
                return None;
            }
        }

        let frame_len = u32::from_le_bytes(len_buf) as usize;
        if frame_len == 0 || frame_len > MAX_FRAME_LEN {
            stem::warn!("VfsNicDevice: invalid RX frame length: {}", frame_len);
            return None;
        }

        // Read the frame payload.
        let mut frame = [0u8; MAX_FRAME_LEN];
        match vfs_read(self.rx_fd, &mut frame[..frame_len]) {
            Ok(n) if n == frame_len => Some((frame, frame_len)),
            Ok(n) => {
                stem::warn!(
                    "VfsNicDevice: short RX frame read: expected {}, got {}",
                    frame_len,
                    n
                );
                None
            }
            Err(e) => {
                stem::warn!("VfsNicDevice: RX frame read error: {:?}", e);
                None
            }
        }
    }

    /// Write one length-prefixed frame to `tx_fd`.
    fn send_frame(&mut self, data: &[u8]) {
        let frame_len = data.len() as u32;
        let len_bytes = frame_len.to_le_bytes();

        if let Err(e) = vfs_write(self.tx_fd, &len_bytes) {
            stem::warn!("VfsNicDevice: failed to write TX length prefix: {:?}", e);
            return;
        }
        if let Err(e) = vfs_write(self.tx_fd, data) {
            stem::warn!("VfsNicDevice: failed to write TX frame data: {:?}", e);
        }
    }
}

impl Device for VfsNicDevice {
    type RxToken<'a>
        = VfsRxToken
    where
        Self: 'a;
    type TxToken<'a>
        = VfsTxToken<'a>
    where
        Self: 'a;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Also check for link-state changes on each poll.
        self.poll_events();

        let (frame, len) = self.try_recv_frame()?;
        Some((VfsRxToken { frame, len }, VfsTxToken { device: self }))
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(VfsTxToken { device: self })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = self.mtu;
        caps.max_burst_size = Some(1);
        caps.medium = Medium::Ethernet;
        caps
    }
}

/// Owned RX frame token (holds a copy of the received frame bytes).
pub struct VfsRxToken {
    frame: [u8; MAX_FRAME_LEN],
    len: usize,
}

impl phy::RxToken for VfsRxToken {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buf = self.frame;
        f(&mut buf[..self.len])
    }
}

/// TX token that writes a length-prefixed frame to `tx_fd` on consume.
pub struct VfsTxToken<'a> {
    device: &'a mut VfsNicDevice,
}

impl<'a> phy::TxToken for VfsTxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = [0u8; MAX_FRAME_LEN];
        let result = f(&mut buffer[..len]);
        self.device.send_frame(&buffer[..len]);
        result
    }
}
