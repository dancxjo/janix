use crate::graph_layout::{LayoutEdge, LayoutNode};
use abi::drawlist::{DrawListBuilder, PointF};
use abi::query::QueryRow;
use abi::schema::{keys, kinds, rels};
use abi::types::HandleId;
use stem::thing::query::RestrictedQuery;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stem::thing::sys::{describe_thing, find};
use stem::thing::ThingId;

#[derive(Debug, Clone, PartialEq)]
pub struct NodeInfo {
    pub id: ThingId,
    pub icon: String,
    pub kind_full: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub fixed: bool,
    pub rank: i32,
    pub gen: u64, // layout generation (0 = never positioned)
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeInfo {
    pub from: ThingId,
    pub to: ThingId,
    pub rel: String,
    pub weight: f32,
}

pub struct GraphLayout {
    pub positions: BTreeMap<ThingId, (f32, f32)>,
}

pub fn scan_system_graph() -> (Vec<NodeInfo>, Vec<EdgeInfo>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut seen = BTreeMap::new();
    let mut queue = Vec::new();

    // Initial seeds - focused on core system entities only
    let interesting_kinds = [
        kinds::SVC_ROOT,     // System root node
        kinds::PROC_KERNEL,  // Kernel process
        kinds::UI_CROWN,     // UI crown (desktop)
        kinds::UI_WINDOW,    // Windows
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
        let (mut name, kind_full, icon) = if let Ok(len) = describe_thing(id, &mut buf) {
            let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
            if desc.contains(":mem.Range")
                || desc.contains(":Bytespace")
                || desc.contains(":log.Entry")
            {
                seen.insert(id, ());
                continue;
            }
            extract_info(desc, id)
        } else {
            (
                format!("unknown_{:X}", id.to_u64_lossy()),
                String::from("unknown"),
                String::from("unknown"),
            )
        };

        // Try to fetch explicit "name" property (Interned string)
        if let Ok(val) = stem::thing::sys::prop_get(id, keys::NAME) {
            if val != 0 {
                // Try to resolve as symbol first
                let mut sym_buf = [0u8; 128];
                if let Ok(len) = stem::thing::sys::describe_symbol(val as u32, &mut sym_buf) {
                     if len > 0 {
                         name = String::from(core::str::from_utf8(&sym_buf[..len]).unwrap_or("?"));
                     }
                }
            }
        }

        // Read layout-specific positions (persisted by photosynthesis itself)
        // These are separate from UI_X/UI_Y which are used by the window manager
        let x = stem::thing::sys::prop_get(id, keys::LAYOUT_POS_X).unwrap_or(0) as f32;
        let y = stem::thing::sys::prop_get(id, keys::LAYOUT_POS_Y).unwrap_or(0) as f32;
        let rank = stem::thing::sys::prop_get(id, keys::UI_RANK).unwrap_or(u64::MAX) as i32;
        let fixed = stem::thing::sys::prop_get(id, keys::UI_FIXED).unwrap_or(0) != 0;
        let gen = stem::thing::sys::prop_get(id, keys::LAYOUT_GEN).unwrap_or(0);

        nodes.push(NodeInfo {
            id,
            icon,
            kind_full,
            name,
            x,
            y,
            fixed,
            rank,
            gen,
        });
        seen.insert(id, ());

        // Scan edges and discover new nodes
        let mut q_buf = [QueryRow::default(); 128];
        let mut q = RestrictedQuery::new(&mut q_buf);

        if let Ok(count) = q.get_edges(id, None, 128) {
            if count > 0 {
                // stem::info!("[photo] Node {:X} has {} edges", id.to_u64_lossy(), count);
            }
             for i in 0..count {
                 let row = &q.buf[i];
                 let predicate_id = row.kind_rel as u32;
                 let target_id = ThingId::from_u64(row.val_dst);
                 
                 stem::trace!("[photo]   Edge: {:X} --[pred={}]--> {:X}", 
                     id.to_u64_lossy(), predicate_id, target_id.to_u64_lossy());
                 
                let weight =
                    stem::thing::sys::prop_get(id, keys::EDGE_WEIGHT).unwrap_or(100) as f32 / 100.0;
                edges.push(EdgeInfo {
                    from: id,
                    to: target_id,
                    rel: get_predicate_name(predicate_id),
                    weight,
                });
                if !seen.contains_key(&target_id) && queue.len() < 512 {
                    queue.push(target_id);
                }
             }
        }
    }

    stem::info!("[photo] Scan complete: {} nodes, {} raw edges", nodes.len(), edges.len());

    // Filter edges to only include those where both endpoints are in the nodes list
    let node_ids: BTreeMap<ThingId, ()> = nodes.iter().map(|n| (n.id, ())).collect();
    let filtered_edges: Vec<EdgeInfo> = edges
        .into_iter()
        .filter(|e| node_ids.contains_key(&e.from) && node_ids.contains_key(&e.to))
        .collect();

    (nodes, filtered_edges)
}

fn extract_info(desc: &str, id: ThingId) -> (String, String, String) {
    // Description is like "(var_ID:Kind { ... })"
    if let Some(start) = desc.find('(') {
        if let Some(end) = desc.find(" {") {
            let identity = &desc[start + 1..end];
            // identity is like "kernel_9F:proc.Kernel"
            if let Some(colon) = identity.find(':') {
                let first = &identity[..colon];
                let kind_full = &identity[colon + 1..];
                let icon = map_kind_to_icon(kind_full);
                return (String::from(first), kind_full.to_string(), icon);
            }
            let icon = map_kind_to_icon(identity);
            return (String::from(identity), identity.to_string(), icon);
        }
    }
    (
        format!("unknown_{:X}", id.to_u64_lossy()),
        String::from("unknown"),
        String::from("unknown"),
    )
}

fn map_kind_to_icon(kind: &str) -> String {
    let lower = kind.to_lowercase();
    match lower.as_str() {
        "bytespace" => "kind.bytespace".into(),
        "ui.window" => "ui.widget".into(),
        "mem.range" => "mem.page".into(),
        "fw.table.acpi" => "dev.host".into(),
        "svc.root" => "ui.root".into(),
        "boot.module" => "bran.bran".into(),
        _ => lower,
    }
}

pub fn generate_layout(nodes: &[NodeInfo]) -> GraphLayout {
    let mut positions = BTreeMap::new();
    for node in nodes {
        positions.insert(node.id, (node.x, node.y));
    }
    GraphLayout { positions }
}

pub fn render_graph(nodes: &[NodeInfo], edges: &[EdgeInfo], layout: &GraphLayout) -> Vec<u8> {
    let mut builder = DrawListBuilder::new();

    // 1. Draw Pipes (Edges)
    for edge in edges {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) = (
            layout.positions.get(&edge.from),
            layout.positions.get(&edge.to),
        ) {
            // Offset from/to to node boundaries
            let (dx, dy) = (x2 - x1, y2 - y1);
            let dist = libm::sqrtf(dx * dx + dy * dy);
            if dist > 0.0 {
                let (ux, uy) = (dx / dist, dy / dist);
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
            if let Ok(icon_sid) = stem::thing::sys::intern(&node.icon) {
                builder.push_draw_icon(
                    (x - w / 2.0) as i32,
                    (y - h / 2.0) as i32,
                    24,
                    24,
                    icon_sid,
                );
            } else {
                builder.push_fill_rect(
                    (x - w / 2.0) as i32,
                    (y - h / 2.0) as i32,
                    w as i32,
                    h as i32,
                    0xFF44AAFF,
                );
            }

            // Draw label centered in rectangle
            // Approx 6px per char
            let tw = node.name.len() as f32 * 6.0;
            builder.push_text_span(&node.name, x - tw / 2.0 + 12.0, y + 4.0, 9.0, 0xFF000000);
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

    builder.push_line(
        to,
        PointF::new(
            to.x + head_len * libm::cosf(a1),
            to.y + head_len * libm::sinf(a1),
        ),
        color,
        width,
    );
    builder.push_line(
        to,
        PointF::new(
            to.x + head_len * libm::cosf(a2),
            to.y + head_len * libm::sinf(a2),
        ),
        color,
        width,
    );
}

fn get_predicate_name(id: u32) -> String {
    match id {
        0x10 | 16 => String::from("CHILD_OF"),
        0x11 | 17 => String::from("HAS_CHILD"),
        _ => {
            let common = [
                rels::HAS_BUS,
                rels::HAS_DEVICE,
                rels::BACKED_BY,
                rels::RUNS_ON,
                rels::PROVIDES,
                rels::HAS_CPU,
                rels::HAS_MEMORY_RANGE,
                rels::HAS_MODULE,
                rels::USES,
                rels::IMPLEMENTS,
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
