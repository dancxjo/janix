#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};
use alloc::vec::Vec;
use alloc::collections::{BTreeSet, VecDeque};
use alloc::format;
use core::fmt::Write;

// We use raw postcard ops or helper structs?
// Let's use GraphClient's generic call if we can, or just manual ops.
// We need to construct GraphOps manually for ScanLinks and GetThing.

use thing_models::abi::wire::graph::{GraphOp, GraphReply};
use thing_models::abi::ThingId;
use thing_models::builtins::ids::THING_BOOT_ROOT;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let g = GraphClient::new();
    let c = StdoutConsole;
    
    c.write_str("GRAPH_DUMP: Starting Root-Walk Dumper...\n");

    // Initial small delay
    for _ in 0..100_000 { core::hint::spin_loop(); }

    loop {
        perform_dump(&g, &c);
        
        // yield/sleep
        for _ in 0..10_000_000 { core::hint::spin_loop(); }
    }
}

struct NodeInfo {
    id: ThingId,
    kind: ThingId,
    // props? v0: just print ID and Kind
}

struct LinkInfo {
    src: ThingId,
    dst: ThingId,
    pred: ThingId,
}

fn perform_dump(g: &GraphClient, c: &StdoutConsole) {
    let mut frontier = VecDeque::new();
    let mut visited_nodes = BTreeSet::new();
 
    // Link has ScanLinks returns (from, to, pred) tuples, not Link IDs usually?
    // ABI ScanLinks returns `Links(Vec<(ThingId, ThingId, ThingId)>)`.
    // We should track visited (from, to, pred) to avoid dupes if graph has cycles?
    // Actually we just walk nodes.

    let mut nodes_out = Vec::new();
    let mut links_out = Vec::new();

    let root = THING_BOOT_ROOT;
    frontier.push_back(root);
    visited_nodes.insert(root);



    let mut buf_scratch = [0u8; 4096]; // Share buffer?

    // BFS
    while let Some(curr) = frontier.pop_front() {
        // 1. Get Thing Details
        // We need GetThing Op.
        // Assuming GraphClient has helper or we allow raw Op.
        // The trait implementation in thing_std might be limited.
        // Let's use `call_op` if available (custom extension or just added).
        // If not, we construct Op and call via "op" method.
        
        let op_get = GraphOp::GetThing { id: curr };
        match g.call_op(&op_get, &mut buf_scratch) {
            Ok(GraphReply::TypedValue(tb)) => {
                 // c.write_str("Found node\n");
                 nodes_out.push(NodeInfo { id: curr, kind: ThingId(tb.type_id.0 as u64) });
            },
            Err(e) => {
                 let _ = c.write_str(&format!("GetThing({}) failed: {:?}\n", curr.0, e));
            },
            Ok(val) => {
                 let _ = c.write_str(&format!("GetThing({}) unexpected reply: {:?}\n", curr.0, val));
            }
        }

        // 2. Scan Links
        let op_scan = GraphOp::ScanLinks { from: Some(curr), to: None, kind: None };
        match g.call_op(&op_scan, &mut buf_scratch) {
            Ok(GraphReply::Links(links)) => {
                for (src, dst, pred) in links {
                    links_out.push(LinkInfo { src, dst, pred });
                    if !visited_nodes.contains(&dst) {
                        visited_nodes.insert(dst);
                        frontier.push_back(dst);
                    }
                }
            },
            Err(e) => {
                 let _ = c.write_str(&format!("ScanLinks({}) failed: {:?}\n", curr.0, e));
            },
             _ => {
                 let _ = c.write_str(&format!("ScanLinks({}) unexpected reply\n", curr.0));
            }
        }
    }

    // Sort
    nodes_out.sort_by_key(|n| n.id);
    links_out.sort_by(|a, b| {
        a.src.cmp(&b.src)
            .then(a.pred.cmp(&b.pred))
            .then(a.dst.cmp(&b.dst))
    });

    // Output
    let _ = c.write_str("\n--- GQL DUMP ---\n");
    
    for n in nodes_out {
        // Resolve Kind Symbol?
        // We can try to resolve `n.kind`? 
        // type_id is u64. SymbolId is u64.
        // It's likely not the same.
        // This is a "Gap" in the GetThing ABI for v0.2.
        // We'll just print ID.
        let kind_str = format!("type_{}", n.kind.0); // Placeholder
        let _ = c.write_str(&format!("(t{} :{}) {{}}\n", n.id.0, kind_str));
    }

    for l in links_out {
        let _ = c.write_str(&format!("(t{})-[:{}]->(t{})\n", l.src.0, l.pred.0, l.dst.0));
    }
    
    let _ = c.write_str("--- END DUMP ---\n");
}
