
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::time::Duration;
use stem::info;
use stem::thing::{ThingId, HandleId};
use stem::thing::sys::{create_node, prop_set, link, find, describe_thing, prop_get};
use abi::schema::{kinds, keys, rels};

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
            if let Some(end) = rest.find('"') { &rest[..end] } else { continue; }
        } else { continue; };
        
        if mod_name.ends_with(".svg") {
            let bs_id = match prop_get(modules[i], "bytespace") { Ok(id) => ThingId::from_u64(id), Err(_) => continue };
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

use pipes::{scan_system_graph, generate_layout, render_graph};

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
    let mut drawlist_gen = 0u64;

    loop {
        // 3. Scan Graph
        let (nodes, edges) = scan_system_graph();

        if nodes != last_nodes || edges != last_edges {
            let layout = generate_layout(&nodes);
            let drawlist_bytes = render_graph(&nodes, &edges, &layout);

            // 4. Update DrawList
            use stem::thing::sys::{bytespace_create, bytespace_write};
            let bs_id = bytespace_create(drawlist_bytes.len(), 0, 0).expect("create bs");
            bytespace_write(bs_id, 0, &drawlist_bytes).ok();
            
            prop_set(win, keys::UI_DRAWLIST_BYTESPACE, bs_id.to_u64_lossy() as u64).ok();
            
            drawlist_gen += 1;
            prop_set(win, keys::UI_DRAWLIST_GEN, drawlist_gen).ok();
            
            last_nodes = nodes;
            last_edges = edges;
        }

        stem::sleep(Duration::from_secs(2));
    }
}
