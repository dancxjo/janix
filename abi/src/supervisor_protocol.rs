//! Supervisor Protocol for Sovereign Driver Registration
//!
//! This protocol defines the handshake between system drivers and the supervisor (sprout).
//! It replaces the "land-rush" model where drivers mount themselves into /dev.

pub const MSG_BIND_READY: u16 = 0x8001;
pub const MSG_BIND_ASSIGNED: u16 = 0x8002;

/// Class bits for identifying the type of device being registered.
pub mod classes {
    pub const DISPLAY_CARD: u32 = 1 << 0;
    pub const FRAMEBUFFER: u32 = 1 << 1;
    pub const INPUT_EVENT: u32 = 1 << 2;
    pub const BLOCK_DEVICE: u32 = 1 << 3;
    pub const NETWORK_INTERFACE: u32 = 1 << 4;
    pub const SOUND_CARD: u32 = 1 << 5;
}

/// Payload for MSG_BIND_READY (Driver -> Supervisor)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BindReadyPayload {
    /// The unique token assigned to the driver by the supervisor at launch.
    pub bind_instance_id: u64,
    /// Bitmask of device classes provided by this driver (see `classes` mod).
    pub class_mask: u32,
    /// Reserved for future alignment/metadata.
    pub _reserved: u32,
}

/// Payload for MSG_BIND_ASSIGNED (Supervisor -> Driver)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BindAssignedPayload {
    /// Echoed bind_instance_id for confirmation.
    pub bind_instance_id: u64,
    /// Status code (0 for success, non-zero for error).
    pub status: u32,
    /// Canonical unit number assigned within the primary class.
    pub unit_number: u32,
    /// Canonical primary path assigned by the supervisor (e.g., "/dev/display/card0").
    pub primary_path: [u8; 64],
}

pub const BIND_READY_PAYLOAD_SIZE: usize = 16;
pub const BIND_ASSIGNED_PAYLOAD_SIZE: usize = 80; // 8 + 4 + 4 + 64

pub fn encode_bind_ready_le(payload: &BindReadyPayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < BIND_READY_PAYLOAD_SIZE {
        return None;
    }
    out[0..8].copy_from_slice(&payload.bind_instance_id.to_le_bytes());
    out[8..12].copy_from_slice(&payload.class_mask.to_le_bytes());
    out[12..16].copy_from_slice(&payload._reserved.to_le_bytes());
    Some(BIND_READY_PAYLOAD_SIZE)
}

pub fn decode_bind_ready_le(buf: &[u8]) -> Option<BindReadyPayload> {
    if buf.len() < BIND_READY_PAYLOAD_SIZE {
        return None;
    }
    Some(BindReadyPayload {
        bind_instance_id: u64::from_le_bytes(buf[0..8].try_into().ok()?),
        class_mask: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        _reserved: u32::from_le_bytes(buf[12..16].try_into().ok()?),
    })
}

pub fn encode_bind_assigned_le(payload: &BindAssignedPayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < BIND_ASSIGNED_PAYLOAD_SIZE {
        return None;
    }
    out[0..8].copy_from_slice(&payload.bind_instance_id.to_le_bytes());
    out[8..12].copy_from_slice(&payload.status.to_le_bytes());
    out[12..16].copy_from_slice(&payload.unit_number.to_le_bytes());
    out[16..80].copy_from_slice(&payload.primary_path);
    Some(BIND_ASSIGNED_PAYLOAD_SIZE)
}

pub fn decode_bind_assigned_le(buf: &[u8]) -> Option<BindAssignedPayload> {
    if buf.len() < BIND_ASSIGNED_PAYLOAD_SIZE {
        return None;
    }
    let mut primary_path = [0u8; 64];
    primary_path.copy_from_slice(&buf[16..80]);
    Some(BindAssignedPayload {
        bind_instance_id: u64::from_le_bytes(buf[0..8].try_into().ok()?),
        status: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        unit_number: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        primary_path,
    })
}
