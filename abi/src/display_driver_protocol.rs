#![allow(dead_code)]

// Wire format is explicitly little-endian for all fields.

use core::mem::size_of;

pub const DRIVER_MAGIC: u32 = 0x4452_5650; // "DRVP"
pub const DRIVER_VERSION: u16 = 0;

pub const MSG_REGISTER: u16 = 1;
pub const MSG_BIND: u16 = 2;
pub const MSG_PRESENT: u16 = 3;
pub const MSG_ACK: u16 = 4;
pub const MSG_ERR: u16 = 5;

pub const DRIVER_KIND_BOOTFB: u32 = 1;
pub const DRIVER_KIND_VIRTIO_GPU: u32 = 2;
pub const DRIVER_KIND_RAMFB: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DriverHeader {
    pub magic: u32,
    pub version: u16,
    pub msg_type: u16,
    pub payload_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RegisterPayload {
    pub driver_kind: u32,
    pub caps: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BindPayload {
    pub bytespace_id: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PresentHeader {
    pub rect_count: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ErrResp {
    pub code: u32,
}

pub const HEADER_SIZE: usize = size_of::<DriverHeader>();
pub const REGISTER_PAYLOAD_WIRE_SIZE: usize = 8;
pub const BIND_PAYLOAD_WIRE_SIZE: usize = 24;
pub const RECT_WIRE_SIZE: usize = 16;
pub const PRESENT_HEADER_WIRE_SIZE: usize = 8;
pub const ERR_RESP_WIRE_SIZE: usize = 4;

pub fn encode_message(buf: &mut [u8], msg_type: u16, payload: &[u8]) -> Option<usize> {
    let total = HEADER_SIZE + payload.len();
    if buf.len() < total {
        return None;
    }

    let header = DriverHeader {
        magic: DRIVER_MAGIC,
        version: DRIVER_VERSION,
        msg_type,
        payload_len: payload.len() as u32,
    };

    buf[0..4].copy_from_slice(&header.magic.to_le_bytes());
    buf[4..6].copy_from_slice(&header.version.to_le_bytes());
    buf[6..8].copy_from_slice(&header.msg_type.to_le_bytes());
    buf[8..12].copy_from_slice(&header.payload_len.to_le_bytes());

    if !payload.is_empty() {
        buf[HEADER_SIZE..total].copy_from_slice(payload);
    }

    Some(total)
}

pub fn parse_message(buf: &[u8]) -> Option<(DriverHeader, &[u8])> {
    if buf.len() < HEADER_SIZE {
        return None;
    }

    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    let msg_type = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[8..12].try_into().ok()?);

    if magic != DRIVER_MAGIC || version != DRIVER_VERSION {
        return None;
    }

    let total = HEADER_SIZE + payload_len as usize;
    if buf.len() < total {
        return None;
    }

    let header = DriverHeader {
        magic,
        version,
        msg_type,
        payload_len,
    };

    Some((header, &buf[HEADER_SIZE..total]))
}

pub fn message_total_len(buf: &[u8]) -> Option<usize> {
    if buf.len() < HEADER_SIZE {
        return None;
    }

    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[8..12].try_into().ok()?);

    if magic != DRIVER_MAGIC || version != DRIVER_VERSION {
        return None;
    }

    Some(HEADER_SIZE + payload_len as usize)
}

pub fn encode_register_payload_le(payload: &RegisterPayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < REGISTER_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&payload.driver_kind.to_le_bytes());
    out[4..8].copy_from_slice(&payload.caps.to_le_bytes());
    Some(REGISTER_PAYLOAD_WIRE_SIZE)
}

pub fn decode_register_payload_le(buf: &[u8]) -> Option<RegisterPayload> {
    if buf.len() < REGISTER_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(RegisterPayload {
        driver_kind: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        caps: u32::from_le_bytes(buf[4..8].try_into().ok()?),
    })
}

pub fn encode_bind_payload_le(payload: &BindPayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < BIND_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..8].copy_from_slice(&payload.bytespace_id.to_le_bytes());
    out[8..12].copy_from_slice(&payload.width.to_le_bytes());
    out[12..16].copy_from_slice(&payload.height.to_le_bytes());
    out[16..20].copy_from_slice(&payload.stride.to_le_bytes());
    out[20..24].copy_from_slice(&payload.format.to_le_bytes());
    Some(BIND_PAYLOAD_WIRE_SIZE)
}

pub fn decode_bind_payload_le(buf: &[u8]) -> Option<BindPayload> {
    if buf.len() < BIND_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(BindPayload {
        bytespace_id: u64::from_le_bytes(buf[0..8].try_into().ok()?),
        width: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        height: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        stride: u32::from_le_bytes(buf[16..20].try_into().ok()?),
        format: u32::from_le_bytes(buf[20..24].try_into().ok()?),
    })
}

pub fn encode_rect_le(rect: &Rect, out: &mut [u8]) -> Option<usize> {
    if out.len() < RECT_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&rect.x.to_le_bytes());
    out[4..8].copy_from_slice(&rect.y.to_le_bytes());
    out[8..12].copy_from_slice(&rect.w.to_le_bytes());
    out[12..16].copy_from_slice(&rect.h.to_le_bytes());
    Some(RECT_WIRE_SIZE)
}

pub fn decode_rect_le(buf: &[u8]) -> Option<Rect> {
    if buf.len() < RECT_WIRE_SIZE {
        return None;
    }
    Some(Rect {
        x: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        y: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        w: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        h: u32::from_le_bytes(buf[12..16].try_into().ok()?),
    })
}

pub fn encode_present_header_le(rect_count: u32, out: &mut [u8]) -> Option<usize> {
    if out.len() < PRESENT_HEADER_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&rect_count.to_le_bytes());
    out[4..8].copy_from_slice(&0u32.to_le_bytes());
    Some(PRESENT_HEADER_WIRE_SIZE)
}

pub fn decode_present_header_le(buf: &[u8]) -> Option<PresentHeader> {
    if buf.len() < PRESENT_HEADER_WIRE_SIZE {
        return None;
    }
    Some(PresentHeader {
        rect_count: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        _pad: u32::from_le_bytes(buf[4..8].try_into().ok()?),
    })
}

pub fn encode_present_payload_le<I>(rect_count: u32, rects: I, out: &mut [u8]) -> Option<usize>
where
    I: IntoIterator<Item = Rect>,
{
    let required = PRESENT_HEADER_WIRE_SIZE + rect_count as usize * RECT_WIRE_SIZE;
    if out.len() < required {
        return None;
    }

    encode_present_header_le(rect_count, out)?;

    let mut written = 0usize;
    let mut offset = PRESENT_HEADER_WIRE_SIZE;
    for rect in rects {
        if written >= rect_count as usize {
            break;
        }
        encode_rect_le(&rect, &mut out[offset..offset + RECT_WIRE_SIZE])?;
        offset += RECT_WIRE_SIZE;
        written += 1;
    }

    if written != rect_count as usize {
        return None;
    }

    Some(required)
}

pub fn encode_err_resp_le(payload: &ErrResp, out: &mut [u8]) -> Option<usize> {
    if out.len() < ERR_RESP_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&payload.code.to_le_bytes());
    Some(ERR_RESP_WIRE_SIZE)
}

pub fn decode_err_resp_le(buf: &[u8]) -> Option<ErrResp> {
    if buf.len() < ERR_RESP_WIRE_SIZE {
        return None;
    }
    Some(ErrResp {
        code: u32::from_le_bytes(buf[0..4].try_into().ok()?),
    })
}
