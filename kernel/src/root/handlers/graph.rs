//! Graph core and property handlers.
//!
//! Mutation handlers (create_node, link, prop_set) now route through
//! the canonical batch pipeline for consistent watch delivery.

use crate::root::graph::Graph;
use crate::root::journal::{Journal, JournalOp};
use crate::root::symbols::Interner;
use crate::root::SymbolShell;
use abi::symbols::SymbolId;
#[allow(unused_imports)]
use core::sync::atomic::Ordering;

use super::batch::{ValidatedOp, apply_ops_and_commit};
use super::HandlerResult;

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
) -> HandlerResult {
    let kid = resolve_shell(kind, interner);
    
    // Build validated op
    let ops = [ValidatedOp::CreateNode { kind: kid, out_idx: 0 }];
    
    // Apply through canonical commit path (watch payload is synthesized there)
    let result = apply_ops_and_commit(graph, &ops);
    
    // Journal entry (kept separate for recovery purposes)
    if result.status == 0 && !result.created_ids.is_empty() {
        let id = result.created_ids[0];
        journal.append(JournalOp::CreateResult {
            id,
            kind: kid as u64,
        });
        (0, id)
    } else {
        (result.status, 0)
    }
}

pub fn handle_prop_get(
    graph: &mut Graph,
    interner: &mut Interner,
    id: u64,
    key: SymbolShell,
) -> HandlerResult {
    let kid = resolve_shell(key, interner);
    if let Some(node) = graph.get_node_mut(id) {
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
    let ops = [ValidatedOp::SetProp { id, key: kid, value }];
    
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

    for (id, node) in &graph.nodes {
        if node.kind == kid {
            if found_count < max_entries {
                unsafe {
                    *out_ptr.add(found_count) = abi::types::ThingId::from_u64(*id);
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
) -> HandlerResult {
    let max_rows = (out_len as usize) / core::mem::size_of::<abi::query::QueryRow>();
    let mut krows = alloc::vec![abi::query::QueryRow::default(); max_rows];

    let res = crate::root::query::execute(graph, plan, &mut krows);

    if let Ok(count) = res {
        unsafe {
            let dst = out_buffer as *mut abi::query::QueryRow;
            for i in 0..count {
                *dst.add(i) = krows[i];
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
