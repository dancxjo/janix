//! Watch payload encoding for commit outputs.

use alloc::vec::Vec;
use abi::root::{WATCH_F_KIND, WATCH_F_PREDICATE, WATCH_F_SUBJECT};
use abi::watch::{self, DecodeError, ValueEncoding, WatchEvent, WatchOp};
use abi::wire::{PredicateId, ThingId as WireThingId};
use abi::symbols::SymbolId;
use crate::root::graph::ThingId;
use crate::root::graph::WatchFilter;
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchEncodeRejectReason {
    InvalidEvent,
    PayloadTooLarge,
    MissingCreateRef,
    BufferTooSmall,
}

fn thing_id_to_wire(id: ThingId) -> WireThingId {
    let mut bytes = [0u8; 16];
    bytes[0..8].copy_from_slice(&id.to_le_bytes());
    WireThingId(bytes)
}

fn symbol_id_to_predicate(sym: SymbolId) -> PredicateId {
    let mut bytes = [0u8; 16];
    bytes[0..4].copy_from_slice(&sym.to_le_bytes());
    PredicateId(bytes)
}

fn log_watch_encode_reject_once(reason: WatchEncodeRejectReason) {
    static SEEN: AtomicU64 = AtomicU64::new(0);

    let bit = match reason {
        WatchEncodeRejectReason::InvalidEvent => 1u64 << 0,
        WatchEncodeRejectReason::PayloadTooLarge => 1u64 << 1,
        WatchEncodeRejectReason::MissingCreateRef => 1u64 << 2,
        WatchEncodeRejectReason::BufferTooSmall => 1u64 << 3,
    };

    let prev = SEEN.fetch_or(bit, Ordering::Relaxed);
    if prev & bit == 0 {
        crate::kinfo!("watch_payload: rejecting payload (reason={:?})", reason);
    }
}

pub fn encode_watch_payload(
    ops: &[super::batch::ValidatedOp],
    local_refs: &[ThingId],
) -> Result<Vec<u8>, WatchEncodeRejectReason> {
    debug_assert_eq!(watch::WATCH_EVENT_VERSION, 1);

    let mut payload = Vec::new();

    for op in ops {
        let mut value_buf = [0u8; 8];
        let (subject, predicate, encoding, value_len) = match op {
            super::batch::ValidatedOp::CreateNode { kind, out_idx } => {
                let idx = *out_idx;
                if idx >= local_refs.len() {
                    return Err(WatchEncodeRejectReason::MissingCreateRef);
                }
                let id = local_refs[idx];
                value_buf[0..4].copy_from_slice(&kind.to_le_bytes());
                (
                    thing_id_to_wire(id),
                    watch::WATCH_PRED_KIND,
                    ValueEncoding::Bytes,
                    4usize,
                )
            }
            super::batch::ValidatedOp::PutEdge { src, rel, dst } => {
                value_buf[0..8].copy_from_slice(&dst.to_le_bytes());
                (
                    thing_id_to_wire(*src),
                    symbol_id_to_predicate(*rel),
                    ValueEncoding::U64LE,
                    8usize,
                )
            }
            super::batch::ValidatedOp::SetProp { id, key, value } => {
                value_buf[0..8].copy_from_slice(&value.to_le_bytes());
                (
                    thing_id_to_wire(*id),
                    symbol_id_to_predicate(*key),
                    ValueEncoding::U64LE,
                    8usize,
                )
            }
        };

        let value = &value_buf[..value_len];
        let event = WatchEvent {
            op: WatchOp::Upsert,
            flags: 0,
            subject,
            predicate,
            value_encoding: encoding,
            value,
        };

        if watch::validate_event(&event).is_err() {
            return Err(WatchEncodeRejectReason::InvalidEvent);
        }

        let event_len = watch::encoded_len(value_len);
        if payload.len() + event_len > watch::MAX_WATCH_PAYLOAD_BYTES {
            return Err(WatchEncodeRejectReason::PayloadTooLarge);
        }

        let start = payload.len();
        payload.resize(start + event_len, 0u8);
        if watch::encode_event(&mut payload[start..], &event).is_err() {
            return Err(WatchEncodeRejectReason::BufferTooSmall);
        }
    }

    Ok(payload)
}

pub fn track_watch_encode_reject(reason: WatchEncodeRejectReason) {
    use super::batch::{
        WATCH_ENCODE_REJECT_INVALID_TOTAL,
        WATCH_ENCODE_REJECT_MISSING_REF_TOTAL,
        WATCH_ENCODE_REJECT_TOO_LARGE_TOTAL,
    };

    match reason {
        WatchEncodeRejectReason::InvalidEvent => {
            WATCH_ENCODE_REJECT_INVALID_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
        WatchEncodeRejectReason::PayloadTooLarge => {
            WATCH_ENCODE_REJECT_TOO_LARGE_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
        WatchEncodeRejectReason::MissingCreateRef => {
            WATCH_ENCODE_REJECT_MISSING_REF_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
        WatchEncodeRejectReason::BufferTooSmall => {
            WATCH_ENCODE_REJECT_INVALID_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
    }
    log_watch_encode_reject_once(reason);
}

pub(crate) struct CoalesceEntry {
    subject: WireThingId,
    predicate: PredicateId,
    encoding: u8,
    value_len: usize,
    offset: usize,
}

fn event_matches_filter(
    header: &watch::WatchEventHeader,
    value: &[u8],
    filter: &WatchFilter,
) -> bool {
    if (filter.flags & WATCH_F_SUBJECT) != 0 {
        if header.subject.to_u64_lossy() != filter.subject_lo {
            return false;
        }
    }

    if (filter.flags & WATCH_F_PREDICATE) != 0 {
        if header.predicate.to_u32_lossy() != filter.predicate_id {
            return false;
        }
    }

    if (filter.flags & WATCH_F_KIND) != 0 {
        if header.predicate != watch::WATCH_PRED_KIND {
            return false;
        }
        if header.value_encoding != ValueEncoding::Bytes as u8 || value.len() != 4 {
            return false;
        }
        let kind_id = u32::from_le_bytes(value.try_into().unwrap());
        if kind_id != filter.kind_id {
            return false;
        }
    }

    true
}

pub(crate) fn filter_watch_payload(
    payload: &[u8],
    filter: &WatchFilter,
    out: &mut Vec<u8>,
    coalesce: &mut Vec<CoalesceEntry>,
) -> Result<(), DecodeError> {
    let mut cursor = 0usize;

    while cursor < payload.len() {
        let event_start = cursor;
        let (header, value) = watch::decode_event(&payload[cursor..])?;
        let event_len = watch::WATCH_EVENT_HEADER_LEN + value.len();
        cursor += event_len;

        if !event_matches_filter(&header, value, filter) {
            continue;
        }

        if let Some(entry) = coalesce.iter_mut().find(|entry| {
            entry.subject == header.subject
                && entry.predicate == header.predicate
                && entry.encoding == header.value_encoding
                && entry.value_len == value.len()
        }) {
            let value_offset = entry.offset + watch::WATCH_EVENT_HEADER_LEN;
            out[value_offset..value_offset + value.len()].copy_from_slice(value);
            continue;
        }

        let offset = out.len();
        out.extend_from_slice(&payload[event_start..event_start + event_len]);
        coalesce.push(CoalesceEntry {
            subject: header.subject,
            predicate: header.predicate,
            encoding: header.value_encoding,
            value_len: value.len(),
            offset,
        });
    }

    Ok(())
}
