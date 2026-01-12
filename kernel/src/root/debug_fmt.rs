//! Structured debug formatting for graph entities
//!
//! Formats nodes and edges in stable key=value format suitable for
//! parsing, diffing, and log analysis.

use super::graph::{Graph, ThingId};
use super::symbols::Interner;
use abi::symbols::SymbolId;
use core::fmt::{self, Write};

pub struct FmtBuffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub pos: usize,
}

impl Write for FmtBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
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

/// Format a graph node in structured key=value format
/// 
/// Output: `node id=0x{ID} kind={kind} {prop}={value}...`
pub fn fmt_thing(
    graph: &Graph,
    interner: &Interner,
    id: ThingId,
    w: &mut dyn Write,
) -> fmt::Result {
    if let Some(node) = graph.nodes.get(&id) {
        let kind_str = interner.resolve(node.kind).unwrap_or("?");

        // Structured format: node id=X kind=Y props...
        write!(w, "node id=0x{:X} kind={}", id, kind_str)?;

        for (k, v) in node.props.iter() {
            let kname = interner.resolve(*k).unwrap_or("p");

            // Heuristic: if property name implies interned string, try to resolve
            let is_string_prop = kname == "name"
                || kname == "arch"
                || kname == "platform_profile"
                || kname == "compatible"
                || kname == "driver.name"
                || kname == "status";

            if is_string_prop {
                if let Ok(sym_id) = (*v).try_into() {
                    if let Some(s) = interner.resolve(sym_id) {
                        write!(w, " {}=\"{}\"", kname, s)?;
                        continue;
                    }
                }
            }

            // Numeric value - use hex for large values, decimal for small
            if *v > 0xFFFF {
                write!(w, " {}=0x{:x}", kname, v)?;
            } else {
                write!(w, " {}={}", kname, v)?;
            }
        }
        Ok(())
    } else {
        write!(w, "node id=0x{:X} kind=?", id)
    }
}

/// Format a graph edge in structured key=value format
///
/// Output: `edge from=0x{SRC} rel={REL} to=0x{DST}`
pub fn fmt_edge(
    graph: &Graph,
    interner: &Interner,
    src: ThingId,
    rel: SymbolId,
    dst: ThingId,
    w: &mut dyn Write,
) -> fmt::Result {
    let src_kind = if let Some(n) = graph.nodes.get(&src) {
        interner.resolve(n.kind).unwrap_or("?")
    } else {
        "?"
    };

    let dst_kind = if let Some(n) = graph.nodes.get(&dst) {
        interner.resolve(n.kind).unwrap_or("?")
    } else {
        "?"
    };

    let rname = interner.resolve(rel).unwrap_or("REL?");

    // Structured format: edge from=X rel=Y to=Z
    write!(
        w,
        "edge from=0x{:X} from_kind={} rel={} to=0x{:X} to_kind={}",
        src, src_kind, rname, dst, dst_kind
    )
}

// --- Legacy Cypher-style format (for backward compat if needed) ---

/// Format a node in Cypher-like syntax: (varN:kind { props })
#[allow(dead_code)]
pub fn fmt_thing_cypher(
    graph: &Graph,
    interner: &Interner,
    id: ThingId,
    w: &mut dyn Write,
) -> fmt::Result {
    if let Some(node) = graph.nodes.get(&id) {
        let kind_str = interner.resolve(node.kind).unwrap_or("?");
        let basename = kind_str.rsplit('.').next().unwrap_or(kind_str);
        let var_name = basename.to_lowercase();

        write!(w, "({}{:X}:{} {{ ", var_name, id, kind_str)?;

        let mut count = 0;
        for (k, v) in node.props.iter() {
            if count > 0 {
                write!(w, ", ")?;
            }
            if count >= 8 {
                write!(w, "...")?;
                break;
            }

            let kname = interner.resolve(*k).unwrap_or("p");

            let is_string_prop = kname == "name"
                || kname == "arch"
                || kname == "platform_profile"
                || kname == "compatible"
                || kname == "driver.name"
                || kname == "status";

            if is_string_prop {
                if let Ok(sym_id) = (*v).try_into() {
                    if let Some(s) = interner.resolve(sym_id) {
                        write!(w, "{}: \"{}\"", kname, s)?;
                        count += 1;
                        continue;
                    }
                }
            }

            if *v > 0x10000 {
                write!(w, "{}: 0x{:x}", kname, v)?;
            } else {
                write!(w, "{}: {}", kname, v)?;
            }
            count += 1;
        }
        write!(w, " }})")
    } else {
        write!(w, "(t{:X}:?)", id)
    }
}
