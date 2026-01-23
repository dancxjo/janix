//! Batch application and commit handling.
//!
//! This module provides the canonical commit path for all Root graph mutations.
//! Both multi-op batches (SYS_ROOT_APPLY_BATCH) and single-op syscalls
//! (CREATE_NODE, LINK, PROP_SET) route through `apply_ops_and_commit()`.

use crate::root::graph::{CommitSummary, Graph, ThingId};
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

    // Push to shared commit history with summary for O(1) filter matching
    graph
        .commit_history
        .push(new_seq, commit_bytes.to_vec(), summary);

    // Wake any watchers waiting for new commits
    graph.check_and_wake_watches();

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
pub fn batch_matches_filter(batch: &[u8], filter: &WatchFilter) -> Result<bool, i32> {
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
                if cursor + 18 > batch.len() {
                    return Err(-22);
                }

                // KIND filter matching:
                // The 16-byte kind in the batch is a hash. To properly match, we'd
                // need to intern it and compare with filter.kind_id. For v0, we
                // match any CREATE_NODE when kind filter is set (conservative).
                if (filter.flags & WATCH_F_KIND) != 0 {
                    // TODO: Full kind matching requires comparing interned symbols
                    // For now, any CREATE_NODE matches if kind filter is set
                    return Ok(true);
                }
                cursor += 18;
            }
            OP_PUT_EDGE => {
                // subject: ThingRef, predicate: 16 bytes, object: ThingRef, flags: 4 bytes

                // Parse subject ThingRef
                if cursor >= batch.len() {
                    return Err(-22);
                }
                let ref_kind = batch[cursor];
                cursor += 1;
                let subject_size = if ref_kind == REF_ABSOLUTE {
                    16
                } else if ref_kind == REF_LOCAL {
                    2
                } else {
                    return Err(-22);
                };
                if cursor + subject_size > batch.len() {
                    return Err(-22);
                }

                // Extract subject ID if absolute
                let subject_id = if ref_kind == REF_ABSOLUTE {
                    u64::from_le_bytes(batch[cursor..cursor + 8].try_into().unwrap())
                } else {
                    0 // Local refs can't match absolute filters
                };
                cursor += subject_size;

                // Predicate: 16 bytes (hash)
                if cursor + 16 > batch.len() {
                    return Err(-22);
                }
                // Note: We store predicate position for future use
                let _pred_start = cursor;
                cursor += 16;

                // Object ThingRef
                if cursor >= batch.len() {
                    return Err(-22);
                }
                let obj_kind = batch[cursor];
                cursor += 1;
                let obj_size = if obj_kind == REF_ABSOLUTE {
                    16
                } else if obj_kind == REF_LOCAL {
                    2
                } else {
                    return Err(-22);
                };
                if cursor + obj_size > batch.len() {
                    return Err(-22);
                }
                cursor += obj_size;

                // Flags: 4 bytes
                if cursor + 4 > batch.len() {
                    return Err(-22);
                }
                cursor += 4;

                // Check SUBJECT filter
                if (filter.flags & WATCH_F_SUBJECT) != 0 {
                    if ref_kind == REF_ABSOLUTE && subject_id == filter.subject_lo {
                        return Ok(true);
                    }
                }

                // Check PREDICATE filter
                // TODO: Full predicate matching requires comparing interned symbols
                // For v0, any PUT_EDGE matches if predicate filter is set
                if (filter.flags & WATCH_F_PREDICATE) != 0 {
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
                let subject_size = if ref_kind == REF_ABSOLUTE {
                    16
                } else if ref_kind == REF_LOCAL {
                    2
                } else {
                    return Err(-22);
                };
                if cursor + subject_size > batch.len() {
                    return Err(-22);
                }

                let subject_id = if ref_kind == REF_ABSOLUTE {
                    u64::from_le_bytes(batch[cursor..cursor + 8].try_into().unwrap())
                } else {
                    0
                };
                cursor += subject_size;

                // Key (16 bytes) + value (8 bytes) = 24 bytes
                if cursor + 24 > batch.len() {
                    return Err(-22);
                }
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
    scratch: &RootBatchScratch,
) -> Result<ThingId, i32> {
    if *cursor >= data.len() {
        return Err(-22);
    }
    let kind = data[*cursor];
    *cursor += 1;
    match kind {
        REF_ABSOLUTE => {
            if *cursor + 16 > data.len() {
                return Err(-22);
            }
            let val = u64::from_le_bytes(data[*cursor..*cursor + 8].try_into().unwrap());
            *cursor += 16;
            Ok(val)
        }
        REF_LOCAL => {
            if *cursor + 2 > data.len() {
                return Err(-22);
            }
            let idx = u16::from_le_bytes(data[*cursor..*cursor + 2].try_into().unwrap()) as usize;
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
        if cursor >= batch.len() {
            return Err(-22);
        }
        let tag = batch[cursor];
        cursor += 1;

        match tag {
            OP_CREATE_NODE => {
                if cursor + 16 > batch.len() {
                    return Err(-22);
                }
                let kind_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let kind_str = bytes_to_hex(&kind_bytes);
                let kind = interner.intern(&kind_str);

                if cursor + 2 > batch.len() {
                    return Err(-22);
                }
                let out_idx =
                    u16::from_le_bytes(batch[cursor..cursor + 2].try_into().unwrap()) as usize;
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

                if cursor + 16 > batch.len() {
                    return Err(-22);
                }
                let rel_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let rel_str = bytes_to_hex(&rel_bytes);
                let rel = interner.intern(&rel_str);

                let dst = parse_ref_scratch(&mut cursor, batch, scratch)?;

                scratch.ops.push(ValidatedOp::PutEdge { src, rel, dst });
            }
            OP_SET_PROP => {
                let id = parse_ref_scratch(&mut cursor, batch, scratch)?;

                if cursor + 16 > batch.len() {
                    return Err(-22);
                }
                let key_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let key_str = bytes_to_hex(&key_bytes);
                let key = interner.intern(&key_str);

                if cursor + 8 > batch.len() {
                    return Err(-22);
                }
                let value = u64::from_le_bytes(batch[cursor..cursor + 8].try_into().unwrap());
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
    let result = apply_ops_and_commit(graph, &scratch.ops, batch);

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
