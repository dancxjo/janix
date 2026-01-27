//! Debug and describe handlers.

use crate::root::graph::Graph;
use crate::root::symbols::Interner;
use crate::root::SymbolShell;
use core::fmt::Write;

use super::graph::resolve_shell;
use super::HandlerResult;

/// Helper for formatting into user-provided buffers.
pub struct FmtBuffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub pos: usize,
}

impl core::fmt::Write for FmtBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let rem = self.len - self.pos;
        let copy_len = core::cmp::min(bytes.len(), rem);
        if copy_len > 0 {
            unsafe {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(self.pos), copy_len);
            }
            self.pos += copy_len;
        }
        Ok(())
    }
}

pub fn handle_describe_thing(
    graph: &Graph,
    interner: &Interner,
    id: u64,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    let mut fmt = FmtBuffer {
        ptr: buffer as *mut u8,
        len: len as usize,
        pos: 0,
    };
    let res = crate::root::debug_fmt::fmt_thing(graph, interner, id, &mut fmt);
    if res.is_ok() {
        (0, fmt.pos as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_describe_symbol(
    interner: &Interner,
    id: u32,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    if let Some(name) = interner.resolve(id) {
        let mut fmt = FmtBuffer {
            ptr: buffer as *mut u8,
            len: len as usize,
            pos: 0,
        };
        let _ = fmt.write_str(name);
        (0, fmt.pos as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_describe_edge(
    graph: &Graph,
    interner: &mut Interner,
    src: u64,
    rel: SymbolShell,
    dst: u64,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    let rid = resolve_shell(rel, interner);
    let mut fmt = FmtBuffer {
        ptr: buffer as *mut u8,
        len: len as usize,
        pos: 0,
    };
    let res = crate::root::debug_fmt::fmt_edge(graph, interner, src, rid, dst, &mut fmt);
    if res.is_ok() {
        (0, fmt.pos as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_dump_edges(
    graph: &Graph,
    interner: &Interner,
    id: u64,
    buffer: u64,
    len: u64,
) -> HandlerResult {
    if let Some(node) = graph.get_kind(id).and_then(|_| graph.nodes.get(&id)) {
        let mut fmt = FmtBuffer {
            ptr: buffer as *mut u8,
            len: len as usize,
            pos: 0,
        };
        let mut count = 0;
        
        // 1. Outgoing edges
        for (rel, dst) in &node.edges {
            if count > 0 {
                let _ = writeln!(fmt);
            }
            let _ = crate::root::debug_fmt::fmt_edge(graph, interner, id, *rel, *dst, &mut fmt);
            count += 1;
            if count >= 8 {
                break;
            }
        }

        // 2. Incoming edges (scan all nodes)
        // OPTIMIZATION: This scan is O(N) and causes massive slowdowns for UI traversal.
        // We now rely on explicit double-linking (HAS_CHILD) for performance.
        /*
        if count < 8 {
            for (src_id, src_node) in &graph.nodes {
                if *src_id == id { continue; } // Already did outgoing
                for (rel, dst_id) in &src_node.edges {
                    if *dst_id == id {
                        if count > 0 {
                            let _ = writeln!(fmt);
                        }
                        let _ = crate::root::debug_fmt::fmt_edge(graph, interner, *src_id, *rel, id, &mut fmt);
                        count += 1;
                        if count >= 8 {
                            break;
                        }
                    }
                }
                if count >= 8 { break; }
            }
        }
        */
        (0, fmt.pos as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_dump_graph(graph: &Graph, interner: &Interner, limit: u64) -> HandlerResult {
    let _txn = crate::logging::LogTransaction::begin("rootdump");

    crate::kinfo!("ROOT DUMP NODES count={}", graph.nodes.len());
    let mut count = 0;
    for (id, _) in &graph.nodes {
        if limit > 0 && count >= limit {
            crate::kinfo!("... truncated ...");
            break;
        }
        let mut buf = [0u8; 256];
        let mut fmt = FmtBuffer {
            ptr: buf.as_mut_ptr(),
            len: buf.len(),
            pos: 0,
        };
        let _ = crate::root::debug_fmt::fmt_thing(graph, interner, *id, &mut fmt);
        if let Ok(s) = core::str::from_utf8(&buf[..fmt.pos]) {
            crate::kprint!("{}\n", s);
        }
        count += 1;
    }

    crate::kinfo!("ROOT DUMP EDGES");
    count = 0;
    'outer: for (src, node) in &graph.nodes {
        for (rel, dst) in &node.edges {
            if limit > 0 && count >= limit {
                crate::kinfo!("... truncated ...");
                break 'outer;
            }
            let mut buf = [0u8; 512];
            let mut fmt = FmtBuffer {
                ptr: buf.as_mut_ptr(),
                len: buf.len(),
                pos: 0,
            };
            let _ = crate::root::debug_fmt::fmt_edge(graph, interner, *src, *rel, *dst, &mut fmt);
            if let Ok(s) = core::str::from_utf8(&buf[..fmt.pos]) {
                crate::kprint!("{}\n", s);
            }
            count += 1;
        }
    }

    (0, 0)
}
