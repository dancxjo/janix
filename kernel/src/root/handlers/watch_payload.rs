//! Watch payload encoding for commit outputs.

use crate::root::graph::ThingId;
use crate::root::graph::WatchFilter;
use abi::root::{WATCH_F_KIND, WATCH_F_PREDICATE, WATCH_F_SUBJECT};
use abi::symbols::SymbolId;
use abi::watch::{self, DecodeError, ValueEncoding, WatchEvent, WatchOp};
use abi::wire::{PredicateId, ThingId as WireThingId};
use alloc::vec::Vec;
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
        WATCH_ENCODE_REJECT_INVALID_TOTAL, WATCH_ENCODE_REJECT_MISSING_REF_TOTAL,
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
        // If it's a create-node event, verify the kind matches.
        // For other events (edges, props), we MUST let them through because the
        // watch_poll matching already guaranteed they belong to a commit that
        // matched the kind filter.
        if header.predicate == watch::WATCH_PRED_KIND && header.value_encoding == ValueEncoding::Bytes as u8 && value.len() == 4 {
            let kind_id = u32::from_le_bytes(value.try_into().unwrap());
            if kind_id != filter.kind_id {
                return false; // Found a create event for a different kind, drop it.
            }
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
    out.clear();
    coalesce.clear();

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

#[cfg(test)]
mod tests {
    use super::*;
    use abi::watch::{self, ValueEncoding, WatchEvent, WatchOp, decode_event, encode_event};
    use abi::wire::{PredicateId, ThingId};

    fn run_filter(payload: &[u8], filter: &WatchFilter) -> Result<Vec<u8>, DecodeError> {
        let mut out = Vec::new();
        let mut coalesce = Vec::new();
        filter_watch_payload(payload, filter, &mut out, &mut coalesce)?;
        Ok(out)
    }

    fn make_thing(val: u64) -> ThingId {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&val.to_le_bytes());
        ThingId(bytes)
    }

    fn make_pred(val: u32) -> PredicateId {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&val.to_le_bytes());
        PredicateId(bytes)
    }

    fn encode_test_event(
        subject: ThingId,
        predicate: PredicateId,
        encoding: ValueEncoding,
        value: &[u8],
        buf: &mut Vec<u8>,
    ) {
        let event = WatchEvent {
            op: WatchOp::Upsert,
            flags: 0,
            subject,
            predicate,
            value_encoding: encoding,
            value,
        };
        let start = buf.len();
        let len = watch::encoded_len(value.len());
        buf.resize(start + len, 0);
        encode_event(&mut buf[start..], &event).expect("encode failed");
    }

    #[test]
    fn test_coalesce_simple() {
        let mut payload = Vec::new();
        let subj = make_thing(1);
        let pred = make_pred(10);

        // 1. Update 1: value = 100
        encode_test_event(
            subj,
            pred,
            ValueEncoding::U64LE,
            &100u64.to_le_bytes(),
            &mut payload,
        );

        // 2. Update 2: value = 200 (Should coalesce)
        encode_test_event(
            subj,
            pred,
            ValueEncoding::U64LE,
            &200u64.to_le_bytes(),
            &mut payload,
        );

        let filter = WatchFilter::default();
        let filtered = run_filter(&payload, &filter).expect("filter failed");

        // Should have only one event
        let mut cursor = 0;
        let (header, value) = decode_event(&filtered[cursor..]).expect("decode 1");
        cursor += watch::WATCH_EVENT_HEADER_LEN + value.len();

        assert_eq!(cursor, filtered.len());
        assert_eq!(header.subject, subj);
        assert_eq!(header.predicate, pred);
        assert_eq!(value, 200u64.to_le_bytes());
    }

    #[test]
    fn test_coalesce_diff_len() {
        let mut payload = Vec::new();
        let subj = make_thing(1);
        let pred = make_pred(10);

        // 1. Bytes: len 4
        encode_test_event(
            subj,
            pred,
            ValueEncoding::Bytes,
            &[1, 2, 3, 4],
            &mut payload,
        );

        // 2. Bytes: len 5 (Should NOT coalesce)
        encode_test_event(
            subj,
            pred,
            ValueEncoding::Bytes,
            &[1, 2, 3, 4, 5],
            &mut payload,
        );

        let filter = WatchFilter::default();
        let filtered = run_filter(&payload, &filter).expect("filter failed");

        let mut cursor = 0;

        // Event 1
        let (_h1, v1) = decode_event(&filtered[cursor..]).expect("decode 1");
        cursor += watch::WATCH_EVENT_HEADER_LEN + v1.len();
        assert_eq!(v1, &[1, 2, 3, 4]);

        // Event 2
        let (_h2, v2) = decode_event(&filtered[cursor..]).expect("decode 2");
        cursor += watch::WATCH_EVENT_HEADER_LEN + v2.len();
        assert_eq!(v2, &[1, 2, 3, 4, 5]);

        assert_eq!(cursor, filtered.len());
    }

    #[test]
    fn test_filter_subject() {
        let mut payload = Vec::new();
        let subj1 = make_thing(10);
        let subj2 = make_thing(20);
        let pred = make_pred(5);

        encode_test_event(
            subj1,
            pred,
            ValueEncoding::U64LE,
            &0u64.to_le_bytes(),
            &mut payload,
        );
        encode_test_event(
            subj2,
            pred,
            ValueEncoding::U64LE,
            &0u64.to_le_bytes(),
            &mut payload,
        );

        let mut filter = WatchFilter::default();
        filter.flags |= WATCH_F_SUBJECT;
        filter.subject_lo = 10;

        let filtered = run_filter(&payload, &filter).expect("filter failed");

        let (h, _) = decode_event(&filtered).expect("decode");
        assert_eq!(h.subject, subj1);
        assert_eq!(filtered.len(), watch::encoded_len(8));
    }

    #[test]
    fn test_filter_predicate() {
        let mut payload = Vec::new();
        let subj = make_thing(1);
        let pred1 = make_pred(100);
        let pred2 = make_pred(200);

        encode_test_event(
            subj,
            pred1,
            ValueEncoding::U64LE,
            &0u64.to_le_bytes(),
            &mut payload,
        );
        encode_test_event(
            subj,
            pred2,
            ValueEncoding::U64LE,
            &0u64.to_le_bytes(),
            &mut payload,
        );

        let mut filter = WatchFilter::default();
        filter.flags |= WATCH_F_PREDICATE;
        filter.predicate_id = 200;

        let filtered = run_filter(&payload, &filter).expect("filter failed");

        let (h, _) = decode_event(&filtered).expect("decode");
        assert_eq!(h.predicate, pred2);
        assert_eq!(filtered.len(), watch::encoded_len(8));
    }

    #[test]
    fn test_filter_kind() {
        let mut payload = Vec::new();
        let subj1 = make_thing(1);
        let subj2 = make_thing(2);

        let kind_pred = watch::WATCH_PRED_KIND;
        let other_pred = make_pred(999);

        // 1. CreateNode (Kind=50) -> Keep
        encode_test_event(
            subj1,
            kind_pred,
            ValueEncoding::Bytes,
            &50u32.to_le_bytes(),
            &mut payload,
        );

        // 2. CreateNode (Kind=60) -> Drop
        encode_test_event(
            subj2,
            kind_pred,
            ValueEncoding::Bytes,
            &60u32.to_le_bytes(),
            &mut payload,
        );

        // 3. Other Predicate -> Drop (because Kind filter implies we only want that Kind creation event?
        //    Wait, logic says: if flag set, header.predicate MUST be WATCH_PRED_KIND AND value must match.
        //    So yes, non-kind predicates are dropped.)
        encode_test_event(
            subj1,
            other_pred,
            ValueEncoding::U64LE,
            &0u64.to_le_bytes(),
            &mut payload,
        );

        let mut filter = WatchFilter::default();
        filter.flags |= WATCH_F_KIND;
        filter.kind_id = 50;

        let filtered = run_filter(&payload, &filter).expect("filter failed");

        assert_eq!(filtered.len(), watch::encoded_len(4));
        let (h, v) = decode_event(&filtered).expect("decode");
        assert_eq!(h.subject, subj1);
        assert_eq!(h.predicate, kind_pred);
        assert_eq!(v, 50u32.to_le_bytes());
    }

    #[test]
    fn test_complex_coalesce() {
        // A, B, A, C, B -> Result: A, C, B (where A and B are coalesced to their last values)
        // Assume same length/encoding.
        let mut payload = Vec::new();
        let subj = make_thing(1);
        let pred_a = make_pred(10);
        let pred_b = make_pred(20);
        let pred_c = make_pred(30);

        encode_test_event(
            subj,
            pred_a,
            ValueEncoding::U64LE,
            &1u64.to_le_bytes(),
            &mut payload,
        );
        encode_test_event(
            subj,
            pred_b,
            ValueEncoding::U64LE,
            &2u64.to_le_bytes(),
            &mut payload,
        );
        encode_test_event(
            subj,
            pred_a,
            ValueEncoding::U64LE,
            &3u64.to_le_bytes(),
            &mut payload,
        ); // Update A
        encode_test_event(
            subj,
            pred_c,
            ValueEncoding::U64LE,
            &4u64.to_le_bytes(),
            &mut payload,
        );
        encode_test_event(
            subj,
            pred_b,
            ValueEncoding::U64LE,
            &5u64.to_le_bytes(),
            &mut payload,
        ); // Update B

        let filter = WatchFilter::default();
        let filtered = run_filter(&payload, &filter).expect("filter failed");

        let mut cursor = 0;

        // 1. A (val=3)
        let (h1, v1) = decode_event(&filtered[cursor..]).expect("decode 1");
        cursor += watch::WATCH_EVENT_HEADER_LEN + v1.len();
        assert_eq!(h1.predicate, pred_a);
        assert_eq!(v1, 3u64.to_le_bytes());

        // 2. B (val=5) - Note: Order in output preserves FIRST appearance for the slot, but value is LAST.
        // Let's check logic:
        // if found in coalesce: update value in place.
        // else: append to out, push to coalesce.
        // So order should be A, B, C.

        let (h2, v2) = decode_event(&filtered[cursor..]).expect("decode 2");
        cursor += watch::WATCH_EVENT_HEADER_LEN + v2.len();
        assert_eq!(h2.predicate, pred_b);
        assert_eq!(v2, 5u64.to_le_bytes());

        // 3. C (val=4)
        let (h3, v3) = decode_event(&filtered[cursor..]).expect("decode 3");
        cursor += watch::WATCH_EVENT_HEADER_LEN + v3.len();
        assert_eq!(h3.predicate, pred_c);
        assert_eq!(v3, 4u64.to_le_bytes());

        assert_eq!(cursor, filtered.len());
    }
}
