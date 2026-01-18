//! Batch application and commit handling.
//!
//! This module provides the canonical commit path for all Root graph mutations.
//! Both multi-op batches (SYS_ROOT_APPLY_BATCH) and single-op syscalls
//! (CREATE_NODE, LINK, PROP_SET) route through `apply_ops_and_commit()`.

use crate::root::graph::{Graph, ThingId};
use crate::root::handlers::HandlerResult;
use abi::root::{BATCH_MAGIC, BATCH_VERSION, OP_CREATE_NODE, OP_PUT_EDGE, OP_SET_PROP, REF_ABSOLUTE, REF_LOCAL};
use abi::symbols::SymbolId;
use core::sync::atomic::Ordering;
use alloc::vec::Vec;
use crate::root::symbols::Interner;
use alloc::string::String;

/// Validated operation ready for application to the graph.
/// This is the internal representation after parsing/validation.
#[derive(Debug, Clone)]
pub enum ValidatedOp {
    /// Create a new node with the given kind.
    CreateNode { kind: SymbolId, out_idx: usize },
    /// Create an edge between two nodes.
    PutEdge { src: ThingId, rel: SymbolId, dst: ThingId },
    /// Set a property on a node.
    SetProp { id: ThingId, key: SymbolId, value: u64 },
}

/// Result from applying operations.
pub struct ApplyResult {
    /// Status code (0 = success, negative = error)
    pub status: i32,
    /// Commit sequence number (if successful)
    pub seq: u64,
    /// IDs of nodes created during this batch
    pub created_ids: Vec<ThingId>,
}

/// Apply validated operations to the graph and commit.
///
/// This is THE canonical commit path. All graph mutations must flow through here.
///
/// # Arguments
/// * `graph` - The root graph to mutate
/// * `ops` - Pre-validated operations to apply
/// * `commit_bytes` - The wire-format batch bytes for watch consumers
///
/// # Returns
/// * `ApplyResult` with status, seq, and created IDs
pub fn apply_ops_and_commit(
    graph: &mut Graph,
    ops: &[ValidatedOp],
    commit_bytes: &[u8],
) -> ApplyResult {
    let mut local_refs: Vec<ThingId> = Vec::with_capacity(16);
    let mut created_ids: Vec<ThingId> = Vec::new();

    // Apply each operation
    for op in ops {
        match op {
            ValidatedOp::CreateNode { kind, out_idx } => {
                let new_id = graph.alloc(*kind);
                created_ids.push(new_id);
                if *out_idx >= local_refs.len() {
                    local_refs.resize(*out_idx + 1, 0);
                }
                local_refs[*out_idx] = new_id;
            }
            ValidatedOp::PutEdge { src, rel, dst } => {
                graph.link(*src, *rel, *dst);
            }
            ValidatedOp::SetProp { id, key, value } => {
                if let Some(node) = graph.get_node_mut(*id) {
                    node.props.insert(*key, *value);
                }
            }
        }
    }

    // Commit: increment sequence number
    let new_seq = graph.root_seq.fetch_add(1, Ordering::SeqCst) + 1;

    // Push to shared commit history (single allocation, not per-watch)
    // Watchers read from this shared buffer via their cursor_seq
    graph.commit_history.push(new_seq, commit_bytes.to_vec());

    ApplyResult {
        status: 0,
        seq: new_seq,
        created_ids,
    }
}

// ============================================================================
// Batch Parsing (for SYS_ROOT_APPLY_BATCH)
// ============================================================================

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    const HEX: &[u8] = b"0123456789abcdef";
    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0xf) as usize] as char);
    }
    s
}

/// Parse a ThingRef from the batch buffer.
fn parse_ref(cursor: &mut usize, data: &[u8], local_refs: &[ThingId]) -> Option<ThingId> {
    if *cursor >= data.len() { return None; }
    let kind = data[*cursor];
    *cursor += 1;
    match kind {
        REF_ABSOLUTE => {
            if *cursor + 16 > data.len() { return None; }
            let val = u64::from_le_bytes(data[*cursor..*cursor+8].try_into().unwrap());
            *cursor += 16;
            Some(val)
        }
        REF_LOCAL => {
            if *cursor + 2 > data.len() { return None; }
            let idx = u16::from_le_bytes(data[*cursor..*cursor+2].try_into().unwrap()) as usize;
            *cursor += 2;
            if idx < local_refs.len() { Some(local_refs[idx]) } else { None }
        }
        _ => None
    }
}

/// Parse batch bytes into validated operations.
fn parse_batch(
    batch: &[u8],
    interner: &mut Interner,
) -> Result<Vec<ValidatedOp>, i32> {
    if batch.len() < 8 {
        return Err(-1);
    }

    let magic = u32::from_le_bytes(batch[0..4].try_into().unwrap());
    let version = u16::from_le_bytes(batch[4..6].try_into().unwrap());
    let op_count = u16::from_le_bytes(batch[6..8].try_into().unwrap());

    if magic != BATCH_MAGIC || version != BATCH_VERSION {
        return Err(-1);
    }

    let mut cursor = 8usize;
    let mut local_refs: Vec<ThingId> = Vec::with_capacity(16);
    let mut ops = Vec::with_capacity(op_count as usize);

    for _ in 0..op_count {
        if cursor >= batch.len() { return Err(-1); }
        let tag = batch[cursor];
        cursor += 1;

        match tag {
            OP_CREATE_NODE => {
                if cursor + 16 > batch.len() { return Err(-1); }
                let kind_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let kind_str = bytes_to_hex(&kind_bytes);
                let kind = interner.intern(&kind_str);

                if cursor + 2 > batch.len() { return Err(-1); }
                let out_idx = u16::from_le_bytes(batch[cursor..cursor+2].try_into().unwrap()) as usize;
                cursor += 2;

                ops.push(ValidatedOp::CreateNode { kind, out_idx });
                
                // Track local refs for subsequent ops
                if out_idx >= local_refs.len() {
                    local_refs.resize(out_idx + 1, 0);
                }
                // Placeholder - will be filled during apply
                local_refs[out_idx] = 0;
            }
            OP_PUT_EDGE => {
                let src = match parse_ref(&mut cursor, batch, &local_refs) {
                    Some(id) => id,
                    None => return Err(-2),
                };

                if cursor + 16 > batch.len() { return Err(-3); }
                let rel_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let rel_str = bytes_to_hex(&rel_bytes);
                let rel = interner.intern(&rel_str);

                let dst = match parse_ref(&mut cursor, batch, &local_refs) {
                    Some(id) => id,
                    None => return Err(-4),
                };

                ops.push(ValidatedOp::PutEdge { src, rel, dst });
            }
            OP_SET_PROP => {
                let id = match parse_ref(&mut cursor, batch, &local_refs) {
                    Some(id) => id,
                    None => return Err(-2),
                };

                if cursor + 16 > batch.len() { return Err(-3); }
                let key_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let key_str = bytes_to_hex(&key_bytes);
                let key = interner.intern(&key_str);

                if cursor + 8 > batch.len() { return Err(-4); }
                let value = u64::from_le_bytes(batch[cursor..cursor + 8].try_into().unwrap());
                cursor += 8;

                ops.push(ValidatedOp::SetProp { id, key, value });
            }
            _ => { return Err(-5); }
        }
    }

    Ok(ops)
}

/// Handle SYS_ROOT_APPLY_BATCH
///
/// Parses the batch, validates operations, and commits through the canonical path.
pub fn handle_apply_batch(
    graph: &mut Graph,
    interner: &mut Interner,
    batch: &[u8],
) -> HandlerResult {
    // Parse batch into validated ops
    let ops = match parse_batch(batch, interner) {
        Ok(ops) => ops,
        Err(code) => return (code, 0),
    };

    // Apply through canonical commit path
    let result = apply_ops_and_commit(graph, &ops, batch);
    
    (result.status, result.seq)
}
