#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::time::Duration;
use stem::info;
use stem::thing::ThingId;
use stem::thing::sys::{bytespace_create, bytespace_write, create_node, find, link, prop_set};
use abi::schema::{kinds, keys, rels};
use abi::drawlist::{DrawListBuilder, FillRule, PathVerb, PointF};
<<<<<<< ours
=======
use abi::geometry::RectI32Wire;
>>>>>>> theirs

fn wait_for_ui_root() -> ThingId {
    let mut ui_root = ThingId::default();
    while ui_root.to_u64_lossy() == 0 {
        let mut roots = [ThingId::default(); 1];
        if let Ok(1) = find(kinds::UI_ROOT, &mut roots) {
            ui_root = roots[0];
        } else {
            stem::sleep(Duration::from_millis(100));
        }
    }
    ui_root
}

fn write_rect_bytespace(rect: [i32; 4]) -> ThingId {
<<<<<<< ours
    let mut bytes = Vec::with_capacity(16);
    for value in rect {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
=======
    let rect = RectI32Wire::new(rect[0], rect[1], rect[2], rect[3]);
    let bytes = rect.as_bytes();
>>>>>>> theirs
    let bs_id = bytespace_create(bytes.len(), 0, 0).expect("viewport bytespace");
    bytespace_write(bs_id, 0, &bytes).ok();
    bs_id
}

<<<<<<< ours
=======
fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    if value.is_empty() {
        prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = bytespace_create(value.len(), 0, 0).expect("string bytespace");
    bytespace_write(bs_id, 0, value.as_bytes()).ok();
    prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

>>>>>>> theirs
fn build_drawlist(color: u32) -> Vec<u8> {
    let mut builder = DrawListBuilder::new();
    builder.push_fill_rect(20, 20, 120, 80, color);
    let path = [
        PathVerb::MoveTo(PointF::new(200.0, 30.0)),
        PathVerb::LineTo(PointF::new(280.0, 140.0)),
        PathVerb::LineTo(PointF::new(120.0, 140.0)),
        PathVerb::Close,
    ];
    builder.push_fill_path(&path, FillRule::NonZero, 0xff3366ff);
    builder.finish()
}

#[stem::main]
fn main() -> ! {
    info!("DrawList demo starting...");
    let ui_root = wait_for_ui_root();

    let win = create_node(kinds::UI_WINDOW).expect("window");
    link(win, rels::CHILD_OF, ui_root).ok();
    link(ui_root, rels::HAS_CHILD, win).ok();
<<<<<<< ours
    prop_set(win, keys::UI_TITLE, 0).ok();
=======
    set_string_prop(win, keys::UI_TITLE, "DrawList Demo");
>>>>>>> theirs
    prop_set(win, keys::UI_X, 100).ok();
    prop_set(win, keys::UI_Y, 100).ok();
    prop_set(win, keys::UI_WIDTH, 360).ok();
    prop_set(win, keys::UI_HEIGHT, 220).ok();

    let viewport_bs = write_rect_bytespace([0, 0, 360, 220]);
    prop_set(win, keys::UI_VIEWPORT_BYTESPACE, viewport_bs.to_u64_lossy()).ok();

    let drawlist_bytes = build_drawlist(0xff22aa66);
<<<<<<< ours
    let drawlist_bs = bytespace_create(drawlist_bytes.len(), 0, 0).expect("drawlist bytespace");
=======
    let mut drawlist_len = drawlist_bytes.len();
    let mut drawlist_bs = bytespace_create(drawlist_bytes.len(), 0, 0).expect("drawlist bytespace");
>>>>>>> theirs
    bytespace_write(drawlist_bs, 0, &drawlist_bytes).ok();
    prop_set(win, keys::UI_DRAWLIST_BYTESPACE, drawlist_bs.to_u64_lossy()).ok();

    let mut gen: u64 = 1;
    prop_set(win, keys::UI_DRAWLIST_GEN, gen).ok();

    loop {
        stem::sleep(Duration::from_millis(750));
        gen += 1;
        let color = if gen % 2 == 0 { 0xff22aa66 } else { 0xffaa2244 };
        let drawlist_bytes = build_drawlist(color);
<<<<<<< ours
=======
        if drawlist_bytes.len() != drawlist_len {
            drawlist_bs = bytespace_create(drawlist_bytes.len(), 0, 0).expect("drawlist bytespace");
            drawlist_len = drawlist_bytes.len();
            prop_set(win, keys::UI_DRAWLIST_BYTESPACE, drawlist_bs.to_u64_lossy()).ok();
        }
>>>>>>> theirs
        bytespace_write(drawlist_bs, 0, &drawlist_bytes).ok();
        prop_set(win, keys::UI_DRAWLIST_GEN, gen).ok();
    }
}
