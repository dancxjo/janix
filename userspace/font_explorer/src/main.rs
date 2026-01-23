#![no_std]
#![no_main]

extern crate alloc;

use abi::schema::{keys, kinds, rels};
use abi::types::HandleId;
use alloc::string::String;
use alloc::vec::Vec;
use stem::thing::ThingId;
use stem::thing::sys::{
    bytespace_create, bytespace_info, bytespace_read, bytespace_write, create_node, find, link,
    prop_get, prop_set,
};

fn set_string_prop(id: ThingId, key: &str, value: &str) {
    if value.is_empty() {
        let _ = prop_set(id, key, 0);
        return;
    }
    let bs_id = match bytespace_create(value.len(), 0, 0) {
        Ok(id) => id,
        Err(_) => return,
    };
    let _ = bytespace_write(bs_id, 0, value.as_bytes());
    let _ = prop_set(id, key, bs_id.to_u64_lossy());
}

fn read_string_prop(id: ThingId, key: &str) -> Option<String> {
    let val = prop_get(id, key).ok()?;
    if val == 0 {
        return None;
    }
    read_bytespace_string(ThingId::from_u64(val))
}

fn read_bytespace_string(id: ThingId) -> Option<String> {
    let size = bytespace_info(id).ok()?;
    if size == 0 {
        return Some(String::new());
    }
    let mut buf = alloc::vec![0u8; size];
    let len = bytespace_read(id, 0, &mut buf).ok()?;
    let text = core::str::from_utf8(&buf[..len]).unwrap_or("");
    Some(text.into())
}

fn select_font_stack() -> Vec<String> {
    let mut names = Vec::new();
    let mut families = [ThingId::default(); 128];
    let count = find(kinds::FONT_FAMILY, &mut families).unwrap_or(0);
    for id in families.iter().take(count) {
        if let Some(name) = read_string_prop(*id, keys::FONT_NAME) {
            names.push(name);
        }
    }

    let mut stack = Vec::new();
    let preferred = ["Noto Sans", "Noto Sans Symbols", "Noto Sans Symbols 2"];
    for name in preferred {
        if names.iter().any(|n| n == name) {
            stack.push(name.into());
        }
    }
    if stack.is_empty() && !names.is_empty() {
        names.sort();
        for name in names.into_iter().take(3) {
            stack.push(name);
        }
    }
    if stack.is_empty() {
        stack.push("Noto Sans".into());
    }
    stack
}

#[stem::main]
fn main() -> ! {
    let mut ui_root = ThingId::default();
    let mut attempts = 0;
    while attempts < 120 {
        let mut roots = [ThingId::default(); 1];
        if find(kinds::UI_ROOT, &mut roots)
            .ok()
            .filter(|c| *c > 0)
            .is_some()
        {
            ui_root = roots[0];
            break;
        }
        stem::sleep_ms(500);
        attempts += 1;
    }

    if ui_root.to_u64_lossy() == 0 {
        loop {
            stem::sleep_ms(10000);
        }
    }

    let stack = select_font_stack();
    let stack_label = stack.join(" -> ");
    let style_label = "Weight 400 / Width 5 / Slope 0";

    let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
    link(win, rels::CHILD_OF, ui_root).expect("link window");
    link(ui_root, rels::HAS_CHILD, win).expect("link window has_child");
    prop_set(win, keys::UI_BG_COLOR, 0xFFF5F5F0).ok();
    prop_set(win, keys::UI_WIDTH, 900).ok();
    prop_set(win, keys::UI_HEIGHT, 520).ok();
    // Position at top-left with margin (to avoid overlap with clock at bottom-right)
    prop_set(win, keys::UI_X, 50).ok();
    prop_set(win, keys::UI_Y, 50).ok();
    set_string_prop(win, keys::UI_TITLE, "Font Explorer");

    let viewport = create_node(kinds::UI_VIEWPORT).expect("create UI_VIEWPORT");
    link(viewport, rels::CHILD_OF, win).expect("link viewport");
    link(win, rels::HAS_CHILD, viewport).expect("link window has_child");
    prop_set(viewport, keys::UI_WIDTH, 900).ok();
    prop_set(viewport, keys::UI_HEIGHT, 520).ok();
    prop_set(viewport, keys::UI_CLIP, 1).ok();

    let stack_text = create_node(kinds::UI_TEXT_RUN).expect("create UI_TEXT_RUN");
    link(stack_text, rels::CHILD_OF, viewport).expect("link stack text");
    link(viewport, rels::HAS_CHILD, stack_text).expect("link viewport has_child");
    prop_set(stack_text, keys::UI_X, 18).ok();
    prop_set(stack_text, keys::UI_Y, 18).ok();
    prop_set(stack_text, keys::UI_FONT_SIZE, 20).ok();
    prop_set(stack_text, keys::UI_FG_COLOR, 0xFF000000).ok();
    set_string_prop(stack_text, keys::UI_TEXT, &stack_label);
    set_string_prop(stack_text, keys::UI_FONT_STACK, &stack_label);

    let style_text = create_node(kinds::UI_TEXT_RUN).expect("create UI_TEXT_RUN");
    link(style_text, rels::CHILD_OF, viewport).expect("link style text");
    link(viewport, rels::HAS_CHILD, style_text).expect("link viewport has_child");
    prop_set(style_text, keys::UI_X, 18).ok();
    prop_set(style_text, keys::UI_Y, 50).ok();
    prop_set(style_text, keys::UI_FONT_SIZE, 14).ok();
    prop_set(style_text, keys::UI_FG_COLOR, 0xFF000000).ok();
    set_string_prop(style_text, keys::UI_TEXT, style_label);
    set_string_prop(style_text, keys::UI_FONT_STACK, &stack_label);

    let demo_text = "Hello World\nα β γ ∑ ∞\n⚙︎ ☺︎ 🛠";

    let main_text = create_node(kinds::UI_TEXT_RUN).expect("create UI_TEXT_RUN");
    link(main_text, rels::CHILD_OF, viewport).expect("link main text");
    link(viewport, rels::HAS_CHILD, main_text).expect("link viewport has_child");
    prop_set(main_text, keys::UI_X, 24).ok();
    prop_set(main_text, keys::UI_Y, 120).ok();
    prop_set(main_text, keys::UI_FONT_SIZE, 40).ok();
    prop_set(main_text, keys::UI_FG_COLOR, 0xFF000000).ok();
    prop_set(main_text, keys::UI_FONT_DEBUG, 1).ok();
    set_string_prop(main_text, keys::UI_TEXT, demo_text);
    set_string_prop(main_text, keys::UI_FONT_STACK, &stack_label);

    loop {
        stem::sleep_ms(1000);
    }
}
