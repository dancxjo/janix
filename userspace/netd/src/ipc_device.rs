//! IPC-backed smoltcp Device implementation
//!
//! This module provides a network device that communicates with virtio_netd
//! via IPC ports instead of directly driving hardware.

use alloc::collections::VecDeque;
use smoltcp::phy::{self, Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;
use stem::syscall::port::{port_recv, port_send, PortHandle};

use crate::driver_protocol::{
    NetDriverMsg, MSG_FRAME_RX, MSG_FRAME_TX, MSG_LINK_DOWN, MSG_LINK_UP,
};

/// Network device that uses IPC to communicate with virtio_netd
pub struct IpcNicDevice {
    /// Port handle to send TX frames to driver
    tx_port: PortHandle,
    /// Port handle to receive RX frames from driver
    rx_port: PortHandle,
    /// MAC address (set after receiving from driver)
    mac: [u8; 6],
    /// Pending received frames
    rx_queue: VecDeque<([u8; 2048], usize)>,
    /// Internal RX buffer
    rx_buf: [u8; 2048],
    /// Current carrier status as reported by driver notifications
    link_up: bool,
}

impl IpcNicDevice {
    /// Create a new IPC NIC device
    pub fn new(tx_port: PortHandle, rx_port: PortHandle, mac: [u8; 6], link_up: bool) -> Self {
        Self {
            tx_port,
            rx_port,
            mac,
            rx_queue: VecDeque::new(),
            rx_buf: [0u8; 2048],
            link_up,
        }
    }

    /// Get current timestamp for smoltcp
    pub fn now() -> Instant {
        Instant::from_millis(stem::time::now().as_millis() as i64)
    }

    /// Get MAC address
    pub fn mac(&self) -> [u8; 6] {
        self.mac
    }

    pub fn link_up(&self) -> bool {
        self.link_up
    }

    pub fn mtu(&self) -> u32 {
        1500
    }

    /// Poll for RX frames from driver (non-blocking)
    pub fn poll_rx(&mut self) {
        // Try to receive frames from driver
        loop {
            match port_recv(self.rx_port, &mut self.rx_buf) {
                Ok(len) if len > 0 => {
                    if let Some(msg) = NetDriverMsg::decode(&self.rx_buf[..len]) {
                        match msg.msg_type {
                            MSG_FRAME_RX if !msg.payload.is_empty() => {
                                // Queue the frame
                                let mut frame = [0u8; 2048];
                                let frame_len = msg.payload.len().min(2048);
                                frame[..frame_len].copy_from_slice(&msg.payload[..frame_len]);
                                self.rx_queue.push_back((frame, frame_len));
                            }
                            MSG_LINK_UP => {
                                self.link_up = true;
                            }
                            MSG_LINK_DOWN => {
                                self.link_up = false;
                            }
                            _ => {}
                        }
                    }
                }
                _ => break, // No more data
            }
        }
    }

    /// Send a frame to the driver for transmission
    fn send_frame(&mut self, data: &[u8]) {
        let msg = NetDriverMsg::new(MSG_FRAME_TX, data);
        if let Err(e) = port_send(self.tx_port, &msg.encode()) {
            stem::warn!("IpcNicDevice: Failed to send TX frame: {:?}", e);
        }
    }
}

impl Device for IpcNicDevice {
    type RxToken<'a>
        = IpcRxToken
    where
        Self: 'a;
    type TxToken<'a>
        = IpcTxToken<'a>
    where
        Self: 'a;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Poll for new frames
        self.poll_rx();

        // Return a frame if available
        if let Some((frame, len)) = self.rx_queue.pop_front() {
            Some((IpcRxToken { frame, len }, IpcTxToken { device: self }))
        } else {
            None
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(IpcTxToken { device: self })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1500;
        caps.max_burst_size = Some(1);
        caps.medium = Medium::Ethernet;
        caps
    }
}

pub struct IpcRxToken {
    frame: [u8; 2048],
    len: usize,
}

impl phy::RxToken for IpcRxToken {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buf = self.frame;
        f(&mut buf[..self.len])
    }
}

pub struct IpcTxToken<'a> {
    device: &'a mut IpcNicDevice,
}

impl<'a> phy::TxToken for IpcTxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = [0u8; 2048];
        let result = f(&mut buffer[..len]);
        self.device.send_frame(&buffer[..len]);
        result
    }
}
