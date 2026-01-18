//! Debug/Describe handlers.

use crate::root::graph::Graph;
use abi::wire::ThingId;
use crate::root::symbols::Interner;
use crate::root::SymbolShell;
use super::HandlerResult;
use super::graph::resolve_shell;
use alloc::format;
use crate::root::debug_fmt;

/// Describe a Thing (Kind + Props) into a buffer.
pub fn handle_describe_thing(
    graph: &Graph,
    interner: &mut Interner,
    id: ThingId,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    if let Some(_kind) = graph.get_kind(id) {
        // Use a temporary buffer to format
        // We use a small string buffer
        let mut s = alloc::string::String::new();
        let _ = debug_fmt::dump_node(graph, interner, id, &mut s);

        let bytes = s.as_bytes();
        let to_copy = core::cmp::min(bytes.len(), len as usize);

        unsafe {
            let _ = crate::memory::copy_to_user(buffer as usize, &bytes[..to_copy]);
        }

        (0, to_copy as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_describe_edge(
    graph: &Graph,
    interner: &mut Interner,
    src: ThingId,
    rel: SymbolShell,
    dst: ThingId,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    let rid = resolve_shell(rel, interner);
    let rname = interner.resolve(rid).unwrap_or("?");

    // Check if edge exists? Or just format?
    // "src --[rel]--> dst"
    let s = format!("{:?} --[{}]--> {:?}", src, rname, dst);
    let bytes = s.as_bytes();
    let to_copy = core::cmp::min(bytes.len(), len as usize);

    unsafe {
        let _ = crate::memory::copy_to_user(buffer as usize, &bytes[..to_copy]);
    }

    (0, to_copy as u64)
}

pub fn handle_dump_edges(
    graph: &Graph,
    interner: &mut Interner,
    id: ThingId,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    if let Some(node) = graph.nodes.get(&id) {
        let mut s = alloc::string::String::new();
        for (r, d) in &node.edges {
            let rname = interner.resolve(*r).unwrap_or("?");
            use core::fmt::Write;
            let _ = writeln!(s, "  --[{}]--> {:?}", rname, d);
        }

        let bytes = s.as_bytes();
        let to_copy = core::cmp::min(bytes.len(), len as usize);

        unsafe {
            let _ = crate::memory::copy_to_user(buffer as usize, &bytes[..to_copy]);
        }
        (0, to_copy as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_dump_graph(
    graph: &Graph,
    interner: &mut Interner,
    limit: u64,
) -> HandlerResult {
    crate::kprintln!("=== GRAPH DUMP (limit={}) ===", limit);

    let mut count = 0;
    for (id, _node) in &graph.nodes {
        if count >= limit { break; }

        let mut s = alloc::string::String::new();
        let _ = debug_fmt::dump_node(graph, interner, *id, &mut s);
        crate::kprint!("{}", s);

        count += 1;
    }

    crate::kprintln!("=== END DUMP ===");
    (0, count)
}
