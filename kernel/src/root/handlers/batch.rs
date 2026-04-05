//! Batch application and commit handling.
//!
//! This module provides the canonical commit path for all Root graph mutations.
//! Both multi-op batches (SYS_ROOT_APPLY_BATCH) and single-op syscalls
//! (CREATE_NODE, LINK, PROP_SET) route through `apply_ops_and_commit()`.

use crate::root::graph::{CommitSummary, Graph, ThingId};
use crate::root::handlers::watch_payload::{encode_watch_payload, track_watch_encode_reject};
use crate::root::handlers::HandlerResult;
use crate::root::symbols::Interner;
use abi::root::{
    BATCH_MAGIC, BATCH_VERSION, MAX_BATCH_BYTES, MAX_BATCH_OPS, MAX_LOCAL_REFS, OP_CREATE_NODE,
    OP_PUT_EDGE, OP_SET_PROP, REF_ABSOLUTE, REF_LOCAL,
};
use abi::symbols::SymbolId;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

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
        self.ops.clear(); // Keeps capacity
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
    PutEdge {
        src: ThingId,
        rel: SymbolId,
        dst: ThingId,
    },
    /// Set a property on a node.
    SetProp {
        id: ThingId,
        key: SymbolId,
        value: u64,
    },
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
pub fn apply_ops_and_commit(graph: &mut Graph, ops: &[ValidatedOp]) -> ApplyResult {
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
                // Track subject AND predicate (key) for summary
                summary.subjects.insert(*id);
                summary.predicates.insert(*key);
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
    graph
        .commit_history
        .push(new_seq, watch_payload_bytes, summary);

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
// Internal Parsing Helpers (Non-Panicking)
// ============================================================================

/// Safely read a u16 from the data buffer and advance the cursor.
fn read_u16(data: &[u8], cursor: &mut usize) -> Result<u16, i32> {
    if *cursor + 2 > data.len() {
        return Err(-22); // EINVAL
    }
    let val = u16::from_le_bytes([data[*cursor], data[*cursor + 1]]);
    *cursor += 2;
    Ok(val)
}

/// Safely read a u32 from the data buffer and advance the cursor.
fn read_u32(data: &[u8], cursor: &mut usize) -> Result<u32, i32> {
    if *cursor + 4 > data.len() {
        return Err(-22); // EINVAL
    }
    let val = u32::from_le_bytes([
        data[*cursor],
        data[*cursor + 1],
        data[*cursor + 2],
        data[*cursor + 3],
    ]);
    *cursor += 4;
    Ok(val)
}

/// Safely read a 16-byte ID and resolve it to a u64 handle.
///
/// Enforces the "u64-handle bridge": the upper 8 bytes MUST be zero.
/// Returns the lower 8 bytes as a u64.
fn read_u64_id(data: &[u8], cursor: &mut usize) -> Result<u64, i32> {
    if *cursor + 16 > data.len() {
        return Err(-22); // EINVAL
    }

    let mut id_bytes = [0u8; 16];
    id_bytes.copy_from_slice(&data[*cursor..*cursor + 16]);

    let lo = u64::from_le_bytes([
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        id_bytes[4],
        id_bytes[5],
        id_bytes[6],
        id_bytes[7],
    ]);
    let hi = u64::from_le_bytes([
        id_bytes[8],
        id_bytes[9],
        id_bytes[10],
        id_bytes[11],
        id_bytes[12],
        id_bytes[13],
        id_bytes[14],
        id_bytes[15],
    ]);

    if hi != 0 {
        // ID Duality Violation: User tried to pass a true 128-bit ID
        // where only u64 handles are currently supported by the kernel.
        static LOGGED_VIOLATION: core::sync::atomic::AtomicBool =
            core::sync::atomic::AtomicBool::new(false);
        if !LOGGED_VIOLATION.swap(true, Ordering::Relaxed) {
            crate::kwarn!(
                "Root: 16-byte ID duality violation (upper 8 bytes non-zero). Kernel only supports u64-bridge handles."
            );
        }
        return Err(-22); // EINVAL
    }

    *cursor += 16;
    Ok(lo)
}

/// Safely read 16 bytes as a fixed array (e.g. for Symbols).
fn read_16(data: &[u8], cursor: &mut usize) -> Result<[u8; 16], i32> {
    if *cursor + 16 > data.len() {
        return Err(-22); // EINVAL
    }
    let mut buf = [0u8; 16];
    buf.copy_from_slice(&data[*cursor..*cursor + 16]);
    *cursor += 16;
    Ok(buf)
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
    graph: &Graph,
) -> Result<bool, i32> {
    let mut cursor = 0usize;

    // Validate header
    let magic = read_u32(batch, &mut cursor)?;
    let version = read_u16(batch, &mut cursor)?;
    let op_count = read_u16(batch, &mut cursor)?;

    if magic != BATCH_MAGIC || version != BATCH_VERSION {
        return Err(-22);
    }

    // flags=0 means match all commits
    if filter.matches_all() {
        return Ok(true);
    }

    let mut local_kinds = [None; MAX_LOCAL_REFS];

    for _ in 0..op_count {
        if cursor >= batch.len() {
            return Err(-22);
        }
        let tag = batch[cursor];
        cursor += 1;

        match tag {
            OP_CREATE_NODE => {
                let kind_bytes = read_16(batch, &mut cursor)?;
                let out_idx = read_u16(batch, &mut cursor)? as usize;

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
                if cursor >= batch.len() {
                    return Err(-22);
                }
                let ref_kind = batch[cursor];
                cursor += 1;

                let subject_val = match ref_kind {
                    REF_ABSOLUTE => read_u64_id(batch, &mut cursor)?,
                    REF_LOCAL => read_u16(batch, &mut cursor)? as u64,
                    _ => return Err(-22),
                };

                // Predicate: 16 bytes (hash)
                let pred_bytes = read_16(batch, &mut cursor)?;

                // Object ThingRef
                if cursor >= batch.len() {
                    return Err(-22);
                }
                let obj_ref_kind = batch[cursor];
                cursor += 1;
                match obj_ref_kind {
                    REF_ABSOLUTE => {
                        let _ = read_u64_id(batch, &mut cursor)?;
                    }
                    REF_LOCAL => {
                        let _ = read_u16(batch, &mut cursor)?;
                    }
                    _ => return Err(-22),
                }

                // Flags: 4 bytes
                let _flags = read_u32(batch, &mut cursor)?;

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
                if cursor >= batch.len() {
                    return Err(-22);
                }
                let ref_kind = batch[cursor];
                cursor += 1;

                let subject_id = match ref_kind {
                    REF_ABSOLUTE => read_u64_id(batch, &mut cursor)?,
                    REF_LOCAL => {
                        // SET_PROP on local ref is allowed, but filter matching
                        // against subject_lo only works for absolute IDs in current ABI.
                        let _idx = read_u16(batch, &mut cursor)?;
                        0 // Not an absolute ID
                    }
                    _ => return Err(-22),
                };

                // Key (16 bytes)
                let key_bytes = read_16(batch, &mut cursor)?;

                // Value (8 bytes)
                if cursor + 8 > batch.len() {
                    return Err(-22);
                }
                let _value = u64::from_le_bytes([
                    batch[cursor],
                    batch[cursor + 1],
                    batch[cursor + 2],
                    batch[cursor + 3],
                    batch[cursor + 4],
                    batch[cursor + 5],
                    batch[cursor + 6],
                    batch[cursor + 7],
                ]);
                cursor += 8;

                // Check SUBJECT filter
                if (filter.flags & WATCH_F_SUBJECT) != 0 {
                    if ref_kind == REF_ABSOLUTE && subject_id == filter.subject_lo {
                        return Ok(true);
                    }
                }

                // Check PREDICATE filter (match key against predicate_id)
                if (filter.flags & WATCH_F_PREDICATE) != 0 {
                    let key_str = bytes_to_hex(&key_bytes);
                    let key = interner.intern(&key_str);
                    if key == filter.predicate_id {
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
    scratch: &RootBatchScratch,
) -> Result<ThingId, i32> {
    if *cursor >= data.len() {
        return Err(-22);
    }
    let kind = data[*cursor];
    *cursor += 1;
    match kind {
        REF_ABSOLUTE => read_u64_id(data, cursor),
        REF_LOCAL => {
            let idx = read_u16(data, cursor)? as usize;
            // Validate local ref is within bounds and initialized
            if idx >= MAX_LOCAL_REFS {
                return Err(-22); // EINVAL: out of bounds
            }
            if !scratch.locals_init[idx] {
                return Err(-22); // EINVAL: not yet initialized
            }
            Ok(scratch.locals[idx])
        }
        _ => Err(-22),
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

    let mut cursor = 0usize;

    // Parse header
    let magic = read_u32(batch, &mut cursor)?;
    let version = read_u16(batch, &mut cursor)?;
    let op_count = read_u16(batch, &mut cursor)? as usize;

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

    for _ in 0..op_count {
        if cursor >= batch.len() {
            return Err(-22);
        }
        let tag = batch[cursor];
        cursor += 1;

        match tag {
            OP_CREATE_NODE => {
                let kind_bytes = read_16(batch, &mut cursor)?;
                let kind_str = bytes_to_hex(&kind_bytes);
                let kind = interner.intern(&kind_str);

                let out_idx = read_u16(batch, &mut cursor)? as usize;

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
                let rel_bytes = read_16(batch, &mut cursor)?;
                let rel_str = bytes_to_hex(&rel_bytes);
                let rel = interner.intern(&rel_str);
                let dst = parse_ref_scratch(&mut cursor, batch, scratch)?;

                scratch.ops.push(ValidatedOp::PutEdge { src, rel, dst });
            }
            OP_SET_PROP => {
                let id = parse_ref_scratch(&mut cursor, batch, scratch)?;
                let key_bytes = read_16(batch, &mut cursor)?;
                let key_str = bytes_to_hex(&key_bytes);
                let key = interner.intern(&key_str);

                if cursor + 8 > batch.len() {
                    return Err(-22);
                }
                let value = u64::from_le_bytes([
                    batch[cursor],
                    batch[cursor + 1],
                    batch[cursor + 2],
                    batch[cursor + 3],
                    batch[cursor + 4],
                    batch[cursor + 5],
                    batch[cursor + 6],
                    batch[cursor + 7],
                ]);
                cursor += 8;

                scratch.ops.push(ValidatedOp::SetProp { id, key, value });
            }
            _ => {
                return Err(-22);
            } // Unknown op tag
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

    // Host fallback links for CreateNode ops in this batch
    if result.status == 0 && !result.created_ids.is_empty() {
        let mut created_idx = 0usize;
        for op in &scratch.ops {
            if let ValidatedOp::CreateNode { kind, .. } = op {
                if let Some(id) = result.created_ids.get(created_idx).copied() {
                    super::graph::maybe_link_host_fallback(graph, interner, id, *kind);
                }
                created_idx += 1;
            }
        }
    }

    (result.status, result.seq)
}

/// Like `handle_apply_batch_with_scratch` but returns the full `ApplyResult`
/// including `created_ids`, so callers can retrieve node IDs from batched
/// CreateNode operations.
pub fn handle_apply_batch_with_scratch_full(
    graph: &mut Graph,
    interner: &mut Interner,
    batch: &[u8],
    scratch: &mut RootBatchScratch,
) -> ApplyResult {
    // Increment call counter
    BATCH_CALLS.fetch_add(1, Ordering::Relaxed);

    // Track capacity before parsing for reallocation detection
    let old_cap = scratch.ops.capacity();

    // Reset scratch for this batch
    scratch.reset();

    // Parse batch into scratch.ops (validation happens here)
    if let Err(code) = parse_batch_scratch(batch, interner, scratch) {
        return ApplyResult {
            status: code,
            seq: 0,
            created_ids: Vec::new(),
        };
    }

    // Track ops and detect reallocations
    BATCH_OPS_TOTAL.fetch_add(scratch.ops.len() as u64, Ordering::Relaxed);
    if scratch.ops.capacity() != old_cap {
        BATCH_REALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    }

    // Apply through canonical commit path
    let result = apply_ops_and_commit(graph, &scratch.ops);

    // Host fallback links for CreateNode ops in this batch
    if result.status == 0 && !result.created_ids.is_empty() {
        let mut created_idx = 0usize;
        for op in &scratch.ops {
            if let ValidatedOp::CreateNode { kind, .. } = op {
                if let Some(id) = result.created_ids.get(created_idx).copied() {
                    super::graph::maybe_link_host_fallback(graph, interner, id, *kind);
                }
                created_idx += 1;
            }
        }
    }

    result
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
    use abi::root::{
        BATCH_MAGIC, BATCH_VERSION, OP_CREATE_NODE, OP_PUT_EDGE, REF_LOCAL, WATCH_F_KIND,
    };

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
                b[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
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

        let result =
            batch_matches_filter(&batch_b, &filter, &mut interner, &graph).expect("parse failed");
        assert!(!result, "Should NOT match kind B when filtering for kind A");

        // 2. Test Match (CreateNode Kind A)
        let mut batch_a = alloc::vec::Vec::new();
        batch_a.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        batch_a.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        batch_a.extend_from_slice(&1u16.to_le_bytes());
        batch_a.push(OP_CREATE_NODE);
        batch_a.extend_from_slice(&kind_a_bytes);
        batch_a.extend_from_slice(&0u16.to_le_bytes());

        let result =
            batch_matches_filter(&batch_a, &filter, &mut interner, &graph).expect("parse failed");
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

        let result = batch_matches_filter(&batch_edge_match, &filter, &mut interner, &graph)
            .expect("parse failed");
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

        let result = batch_matches_filter(&batch_edge_mismatch, &filter, &mut interner, &graph)
            .expect("parse failed");
        assert!(!result, "Should NOT match Edge from Kind B (local ref)");
    }

    #[test]
    fn test_batch_hardening_malformed_input() {
        let mut interner = Interner::new();
        let graph = Graph::new();
        let mut scratch = RootBatchScratch::new();

        // 1. Too short for header
        let small = [0u8; 4];
        assert_eq!(
            parse_batch_scratch(&small, &mut interner, &mut scratch),
            Err(-22)
        );
        assert_eq!(
            batch_matches_filter(&small, &WatchFilter::default(), &mut interner, &graph),
            Err(-22)
        );

        // 2. Correct header but magic mismatch
        let mut bad_magic = alloc::vec::Vec::new();
        bad_magic.extend_from_slice(&0u32.to_le_bytes()); // Not BATCH_MAGIC
        bad_magic.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        bad_magic.extend_from_slice(&1u16.to_le_bytes());
        assert_eq!(
            parse_batch_scratch(&bad_magic, &mut interner, &mut scratch),
            Err(-22)
        );

        // 3. Truncated Op (CreateNode)
        let mut truncated_op = alloc::vec::Vec::new();
        truncated_op.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        truncated_op.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        truncated_op.extend_from_slice(&1u16.to_le_bytes());
        truncated_op.push(OP_CREATE_NODE);
        truncated_op.extend_from_slice(&[0u8; 10]); // Truncated kind_id (should be 16)
        assert_eq!(
            parse_batch_scratch(&truncated_op, &mut interner, &mut scratch),
            Err(-22)
        );

        // Use a filter that forces op parsing
        let mut filter_kind = WatchFilter::default();
        filter_kind.flags = abi::root::WATCH_F_KIND;
        assert_eq!(
            batch_matches_filter(&truncated_op, &filter_kind, &mut interner, &graph),
            Err(-22)
        );

        // 4. Unknown Op Tag
        let mut unknown_tag = alloc::vec::Vec::new();
        unknown_tag.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        unknown_tag.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        unknown_tag.extend_from_slice(&1u16.to_le_bytes());
        unknown_tag.push(0xFF); // Invalid tag
        assert_eq!(
            parse_batch_scratch(&unknown_tag, &mut interner, &mut scratch),
            Err(-22)
        );
        assert_eq!(
            batch_matches_filter(&unknown_tag, &filter_kind, &mut interner, &graph),
            Err(-22)
        );

        // 5. Local Ref before init
        let mut uninit_ref = alloc::vec::Vec::new();
        uninit_ref.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        uninit_ref.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        uninit_ref.extend_from_slice(&1u16.to_le_bytes());
        uninit_ref.push(OP_PUT_EDGE);
        uninit_ref.push(REF_LOCAL);
        uninit_ref.extend_from_slice(&0u16.to_le_bytes()); // Index 0 not initialized
        assert_eq!(
            parse_batch_scratch(&uninit_ref, &mut interner, &mut scratch),
            Err(-22)
        );

        // 6. ID Duality Violation (non-zero upper bytes)
        let mut bad_id = alloc::vec::Vec::new();
        bad_id.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        bad_id.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        bad_id.extend_from_slice(&1u16.to_le_bytes());
        bad_id.push(OP_PUT_EDGE);
        bad_id.push(REF_ABSOLUTE);
        bad_id.extend_from_slice(&1u64.to_le_bytes()); // low 8
        bad_id.extend_from_slice(&1u64.to_le_bytes()); // upper 8 (VIOLATION)
        assert_eq!(
            parse_batch_scratch(&bad_id, &mut interner, &mut scratch),
            Err(-22)
        );

        let mut filter = WatchFilter::default();
        filter.flags = abi::root::WATCH_F_SUBJECT;
        filter.subject_lo = 1;
        assert_eq!(
            batch_matches_filter(&bad_id, &filter, &mut interner, &graph),
            Err(-22)
        );
    }
}
