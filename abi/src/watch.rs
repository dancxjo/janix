//! Watch event ABI contract.
//!
//! A watch payload is a concatenation of one or more watch events.
//! Each event is a fixed-size header followed immediately by its value bytes.
//! Consumers must decode events sequentially until the payload is exhausted.
//!
//! CreateNode events use:
//! - predicate = WATCH_PRED_KIND
//! - value = SymbolId (u32, little-endian) encoded as Bytes with length 4
//!
//! Header layout (41 bytes):
//! [version: u8][op: u8][flags: u16][subject: ThingId (16)]
//! [predicate: PredicateId (16)][value_len: u32][value_encoding: u8]
//!
//! All integers are little-endian.

use crate::errors::Errno;
use crate::wire::{PredicateId, ThingId};

/// Predicate used for CreateNode events.
pub const WATCH_PRED_KIND: PredicateId = PredicateId([
    b'k', b'i', b'n', b'd', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

/// Current watch event version.
pub const WATCH_EVENT_VERSION: u8 = 1;

/// Fixed-size header length for a watch event.
pub const WATCH_EVENT_HEADER_LEN: usize = 41;

/// Maximum payload size returned by root_watch_next.
pub const MAX_WATCH_PAYLOAD_BYTES: usize = 256 * 1024;

/// Canonical watch operation types.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchOp {
    Upsert = 1,
    Delete = 2,
}

impl WatchOp {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Upsert),
            2 => Some(Self::Delete),
            _ => None,
        }
    }
}

/// Value encoding tags for watch events.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueEncoding {
    None = 0,
    U64LE = 1,
    I64LE = 2,
    Utf8 = 3,
    Bytes = 4,
}

impl ValueEncoding {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::U64LE),
            2 => Some(Self::I64LE),
            3 => Some(Self::Utf8),
            4 => Some(Self::Bytes),
            _ => None,
        }
    }
}

/// Fixed-size event header (packed, ABI-visible).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatchEventHeader {
    pub version: u8,
    pub op: u8,
    pub flags: u16,
    pub subject: ThingId,
    pub predicate: PredicateId,
    pub value_len: u32,
    pub value_encoding: u8,
}

impl WatchEventHeader {
    pub const SIZE: usize = WATCH_EVENT_HEADER_LEN;
}

/// Borrowed watch event payload used for encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatchEvent<'a> {
    pub op: WatchOp,
    pub flags: u16,
    pub subject: ThingId,
    pub predicate: PredicateId,
    pub value_encoding: ValueEncoding,
    pub value: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    NonZeroFlags(u16),
    BadLength { expected: usize, got: usize },
    InvalidUtf8,
    ValueTooLarge(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    BadVersion(u8),
    UnknownOp(u8),
    UnknownEncoding(u8),
    BadLength { expected: usize, got: usize },
    InvalidUtf8,
    NonZeroFlags(u16),
}

pub fn encoded_len(value_len: usize) -> usize {
    WATCH_EVENT_HEADER_LEN + value_len
}

pub fn validate_event(event: &WatchEvent<'_>) -> Result<(), EncodeError> {
    if event.flags != 0 {
        return Err(EncodeError::NonZeroFlags(event.flags));
    }
    if event.value.len() > (u32::MAX as usize) {
        return Err(EncodeError::ValueTooLarge(event.value.len()));
    }
    match event.value_encoding {
        ValueEncoding::None => {
            if !event.value.is_empty() {
                return Err(EncodeError::BadLength { expected: 0, got: event.value.len() });
            }
        }
        ValueEncoding::U64LE | ValueEncoding::I64LE => {
            if event.value.len() != 8 {
                return Err(EncodeError::BadLength { expected: 8, got: event.value.len() });
            }
        }
        ValueEncoding::Utf8 => {
            if core::str::from_utf8(event.value).is_err() {
                return Err(EncodeError::InvalidUtf8);
            }
        }
        ValueEncoding::Bytes => {}
    }
    Ok(())
}

pub fn encode_event(buf: &mut [u8], event: &WatchEvent<'_>) -> Result<usize, Errno> {
    validate_event(event).map_err(|_| Errno::EINVAL)?;

    let value_len = event.value.len();
    let total_len = encoded_len(value_len);
    if buf.len() < total_len {
        return Err(Errno::ENOSPC);
    }

    buf[0] = WATCH_EVENT_VERSION;
    buf[1] = event.op as u8;
    buf[2..4].copy_from_slice(&event.flags.to_le_bytes());
    buf[4..20].copy_from_slice(&event.subject.0);
    buf[20..36].copy_from_slice(&event.predicate.0);
    buf[36..40].copy_from_slice(&(value_len as u32).to_le_bytes());
    buf[40] = event.value_encoding as u8;

    let value_start = WATCH_EVENT_HEADER_LEN;
    buf[value_start..value_start + value_len].copy_from_slice(event.value);

    Ok(total_len)
}

pub fn decode_event(bytes: &[u8]) -> Result<(WatchEventHeader, &[u8]), DecodeError> {
    if bytes.len() < WATCH_EVENT_HEADER_LEN {
        return Err(DecodeError::BadLength {
            expected: WATCH_EVENT_HEADER_LEN,
            got: bytes.len(),
        });
    }

    let version = bytes[0];
    if version != WATCH_EVENT_VERSION {
        return Err(DecodeError::BadVersion(version));
    }

    let op_raw = bytes[1];
    if WatchOp::from_u8(op_raw).is_none() {
        return Err(DecodeError::UnknownOp(op_raw));
    }

    let flags = u16::from_le_bytes([bytes[2], bytes[3]]);
    if flags != 0 {
        return Err(DecodeError::NonZeroFlags(flags));
    }

    let mut subject = [0u8; 16];
    subject.copy_from_slice(&bytes[4..20]);
    let mut predicate = [0u8; 16];
    predicate.copy_from_slice(&bytes[20..36]);

    let value_len = u32::from_le_bytes(bytes[36..40].try_into().unwrap()) as usize;
    let encoding_raw = bytes[40];
    let encoding = ValueEncoding::from_u8(encoding_raw).ok_or(DecodeError::UnknownEncoding(encoding_raw))?;

    let total_len = encoded_len(value_len);
    if bytes.len() < total_len {
        return Err(DecodeError::BadLength {
            expected: total_len,
            got: bytes.len(),
        });
    }

    let value = &bytes[WATCH_EVENT_HEADER_LEN..WATCH_EVENT_HEADER_LEN + value_len];

    match encoding {
        ValueEncoding::None => {
            if value_len != 0 {
                return Err(DecodeError::BadLength { expected: 0, got: value_len });
            }
        }
        ValueEncoding::U64LE | ValueEncoding::I64LE => {
            if value_len != 8 {
                return Err(DecodeError::BadLength { expected: 8, got: value_len });
            }
        }
        ValueEncoding::Utf8 => {
            if core::str::from_utf8(value).is_err() {
                return Err(DecodeError::InvalidUtf8);
            }
        }
        ValueEncoding::Bytes => {}
    }

    let header = WatchEventHeader {
        version,
        op: op_raw,
        flags,
        subject: ThingId(subject),
        predicate: PredicateId(predicate),
        value_len: value_len as u32,
        value_encoding: encoding_raw,
    };

    Ok((header, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ids() -> (ThingId, PredicateId) {
        let mut thing = [0u8; 16];
        thing[0..8].copy_from_slice(&0x1122334455667788u64.to_le_bytes());
        let mut pred = [0u8; 16];
        pred[0..4].copy_from_slice(&0x99aabbccu32.to_le_bytes());
        (ThingId(thing), PredicateId(pred))
    }

    #[test]
    fn roundtrip_u64_upsert() {
        let (subject, predicate) = sample_ids();
        let value = 42u64.to_le_bytes();
        let event = WatchEvent {
            op: WatchOp::Upsert,
            flags: 0,
            subject,
            predicate,
            value_encoding: ValueEncoding::U64LE,
            value: &value,
        };

        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN + 8];
        let len = encode_event(&mut buf, &event).expect("encode");
        assert_eq!(len, WATCH_EVENT_HEADER_LEN + 8);

        let (header, payload) = decode_event(&buf).expect("decode");
        assert_eq!(header.version, WATCH_EVENT_VERSION);
        assert_eq!(header.op, WatchOp::Upsert as u8);
        assert_eq!(header.flags, 0);
        assert_eq!(header.subject, subject);
        assert_eq!(header.predicate, predicate);
        assert_eq!(header.value_len, 8);
        assert_eq!(header.value_encoding, ValueEncoding::U64LE as u8);
        assert_eq!(payload, &value);
    }

    #[test]
    fn roundtrip_none_delete() {
        let (subject, predicate) = sample_ids();
        let event = WatchEvent {
            op: WatchOp::Delete,
            flags: 0,
            subject,
            predicate,
            value_encoding: ValueEncoding::None,
            value: &[],
        };

        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN];
        let len = encode_event(&mut buf, &event).expect("encode");
        assert_eq!(len, WATCH_EVENT_HEADER_LEN);

        let (header, payload) = decode_event(&buf).expect("decode");
        assert_eq!(header.op, WatchOp::Delete as u8);
        assert_eq!(payload.len(), 0);
    }

    #[test]
    fn roundtrip_utf8() {
        let (subject, predicate) = sample_ids();
        let text = b"hello";
        let event = WatchEvent {
            op: WatchOp::Upsert,
            flags: 0,
            subject,
            predicate,
            value_encoding: ValueEncoding::Utf8,
            value: text,
        };

        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN + 5];
        encode_event(&mut buf, &event).expect("encode");
        let (_header, payload) = decode_event(&buf).expect("decode");
        assert_eq!(payload, text);
    }

    #[test]
    fn decode_bad_version() {
        let (subject, predicate) = sample_ids();
        let value = [0u8; 8];
        let event = WatchEvent {
            op: WatchOp::Upsert,
            flags: 0,
            subject,
            predicate,
            value_encoding: ValueEncoding::U64LE,
            value: &value,
        };
        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN + 8];
        encode_event(&mut buf, &event).expect("encode");
        buf[0] = 99;
        assert_eq!(decode_event(&buf), Err(DecodeError::BadVersion(99)));
    }

    #[test]
    fn decode_unknown_op() {
        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN];
        buf[0] = WATCH_EVENT_VERSION;
        buf[1] = 0x7f;
        let err = decode_event(&buf).unwrap_err();
        assert_eq!(err, DecodeError::UnknownOp(0x7f));
    }

    #[test]
    fn decode_unknown_encoding() {
        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN];
        buf[0] = WATCH_EVENT_VERSION;
        buf[1] = WatchOp::Upsert as u8;
        buf[40] = 0xfe;
        let err = decode_event(&buf).unwrap_err();
        assert_eq!(err, DecodeError::UnknownEncoding(0xfe));
    }

    #[test]
    fn decode_bad_length_for_u64() {
        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN + 4];
        buf[0] = WATCH_EVENT_VERSION;
        buf[1] = WatchOp::Upsert as u8;
        buf[36..40].copy_from_slice(&(4u32).to_le_bytes());
        buf[40] = ValueEncoding::U64LE as u8;
        let err = decode_event(&buf).unwrap_err();
        assert_eq!(err, DecodeError::BadLength { expected: 8, got: 4 });
    }

    #[test]
    fn decode_nonzero_flags() {
        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN];
        buf[0] = WATCH_EVENT_VERSION;
        buf[1] = WatchOp::Upsert as u8;
        buf[2..4].copy_from_slice(&1u16.to_le_bytes());
        let err = decode_event(&buf).unwrap_err();
        assert_eq!(err, DecodeError::NonZeroFlags(1));
    }

    #[test]
    fn decode_invalid_utf8() {
        let mut buf = [0u8; WATCH_EVENT_HEADER_LEN + 1];
        buf[0] = WATCH_EVENT_VERSION;
        buf[1] = WatchOp::Upsert as u8;
        buf[36..40].copy_from_slice(&(1u32).to_le_bytes());
        buf[40] = ValueEncoding::Utf8 as u8;
        buf[WATCH_EVENT_HEADER_LEN] = 0xff;
        let err = decode_event(&buf).unwrap_err();
        assert_eq!(err, DecodeError::InvalidUtf8);
    }

    #[test]
    fn decode_multiple_events_in_payload() {
        let (subject, predicate) = sample_ids();
        let value_a = 1u64.to_le_bytes();
        let value_b = 2u64.to_le_bytes();
        let event_a = WatchEvent {
            op: WatchOp::Upsert,
            flags: 0,
            subject,
            predicate,
            value_encoding: ValueEncoding::U64LE,
            value: &value_a,
        };
        let event_b = WatchEvent {
            op: WatchOp::Upsert,
            flags: 0,
            subject,
            predicate,
            value_encoding: ValueEncoding::U64LE,
            value: &value_b,
        };

        let mut payload = [0u8; (WATCH_EVENT_HEADER_LEN + 8) * 2];
        let first_len = encode_event(&mut payload[..], &event_a).expect("encode a");
        let second_len = encode_event(&mut payload[first_len..], &event_b).expect("encode b");

        let (header_a, payload_a) = decode_event(&payload[..first_len]).expect("decode a");
        assert_eq!(header_a.value_len, 8);
        assert_eq!(payload_a, &value_a);

        let (header_b, payload_b) =
            decode_event(&payload[first_len..first_len + second_len]).expect("decode b");
        assert_eq!(header_b.value_len, 8);
        assert_eq!(payload_b, &value_b);
    }
}
