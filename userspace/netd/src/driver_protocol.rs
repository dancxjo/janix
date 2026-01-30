//! VirtIO-NET driver protocol messages
//!
//! Defines the IPC message format between virtio_netd and netd.

use alloc::vec::Vec;

/// Frame received from hardware (virtio_netd -> netd)
pub const MSG_FRAME_RX: u16 = 0x0001;

/// Frame to transmit to hardware (netd -> virtio_netd)
pub const MSG_FRAME_TX: u16 = 0x0002;

/// Request MAC address (netd -> virtio_netd)
pub const MSG_MAC_REQ: u16 = 0x0010;

/// MAC address response (virtio_netd -> netd)
pub const MSG_MAC_RESP: u16 = 0x0011;

/// Link is up notification
pub const MSG_LINK_UP: u16 = 0x0020;

/// Link is down notification
pub const MSG_LINK_DOWN: u16 = 0x0021;

/// Message format for driver IPC
/// Format: [2: msg_type] [2: payload_length] [payload...]
#[derive(Debug)]
pub struct NetDriverMsg<'a> {
    pub msg_type: u16,
    pub payload: &'a [u8],
}

impl<'a> NetDriverMsg<'a> {
    /// Create a new message
    pub fn new(msg_type: u16, payload: &'a [u8]) -> Self {
        Self { msg_type, payload }
    }

    /// Encode message into bytes
    pub fn encode(&self) -> Vec<u8> {
        let len = self.payload.len() as u16;
        let mut buf = Vec::with_capacity(4 + self.payload.len());
        buf.extend_from_slice(&self.msg_type.to_le_bytes());
        buf.extend_from_slice(&len.to_le_bytes());
        buf.extend_from_slice(self.payload);
        buf
    }

    /// Decode message from bytes (returns None if invalid)
    pub fn decode(data: &'a [u8]) -> Option<Self> {
        if data.len() < 4 {
            return None;
        }
        let msg_type = u16::from_le_bytes([data[0], data[1]]);
        let len = u16::from_le_bytes([data[2], data[3]]) as usize;
        if data.len() < 4 + len {
            return None;
        }
        Some(Self {
            msg_type,
            payload: &data[4..4 + len],
        })
    }
}
