//! Batch application and commit handling.
//!
//! This module provides the canonical commit path for all Root graph mutations.
//! Both multi-op batches (SYS_ROOT_APPLY_BATCH) and single-op syscalls
//! (CREATE_NODE, LINK, PROP_SET) route through `apply_ops_and_commit()`.

use crate::root::graph::{Graph, ThingId, CommitSummary};
use crate::root::handlers::HandlerResult;
use abi::root::{
    BATCH_MAGIC, BATCH_VERSION, OP_CREATE_NODE, OP_PUT_EDGE, OP_SET_PROP,
    REF_ABSOLUTE, REF_LOCAL, MAX_BATCH_BYTES, MAX_BATCH_OPS, MAX_LOCAL_REFS,
};
use crate::root::handlers::watch_payload::{encode_watch_payload, track_watch_encode_reject};
use abi::symbols::SymbolId;
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::vec::Vec;
use crate::root::symbols::Interner;
use alloc::string::String;

// ============================================================================
// Instrumentation Counters
// ============================================================================

/// Total ApplyBatch calls
pub static BATCH_CALLS: AtomicU64 = AtomicU64::new(0);
/// Total ops processed across all batches
pub static BATCH_OPS_TOTAL: AtomicU64 = AtomicU64::new(0);
/// Times scratch.ops Vec had to reallocate (should stabilize to 0)
pub static BATCH_REALLOCATIONS: AtomicU64 = AtomicU64::new(0);
/// Total watch payloads encoded successfully
pub static WATCH_ENCODE_OK_TOTAL: AtomicU64 = AtomicU64::new(0);
/// Total watch payloads rejected (all reasons)
pub static WATCH_ENCODE_REJECT_TOTAL: AtomicU64 = AtomicU64::new(0);
/// Watch payloads rejected due to invalid event encoding
pub static WATCH_ENCODE_REJECT_INVALID_TOTAL: AtomicU64 = AtomicU64::new(0);
/// Watch payloads rejected due to size constraints
pub static WATCH_ENCODE_REJECT_TOO_LARGE_TOTAL: AtomicU64 = AtomicU64::new(0);
/// Watch payloads rejected due to missing create mapping
pub static WATCH_ENCODE_REJECT_MISSING_REF_TOTAL: AtomicU64 = AtomicU64::new(0);
/// Throttle ROOT COMMIT logging (every N commits)
pub static ROOT_COMMIT_LOG_EVERY: u64 = 1024;
static ROOT_COMMIT_LOG_COUNTER: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// Per-CPU Scratch Buffer
// ============================================================================

/// Reusable scratch buffer for batch parsing and validation.
/// 
/// Avoids repeated allocations by reusing capacity across ApplyBatch calls.
/// Each root service instance owns one of these.
/// 
/// Note: locals arrays are boxed to avoid stack overflow (9KB total).
pub struct RootBatchScratch {
    /// Validated ops staging area (capacity preserved across calls)
    pub ops: Vec<ValidatedOp>,
    /// Local reference table (fixed size for zero-alloc, heap-allocated)
    pub locals: alloc::boxed::Box<[ThingId; MAX_LOCAL_REFS]>,
    /// Tracks which local refs have been initialized (heap-allocated)
    pub locals_init: alloc::boxed::Box<[bool; MAX_LOCAL_REFS]>,
}

impl RootBatchScratch {
    /// Create a new scratch buffer with pre-reserved capacity.
    pub fn new() -> Self {
        let mut ops = Vec::new();
        ops.reserve_exact(512); // Reasonable default capacity
        Self {
            ops,
            // Use Box to avoid 9KB stack allocation
            locals: alloc::boxed::Box::new([0; MAX_LOCAL_REFS]),
            locals_init: alloc::boxed::Box::new([false; MAX_LOCAL_REFS]),
        }
    }
    
    /// Reset scratch for next batch (preserves allocation capacity).
    pub fn reset(&mut self) {
        self.ops.clear();  // Keeps capacity
        self.locals_init.fill(false);
        // locals array can be left as-is since locals_init guards access
    }
}

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
/// # Returns
/// * `ApplyResult` with status, seq, and created IDs
pub fn apply_ops_and_commit(
    graph: &mut Graph,
    ops: &[ValidatedOp],
) -> ApplyResult {
    let mut local_refs: Vec<ThingId> = Vec::with_capacity(16);
    let mut created_ids: Vec<ThingId> = Vec::new();
    
    // Compute summary from validated ops (O(1) filtering at read time)
    let mut summary = CommitSummary::default();

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
                // Track kind for summary
                summary.kinds.insert(*kind);
            }
            ValidatedOp::PutEdge { src, rel, dst } => {
                graph.link(*src, *rel, *dst);
                // Track predicate and subject for summary
                summary.predicates.insert(*rel);
                summary.subjects.insert(*src);
            }
            ValidatedOp::SetProp { id, key, value } => {
                if let Some(node) = graph.get_node_mut(*id) {
                    node.props.insert(*key, *value);
                }
                // Track subject for summary
                summary.subjects.insert(*id);
            }
        }
    }

    // Commit: increment sequence number
    let new_seq = graph.root_seq.fetch_add(1, Ordering::SeqCst) + 1;

    // Encode canonical watch payload bytes for consumers
    let watch_payload_bytes = match encode_watch_payload(ops, &local_refs) {
        Ok(bytes) => {
            WATCH_ENCODE_OK_TOTAL.fetch_add(1, Ordering::Relaxed);
            bytes
        }
        Err(reason) => {
            WATCH_ENCODE_REJECT_TOTAL.fetch_add(1, Ordering::Relaxed);
            track_watch_encode_reject(reason);
            if cfg!(debug_assertions) {
                panic!("watch_encode: contract violation ({:?})", reason);
            }
            Vec::new()
        }
    };

    // Push to shared commit history with summary for O(1) filter matching
    graph.commit_history.push(new_seq, watch_payload_bytes, summary);

    // Diagnostic logging: throttle commit logs when watches are active
    if cfg!(debug_assertions) && !graph.global_watches.is_empty() {
        let n = ROOT_COMMIT_LOG_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        if n % ROOT_COMMIT_LOG_EVERY == 0 {
            crate::kinfo!(
                "ROOT COMMIT: seq={} ops={} watches={}",
                new_seq,
                ops.len(),
                graph.global_watches.len()
            );
        }
    }

    ApplyResult {
        status: 0,
        seq: new_seq,
        created_ids,
    }
}
// ============================================================================
// Batch Filter Matching
// ============================================================================

use crate::root::graph::WatchFilter;
use abi::root::{WATCH_F_KIND, WATCH_F_PREDICATE, WATCH_F_SUBJECT};

/// Check if a batch contains at least one op matching the filter.
/// 
/// This function scans the batch without allocating, checking each operation
/// against the filter criteria. It returns early on first match.
///
/// # Returns
/// - `Ok(true)` if at least one op matches the filter
/// - `Ok(false)` if no ops match
/// - `Err(-22)` if batch is malformed (EINVAL)
pub fn batch_matches_filter(
    batch: &[u8],
    filter: &WatchFilter,
    interner: &mut Interner,
    graph: &Graph
) -> Result<bool, i32> {
    // flags=0 means match all commits
    if filter.matches_all() {
        return Ok(true);
    }
    
    // Validate header
    if batch.len() < 8 {
        return Err(-22); // EINVAL
    }
    let magic = u32::from_le_bytes(batch[0..4].try_into().unwrap());
    let version = u16::from_le_bytes(batch[4..6].try_into().unwrap());
    let op_count = u16::from_le_bytes(batch[6..8].try_into().unwrap());
    
    if magic != BATCH_MAGIC || version != BATCH_VERSION {
        return Err(-22);
    }

    let mut local_kinds = [None; MAX_LOCAL_REFS];
    let mut cursor = 8usize;
    
    for _ in 0..op_count {
        if cursor >= batch.len() {
            return Err(-22);
        }
        let tag = batch[cursor];
        cursor += 1;
        
        match tag {
            OP_CREATE_NODE => {
                // kind_id: 16 bytes, out_ref: 2 bytes = 18 bytes total
                if cursor + 16 > batch.len() { return Err(-22); }
                let kind_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;

                // out_ref: 2 bytes
                if cursor + 2 > batch.len() { return Err(-22); }
                let out_idx = u16::from_le_bytes(batch[cursor..cursor+2].try_into().unwrap()) as usize;
                cursor += 2;

                let kind_str = bytes_to_hex(&kind_bytes);
                let kind = interner.intern(&kind_str);

                if out_idx < MAX_LOCAL_REFS {
                    local_kinds[out_idx] = Some(kind);
                }
                
                if (filter.flags & WATCH_F_KIND) != 0 {
                    if kind == filter.kind_id {
                        return Ok(true);
                    }
                }
            }
            OP_PUT_EDGE => {
                // subject: ThingRef, predicate: 16 bytes, object: ThingRef, flags: 4 bytes
                
                // Parse subject ThingRef
                if cursor >= batch.len() { return Err(-22); }
                let ref_kind = batch[cursor];
                cursor += 1;
                let subject_size = if ref_kind == REF_ABSOLUTE { 16 } else if ref_kind == REF_LOCAL { 2 } else { return Err(-22); };
                if cursor + subject_size > batch.len() { return Err(-22); }
                
                // Extract subject ID/Index
                let subject_val = if ref_kind == REF_ABSOLUTE {
                    u64::from_le_bytes(batch[cursor..cursor+8].try_into().unwrap())
                } else if ref_kind == REF_LOCAL {
                    u16::from_le_bytes(batch[cursor..cursor+2].try_into().unwrap()) as u64
                } else {
                    0
                };
                cursor += subject_size;
                
                // Predicate: 16 bytes (hash)
                if cursor + 16 > batch.len() { return Err(-22); }
                let pred_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                
                // Object ThingRef
                if cursor >= batch.len() { return Err(-22); }
                let obj_kind = batch[cursor];
                cursor += 1;
                let obj_size = if obj_kind == REF_ABSOLUTE { 16 } else if obj_kind == REF_LOCAL { 2 } else { return Err(-22); };
                if cursor + obj_size > batch.len() { return Err(-22); }
                cursor += obj_size;
                
                // Flags: 4 bytes
                if cursor + 4 > batch.len() { return Err(-22); }
                cursor += 4;
                
                // Match logic
                let mut matches = true;

                // Check SUBJECT filter
                if (filter.flags & WATCH_F_SUBJECT) != 0 {
                    if ref_kind != REF_ABSOLUTE || subject_val != filter.subject_lo {
                        matches = false;
                    }
                }
                
                // Check PREDICATE filter
                if matches && (filter.flags & WATCH_F_PREDICATE) != 0 {
                    let pred_str = bytes_to_hex(&pred_bytes);
                    let pred = interner.intern(&pred_str);
                    if pred != filter.predicate_id {
                        matches = false;
                    }
                }

                // Check KIND filter (of the subject)
                if matches && (filter.flags & WATCH_F_KIND) != 0 {
                    let mut found_kind = None;
                    if ref_kind == REF_ABSOLUTE {
                        found_kind = graph.get_kind(subject_val);
                    } else if ref_kind == REF_LOCAL {
                        let idx = subject_val as usize;
                        if idx < MAX_LOCAL_REFS {
                            found_kind = local_kinds[idx];
                        }
                    }

                    if found_kind != Some(filter.kind_id) {
                        matches = false;
                    }
                }

                if matches {
                    return Ok(true);
                }
            }
            OP_SET_PROP => {
                // subject: ThingRef, key: 16 bytes, value: 8 bytes
                if cursor >= batch.len() { return Err(-22); }
                let ref_kind = batch[cursor];
                cursor += 1;
                let subject_size = if ref_kind == REF_ABSOLUTE { 16 } else if ref_kind == REF_LOCAL { 2 } else { return Err(-22); };
                if cursor + subject_size > batch.len() { return Err(-22); }
                
                let subject_id = if ref_kind == REF_ABSOLUTE {
                    u64::from_le_bytes(batch[cursor..cursor+8].try_into().unwrap())
                } else {
                    0
                };
                cursor += subject_size;
                
                // Key (16 bytes) + value (8 bytes) = 24 bytes
                if cursor + 24 > batch.len() { return Err(-22); }
                cursor += 24;
                
                // Check SUBJECT filter
                if (filter.flags & WATCH_F_SUBJECT) != 0 {
                    if ref_kind == REF_ABSOLUTE && subject_id == filter.subject_lo {
                        return Ok(true);
                    }
                }
            }
            _ => return Err(-22), // Unknown op tag
        }
    }
    
    // No ops matched the filter
    Ok(false)
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

/// Parse a ThingRef from the batch buffer using scratch locals.
/// 
/// Returns the resolved ThingId or an error code:
/// - `-22` (EINVAL) for invalid format or uninitialized local ref
fn parse_ref_scratch(
    cursor: &mut usize, 
    data: &[u8], 
    scratch: &RootBatchScratch
) -> Result<ThingId, i32> {
    if *cursor >= data.len() { return Err(-22); }
    let kind = data[*cursor];
    *cursor += 1;
    match kind {
        REF_ABSOLUTE => {
            if *cursor + 16 > data.len() { return Err(-22); }
            let val = u64::from_le_bytes(data[*cursor..*cursor+8].try_into().unwrap());
            *cursor += 16;
            Ok(val)
        }
        REF_LOCAL => {
            if *cursor + 2 > data.len() { return Err(-22); }
            let idx = u16::from_le_bytes(data[*cursor..*cursor+2].try_into().unwrap()) as usize;
            *cursor += 2;
            // Validate local ref is within bounds and initialized
            if idx >= MAX_LOCAL_REFS {
                return Err(-22); // EINVAL: out of bounds
            }
            if !scratch.locals_init[idx] {
                return Err(-22); // EINVAL: not yet initialized
            }
            Ok(scratch.locals[idx])
        }
        _ => Err(-22)
    }
}

/// Parse batch bytes into validated operations using scratch buffer.
/// 
/// # Errors
/// - `-7` (E2BIG): batch too large or too many ops
/// - `-22` (EINVAL): malformed format, invalid refs
fn parse_batch_scratch(
    batch: &[u8],
    interner: &mut Interner,
    scratch: &mut RootBatchScratch,
) -> Result<(), i32> {
    // Cap validation: batch size
    if batch.len() > MAX_BATCH_BYTES {
        return Err(-7); // E2BIG
    }
    
    if batch.len() < 8 {
        return Err(-22); // EINVAL: too short for header
    }

    let magic = u32::from_le_bytes(batch[0..4].try_into().unwrap());
    let version = u16::from_le_bytes(batch[4..6].try_into().unwrap());
    let op_count = u16::from_le_bytes(batch[6..8].try_into().unwrap()) as usize;

    if magic != BATCH_MAGIC || version != BATCH_VERSION {
        return Err(-22); // EINVAL
    }
    
    // Cap validation: op count
    if op_count > MAX_BATCH_OPS {
        return Err(-7); // E2BIG
    }

    // Reserve additional capacity if needed (best-effort allocation tracking)
    if scratch.ops.capacity() < op_count {
        scratch.ops.reserve(op_count - scratch.ops.capacity());
    }

    let mut cursor = 8usize;

    for _ in 0..op_count {
        if cursor >= batch.len() { return Err(-22); }
        let tag = batch[cursor];
        cursor += 1;

        match tag {
            OP_CREATE_NODE => {
                if cursor + 16 > batch.len() { return Err(-22); }
                let kind_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let kind_str = bytes_to_hex(&kind_bytes);
                let kind = interner.intern(&kind_str);

                if cursor + 2 > batch.len() { return Err(-22); }
                let out_idx = u16::from_le_bytes(batch[cursor..cursor+2].try_into().unwrap()) as usize;
                cursor += 2;
                
                // Validate out_ref within bounds
                if out_idx >= MAX_LOCAL_REFS {
                    return Err(-22); // EINVAL: out_ref too large
                }

                scratch.ops.push(ValidatedOp::CreateNode { kind, out_idx });
                
                // Mark local ref as initialized (placeholder value, filled at apply time)
                scratch.locals[out_idx] = 0;
                scratch.locals_init[out_idx] = true;
            }
            OP_PUT_EDGE => {
                let src = parse_ref_scratch(&mut cursor, batch, scratch)?;

                if cursor + 16 > batch.len() { return Err(-22); }
                let rel_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let rel_str = bytes_to_hex(&rel_bytes);
                let rel = interner.intern(&rel_str);

                let dst = parse_ref_scratch(&mut cursor, batch, scratch)?;

                scratch.ops.push(ValidatedOp::PutEdge { src, rel, dst });
            }
            OP_SET_PROP => {
                let id = parse_ref_scratch(&mut cursor, batch, scratch)?;

                if cursor + 16 > batch.len() { return Err(-22); }
                let key_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let key_str = bytes_to_hex(&key_bytes);
                let key = interner.intern(&key_str);

                if cursor + 8 > batch.len() { return Err(-22); }
                let value = u64::from_le_bytes(batch[cursor..cursor + 8].try_into().unwrap());
                cursor += 8;

                scratch.ops.push(ValidatedOp::SetProp { id, key, value });
            }
            _ => { return Err(-22); } // Unknown op tag
        }
    }

    Ok(())
}

/// Handle SYS_ROOT_APPLY_BATCH with scratch buffer for zero-alloc hot path.
///
/// Parses the batch, validates operations, and commits through the canonical path.
pub fn handle_apply_batch_with_scratch(
    graph: &mut Graph,
    interner: &mut Interner,
    batch: &[u8],
    scratch: &mut RootBatchScratch,
) -> HandlerResult {
    // Increment call counter
    BATCH_CALLS.fetch_add(1, Ordering::Relaxed);
    
    // Track capacity before parsing for reallocation detection
    let old_cap = scratch.ops.capacity();
    
    // Reset scratch for this batch
    scratch.reset();
    
    // Parse batch into scratch.ops (validation happens here)
    if let Err(code) = parse_batch_scratch(batch, interner, scratch) {
        return (code, 0);
    }
    
    // Track ops and detect reallocations
    BATCH_OPS_TOTAL.fetch_add(scratch.ops.len() as u64, Ordering::Relaxed);
    if scratch.ops.capacity() != old_cap {
        BATCH_REALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    }

    // Apply through canonical commit path
    let result = apply_ops_and_commit(graph, &scratch.ops);
    
    (result.status, result.seq)
}

/// Handle SYS_ROOT_APPLY_BATCH (legacy interface without scratch - allocates each call)
///
/// Parses the batch, validates operations, and commits through the canonical path.
/// 
/// NOTE: This allocates a new scratch each call. For zero-alloc hot path, 
/// use `handle_apply_batch_with_scratch` instead.
pub fn handle_apply_batch(
    graph: &mut Graph,
    interner: &mut Interner,
    batch: &[u8],
) -> HandlerResult {
    let mut scratch = RootBatchScratch::new();
    handle_apply_batch_with_scratch(graph, interner, batch, &mut scratch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::root::graph::WatchFilter;
    use crate::root::symbols::Interner;
    use abi::root::{OP_CREATE_NODE, OP_PUT_EDGE, REF_LOCAL, WATCH_F_KIND, BATCH_MAGIC, BATCH_VERSION};

    #[test]
    fn test_batch_matches_filter_behavior() {
        let mut interner = Interner::new();
        let graph = Graph::new();

        // Define kinds
        let kind_a_str = "aaaaaaaabbbbbbbbccccccccdddddddd"; // 32 hex chars = 16 bytes
        let kind_b_str = "11111111222222223333333344444444";

        let kind_a_id = interner.intern(kind_a_str);

        let mut filter = WatchFilter::default();
        filter.flags = WATCH_F_KIND;
        filter.kind_id = kind_a_id;

        // Helper to make bytes from hex
        let hex_to_bytes = |s: &str| -> [u8; 16] {
            let mut b = [0u8; 16];
            for i in 0..16 {
                b[i] = u8::from_str_radix(&s[i*2..i*2+2], 16).unwrap();
            }
            b
        };

        let kind_a_bytes = hex_to_bytes(kind_a_str);
        let kind_b_bytes = hex_to_bytes(kind_b_str);

        // 1. Test Mismatch (CreateNode Kind B)
        let mut batch_b = alloc::vec::Vec::new();
        batch_b.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        batch_b.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        batch_b.extend_from_slice(&1u16.to_le_bytes());
        batch_b.push(OP_CREATE_NODE);
        batch_b.extend_from_slice(&kind_b_bytes);
        batch_b.extend_from_slice(&0u16.to_le_bytes());

        let result = batch_matches_filter(&batch_b, &filter, &mut interner, &graph).expect("parse failed");
        assert!(!result, "Should NOT match kind B when filtering for kind A");

        // 2. Test Match (CreateNode Kind A)
        let mut batch_a = alloc::vec::Vec::new();
        batch_a.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        batch_a.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        batch_a.extend_from_slice(&1u16.to_le_bytes());
        batch_a.push(OP_CREATE_NODE);
        batch_a.extend_from_slice(&kind_a_bytes);
        batch_a.extend_from_slice(&0u16.to_le_bytes());

        let result = batch_matches_filter(&batch_a, &filter, &mut interner, &graph).expect("parse failed");
        assert!(result, "Should match kind A when filtering for kind A");

        // 3. Test Edge Creation with Local Ref kind lookup
        // Filter is Kind=A.
        // Batch: CreateNode(Kind A, local=0) -> PutEdge(Src=local:0) => Should Match
        let mut batch_edge_match = alloc::vec::Vec::new();
        batch_edge_match.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        batch_edge_match.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        batch_edge_match.extend_from_slice(&2u16.to_le_bytes()); // 2 ops

        // Op 1: CreateNode Kind A -> local 0
        batch_edge_match.push(OP_CREATE_NODE);
        batch_edge_match.extend_from_slice(&kind_a_bytes);
        batch_edge_match.extend_from_slice(&0u16.to_le_bytes());

        // Op 2: PutEdge Src=local:0
        batch_edge_match.push(OP_PUT_EDGE);
        batch_edge_match.push(REF_LOCAL);
        batch_edge_match.extend_from_slice(&0u16.to_le_bytes()); // local index 0
        batch_edge_match.extend_from_slice(&[0u8; 16]); // predicate (irrelevant)
        batch_edge_match.push(REF_LOCAL); // object (irrelevant)
        batch_edge_match.extend_from_slice(&0u16.to_le_bytes());
        batch_edge_match.extend_from_slice(&0u32.to_le_bytes()); // flags

        let result = batch_matches_filter(&batch_edge_match, &filter, &mut interner, &graph).expect("parse failed");
        assert!(result, "Should match Edge from Kind A (local ref)");

        // 4. Test Edge Creation Mismatch
        // Filter is Kind=A.
        // Batch: CreateNode(Kind B, local=0) -> PutEdge(Src=local:0) => Should NOT Match
        let mut batch_edge_mismatch = alloc::vec::Vec::new();
        batch_edge_mismatch.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        batch_edge_mismatch.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        batch_edge_mismatch.extend_from_slice(&2u16.to_le_bytes());

        batch_edge_mismatch.push(OP_CREATE_NODE);
        batch_edge_mismatch.extend_from_slice(&kind_b_bytes);
        batch_edge_mismatch.extend_from_slice(&0u16.to_le_bytes());

        batch_edge_mismatch.push(OP_PUT_EDGE);
        batch_edge_mismatch.push(REF_LOCAL);
        batch_edge_mismatch.extend_from_slice(&0u16.to_le_bytes());
        batch_edge_mismatch.extend_from_slice(&[0u8; 16]);
        batch_edge_mismatch.push(REF_LOCAL);
        batch_edge_mismatch.extend_from_slice(&0u16.to_le_bytes());
        batch_edge_mismatch.extend_from_slice(&0u32.to_le_bytes());

        let result = batch_matches_filter(&batch_edge_mismatch, &filter, &mut interner, &graph).expect("parse failed");
        assert!(!result, "Should NOT match Edge from Kind B (local ref)");
    }
}
