#![no_std]
#![no_main]

extern crate alloc;

use abi::schema::{keys, kinds, rels};
use abi::types::HandleId;
use alloc::string::String;
use alloc::vec::Vec;
use stem::petals::{AlignItems, Color, Flex, FontKey, Scene, Scroll, Separator, Styled, Text, Window};
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

fn list_fonts() -> Vec<String> {
    let mut names = Vec::new();
    let mut families = [ThingId::default(); 256];
    let count = find(kinds::FONT_FAMILY, &mut families).unwrap_or(0);
    for id in families.iter().take(count) {
        if let Some(name) = read_string_prop(*id, keys::FONT_NAME) {
            names.push(name);
        }
    }
    names.sort();
    names.dedup();
    names
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

    const SAMPLE: &str = "Sphinx of black quartz, judge my vow. 0123456789";
    let fonts = list_fonts();

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

    let row_hint = 28u16;
    let content_min = (row_hint as i32).saturating_mul(fonts.len() as i32);
    let mut scroll_y = 0i32;
    let mut last_tick = stem::monotonic_ns();

    loop {
        let now = stem::monotonic_ns();
        let dt_sec = (now.saturating_sub(last_tick) as f32) / 1_000_000_000.0;
        last_tick = now;

        // Simple auto-scroll for demonstration
        scroll_y = (scroll_y + (10.0 * dt_sec) as i32) % content_min.max(1);

        let mut rows = Flex::column().gap(0);
        for (i, name) in fonts.iter().enumerate() {
            let row = Flex::row()
                .align_items(AlignItems::Center)
                .gap(12)
                .padding(6)
                .push(
                    Text::new(SAMPLE)
                        .font(FontKey::name(name).size(18))
                        .nowrap()
                        .ellipsis(true)
                        .flex_grow(1.0),
                )
                .push(
                    Text::new(name)
                        .font(FontKey::new("NotoSans-Regular").size(12))
                        .color(Color::rgb(80, 80, 80)),
                );
            rows = rows.push(row);
            if i + 1 < fonts.len() {
                rows = rows.push(Separator::new(1, Color::from_argb_u32(0xFFE0E0E0)));
            }
        }

        let scroll = Scroll::vertical()
            .scroll_y(scroll_y)
            .content_min_height(content_min)
            .estimated_row_height(row_hint)
            .total_rows(fonts.len() as u32)
            .clip(true)
            .flex_grow(1.0)
            .push(rows);

        let scene = Scene::new().window(
            Window::new(win)
                .title("Font Explorer")
                .initial_size(900, 520)
                .root(
                    Flex::column()
                        .gap(8)
                        .padding(16)
                        .push(
                            Text::new("Fonts")
                                .font(FontKey::new("NotoSans-Regular").size(20))
                                .color(Color::rgb(0, 0, 0)),
                        )
                        .push(scroll),
                ),
        );

        let _ = stem::petals::publish_window(&scene);

        stem::sleep_ms(33); // ~30 FPS
    }
}
