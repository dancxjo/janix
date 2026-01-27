#![no_std]
#![no_main]

extern crate alloc;
use abi::ids::HandleId;
use abi::schema::{keys, kinds, rels};
use alloc::string::String;
use alloc::vec::Vec;
use core::time::Duration;
use stem::info;
use stem::petals::{Canvas, Color, FontKey, Icon, Line, Rect, Scene, Size, Styled, Text, Window};
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

mod pipes;

use pipes::{generate_layout, scan_system_graph};

#[stem::main]
fn main() -> ! {
    info!("Photosynthesis starting...");

    // 1. Wait for Bloom (UI Root)
    let mut ui_root = ThingId::default();
    while ui_root.to_u64_lossy() == 0 {
        let mut roots = [ThingId::default(); 1];
        if let Ok(1) = find(kinds::UI_ROOT, &mut roots) {
            ui_root = roots[0];
        } else {
            stem::sleep(Duration::from_millis(100));
        }
    }
    info!("Found UI Root: {}", ui_root.to_u64_lossy());

    // 2. Create Window
    let win = create_node(kinds::UI_WINDOW).expect("win");
    link(win, rels::CHILD_OF, ui_root).expect("link");
    link(ui_root, rels::HAS_CHILD, win).expect("has_child");

    prop_set(win, keys::UI_BG_COLOR, 0xFFF5F5F0).ok(); // Off-white
    prop_set(win, keys::UI_WIDTH, 800).ok();
    prop_set(win, keys::UI_HEIGHT, 600).ok();
    prop_set(win, keys::UI_X, 200).ok();
    prop_set(win, keys::UI_Y, 100).ok();
    set_string_prop(win, keys::UI_TITLE, "Photosynthesis (System Graph)");

    let mut last_nodes = Vec::new();
    let mut last_edges = Vec::new();

    loop {
        // 3. Scan Graph
        let (nodes, edges) = scan_system_graph();

        if nodes != last_nodes || edges != last_edges {
            let layout = generate_layout(&nodes);
            let scene = build_graph_scene(win, &nodes, &edges, &layout);
            let _ = stem::petals::publish_window(&scene);
            last_nodes = nodes;
            last_edges = edges;
        }

        stem::sleep(Duration::from_secs(2));
    }
}

fn build_graph_scene(
    win: ThingId,
    nodes: &[pipes::NodeInfo],
    edges: &[pipes::EdgeInfo],
    layout: &pipes::GraphLayout,
) -> Scene {
    let mut canvas = Canvas::new()
        .width(Size::Pct(100))
        .height(Size::Pct(100));

    for edge in edges {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) =
            (layout.positions.get(&edge.from), layout.positions.get(&edge.to))
        {
            let (dx, dy) = (x2 - x1, y2 - y1);
            let dist = libm::sqrtf(dx * dx + dy * dy);
            if dist > 0.0 {
                let (ux, uy) = (dx / dist, dy / dist);
                let start_x = (x1 + ux * 55.0) as i32;
                let start_y = (y1 + uy * 15.0) as i32;
                let end_x = (x2 - ux * 55.0) as i32;
                let end_y = (y2 - uy * 15.0) as i32;
                canvas = canvas.push(
                    Line::new(start_x, start_y, end_x, end_y)
                        .width(2)
                        .color(Color::from_argb_u32(0xFF888888)),
                );

                let angle = libm::atan2f(dy, dx);
                let head_len = 8.0;
                let a1 = angle + 3.14159 * 0.85;
                let a2 = angle - 3.14159 * 0.85;
                let hx1 = (x2 + head_len * libm::cosf(a1)) as i32;
                let hy1 = (y2 + head_len * libm::sinf(a1)) as i32;
                let hx2 = (x2 + head_len * libm::cosf(a2)) as i32;
                let hy2 = (y2 + head_len * libm::sinf(a2)) as i32;
                canvas = canvas
                    .push(
                        Line::new(end_x, end_y, hx1, hy1)
                            .width(2)
                            .color(Color::from_argb_u32(0xFF888888)),
                    )
                    .push(
                        Line::new(end_x, end_y, hx2, hy2)
                            .width(2)
                            .color(Color::from_argb_u32(0xFF888888)),
                    );

                let mid_x = ((x1 + x2) / 2.0) as i32;
                let mid_y = ((y1 + y2) / 2.0) as i32;
                let label_w = (edge.rel.len() as i32 * 6).max(10);
                canvas = canvas.push_at(
                    Text::new(&edge.rel)
                        .font(FontKey::new("NotoSans-Regular").size(9))
                        .color(Color::from_argb_u32(0xFF666666))
                        .width(Size::Px(label_w))
                        .height(Size::Px(10)),
                    mid_x,
                    mid_y,
                );
            }
        }
    }

    for node in nodes {
        if let Some(&(x, y)) = layout.positions.get(&node.id) {
            let w = 110;
            let h = 30;
            let left = x as i32 - w / 2;
            let top = y as i32 - h / 2;
            canvas = canvas.push_at(
                Rect::new()
                    .color(Color::from_argb_u32(0xFF44AAFF))
                    .width(Size::Px(w))
                    .height(Size::Px(h)),
                left,
                top,
            );
            let icon_size = 24;
            let icon_x = left + 6;
            let icon_y = top + (h - icon_size) / 2;
            canvas = canvas.push_at(
                Icon::new(&node.kind)
                    .size(icon_size)
                    .width(Size::Px(icon_size))
                    .height(Size::Px(icon_size)),
                icon_x,
                icon_y,
            );
            let text_x = left + 12 + icon_size;
            let text_y = top + 4;
            let name_w = (node.name.len() as i32 * 6).max(10);
            canvas = canvas.push_at(
                Text::new(&node.name)
                    .font(FontKey::new("NotoSans-Regular").size(9))
                    .color(Color::from_argb_u32(0xFF000000))
                    .width(Size::Px(name_w))
                    .height(Size::Px(10)),
                text_x,
                text_y,
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
