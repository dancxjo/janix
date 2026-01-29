#![no_std]
#![no_main]

extern crate alloc;
use abi::ids::HandleId;
use abi::root::RootWatchFilter;
use abi::schema::{keys, kinds, rels};
use abi::types::{WatchMode, WatchSpec};
use abi::watch;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use blossom::widgets::ThingosIcon;
use core::time::Duration;
use stem::info;
use stem::petals::{
    Canvas, Color, FontKey, Line, PanZoomController, Rect, Scene, Size, Styled, Text, Viewport,
    ViewportConstraints, Window,
};
use stem::thing::sys::{create_node, describe_thing, find, link, prop_get, prop_set};
use stem::thing::ThingId;

fn find_svg_assets() -> Vec<(String, ThingId)> {
    let mut assets = Vec::new();
    let mut modules = [ThingId::default(); 128];
    let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);

    for i in 0..count {
        let mut buf = [0u8; 512];
        let len = match describe_thing(modules[i], &mut buf) {
            Ok(l) => l,
            Err(_) => continue,
        };
        let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
        // Format is often: 'Boot Module: "name" (size=...)' or just properties?
        // Actually describe_thing usually returns the debug string.
        // Let's rely on checking the "name" property if possible?
        // But bloom checks description string.
        // "BootModule(id) name: \"foo.svg\" ..."

        let mod_name = if let Some(pos) = desc.find("name: \"") {
            let rest = &desc[pos + 7..];
            if let Some(end) = rest.find('"') {
                &rest[..end]
            } else {
                continue;
            }
        } else {
            continue;
        };

        if mod_name.ends_with(".svg") {
            let bs_id = match prop_get(modules[i], "bytespace") {
                Ok(id) => ThingId::from_u64(id),
                Err(_) => continue,
            };
            assets.push((String::from(mod_name), bs_id));
        }
    }
    assets
}

fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    if value.is_empty() {
        prop_set(id, key_name, 0).ok();
        return;
    }
    use stem::thing::sys::{bytespace_create, bytespace_write};
    let bs_id = bytespace_create(value.len(), 0, 0).expect("create bytespace");
    bytespace_write(bs_id, 0, value.as_bytes()).ok();
    // For string props, we set the bytespace ID
    prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

mod graph_layout;
mod input;
mod pipes;

use alloc::collections::BTreeMap;
use graph_layout::{compute_layout, route_edges, LayoutEdge, LayoutNode, LayoutSettings};
use pipes::{generate_layout, scan_system_graph};

const TILE_WIDTH: i32 = 120;
const TILE_HEIGHT: i32 = 160;
const TILE_BORDER: i32 = 2;
const TILE_RADIUS: i32 = 18;
const ICON_SIZE: i32 = 64;
const ICON_TOP_PADDING: i32 = 14;
const TYPE_FONT_SIZE: i32 = 18;
const ID_FONT_SIZE: i32 = 12;
const TYPE_LINE_HEIGHT: i32 = 24;
const ID_LINE_HEIGHT: i32 = 18;
const TYPE_CHAR_WIDTH: i32 = 10;
const ID_CHAR_WIDTH: i32 = 8;
const TILE_BORDER_COLOR: Color = Color::from_argb_u32(0xFFE0E0E8);
const TILE_FILL_COLOR: Color = Color::from_argb_u32(0x88FFFFFF);
const TYPE_TEXT_COLOR: Color = Color::from_argb_u32(0xFF383838);
const ID_TEXT_COLOR: Color = Color::from_argb_u32(0xFF8A8A8C);

#[stem::main]
fn main() -> ! {
    info!("Photosynthesis starting...");

    // 1. Wait for Bloom (UI Root)
    let mut ui_crown = ThingId::default();
    while ui_crown.to_u64_lossy() == 0 {
        let mut roots = [ThingId::default(); 1];
        if let Ok(1) = find(kinds::UI_CROWN, &mut roots) {
            ui_crown = roots[0];
        } else {
            stem::sleep(Duration::from_millis(100));
        }
    }
    info!("Found UI Root: {}", ui_crown.to_u64_lossy());

    // 2. Create Window
    let win = create_node(kinds::UI_WINDOW).expect("win");
    link(win, rels::CHILD_OF, ui_crown).expect("link");
    link(ui_crown, rels::HAS_CHILD, win).expect("has_child");

    prop_set(win, keys::UI_BG_COLOR, 0xFFF5F5F0).ok(); // Off-white
    prop_set(win, keys::UI_WIDTH, 800).ok();
    prop_set(win, keys::UI_HEIGHT, 600).ok();
    prop_set(win, keys::UI_X, 200).ok();
    prop_set(win, keys::UI_Y, 100).ok();
    set_string_prop(win, keys::UI_TITLE, "Photosynthesis (System Graph)");

    // Initialize viewport controller for pan/zoom
    let mut viewport_controller = PanZoomController::new(
        Viewport::new(800.0, 600.0),
        ViewportConstraints {
            min_zoom: 0.1,
            max_zoom: 5.0,
            bounds: None, // Infinite canvas for now
        },
    );

    let mut last_nodes = Vec::new();
    let mut last_edges = Vec::new();
    let mut last_routes = BTreeMap::new();
    let mut last_scan = stem::monotonic_ns();
    let mut dirty = true;
    let mut graph_watch = None;

    // Input state for viewport control
    let mut input_state = input::InputState::new();

    let filter = RootWatchFilter::all();
    let spec = WatchSpec {
        mode: WatchMode::StreamOnly as u32,
        filter_ptr: &filter as *const _ as u64,
        filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
        ..Default::default()
    };
    graph_watch = stem::syscall::root_watch_open(&spec).ok();

    loop {
        // Poll input from system graph and apply to viewport
        if input::poll_and_apply(&mut viewport_controller, &mut input_state) {
            dirty = true;
        }

        if let Some(watch_id) = graph_watch {
            let mut seq = 0u64;
            let mut buf = [0u8; 2048];
            let mut drained = 0u32;
            while let Ok(len) = stem::syscall::root_watch_next(watch_id, &mut seq, &mut buf) {
                if len > 0 {
                    drained += 1;
                } else {
                    break;
                }
            }
            if drained > 0 {
                dirty = true;
            }
        }

        let now = stem::monotonic_ns();
        if dirty && now.saturating_sub(last_scan) > 200_000_000 {
            let (mut nodes, edges) = scan_system_graph();
            // Explicitly sort and deduplicate nodes by ID to ensure uniqueness
            nodes.sort_by_key(|n| n.id.to_u64_lossy());
            nodes.dedup_by_key(|n| n.id.to_u64_lossy());

            if nodes != last_nodes || edges != last_edges {
                // 1. Convert to Layout types
                let mut layout_nodes: Vec<LayoutNode> = nodes
                    .iter()
                    .map(|n| LayoutNode {
                        id: n.id,
                        x: n.x,
                        y: n.y,
                        w: TILE_WIDTH as f32,
                        h: TILE_HEIGHT as f32,
                        fixed: n.fixed,
                        rank: n.rank,
                    })
                    .collect();

                let layout_edges: Vec<LayoutEdge> = edges
                    .iter()
                    .map(|e| LayoutEdge {
                        from: e.from,
                        to: e.to,
                        weight: e.weight,
                    })
                    .collect();

                // 2. Compute Layout
                let settings = LayoutSettings::default();
                compute_layout(&mut layout_nodes, &layout_edges, &settings);

                // 3. Compute Edge Routes
                last_routes = route_edges(&layout_nodes, &layout_edges, &settings);

                // 4. Persist back to graph (if changed significantly)
                for ln in &layout_nodes {
                    let old = nodes.iter().find(|n| n.id == ln.id);
                    let changed = old
                        .map(|o| (o.x - ln.x).abs() > 1.0 || (o.y - ln.y).abs() > 1.0)
                        .unwrap_or(true);

                    // FIX: Don't move actual windows, they are managed by the window manager (bloom)
                    let is_window = old
                        .map(|o| o.kind_full == kinds::UI_WINDOW)
                        .unwrap_or(false);

                    if changed && !is_window {
                        prop_set(ln.id, keys::UI_X, ln.x as i32 as u64).ok();
                        prop_set(ln.id, keys::UI_Y, ln.y as i32 as u64).ok();
                        prop_set(ln.id, keys::UI_RANK, ln.rank as u64).ok();
                    }
                }

                // 5. Update local NodeInfo with new positions for rendering
                let mut final_nodes = nodes.clone();
                for n in &mut final_nodes {
                    if let Some(ln) = layout_nodes.iter().find(|l| l.id == n.id) {
                        n.x = ln.x;
                        n.y = ln.y;
                    }
                }

                let layout = generate_layout(&final_nodes);
                let scene = build_graph_scene(
                    win,
                    &final_nodes,
                    &edges,
                    &layout,
                    &last_routes,
                    &viewport_controller.viewport,
                );
                let _ = stem::petals::publish_window(&scene);
                last_nodes = final_nodes;
                last_edges = edges;
            }
            dirty = false;
            last_scan = now;
        }

        stem::sleep(Duration::from_millis(50));
    }
}

fn build_graph_scene(
    win: ThingId,
    nodes: &[pipes::NodeInfo],
    edges: &[pipes::EdgeInfo],
    layout: &pipes::GraphLayout,
    routes: &BTreeMap<(ThingId, ThingId), Vec<(f32, f32)>>,
    viewport: &Viewport,
) -> Scene {
    let mut canvas = Canvas::new().width(Size::Pct(100)).height(Size::Pct(100));

    // Helper closure to transform world coords to screen coords
    let to_screen = |wx: f32, wy: f32| -> (i32, i32) {
        let (sx, sy) = viewport.world_to_screen(wx, wy);
        (sx as i32, sy as i32)
    };

    for edge in edges {
        if let Some(path) = routes.get(&(edge.from, edge.to)) {
            if path.len() < 2 {
                continue;
            }

            // Draw segments (transformed)
            for i in 0..path.len() - 1 {
                let (x1, y1) = path[i];
                let (x2, y2) = path[i + 1];
                let (sx1, sy1) = to_screen(x1, y1);
                let (sx2, sy2) = to_screen(x2, y2);

                canvas = canvas.push(
                    Line::new(sx1, sy1, sx2, sy2)
                        .width(2)
                        .color(Color::from_argb_u32(0xFF888888)),
                );
            }

            // Draw Arrow at the end (transformed)
            let (end_x, end_y) = path[path.len() - 1];
            let (prev_x, prev_y) = path[path.len() - 2];
            let (send_x, send_y) = to_screen(end_x, end_y);

            let angle = libm::atan2f(end_y - prev_y, end_x - prev_x);
            let head_len = 8.0 * viewport.zoom;
            let a1 = angle + 3.14159 * 0.85;
            let a2 = angle - 3.14159 * 0.85;

            let shx1 = send_x + (head_len * libm::cosf(a1)) as i32;
            let shy1 = send_y + (head_len * libm::sinf(a1)) as i32;
            let shx2 = send_x + (head_len * libm::cosf(a2)) as i32;
            let shy2 = send_y + (head_len * libm::sinf(a2)) as i32;

            canvas = canvas
                .push(
                    Line::new(send_x, send_y, shx1, shy1)
                        .width(2)
                        .color(Color::from_argb_u32(0xFF888888)),
                )
                .push(
                    Line::new(send_x, send_y, shx2, shy2)
                        .width(2)
                        .color(Color::from_argb_u32(0xFF888888)),
                );

            // Draw Label in Middle (Middle Segment, transformed)
            let mid_seg_idx = (path.len() - 1) / 2;
            let (aa, bb) = (path[mid_seg_idx], path[mid_seg_idx + 1]);
            let mid_x = (aa.0 + bb.0) / 2.0;
            let mid_y = (aa.1 + bb.1) / 2.0;
            let (smid_x, smid_y) = to_screen(mid_x, mid_y);

            let label_w = (edge.rel.len() as i32 * 6).max(10);
            canvas = canvas.push_at(
                Text::new(&edge.rel)
                    .font(FontKey::new("NotoSans-Regular").size(9))
                    .color(Color::from_argb_u32(0xFF666666))
                    .width(Size::Px(label_w))
                    .height(Size::Px(10)),
                smid_x,
                smid_y,
            );
        } else {
            // Fallback (Direct Line)
            // if let (Some(&(x1, y1)), Some(&(x2, y2))) = (
            //     layout.positions.get(&edge.from),
            //     layout.positions.get(&edge.to),
            // ) {
            //      // ... legacy direct line drawing ...
            // }
        }
    }

    // Draw nodes (transformed)
    for node in nodes {
        if let Some(&(x, y)) = layout.positions.get(&node.id) {
            // Transform center point to screen space
            let (scx, scy) = to_screen(x, y);
            let left = scx - TILE_WIDTH / 2;
            let top = scy - TILE_HEIGHT / 2;

            canvas = canvas.push_at(
                Rect::new()
                    .color(TILE_BORDER_COLOR)
                    .radius(TILE_RADIUS)
                    .width(Size::Px(TILE_WIDTH))
                    .height(Size::Px(TILE_HEIGHT)),
                left,
                top,
            );

            let inner_left = left + TILE_BORDER;
            let inner_top = top + TILE_BORDER;
            let inner_width = TILE_WIDTH - TILE_BORDER * 2;
            let inner_height = TILE_HEIGHT - TILE_BORDER * 2;
            canvas = canvas.push_at(
                Rect::new()
                    .color(TILE_FILL_COLOR)
                    .radius(TILE_RADIUS - TILE_BORDER)
                    .width(Size::Px(inner_width))
                    .height(Size::Px(inner_height)),
                inner_left,
                inner_top,
            );

            let icon_x = scx - ICON_SIZE / 2;
            let icon_y = top + ICON_TOP_PADDING;
            canvas = canvas.push_at(
                ThingosIcon::new(&node.icon)
                    .size(ICON_SIZE)
                    .width(Size::Px(ICON_SIZE))
                    .height(Size::Px(ICON_SIZE)),
                icon_x,
                icon_y,
            );

            let type_text = format_type_label(&node.kind_full, &node.name);
            let id_text = node.kind_full.clone();

            let type_width = (type_text.chars().count() as i32 * TYPE_CHAR_WIDTH).max(40);
            let type_left = scx - type_width / 2;
            let type_y = icon_y + ICON_SIZE + 12;
            canvas = canvas.push_at(
                Text::new(&type_text)
                    .font(FontKey::new("NotoSans-Regular").size(TYPE_FONT_SIZE))
                    .color(TYPE_TEXT_COLOR)
                    .width(Size::Px(type_width))
                    .height(Size::Px(TYPE_LINE_HEIGHT)),
                type_left,
                type_y,
            );

            let id_width = (id_text.chars().count() as i32 * ID_CHAR_WIDTH).max(30);
            let id_left = scx - id_width / 2;
            let id_y = type_y + TYPE_LINE_HEIGHT;
            canvas = canvas.push_at(
                Text::new(&id_text)
                    .font(FontKey::new("NotoSans-Regular").size(ID_FONT_SIZE))
                    .color(ID_TEXT_COLOR)
                    .width(Size::Px(id_width))
                    .height(Size::Px(ID_LINE_HEIGHT)),
                id_left,
                id_y,
            );
        }
    }

    Scene::new().window(
        Window::new(win)
            .title("Photosynthesis")
            .initial_size(800, 600)
            .root(canvas),
    )
}

fn format_type_label(kind_full: &str, name: &str) -> String {
    if !name.is_empty() && name != "unknown" && !name.starts_with("unknown_") {
        return name.to_string();
    }
    if kind_full.is_empty() {
        String::from("UNKNOWN")
    } else {
        kind_full.to_string()
    }
}


