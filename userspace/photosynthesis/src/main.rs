
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
    prop_set(win, keys::UI_WIDTH, 600).ok();
    prop_set(win, keys::UI_HEIGHT, 400).ok();
    prop_set(win, keys::UI_X, 970).ok(); // To the right of font_explorer
    prop_set(win, keys::UI_Y, 50).ok();
    set_string_prop(win, keys::UI_TITLE, "Photosynthesis (SVG Grid)");

    // 3. Find SVGs
    let svgs = find_svg_assets();
    info!("Found {} SVG assets", svgs.len());

    // 4. Set Window Icon (preferences-desktop-font.svg)
    if let Some((_, bs_id)) = svgs.iter().find(|(name, _)| name.contains("preferences-desktop-font.svg")) {
        prop_set(win, keys::UI_WINDOW_ICON, bs_id.to_u64_lossy()).ok();
    } else if let Some((_, bs_id)) = svgs.first() {
        prop_set(win, keys::UI_WINDOW_ICON, bs_id.to_u64_lossy()).ok(); // Fallback to first found
    }

    let cols = 6;
    let icon_size = 64;
    let padding = 10;

    let viewport = create_node(kinds::UI_VIEWPORT).expect("viewport");
    link(viewport, rels::CHILD_OF, win).expect("link viewport");
    link(win, rels::HAS_CHILD, viewport).expect("has_child");
    prop_set(viewport, keys::UI_WIDTH, 600).ok();
    prop_set(viewport, keys::UI_HEIGHT, 400).ok();
    prop_set(viewport, keys::UI_CLIP, 1).ok();

    for (i, (name, bs_id)) in svgs.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;

        let tile = create_node(kinds::UI_TILE).expect("tile");
        link(tile, rels::CHILD_OF, viewport).expect("child");
        link(viewport, rels::HAS_CHILD, tile).expect("has_child");

        let x = padding + col * (icon_size + padding);
        let y = padding + row * (icon_size + padding) + 30;
        prop_set(tile, keys::UI_X, x as u64).ok();
        prop_set(tile, keys::UI_Y, y as u64).ok();
        prop_set(tile, keys::UI_WIDTH, icon_size as u64).ok();
        prop_set(tile, keys::UI_HEIGHT, icon_size as u64).ok();
        prop_set(tile, keys::UI_TILE_ASSET, bs_id.to_u64_lossy()).ok();

        info!("Tile {} -> {} ({})", name, tile.to_u64_lossy(), bs_id.to_u64_lossy());
    }

    info!("Photosynthesis ready. Floating...");
    loop {
        stem::sleep(Duration::from_secs(10));
    }
}
