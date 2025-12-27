#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};
use alloc::vec::Vec;
use alloc::collections::{BTreeSet, VecDeque, BTreeMap};
use alloc::format;
use alloc::string::String;
use core::fmt::Write;

use thing_models::abi::wire::graph::{GraphOp, GraphReply};
use thing_models::abi::{ThingId, SymbolId};
use thing_models::builtins::ids::*;
use thing_models::kind::KindBody;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let g = GraphClient::new();
    let c = StdoutConsole;
    
    c.write_str("GRAPH_DUMP: Starting Root-Walk Dumper...\n");

    // Initial small delay
    for _ in 0..100_000 { core::hint::spin_loop(); }

    let mut kind_cache: BTreeMap<ThingId, String> = BTreeMap::new();
    
    // Seed generic fallbacks in case symbol resolution fails or kernel is early
    kind_cache.insert(THING_PROCESS_KIND, "Process".into());
    kind_cache.insert(THING_BOOT_PROGRAM_KIND, "BootProgram".into());
    kind_cache.insert(THING_LAUNCHES_KIND, "LAUNCHES".into());
    kind_cache.insert(THING_TIME_NOW_KIND, "TimeNow".into());
    kind_cache.insert(THING_LINK_KIND, "Link".into());
    kind_cache.insert(THING_KIND_KIND, "Kind".into());
    kind_cache.insert(THING_SCHEMA_KIND, "Schema".into());

    loop {
        perform_dump(&g, &c, &mut kind_cache);
        
        // yield/sleep
        for _ in 0..10_000_000 { core::hint::spin_loop(); }
    }
}

struct NodeInfo {
    id: ThingId,
    kind: ThingId,
}

struct LinkInfo {
    src: ThingId,
    dst: ThingId,
    pred: ThingId,
}

fn resolve_symbol(g: &GraphClient, id: SymbolId, buf: &mut [u8]) -> Option<String> {
    let op = GraphOp::SymbolResolve { id };
    if let Ok(GraphReply::SymbolResolved { text }) = g.call_op(&op, buf) {
        Some(text)
    } else {
        None
    }
}

fn resolve_kind_name(g: &GraphClient, id: ThingId, cache: &mut BTreeMap<ThingId, String>, buf: &mut [u8], _c: &StdoutConsole) -> String {
    if let Some(name) = cache.get(&id) {
        return name.clone();
    }

    // 1. Get the Thing (Type Definition)
    let op_get = GraphOp::GetThing { id };
    if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op_get, buf) {
         // 2. Decode as KindBody
        if let Ok(kind_body) = postcard::from_bytes::<KindBody>(&tb.bytes) {
            // 3. Resolve Symbol
            if let Some(name) = resolve_symbol(g, kind_body.name, buf) {
                cache.insert(id, name.clone());
                return name;
            }
        }
    }

    let fallback = format!("{}", id.0);
    cache.insert(id, fallback.clone());
    fallback
}

fn perform_dump(g: &GraphClient, c: &StdoutConsole, kind_cache: &mut BTreeMap<ThingId, String>) {
    let mut frontier = VecDeque::new();
    let mut visited_nodes = BTreeSet::new();

    let mut nodes_out = Vec::new();
    let mut links_out = Vec::new();

    let root = THING_BOOT_ROOT;
    frontier.push_back(root);
    visited_nodes.insert(root);

    let mut buf_scratch = [0u8; 4096];

    // BFS
    while let Some(curr) = frontier.pop_front() {
        let op_get = GraphOp::GetThing { id: curr };
        if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op_get, &mut buf_scratch) {
             nodes_out.push(NodeInfo { id: curr, kind: ThingId(tb.type_id.0 as u64) });
        }

        let op_scan = GraphOp::ScanLinks { from: Some(curr), to: None, kind: None };
        if let Ok(GraphReply::Links(links)) = g.call_op(&op_scan, &mut buf_scratch) {
            for (src, dst, pred) in links {
                links_out.push(LinkInfo { src, dst, pred });
                if !visited_nodes.contains(&dst) {
                    visited_nodes.insert(dst);
                    frontier.push_back(dst);
                }
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
        let kind_str = resolve_kind_name(g, n.kind, kind_cache, &mut buf_scratch, c);
        let _ = c.write_str(&format!("(t{} :{}) {{}}\n", n.id.0, kind_str));
    }

    for l in links_out {
        let pred_str = resolve_kind_name(g, l.pred, kind_cache, &mut buf_scratch, c);
        let _ = c.write_str(&format!("(t{})-[:{}]->(t{})\n", l.src.0, pred_str, l.dst.0));
    }
    
    let _ = c.write_str("--- END DUMP ---\n");
}
