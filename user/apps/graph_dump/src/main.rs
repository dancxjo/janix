#![no_std]
#![no_main]

extern crate alloc;

use thing_std as std;
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
use thing_models::core::process::ProcessBody;
use thing_models::builtins::core_kinds::BootProgramBody;
use thing_models::builtins::core_kinds::{ModuleBody, FontBody, BitmapBody, ProgramImageBody};
use thing_models::core::time::TimeNow;
use thing_models::core::input::{KeyboardBody, KeyEventStreamBody, MouseBody, PointerEventStreamBody};

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();
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
    kind_cache.insert(THING_GRAPH_KIND, "Graph".into());
    kind_cache.insert(THING_MODULE_KIND, "Module".into());
    kind_cache.insert(THING_FONT_KIND, "Font".into());
    kind_cache.insert(THING_BITMAP_KIND, "Bitmap".into());
    kind_cache.insert(THING_PROGRAM_IMAGE_KIND, "ProgramImage".into());
    kind_cache.insert(THING_HAS_MODULE_KIND, "HAS_MODULE".into());
    kind_cache.insert(THING_PROVIDES_FONT_KIND, "PROVIDES_FONT".into());
    kind_cache.insert(THING_DEFAULT_FONT_KIND, "DEFAULT_FONT".into());
    kind_cache.insert(THING_BINARY_IMAGE_KIND, "BINARY_IMAGE".into());
    kind_cache.insert(THING_ASSET_KIND, "ASSET".into());
    kind_cache.insert(THING_USES_MODULE_KIND, "USES_MODULE".into());
    kind_cache.insert(THING_HAS_DEVICE_KIND, "HAS_DEVICE".into());
    kind_cache.insert(THING_EMITS_KIND, "EMITS".into());
    kind_cache.insert(THING_SPAWNED_KIND, "SPAWNED".into());
    kind_cache.insert(THING_RUNS_KIND, "RUNS".into());
    
    kind_cache.insert(THING_MOUSE_KIND, "MouseDevice".into());
    kind_cache.insert(THING_POINTER_EVENT_STREAM_KIND, "PointerStream".into());
    kind_cache.insert(THING_PCI_DEVICE_KIND, "PciDevice".into());

    loop {
        perform_dump(&g, &c, &mut kind_cache);
        
        // yield/sleep
        for _ in 0..10_000_000 { core::hint::spin_loop(); }
    }
}

struct NodeInfo {
    id: ThingId,
    kind: ThingId,
    data: Vec<u8>,
}

struct LinkInfo {
    src: ThingId,
    dst: ThingId,
    pred: ThingId,
}

#[derive(serde::Deserialize)]
struct SystemTimeProps {
    unix_seconds: u64,
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
         // 2. Decode as KindBody (Types are Things!)
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
             nodes_out.push(NodeInfo { 
                 id: curr, 
                 kind: ThingId(tb.type_id.0 as u64),
                 data: tb.bytes.to_vec() 
            });
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
        // Print header part
        let _ = c.write_str(&format!("(t{} :{}) ", n.id.0, kind_str));
        
        // Print body part
        match n.kind {
            THING_PROCESS_KIND => {
                 if let Ok(b) = postcard::from_bytes::<ProcessBody>(&n.data) {
                      let name_str = resolve_symbol(g, b.name, &mut buf_scratch).unwrap_or_else(|| format!("{}", b.name.0));
                      let _ = c.write_str(&format!("{{ pid: {}, name: \"{}\", state: {:?} }}\n", b.pid, name_str, b.state));
                 } else {
                      let _ = c.write_str("{ <decode failed> }\n");
                 }
            },
            THING_BOOT_PROGRAM_KIND => {
                 if let Ok(b) = postcard::from_bytes::<BootProgramBody>(&n.data) {
                     let _ = c.write_str(&format!("{{ name: \"{}\", binary: \"{}\", priority: {} }}\n", b.name, b.binary, b.priority));
                 } else {
                      let _ = c.write_str("{ <decode failed> }\n");
                 }
            },
            _ => {
                 // Try new kinds
                 if n.kind == THING_MODULE_KIND {
                     if let Ok(b) = postcard::from_bytes::<ModuleBody>(&n.data) {
                         let _ = c.write_str(&format!("{{ path: \"{}\", kind: \"{}\", role: \"{}\", size: {}, valid: {}, sniff: 0x{:08X} }}\n", b.path, b.kind, b.role, b.size_bytes, b.valid, b.sniff));
                         continue;
                     }
                 }
                 if n.kind == THING_MOUSE_KIND {
                     if let Ok(b) = postcard::from_bytes::<MouseBody>(&n.data) {
                         let _ = c.write_str(&format!("{{ bus: {} }}\n", b.bus.0));
                         continue;
                     }
                 }
                 if n.kind == THING_POINTER_EVENT_STREAM_KIND {
                     if let Ok(b) = postcard::from_bytes::<PointerEventStreamBody>(&n.data) {
                         let _ = c.write_str(&format!("{{ head_seq: {}, capacity: {}, events: {} }}\n", b.head_seq, b.capacity, b.events.len()));
                         continue;
                     }
                 }
                 if n.kind == THING_FONT_KIND {
                     if let Ok(b) = postcard::from_bytes::<FontBody>(&n.data) {
                         let _ = c.write_str(&format!("{{ name: \"{}\", format: \"{}\", glyphs: {}x{} ({}) }}\n", b.name, b.format, b.glyph_width, b.glyph_height, b.glyph_count));
                         continue;
                     }
                 }
                 if n.kind == THING_TIME_NOW_KIND {
                     if let Ok(b) = postcard::from_bytes::<TimeNow>(&n.data) {
                         let _ = c.write_str(&format!("{{ system_ns: {}, monotonic_ns: {} }}\n", b.system_ns, b.monotonic_ns));
                         continue;
                     }
                     if let Ok(b) = postcard::from_bytes::<SystemTimeProps>(&n.data) {
                         let _ = c.write_str(&format!("{{ unix_seconds: {} }}\n", b.unix_seconds));
                         continue;
                     }
                 }
                 if n.kind == THING_BITMAP_KIND {
                     if let Ok(b) = postcard::from_bytes::<BitmapBody>(&n.data) {
                         let _ = c.write_str(&format!("{{ format: \"{}\", {}x{} }}\n", b.format, b.width, b.height));
                         continue;
                     }
                 }
                 if n.kind == THING_PROGRAM_IMAGE_KIND {
                     if let Ok(b) = postcard::from_bytes::<ProgramImageBody>(&n.data) {
                         let _ = c.write_str(&format!("{{ format: \"{}\" }}\n", b.format));
                         continue;
                     }
                 }
                 if n.kind == THING_KEYBOARD_KIND {
                     if let Ok(b) = postcard::from_bytes::<KeyboardBody>(&n.data) {
                         let _ = c.write_str(&format!("{{ bus: {} }}\n", b.bus.0));
                         continue;
                     }
                 }
                 if n.kind == THING_KEY_EVENT_STREAM_KIND {
                     if let Ok(b) = postcard::from_bytes::<KeyEventStreamBody>(&n.data) {
                         let _ = c.write_str(&format!("{{ head_seq: {}, capacity: {}, events: {} }}\n", b.head_seq, b.capacity, b.events.len()));
                         continue;
                     }
                 }
                 if n.kind == THING_PCI_DEVICE_KIND {
                     use thing_models::core::pci::PciDeviceBody;
                     if let Ok(b) = postcard::from_bytes::<PciDeviceBody>(&n.data) {
                          let _ = c.write_str(&format!("{{ loc: {:02x}:{:02x}.{}, id: {:04x}:{:04x}, class: {:02x}.{:02x} }}\n", 
                              b.bus, b.device, b.function, b.vendor_id, b.device_id, b.class_id, b.subclass_id));
                          continue;
                     }
                 }
                 
                 let _ = c.write_str("{}\n");
            }
        }
    }

    for l in links_out {
        let pred_str = resolve_kind_name(g, l.pred, kind_cache, &mut buf_scratch, c);
        let _ = c.write_str(&format!("(t{})-[:{}]->(t{})\n", l.src.0, pred_str, l.dst.0));
    }
    
    let _ = c.write_str("--- END DUMP ---\n");
}
