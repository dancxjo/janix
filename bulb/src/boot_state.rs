//! Boot state wire format for Wasm guests.
//!
//! This module defines the stable ABI for communicating boot progress
//! to Wasm-based boot animation modules.

/// Stable wire format for boot state passed to Wasm modules.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BootStateWire {
    /// Boot stage: 0=Dormant, 1=KernelLoaded, 2=ServicesLoaded
    pub stage: u8,
    /// Number of CPUs currently online
    pub cpu_online: u8,
    /// Total CPUs detected
    pub cpu_total: u8,
    /// Reserved for alignment
    pub _reserved: u8,
    /// Bitset of completed milestones
    pub milestones: u32,
}

impl BootStateWire {
    pub const STAGE_DORMANT: u8 = 0;
    pub const STAGE_KERNEL_LOADED: u8 = 1;
    pub const STAGE_SERVICES_LOADED: u8 = 2;

    /// Serialize to bytes for Wasm linear memory
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        buf[0] = self.stage;
        buf[1] = self.cpu_online;
        buf[2] = self.cpu_total;
        buf[3] = self._reserved;
        buf[4..8].copy_from_slice(&self.milestones.to_le_bytes());
        buf
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8; 8]) -> Self {
        Self {
            stage: bytes[0],
            cpu_online: bytes[1],
            cpu_total: bytes[2],
            _reserved: bytes[3],
            milestones: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        }
    }
}
