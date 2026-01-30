//! Network subsystem for Thing-OS
//!
//! This module provides network device drivers and link-layer interfaces.

pub mod virtio_net;

/// Network interface card (NIC) trait
pub trait Nic {
    /// Get the MAC address
    fn mac(&self) -> [u8; 6];
    
    /// Check if link is up
    fn link_up(&self) -> bool;
    
    /// Poll for received frames (non-blocking)
    /// Returns Some(frame_bytes) if a frame is available
    fn poll_rx(&mut self) -> Option<&[u8]>;
    
    /// Transmit a frame
    fn tx(&mut self, bytes: &[u8]) -> Result<(), TxError>;
    
    /// Get statistics
    fn stats(&self) -> NicStats;
}

/// NIC statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct NicStats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
}

/// Transmit errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxError {
    /// Queue is full
    QueueFull,
    /// Frame too large
    FrameTooLarge,
    /// Device not ready
    NotReady,
}

/// Ethernet frame wrapper
pub struct EthernetFrame {
    pub data: [u8; 2048],
    pub len: usize,
}

impl EthernetFrame {
    pub fn new() -> Self {
        Self {
            data: [0; 2048],
            len: 0,
        }
    }
    
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}
