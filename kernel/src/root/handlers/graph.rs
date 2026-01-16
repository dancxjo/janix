//! Graph core and property handlers.

use crate::root::graph::Graph;
use crate::root::journal::{Journal, JournalOp};
use crate::root::resources::{ResourceHandle, stream};
use crate::root::symbols::Interner;
use crate::root::{RootMsg, SymbolShell};
use abi::symbols::SymbolId;
use core::sync::atomic::Ordering;

use super::HandlerResult;

/// Helper to resolve Shell to SymbolId
pub fn resolve_shell(shell: SymbolShell, interner: &mut Interner) -> SymbolId {
    match shell {
        SymbolShell::Id(id) => id,
        SymbolShell::Str(s) => interner.intern(&s),
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

pub fn handle_create_node(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    kind: SymbolShell,
) -> HandlerResult {
    let kid = resolve_shell(kind, interner);
    let id = graph.alloc(kid);
    journal.append(JournalOp::CreateResult {
        id,
        kind: kid as u64,
    });
    (0, id)
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
            crate::kprintln!("ROOT: PropGet failed: node {} has no property {}", id, kid);
            (-1, 0)
        }
    } else {
        crate::kprintln!("ROOT: PropGet failed: node {} not found", id);
        (-1, 0)
    }
}

pub fn handle_prop_set(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    id: u64,
    key: SymbolShell,
    value: u64,
) -> HandlerResult {
    let kid = resolve_shell(key, interner);
    let watches = if let Some(node) = graph.get_node_mut(id) {
        node.props.insert(kid, value);
        journal.append(JournalOp::UpdateProp {
            id,
            key: kid as u64,
            val: value,
        });
        Some(node.watches.clone())
    } else {
        None
    };

    if let Some(watches_vec) = watches {
        for (_mask, stream_id) in watches_vec {
            if let Some(stream_node) = graph.get_node_mut(stream_id) {
                if let Some(ResourceHandle::Stream(handle)) = &stream_node.resource {
                    let mut lock = handle.lock();
                    if lock.events.len() < lock.capacity {
                        lock.events.push_back(crate::root::resources::stream::WatchEvent {
                            target: id,
                            key: kid as u64,
                            value,
                        });
                    }
                }
            }
        }
        (0, 0)
    } else {
        (-1, 0)
    }
}

pub fn handle_link(
    graph: &mut Graph,
    interner: &mut Interner,
    src: u64,
    rel: SymbolShell,
    dst: u64,
) -> HandlerResult {
    let rid = resolve_shell(rel, interner);
    graph.link(src, rid, dst);
    (0, 0)
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
    let out_ptr = buffer as *mut u64;
    let max_entries = (len as usize) / 8;

    for (id, node) in &graph.nodes {
        if node.kind == kid {
            if found_count < max_entries {
                unsafe {
                    *out_ptr.add(found_count) = *id;
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
