//! Graph core and property handlers.
//!
//! Mutation handlers (create_node, link, prop_set) now route through
//! the canonical batch pipeline for consistent watch delivery.

use crate::root::graph::Graph;
use crate::root::journal::{Journal, JournalOp};
use crate::root::resources::ResourceHandle;
use crate::root::symbols::Interner;
use crate::root::SymbolShell;
use abi::symbols::SymbolId;
use abi::wire::ThingId;
#[allow(unused_imports)]
use core::sync::atomic::Ordering;

use super::batch::{ValidatedOp, apply_ops_and_commit};
use super::encode;
use super::HandlerResult;

/// Helper to resolve Shell to SymbolId
pub fn resolve_shell(shell: SymbolShell, interner: &mut Interner) -> SymbolId {
    match shell {
        SymbolShell::Id(id) => id,
        SymbolShell::Str(s) => interner.intern(&s),
        SymbolShell::Static(s) => interner.intern(s),
    }
}

pub fn handle_intern(interner: &mut Interner, name: &str, out_ptr: u64) -> HandlerResult {
    let id = interner.intern(name);
    if out_ptr != 0 {
        let _ = unsafe { crate::memory::copy_to_user(out_ptr as usize, &id.0) };
    }
    (0, 0)
}

pub fn handle_get_kind(graph: &Graph, id: ThingId, out_ptr: u64) -> HandlerResult {
    if let Some(k) = graph.get_kind(id) {
        if out_ptr != 0 {
            let _ = unsafe { crate::memory::copy_to_user(out_ptr as usize, &k.0) };
        }
        (0, 0)
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
    out_ptr: u64,
) -> HandlerResult {
    let kid = resolve_shell(kind, interner);
    
    // Build validated op
    let ops = [ValidatedOp::CreateNode { kind: kid, out_idx: 0 }];
    
    // Encode as batch for watch consumers
    let kind_bytes = encode::symbol_to_bytes(kid);
    let commit_bytes = encode::encode_create_node(&kind_bytes, 0);
    
    // Apply through canonical commit path
    let result = apply_ops_and_commit(graph, &ops, &commit_bytes);
    
    // Journal entry (kept separate for recovery purposes)
    if result.status == 0 && !result.created_ids.is_empty() {
        let id = result.created_ids[0];
        journal.append(JournalOp::CreateResult {
            id,
            kind: kid,
        });

        if out_ptr != 0 {
            let _ = unsafe { crate::memory::copy_to_user(out_ptr as usize, &id.0) };
        }
        (0, 0)
    } else {
        (result.status, 0)
    }
}

pub fn handle_prop_get(
    graph: &mut Graph,
    interner: &mut Interner,
    id: ThingId,
    key: SymbolShell,
    out_ptr: u64,
) -> HandlerResult {
    let kid = resolve_shell(key, interner);
    if let Some(node) = graph.get_node_mut(id) {
        if let Some(val) = node.props.get(&kid) {
            if out_ptr != 0 {
                let _ = unsafe { crate::memory::copy_to_user(out_ptr as usize, val) };
            }
            (0, 0)
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
    id: ThingId,
    key: SymbolShell,
    value: [u8; 16],
) -> HandlerResult {
    let kid = resolve_shell(key, interner);
    
    // Check if node exists
    if graph.get_node_mut(id).is_none() {
        return (-1, 0);
    }
    
    // Build validated op
    let ops = [ValidatedOp::SetProp { id, key: kid, value }];
    
    // Encode as batch for watch consumers
    let key_bytes = encode::symbol_to_bytes(kid);
    let commit_bytes = encode::encode_set_prop(id, &key_bytes, &value);
    
    // Apply through canonical commit path
    let result = apply_ops_and_commit(graph, &ops, &commit_bytes);
    
    if result.status != 0 {
        return (result.status, 0);
    }
    
    // Journal entry
    journal.append(JournalOp::UpdateProp {
        id,
        key: kid,
        val: value,
    });
    
    // Also notify node-level stream watches (legacy mechanism)
    let watches = graph
        .get_node_mut(id)
        .map(|n| n.watches.clone())
        .unwrap_or_default();
    
    for (_mask, stream_id) in watches {
        if let Some(stream_node) = graph.get_node_mut(stream_id) {
            if let Some(ResourceHandle::Stream(handle)) = &stream_node.resource {
                let mut lock = handle.lock();
                if lock.events.len() < lock.capacity {
                    lock.events.push_back(crate::root::resources::stream::WatchEvent {
                        target: id,
                        key: kid,
                        value, // [u8; 16]
                    });
                }
            }
        }
    }
    
    (0, 0)
}

/// Create an edge between two nodes.
///
/// Routes through the canonical batch pipeline for consistent watch delivery.
pub fn handle_link(
    graph: &mut Graph,
    interner: &mut Interner,
    src: ThingId,
    rel: SymbolShell,
    dst: ThingId,
) -> HandlerResult {
    let rid = resolve_shell(rel, interner);
    
    // Build validated op
    let ops = [ValidatedOp::PutEdge { src, rel: rid, dst }];
    
    // Encode as batch for watch consumers
    let rel_bytes = encode::symbol_to_bytes(rid);
    let commit_bytes = encode::encode_put_edge(src, &rel_bytes, dst);
    
    // Apply through canonical commit path
    let result = apply_ops_and_commit(graph, &ops, &commit_bytes);
    
    (result.status, 0)
}

pub fn handle_find(
    graph: &Graph,
    interner: &mut Interner,
    kind: SymbolShell,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    let kid = resolve_shell(kind, interner);
    let mut found_count = 0;
    let max_entries = (len as usize) / core::mem::size_of::<abi::wire::ThingId>();

    for (id, node) in &graph.nodes {
        if node.kind == kid {
            if found_count < max_entries {
                let offset = found_count * core::mem::size_of::<abi::wire::ThingId>();
                unsafe {
                    let _ = crate::memory::copy_to_user(buffer as usize + offset, &id.0);
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

pub fn handle_get_edges(graph: &Graph, id: ThingId, buffer: u64, len: u64) -> HandlerResult {
    if let Some(node) = graph.nodes.get(&id) {
        let max_entries = (len as usize) / core::mem::size_of::<abi::types::Edge>();
        let mut count = 0;

        for (rel, dst) in &node.edges {
            if count < max_entries {
                let edge = abi::types::Edge {
                    from: id,
                    predicate: *rel,
                    to: *dst,
                    flags: 0,
                };
                let offset = count * core::mem::size_of::<abi::types::Edge>();
                unsafe {
                    let slice = core::slice::from_raw_parts(&edge as *const _ as *const u8, core::mem::size_of::<abi::types::Edge>());
                    let _ = crate::memory::copy_to_user(buffer as usize + offset, slice);
                }
            }
            count += 1;
        }
        (0, count as u64)
    } else {
        (-1, 0)
    }
}
