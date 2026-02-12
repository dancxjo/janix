#![feature(restricted_std)]
#![no_main]

extern crate alloc;

use abi::drawlist::{DrawListBuilder, FillRule, PathVerb, PointF};
use abi::geometry::RectI32Wire;
use abi::schema::{keys, kinds, rels};
use alloc::vec::Vec;
use core::time::Duration;
use stem::info;
use stem::petals::DrawList;
use stem::thing::sys::{bytespace_create, bytespace_write, create_node, find, link, prop_set};
use stem::thing::ThingId;

fn wait_for_ui_crown() -> ThingId {
    let mut ui_crown = ThingId::default();
    while ui_crown.to_u64_lossy() == 0 {
        let mut roots = [ThingId::default(); 1];
        if let Ok(1) = find(kinds::UI_CROWN, &mut roots) {
            ui_crown = roots[0];
        } else {
            stem::sleep(Duration::from_millis(100));
        }
    }
    ui_crown
}

fn write_rect_bytespace(rect: [i32; 4]) -> ThingId {
    let rect = RectI32Wire::new(rect[0], rect[1], rect[2], rect[3]);
    let bytes = rect.as_bytes();
    let bs_id = bytespace_create(bytes.len(), 0, 0).expect("viewport bytespace");
    bytespace_write(bs_id, 0, &bytes).ok();
    bs_id
}

fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    if value.is_empty() {
        prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = bytespace_create(value.len(), 0, 0).expect("string bytespace");
    bytespace_write(bs_id, 0, value.as_bytes()).ok();
    prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

fn build_drawlist(color: u32, use_clip: bool, scale: f32) -> DrawListBuilder {
    let mut builder = DrawListBuilder::new();

    // Demonstrate Save/Restore with clipping
    builder.push_save();

    if use_clip {
        // Demonstrate SetClipRect
        builder.push_set_clip_rect(10, 10, 300, 180);
    }

    // Demonstrate SetTransform (simple scale around center)
    if scale != 1.0 {
        let cx = 180.0;
        let cy = 110.0;
        // Translate to origin, scale, translate back
        builder.push_set_transform(scale, 0.0, 0.0, scale, cx - cx * scale, cy - cy * scale);
    }

    // Draw background rectangle
    builder.push_fill_rect(20, 20, 120, 80, color);

    // Draw a triangle path
    let path = [
        PathVerb::MoveTo(PointF::new(200.0, 30.0)),
        PathVerb::LineTo(PointF::new(280.0, 140.0)),
        PathVerb::LineTo(PointF::new(120.0, 140.0)),
        PathVerb::Close,
    ];
    builder.push_fill_path(&path, FillRule::NonZero, 0xff3366ff);

    // Draw a line
    builder.push_line(
        PointF::new(10.0, 10.0),
        PointF::new(350.0, 10.0),
        0xff00ff00,
        2.0,
    );

    builder.push_restore();

    builder
}

#[stem::main]
fn main() -> ! {
    info!("DrawList demo starting (with new commands)...");
    let ui_crown = wait_for_ui_crown();

    let win = create_node(kinds::UI_WINDOW).expect("window");
    link(win, rels::CHILD_OF, ui_crown).ok();
    link(ui_crown, rels::HAS_CHILD, win).ok();
    set_string_prop(win, keys::UI_TITLE, "DrawList Demo (Enhanced)");
    prop_set(win, keys::UI_X, 100).ok();
    prop_set(win, keys::UI_Y, 100).ok();
    prop_set(win, keys::UI_WIDTH, 360).ok();
    prop_set(win, keys::UI_HEIGHT, 220).ok();

    let viewport_bs = write_rect_bytespace([0, 0, 360, 220]);
    prop_set(win, keys::UI_VIEWPORT_BYTESPACE, viewport_bs.to_u64_lossy()).ok();

    // Use the new DrawList helper
    let mut dl = DrawList::new(win);
    dl.set_debug_name("main_drawlist").ok();
    dl.set_bounds(0, 0, 360, 220).ok();

    // Initial publish
    let builder = build_drawlist(0xff22aa66, false, 1.0);
    dl.publish(builder).expect("initial publish");

    let mut frame = 0u32;
    loop {
        stem::sleep(Duration::from_millis(750));
        frame += 1;

        // Cycle through different states to demonstrate features
        let color = if frame % 4 < 2 {
            0xff22aa66
        } else {
            0xffaa2244
        };
        let use_clip = (frame / 2) % 2 == 0;
        let scale = if frame % 8 < 4 { 1.0 } else { 0.8 };

        let builder = build_drawlist(color, use_clip, scale);
        dl.publish(builder).expect("publish");

        if frame % 4 == 0 {
            info!(
                "Frame {}: color cycle, clip={}, scale={}",
                frame, use_clip, scale
            );
        }
    }
}
