use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use stem::thing::{ThingId};
use stem::thing::sys::{find, describe_thing};
use abi::schema::{kinds, rels};
use alloc::collections::BTreeMap;
use abi::drawlist::{DrawListBuilder, PointF};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeInfo {
    pub id: ThingId,
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeInfo {
    pub from: ThingId,
    pub to: ThingId,
    pub rel: String,
}

pub struct GraphLayout {
    pub positions: BTreeMap<ThingId, (f32, f32)>,
}

pub fn scan_system_graph() -> (Vec<NodeInfo>, Vec<EdgeInfo>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut seen = BTreeMap::new();
    let mut queue = Vec::new();

    // Initial seeds
    let interesting_kinds = [
        kinds::UI_ROOT,
        kinds::UI_WINDOW,
        kinds::PROC_KERNEL,
    ];

    for &kind_name in &interesting_kinds {
        let mut ids = [ThingId::default(); 32];
        if let Ok(count) = find(kind_name, &mut ids) {
            for i in 0..count {
                queue.push(ids[i]);
            }
        }
    }

    let mut nodes_processed = 0;
    while nodes_processed < queue.len() && nodes_processed < 256 {
        let id = queue[nodes_processed];
        nodes_processed += 1;

        if seen.contains_key(&id) {
            continue;
        }

        let mut buf = [0u8; 128];
        let (name, kind) = if let Ok(len) = describe_thing(id, &mut buf) {
            let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
            if desc.contains(":mem.Range") || desc.contains(":Bytespace") || desc.contains(":log.Entry") {
                seen.insert(id, ());
                continue;
            }
            extract_info(desc, id)
        } else {
            (format!("unknown_{:X}", id.to_u64_lossy()), String::from("unknown"))
        };

        nodes.push(NodeInfo { id, kind, name });
        seen.insert(id, ());

        // Scan edges and discover new nodes
        let mut buf = [abi::types::Edge::default(); 32];
        if let Ok(count) = stem::thing::sys::get_edges(id, &mut buf) {
            let count = core::cmp::min(count, buf.len());
            for i in 0..count {
                let edge = &buf[i];
                let predicate_id = edge.predicate.to_u64_lossy() as u32;
                edges.push(EdgeInfo {
                    from: id,
                    to: edge.to,
                    rel: get_predicate_name(predicate_id),
                });
                if !seen.contains_key(&edge.to) && queue.len() < 512 {
                    queue.push(edge.to);
                }
            }
        }
    }

    (nodes, edges)
}

fn extract_info(desc: &str, id: ThingId) -> (String, String) {
    // Description is like "(var_ID:Kind { ... })"
    if let Some(start) = desc.find('(') {
        if let Some(end) = desc.find(" {") {
            let identity = &desc[start + 1..end];
            // identity is like "kernel_9F:proc.Kernel"
            if let Some(colon) = identity.find(':') {
                let first = &identity[..colon];
                let kind_full = &identity[colon + 1..];
                return (format!("{}:{}", first, kind_full.rsplit('.').next().unwrap_or(kind_full)), String::from(kind_full));
            }
            return (String::from(identity), String::from("unknown"));
        }
    }
    (format!("unknown_{:X}", id.to_u64_lossy()), String::from("unknown"))
}

pub fn generate_layout(nodes: &[NodeInfo]) -> GraphLayout {
    let mut positions = BTreeMap::new();
    let cols = 4;
    let padding_x = 150.0;
    let padding_y = 100.0;
    let start_x = 60.0; // Slightly more start padding
    let start_y = 60.0;

    for (i, node) in nodes.iter().enumerate() {
        let col = (i % cols) as f32;
        let row = (i / cols) as f32;
        let x = start_x + col * padding_x;
        let y = start_y + row * padding_y;
        positions.insert(node.id, (x, y));
    }

    GraphLayout { positions }
}

pub fn render_graph(nodes: &[NodeInfo], edges: &[EdgeInfo], layout: &GraphLayout) -> Vec<u8> {
    let mut builder = DrawListBuilder::new();

    // 1. Draw Pipes (Edges)
    for edge in edges {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) = (layout.positions.get(&edge.from), layout.positions.get(&edge.to)) {
            // Offset from/to to node boundaries
            let (dx, dy) = (x2 - x1, y2 - y1);
            let dist = libm::sqrtf(dx*dx + dy*dy);
            if dist > 0.0 {
                let (ux, uy) = (dx/dist, dy/dist);
                let start = PointF::new(x1 + ux * 55.0, y1 + uy * 15.0);
                let end = PointF::new(x2 - ux * 55.0, y2 - uy * 15.0);
                draw_arrow(&mut builder, start, end, 0xFF888888, 2.0);

                // Draw relation label in middle of pipe
                let mid_x = (x1 + x2) / 2.0;
                let mid_y = (y1 + y2) / 2.0;
                builder.push_text_span(&edge.rel, mid_x, mid_y, 7.0, 0xFF666666);
            }
        }
    }

    // 2. Draw Nodes
    for node in nodes {
        if let Some(&(x, y)) = layout.positions.get(&node.id) {
            // Draw a rectangle for the node - wider to fit labels
            let w = 110.0;
            let h = 30.0;
            
            // Draw Icon instead of blue rectangle
            if let Ok(icon_sid) = stem::thing::sys::intern(&node.kind) {
                builder.push_draw_icon((x - w/2.0) as i32, (y - h/2.0) as i32, 24, 24, icon_sid);
            } else {
                builder.push_fill_rect((x - w/2.0) as i32, (y - h/2.0) as i32, w as i32, h as i32, 0xFF44AAFF);
            }
            
            // Draw label centered in rectangle
            // Approx 6px per char
            let tw = node.name.len() as f32 * 6.0;
            builder.push_text_span(&node.name, x - tw/2.0 + 12.0, y + 4.0, 9.0, 0xFF000000);
        }
    }

    builder.finish()
}

fn draw_arrow(builder: &mut DrawListBuilder, from: PointF, to: PointF, color: u32, width: f32) {
    builder.push_line(from, to, color, width);
    
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let angle = libm::atan2f(dy, dx);
    let head_len = 8.0;
    
    let a1 = angle + 3.14159 * 0.85;
    let a2 = angle - 3.14159 * 0.85;
    
    builder.push_line(to, PointF::new(to.x + head_len * libm::cosf(a1), to.y + head_len * libm::sinf(a1)), color, width);
    builder.push_line(to, PointF::new(to.x + head_len * libm::cosf(a2), to.y + head_len * libm::sinf(a2)), color, width);
}

fn get_predicate_name(id: u32) -> String {
    match id {
        0x10 | 16 => String::from("CHILD_OF"),
        0x11 | 17 => String::from("HAS_CHILD"),
        _ => {
            let common = [
                rels::HAS_BUS, rels::HAS_DEVICE, rels::BACKED_BY, rels::RUNS_ON,
                rels::PROVIDES, rels::HAS_CPU, rels::HAS_MEMORY_RANGE, rels::HAS_MODULE,
                rels::USES, rels::IMPLEMENTS,
            ];
            for &r in &common {
                if let Ok(sid) = stem::thing::sys::intern(r) {
                    if sid == id {
                        return String::from(r);
                    }
                }
            }
            format!("rel_{}", id)
        }
    }
}
