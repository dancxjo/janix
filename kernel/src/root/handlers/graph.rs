//! Graph core and property handlers.
//!
//! Mutation handlers (create_node, link, prop_set) now route through
//! the canonical batch pipeline for consistent watch delivery.

use crate::root::graph::Graph;
use crate::root::journal::{Journal, JournalOp};
use crate::root::symbols::Interner;
use crate::root::{SymbolShell, graph_anchors};
use abi::schema::{kinds, rels};
use abi::symbols::SymbolId;
#[allow(unused_imports)]
use core::sync::atomic::Ordering;

use super::HandlerResult;
use super::batch::{ValidatedOp, apply_ops_and_commit};

/// Helper to resolve Shell to SymbolId
pub fn resolve_shell(shell: SymbolShell, interner: &mut Interner) -> SymbolId {
    match shell {
        SymbolShell::Id(id) => id,
        SymbolShell::Str(s) => interner.intern(&s),
        SymbolShell::Static(s) => interner.intern(s),
    }
}

pub fn handle_intern(interner: &mut Interner, name: &str) -> HandlerResult {
    let id = interner.intern(name);
    (0, id as u64)
}

pub fn handle_get_kind(graph: &Graph, id: u64) -> HandlerResult {
    if let Some(k) = graph.get_kind(id) {
        (0, k as u64)
    } else {
        (-1, 0)
    }
}

/// Create a new node in the graph.
///
/// Routes through the canonical batch pipeline for consistent watch delivery.
pub fn handle_create_node(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    kind: SymbolShell,
    creator_tid: u64,
    owner_thing_id: Option<u64>,
) -> HandlerResult {
    let kid = resolve_shell(kind, interner);

    // Build validated op
    let ops = [ValidatedOp::CreateNode {
        kind: kid,
        out_idx: 0,
    }];

    // Apply through canonical commit path (watch payload is synthesized there)
    let result = apply_ops_and_commit(graph, &ops);

    // Journal entry (kept separate for recovery purposes)
    if result.status == 0 && !result.created_ids.is_empty() {
        let id = result.created_ids[0];
        journal.append(JournalOp::CreateResult {
            id,
            kind: kid as u64,
        });

        // Set the owner if we have an owner_thing_id
        if let Some(owner_id) = owner_thing_id {
            graph.set_owner(id, Some(owner_id));
        }

        maybe_link_host_fallback(graph, interner, id, kid);
        (0, id)
    } else {
        (result.status, 0)
    }
}

pub(super) fn maybe_link_host_fallback(
    graph: &mut Graph,
    interner: &mut Interner,
    id: u64,
    kind: SymbolId,
) {
    let kind_str = match interner.resolve(kind) {
        Some(s) => s,
        None => return,
    };

    // Only attach for known host-orphaned kinds.
    let needs_host = kind_str == kinds::BOOT_MODULE
        || kind_str.starts_with("dev.bus.")
        || kind_str.starts_with("svc.");

    if !needs_host {
        return;
    }

    let host = match graph_anchors::host() {
        Some(host) if host != id => host,
        _ => return,
    };

    // Attach with a generic relationship so higher-fidelity links can still be added.
    let _ = handle_link(
        graph,
        interner,
        host,
        SymbolShell::Static(rels::HAS_RESOURCE),
        id,
    );
}

pub fn handle_orphan_thing(graph: &mut Graph, thing_id: u64) -> HandlerResult {
    if graph.orphan_thing(thing_id) {
        (0, 0)
    } else {
        // Thing not found
        (-1, 0)
    }
}

pub fn handle_cleanup_task_things(graph: &mut Graph, owner_thing_id: u64) -> HandlerResult {
    // Get all things owned by this owner
    let owned_things = graph.get_owned_things(owner_thing_id);
    let count = owned_things.len();

    // Remove all owned things
    for thing_id in owned_things {
        graph.remove_node(thing_id);
    }

    (0, count as u64)
}

pub fn handle_prop_get(
    graph: &mut Graph,
    interner: &mut Interner,
    id: u64,
    key: SymbolShell,
) -> HandlerResult {
    let kid = resolve_shell(key, interner);

    // Intern the special "kind" key to check for virtual property
    let kind_key = interner.intern("kind");

    if let Some(node) = graph.get_node_mut(id) {
        // Virtual "kind" property: return node.kind field
        if kid == kind_key {
            return (0, node.kind as u64);
        }

        // Regular property lookup
        if let Some(val) = node.props.get(&kid) {
            (0, *val)
        } else {
            (-1, 0)
        }
    } else {
        (-1, 0)
    }
}

/// Set a property on a node.
///
/// Routes through the canonical batch pipeline for consistent watch delivery,
/// then also notifies node-level stream watches.
pub fn handle_prop_set(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    id: u64,
    key: SymbolShell,
    value: u64,
) -> HandlerResult {
    let kid = resolve_shell(key, interner);

    // Check if node exists
    if graph.get_node_mut(id).is_none() {
        return (-1, 0);
    }

    // Build validated op
    let ops = [ValidatedOp::SetProp {
        id,
        key: kid,
        value,
    }];

    // Apply through canonical commit path (watch payload is synthesized there)
    let result = apply_ops_and_commit(graph, &ops);

    if result.status != 0 {
        return (result.status, 0);
    }

    // Journal entry
    journal.append(JournalOp::UpdateProp {
        id,
        key: kid as u64,
        val: value,
    });

    (0, 0)
}

/// Create an edge between two nodes.
///
/// Routes through the canonical batch pipeline for consistent watch delivery.
pub fn handle_link(
    graph: &mut Graph,
    interner: &mut Interner,
    src: u64,
    rel: SymbolShell,
    dst: u64,
) -> HandlerResult {
    let rid = resolve_shell(rel, interner);

    // Build validated op
    let ops = [ValidatedOp::PutEdge { src, rel: rid, dst }];

    // Apply through canonical commit path (watch payload is synthesized there)
    let result = apply_ops_and_commit(graph, &ops);

    (result.status, 0)
}

pub fn handle_find(
    graph: &Graph,
    interner: &mut Interner,
    kind: SymbolShell,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    use abi::ids::HandleId;

    let kid = resolve_shell(kind, interner);
    let mut found_count = 0;
    let out_ptr = buffer as *mut abi::types::ThingId;
    let max_entries = (len as usize) / core::mem::size_of::<abi::types::ThingId>();

    if let Some(ids) = graph.kind_index.get(&kid) {
        for &id in ids {
            if found_count < max_entries {
                unsafe {
                    *out_ptr.add(found_count) = abi::types::ThingId::from_u64(id);
                }
            }
            found_count += 1;
        }
    }
    (0, found_count as u64)
}

pub fn handle_query(
    graph: &Graph,
    plan: &[crate::root::query::PreparedStep],
    out_buffer: u64,
    out_len: u64,
    scratch: &mut crate::root::query::QueryScratch,
) -> HandlerResult {
    let max_rows = (out_len as usize) / core::mem::size_of::<abi::query::QueryRow>();
    // Use stack buffer for small queries (<= 32 rows, 1KB) to avoid allocation.
    // This optimization is crucial for high-frequency small queries.
    const STACK_CAP: usize = 32;

    if max_rows <= STACK_CAP {
        let mut stack_buf = [abi::query::QueryRow::default(); STACK_CAP];
        execute_and_copy(graph, plan, &mut stack_buf[..max_rows], out_buffer, scratch)
    } else {
        // Steward Improvement: Reuse scratch buffer for large queries to avoid heap allocations.
        // We take the buffer out to satisfy borrow checker, ensuring `execute_and_copy`
        // can mutate `scratch` independently.
        let mut temp_buf = core::mem::take(&mut scratch.out_buf);

        // Ensure deterministic zero-initialization to avoid leaking stale data
        temp_buf.clear();
        temp_buf.resize(max_rows, abi::query::QueryRow::default());

        let slice = &mut temp_buf[..max_rows];
        let result = execute_and_copy(graph, plan, slice, out_buffer, scratch);

        // Restore the buffer to scratch for reuse
        scratch.out_buf = temp_buf;

        result
    }
}

fn execute_and_copy(
    graph: &Graph,
    plan: &[crate::root::query::PreparedStep],
    buffer: &mut [abi::query::QueryRow],
    out_buffer: u64,
    scratch: &mut crate::root::query::QueryScratch,
) -> HandlerResult {
    let res = crate::root::query::execute(graph, plan, buffer, scratch);

    if let Ok(count) = res {
        unsafe {
            let dst = out_buffer as *mut abi::query::QueryRow;
            for i in 0..count {
                *dst.add(i) = buffer[i];
            }
        }
        (0, count as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_get_edges(graph: &Graph, id: u64, buffer: u64, len: u64) -> HandlerResult {
    use abi::ids::HandleId;

    if let Some(node) = graph.nodes.get(&id) {
        let max_entries = (len as usize) / core::mem::size_of::<abi::types::Edge>();
        let mut count = 0;
        let out_ptr = buffer as *mut abi::types::Edge;

        for (rel, dst) in &node.edges {
            if count < max_entries {
                unsafe {
                    *out_ptr.add(count) = abi::types::Edge {
                        from: abi::types::ThingId::from_u64(id),
                        predicate: abi::types::ThingId::from_u64(*rel as u64),
                        to: abi::types::ThingId::from_u64(*dst),
                        flags: 0,
                    };
                }
            }
            count += 1;
        }
        (0, count as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_get_props(graph: &Graph, id: u64, buffer: u64, len: u64) -> HandlerResult {
    if let Some(node) = graph.nodes.get(&id) {
        let entry_size = core::mem::size_of::<abi::types::GraphProp>();
        if entry_size == 0 {
            return (-1, 0);
        }
        let max_entries = (len as usize) / entry_size;
        let mut count = 0;
        let out_ptr = buffer as *mut abi::types::GraphProp;

        for (key, val) in &node.props {
            if count < max_entries {
                unsafe {
                    *out_ptr.add(count) = abi::types::GraphProp {
                        key: *key,
                        _pad: 0,
                        value: *val,
                    };
                }
            }
            count += 1;
        }
        (0, count as u64)
    } else {
        (-1, 0)
    }
}

/// Bulk property fetch: get multiple properties for a node in a single call
pub fn handle_props_get_many(
    graph: &Graph,
    node_id: u64,
    keys: &[u32],
    out: &mut abi::types::BulkPropsResponse,
) -> HandlerResult {
    out.node_id = node_id;
    out.present_mask = 0;

    if let Some(node) = graph.nodes.get(&node_id) {
        for (i, &key) in keys.iter().enumerate() {
            if i >= abi::types::BULK_PROPS_MAX_KEYS {
                break;
            }
            if let Some(&val) = node.props.get(&key) {
                out.values[i] = val;
                out.present_mask |= 1 << i;
            }
        }
        (0, keys.len() as u64)
    } else {
        (-1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::root::SymbolShell;
    use crate::root::graph::Graph;
    use crate::root::symbols::Interner;
    use abi::ids::HandleId;
    use abi::types::ThingId;

    #[test]
    fn test_handle_find_correctness() {
        let mut graph = Graph::new();
        let mut interner = Interner::new();

        let kind_a = interner.intern("KindA");
        let kind_b = interner.intern("KindB");

        // Alloc nodes
        let a1 = graph.alloc(kind_a);
        let a2 = graph.alloc(kind_a);
        let b1 = graph.alloc(kind_b);
        let a3 = graph.alloc(kind_a);
        let b2 = graph.alloc(kind_b);

        // Prepare buffer
        let max_ids = 10;
        let mut buffer = vec![0u8; max_ids * core::mem::size_of::<ThingId>()];
        let buf_ptr = buffer.as_mut_ptr() as u64;
        let buf_len = buffer.len() as u64;

        // Find KindA
        let (status, count) = handle_find(
            &graph,
            &mut interner,
            SymbolShell::Id(kind_a),
            buf_ptr,
            buf_len,
        );

        assert_eq!(status, 0);
        assert_eq!(count, 3);

        // Verify IDs
        let ids_slice =
            unsafe { core::slice::from_raw_parts(buf_ptr as *const ThingId, count as usize) };

        let mut found_ids = ids_slice.to_vec();
        found_ids.sort_by_key(|t| t.0);

        // Let's just check containment for safety.
        assert!(ids_slice.contains(&ThingId::from_u64(a1)));
        assert!(ids_slice.contains(&ThingId::from_u64(a2)));
        assert!(ids_slice.contains(&ThingId::from_u64(a3)));

        // Find KindB
        let (status, count) = handle_find(
            &graph,
            &mut interner,
            SymbolShell::Id(kind_b),
            buf_ptr,
            buf_len,
        );

        assert_eq!(status, 0);
        assert_eq!(count, 2);

        let ids_slice =
            unsafe { core::slice::from_raw_parts(buf_ptr as *const ThingId, count as usize) };
        assert!(ids_slice.contains(&ThingId::from_u64(b1)));
        assert!(ids_slice.contains(&ThingId::from_u64(b2)));

        // Find non-existent
        let kind_c = interner.intern("KindC");
        let (status, count) = handle_find(
            &graph,
            &mut interner,
            SymbolShell::Id(kind_c),
            buf_ptr,
            buf_len,
        );
        assert_eq!(status, 0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_handle_query_optimization() {
        use crate::root::query::PreparedStep;
        use crate::root::query::QueryScratch;
        use abi::query::QueryRow;

        let mut graph = Graph::new();
        let mut interner = Interner::new();
        let mut scratch = QueryScratch::new();

        let kind = interner.intern("TestKind");
        let id1 = graph.alloc(kind);
        let id2 = graph.alloc(kind);
        let id3 = graph.alloc(kind);

        // Simple plan: Scan(TestKind)
        let plan = vec![PreparedStep {
            op: 1, // Scan
            symbol: kind,
            arg1: 0,
            arg2: 0,
        }];

        // 1. Test Stack Path (Small buffer)
        // 3 items, buffer for 5 items.
        let max_rows_stack = 5;
        let mut buffer_stack = vec![0u8; max_rows_stack * core::mem::size_of::<QueryRow>()];
        let buf_ptr = buffer_stack.as_mut_ptr() as u64;
        let buf_len = buffer_stack.len() as u64;

        let (status, count) = handle_query(&graph, &plan, buf_ptr, buf_len, &mut scratch);
        assert_eq!(status, 0);
        assert_eq!(count, 3);

        // Verify content
        let rows =
            unsafe { core::slice::from_raw_parts(buf_ptr as *const QueryRow, count as usize) };
        let mut ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
        ids.sort();
        let mut expected = vec![id1, id2, id3];
        expected.sort();
        assert_eq!(ids, expected);

        // 2. Test Heap Path (Large buffer)
        // 3 items, buffer for 40 items (> 32).
        let max_rows_heap = 40;
        let mut buffer_heap = vec![0u8; max_rows_heap * core::mem::size_of::<QueryRow>()];
        let buf_ptr_heap = buffer_heap.as_mut_ptr() as u64;
        let buf_len_heap = buffer_heap.len() as u64;

        let (status, count) = handle_query(&graph, &plan, buf_ptr_heap, buf_len_heap, &mut scratch);
        assert_eq!(status, 0);
        assert_eq!(count, 3);

        let rows =
            unsafe { core::slice::from_raw_parts(buf_ptr_heap as *const QueryRow, count as usize) };
        let mut ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
        ids.sort();
        assert_eq!(ids, expected);
    }

    #[test]
    fn test_handle_query_alloc_reuse() {
        use crate::root::query::PreparedStep;
        use crate::root::query::QueryScratch;
        use abi::query::QueryRow;

        let mut graph = Graph::new();
        let mut interner = Interner::new();
        let mut scratch = QueryScratch::new();

        let kind = interner.intern("BulkKind");

        // Create 40 nodes (> 32 STACK_CAP)
        let count_target = 40;
        for _ in 0..count_target {
            graph.alloc(kind);
        }

        let plan = vec![PreparedStep {
            op: 1, // Scan
            symbol: kind,
            arg1: 0,
            arg2: 0,
        }];

        let max_rows = 50;
        let mut buffer = vec![0u8; max_rows * core::mem::size_of::<QueryRow>()];
        let buf_ptr = buffer.as_mut_ptr() as u64;
        let buf_len = buffer.len() as u64;

        // Verify initial state
        assert_eq!(scratch.out_buf.capacity(), 128);
        // We can't check len() because it's implementation detail of how scratch is used,
        // but initially it's 0.

        // Run query
        let (status, count) = handle_query(&graph, &plan, buf_ptr, buf_len, &mut scratch);

        assert_eq!(status, 0);
        assert_eq!(count, count_target as u64);

        // Verify scratch buffer was used and retained
        // The scratch buffer should have grown (or been resized) to at least max_rows (50).
        // Since we initialized with capacity 128, it might not have reallocated, but its length should be affected if we didn't clear it?
        // Wait, handle_query restores the buffer.
        // Inside handle_query:
        // temp_buf.len() < 50 -> resize(50).
        // scratch.out_buf = temp_buf.
        // So scratch.out_buf.len() should be 50.

        assert!(
            scratch.out_buf.len() >= max_rows,
            "Buffer length should be at least max_rows after reuse"
        );
        assert!(
            scratch.out_buf.capacity() >= 128,
            "Capacity should be preserved"
        );
    }
}
