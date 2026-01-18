//! Debug dump/format helpers

use crate::root::graph::Graph;
use crate::root::symbols::Interner;
use abi::wire::ThingId;
use core::fmt::{self, Write};

pub fn format_prop_value(v: &[u8; 16]) -> alloc::string::String {
    // Check if it looks like a u64 (last 8 bytes zero)
    let is_u64 = v[8..].iter().all(|&b| b == 0);
    if is_u64 {
        let mut b = [0u8; 8];
        b.copy_from_slice(&v[0..8]);
        let val = u64::from_le_bytes(b);
        if val < 1000 {
            alloc::format!("{}", val)
        } else {
            alloc::format!("{:#x}", val)
        }
    } else {
        // Assume UUID/String
        alloc::format!("{:02x}{:02x}{:02x}{:02x}-...", v[0], v[1], v[2], v[3])
    }
}

pub fn dump_node(graph: &Graph, interner: &Interner, id: ThingId, w: &mut dyn Write) -> fmt::Result {
    if let Some(node) = graph.nodes.get(&id) {
        if let Some(kname) = interner.resolve(node.kind) {
            writeln!(w, "Node {:?}: Kind = {}", id, kname)?;
        } else {
            writeln!(w, "Node {:?}: Kind = {:?}", id, node.kind)?;
        }

        for (k, v) in &node.props {
            let kname = interner.resolve(*k).unwrap_or("?");
            writeln!(w, "  .{} = {}", kname, format_prop_value(v))?;
        }
    }
    Ok(())
}
