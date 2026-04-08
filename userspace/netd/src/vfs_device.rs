//! VFS-backed smoltcp Device implementation
//!
//! Opens `/dev/net/virtio0/{rx,tx,mac,mtu}` and uses them for frame I/O.
//! Frame format is length-prefixed: `[4 bytes: frame_length_le][frame bytes]`.
//!
//! This is the replacement for `ipc_device.rs` (issue #540).

use alloc::collections::VecDeque;
use abi::syscall::vfs_flags;
use smoltcp::phy::{self, Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_write};

const DRIVER_BASE: &str = "/dev/net/virtio0";
const MAX_FRAME: usize = 2048;
const RX_STAGING_BUF: usize = 65536;

/// Network device that reads/writes Ethernet frames via the VFS path
/// exported by `virtio_netd`.
pub struct VfsNicDevice {
    /// File descriptor for the rx stream (read Ethernet frames from here).
    rx_fd: u32,
    /// File descriptor for the tx stream (write Ethernet frames here).
    tx_fd: u32,
    /// File descriptor for the events stream (polls link-state changes).
    events_fd: u32,
    /// MAC address read from `/dev/net/virtio0/mac`.
    mac: [u8; 6],
    /// MTU read from `/dev/net/virtio0/mtu`.
    mtu: usize,
    /// True when carrier is up.
    link_up: bool,
    /// Staging buffer for incomplete frames arriving on rx_fd.
    rx_staging: [u8; RX_STAGING_BUF],
    /// How many bytes of staging buffer contain live data.
    rx_staging_len: usize,
    /// Complete decoded frames ready for smoltcp to consume.
    rx_queue: VecDeque<([u8; MAX_FRAME], usize)>,
}

impl VfsNicDevice {
    /// Open the virtio_netd VFS tree.  Retries with 100 ms back-off until
    /// the driver has mounted its paths.
    pub fn open() -> Self {
        loop {
            if let Some(dev) = Self::try_open() {
                return dev;
            }
            stem::info!("VfsNicDevice: /dev/net/virtio0 not ready, retrying...");
            stem::time::sleep_ms(100);
        }
    }

    fn try_open() -> Option<Self> {
        let rx_path = "/dev/net/virtio0/rx";
        let tx_path = "/dev/net/virtio0/tx";
        let events_path = "/dev/net/virtio0/events";
        let mac_path = "/dev/net/virtio0/mac";
        let mtu_path = "/dev/net/virtio0/mtu";

        let rx_fd =
            vfs_open(rx_path, vfs_flags::O_RDONLY | vfs_flags::O_NONBLOCK).ok()?;
        let tx_fd = match vfs_open(tx_path, vfs_flags::O_WRONLY) {
            Ok(fd) => fd,
            Err(e) => {
                stem::warn!("VfsNicDevice: failed to open tx: {:?}", e);
                let _ = vfs_close(rx_fd);
                return None;
            }
        };
        let events_fd =
            match vfs_open(events_path, vfs_flags::O_RDONLY | vfs_flags::O_NONBLOCK) {
                Ok(fd) => fd,
                Err(_) => {
                    // Events file is optional
                    u32::MAX
                }
            };

        // Read MAC — "xx:xx:xx:xx:xx:xx\n"
        let mac = match Self::read_mac(mac_path) {
            Some(m) => m,
            None => {
                stem::warn!("VfsNicDevice: failed to read MAC, using default");
                [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]
            }
        };

        // Read MTU
        let mtu = match Self::read_mtu(mtu_path) {
            Some(m) => m,
            None => 1500,
        };

        stem::info!(
            "VfsNicDevice: opened — MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x} MTU {}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5],
            mtu
        );

        Some(Self {
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
            link_up: true,
            rx_staging: [0u8; RX_STAGING_BUF],
            rx_staging_len: 0,
            rx_queue: VecDeque::new(),
        })
    }

    /// Parse a MAC string like "52:54:00:12:34:56" into bytes.
    fn read_mac(path: &str) -> Option<[u8; 6]> {
        let fd = vfs_open(path, vfs_flags::O_RDONLY).ok()?;
        let mut buf = [0u8; 32];
        let n = vfs_read(fd, &mut buf).ok()?;
        let _ = vfs_close(fd);
        let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
        Self::parse_mac(s)
    }

    fn parse_mac(s: &str) -> Option<[u8; 6]> {
        let mut mac = [0u8; 6];
        let parts: alloc::vec::Vec<&str> = s.split(':').collect();
        if parts.len() != 6 {
            return None;
        }
        for (i, p) in parts.iter().enumerate() {
            mac[i] = u8::from_str_radix(p.trim(), 16).ok()?;
        }
        Some(mac)
    }

    fn read_mtu(path: &str) -> Option<usize> {
        let fd = vfs_open(path, vfs_flags::O_RDONLY).ok()?;
        let mut buf = [0u8; 16];
        let n = vfs_read(fd, &mut buf).ok()?;
        let _ = vfs_close(fd);
        let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
        s.parse::<usize>().ok()
    }

    /// Get current timestamp for smoltcp.
            link_up,
        }
    }

    /// Return a smoltcp-compatible timestamp from the kernel clock.
    pub fn now() -> Instant {
        Instant::from_millis(stem::time::now().as_millis() as i64)
    }

    /// MAC address.
    /// Return the MAC address.
    pub fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// MTU.
    /// Return the MTU.
    pub fn mtu(&self) -> usize {
        self.mtu
    }

    /// Current link state.
    /// Return the current link state.
    pub fn link_up(&self) -> bool {
        self.link_up
    }

    /// Poll the events file for link-state notifications (non-blocking).
    fn poll_events(&mut self) {
        if self.events_fd == u32::MAX {
            return;
        }
        let mut buf = [0u8; 64];
        loop {
            match vfs_read(self.events_fd, &mut buf) {
                Ok(n) if n > 0 => {
                    if let Ok(s) = core::str::from_utf8(&buf[..n]) {
                        if s.contains("link-down") {
                            self.link_up = false;
                        } else if s.contains("link-up") {
                            self.link_up = true;
                        }
                    }
                }
                _ => break,
            }
        }
    }

    /// Non-blocking drain of the rx_fd, decoding length-prefixed frames into
    /// the internal rx_queue.
    pub fn poll_rx(&mut self) {
        self.poll_events();

        // Read as many bytes as we can from the non-blocking rx_fd.
        loop {
            let space = self.rx_staging.len().saturating_sub(self.rx_staging_len);
            if space == 0 {
                break;
            }
            match vfs_read(self.rx_fd, &mut self.rx_staging[self.rx_staging_len..]) {
                Ok(n) if n > 0 => {
                    self.rx_staging_len += n;
                }
                _ => break, // EAGAIN or EOF
            }
        }

        // Decode complete length-prefixed frames.
        let mut offset = 0usize;
        loop {
            let remaining = &self.rx_staging[offset..self.rx_staging_len];
            if remaining.len() < 4 {
                break;
            }
            let frame_len =
                u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]])
                    as usize;

            if frame_len == 0 || frame_len > MAX_FRAME {
                // Invalid length prefix — skip one byte and resync.
                stem::warn!(
                    "VfsNicDevice: invalid frame length {} in RX stream, resyncing",
                    frame_len
                );
                offset += 1;
                continue;
            }

            if remaining.len() < 4 + frame_len {
                break; // Incomplete frame, wait for more data.
            }

            let data = &remaining[4..4 + frame_len];
            let mut frame = [0u8; MAX_FRAME];
            frame[..frame_len].copy_from_slice(data);
            self.rx_queue.push_back((frame, frame_len));

            offset += 4 + frame_len;
        }

        // Compact the staging buffer.
        if offset > 0 {
            if offset < self.rx_staging_len {
                self.rx_staging.copy_within(offset..self.rx_staging_len, 0);
                self.rx_staging_len -= offset;
            } else {
                self.rx_staging_len = 0;
            }
        }
    }

    /// Write a single Ethernet frame to tx_fd using the length-prefix format.
    fn send_frame(&mut self, data: &[u8]) {
        let len = data.len() as u32;
        let mut hdr = [0u8; 4];
        hdr.copy_from_slice(&len.to_le_bytes());

        // Combine header + frame in a single write to avoid partial sends.
        let mut msg: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(4 + data.len());
        msg.extend_from_slice(&hdr);
        msg.extend_from_slice(data);

        if let Err(e) = vfs_write(self.tx_fd, &msg) {
            stem::warn!("VfsNicDevice: TX write failed: {:?}", e);
        }
    }
}

impl Drop for VfsNicDevice {
    fn drop(&mut self) {
        let _ = vfs_close(self.rx_fd);
        let _ = vfs_close(self.tx_fd);
        if self.events_fd != u32::MAX {
            let _ = vfs_close(self.events_fd);
        }
    }
}

// ── smoltcp Device impl ──────────────────────────────────────────────────────

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
        self.poll_rx();
        if let Some((frame, len)) = self.rx_queue.pop_front() {
            Some((VfsRxToken { frame, len }, VfsTxToken { device: self }))
        } else {
            None
        }
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

pub struct VfsRxToken {
    frame: [u8; MAX_FRAME],
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
        let mut buffer = [0u8; MAX_FRAME];
        let mut buffer = [0u8; MAX_FRAME_LEN];
        let result = f(&mut buffer[..len]);
        self.device.send_frame(&buffer[..len]);
        result
    }
}
