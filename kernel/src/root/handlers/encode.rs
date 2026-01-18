//! Batch Encoding for Single-Op Syscalls
//!
//! Synthesizes valid THRT batch bytes for watch consumers.
//! This ensures that single-op mutations produce the same wire format
//! as multi-op batches, enabling uniform treatment by watchers.

use alloc::vec::Vec;
use abi::root::{BATCH_MAGIC, BATCH_VERSION, OP_CREATE_NODE, OP_PUT_EDGE, OP_SET_PROP, REF_ABSOLUTE};
use abi::wire::{ThingId, SymbolId};

/// Convert a SymbolId to 16-byte hex representation.
/// This matches the batch decoder's `bytes_to_hex` conversion.
pub fn symbol_to_bytes(sym: SymbolId) -> [u8; 16] {
    sym.0
}

/// Encode a batch header.
fn encode_header(buf: &mut Vec<u8>, op_count: u16) {
    buf.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
    buf.extend_from_slice(&BATCH_VERSION.to_le_bytes());
    buf.extend_from_slice(&op_count.to_le_bytes());
}

/// Encode an absolute ThingRef (1 tag byte + 16 bytes for ThingId).
fn encode_absolute_ref(buf: &mut Vec<u8>, id: ThingId) {
    buf.push(REF_ABSOLUTE);
    buf.extend_from_slice(&id.0);
}

/// Encode a CREATE_NODE operation as a 1-op batch.
///
/// Format:
/// - Header (8 bytes)
/// - Tag: OP_CREATE_NODE (1 byte)
/// - kind_id: 16 bytes
/// - out_ref: u16 (2 bytes)
pub fn encode_create_node(kind_bytes: &[u8; 16], out_ref: u16) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + 1 + 16 + 2);
    encode_header(&mut buf, 1);
    buf.push(OP_CREATE_NODE);
    buf.extend_from_slice(kind_bytes);
    buf.extend_from_slice(&out_ref.to_le_bytes());
    buf
}

/// Encode a PUT_EDGE operation as a 1-op batch.
///
/// Format:
/// - Header (8 bytes)
/// - Tag: OP_PUT_EDGE (1 byte)
/// - subject: ThingRef (17 bytes for absolute)
/// - predicate_id: 16 bytes
/// - object: ThingRef (17 bytes for absolute)
pub fn encode_put_edge(src: ThingId, rel_bytes: &[u8; 16], dst: ThingId) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + 1 + 17 + 16 + 17);
    encode_header(&mut buf, 1);
    buf.push(OP_PUT_EDGE);
    encode_absolute_ref(&mut buf, src);
    buf.extend_from_slice(rel_bytes);
    encode_absolute_ref(&mut buf, dst);
    buf
}

/// Encode a SET_PROP operation as a 1-op batch.
///
/// Format:
/// - Header (8 bytes)
/// - Tag: OP_SET_PROP (1 byte)
/// - subject: ThingRef (17 bytes for absolute)
/// - key_id: 16 bytes
/// - value: 16 bytes
pub fn encode_set_prop(id: ThingId, key_bytes: &[u8; 16], value: &[u8; 16]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + 1 + 17 + 16 + 16);
    encode_header(&mut buf, 1);
    buf.push(OP_SET_PROP);
    encode_absolute_ref(&mut buf, id);
    buf.extend_from_slice(key_bytes);
    buf.extend_from_slice(value);
    buf
}
