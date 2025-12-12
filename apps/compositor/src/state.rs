use userland::prelude::*;
use userland_std::MODE_INDEX_CONSOLE;
use userland_std::thing_models::{DisplayPresentRequest, MousePacketEvent};

use crate::config::FRAME_INTERVAL_NS;
use crate::graph::{
    active_framebuffer, collect_surfaces_for_windows, collect_windows_for_place,
    console_mode_active, current_mode, handle_mode_switches, layout_policy_for_mode,
    swap_display_buffers,
};
use crate::layout::{self, StackedWindow};
use crate::model::BackgroundImage;
use crate::model::Compositor;
use crate::render::{build_display_list, render_display_list};
use abi::MapFlags;
use alloc::boxed::Box;
use alloc::format;
use userland_std::{RawModule, shared_buffer_map};

fn load_background_image<S: Sys>(sys: &mut S) -> Option<BackgroundImage> {
    let modules = list_things_by_kind::<S, RawModule>(sys);
    let clouds_module = modules.iter().find(|m| m.identifier == "clouds.bmp")?;

    if let Some(buffer_id) = clouds_module.framebuffer_id {
        // Map the buffer
        if let Some((vaddr, size)) =
            shared_buffer_map(sys, buffer_id, MapFlags::READ.union(MapFlags::USER)).ok()
        {
            let ptr = vaddr as *const u8;
            // Parse BMP header
            // Signature "BM" at 0
            unsafe {
                if *ptr != b'B' || *ptr.add(1) != b'M' {
                    println(sys, "clouds.bmp: invalid signature");
                    return None;
                }
                // Little endian parsing helper
                let read_u32 = |offset| {
                    let p = ptr.add(offset);
                    u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
                };
                let read_i32 = |offset| {
                    let p = ptr.add(offset);
                    i32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
                };

                let read_u16 = |offset| {
                    let p = ptr.add(offset);
                    u16::from_le_bytes([*p, *p.add(1)])
                };

                let data_offset = read_u32(0x0A);
                let width = read_i32(0x12);
                let height = read_i32(0x16);
                let bpp = read_u16(0x1C);

                let msg = format!(
                    "clouds.bmp: mapped. {}x{} offset={} bpp={}",
                    width, height, data_offset, bpp
                );
                let leaked = Box::leak(msg.into_boxed_str());
                println(sys, leaked);

                if bpp != 24 && bpp != 32 {
                    println(sys, "clouds.bmp: unsupported bpp");
                    return None;
                }

                return Some(BackgroundImage {
                    ptr: ptr.add(data_offset as usize),
                    size: size as usize,
                    width,
                    height,
                    bpp,
                });
            }
        }
    }
    println(
        sys,
        "clouds.bmp: module found but no buffer_id or map failed",
    );
    None
}

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "compositor: starting");
    ensure_ui_schemas(sys);
    let _ = register_schema_for::<MousePacketEvent>(sys);
    let _ = register_schema_for::<DisplayPresentRequest>(sys);

    let fb = loop {
        if let Some(fb) = active_framebuffer(sys) {
            break fb;
        }
        println(sys, "compositor: waiting for primary display");
        sys.sleep_for_ns(50_000_000);
    };

    let mut compositor = Compositor::new(fb);

    if let Some(bg) = load_background_image(sys) {
        compositor.background_image = Some(bg);
    }

    loop {
        tick_once(sys, &mut compositor);
        sys.sleep_for_ns(FRAME_INTERVAL_NS);
    }
}

pub fn tick_once<S: Sys>(sys: &mut S, compositor: &mut Compositor) {
    compositor.ensure_display_contracts(sys);
    handle_mode_switches(sys);

    if console_mode_active(sys) {
        return;
    }

    let mode = match current_mode(sys) {
        Some(mode) => mode,
        None => {
            return;
        }
    };

    if mode.index == MODE_INDEX_CONSOLE {
        return;
    }

    let place_id = mode.place_id.unwrap_or(ThingId(0));
    let windows = collect_windows_for_place(sys, place_id);
    let surface_map = collect_surfaces_for_windows(sys, &windows);

    let fb_w = compositor.fb.info.width as i32;
    let fb_h = compositor.fb.info.height as i32;
    let policy = layout_policy_for_mode(&mode);
    let stacked: Vec<StackedWindow> = layout::apply_layout(policy, &windows, fb_w, fb_h);
    layout::persist_stack(sys, &stacked);

    compositor.sync_active_from_layout(&stacked);
    compositor.process_mouse_packets(sys, &stacked);

    // Animate background: scroll up and left (requires incrementing offset)
    compositor.background_offset.0 = compositor.background_offset.0.wrapping_add(1);
    compositor.background_offset.1 = compositor.background_offset.1.wrapping_add(1);

    let ops = build_display_list(compositor, &stacked, &windows, &surface_map);
    render_display_list(compositor, &ops);
    if let Some(active_index) = swap_display_buffers(sys, compositor.fb.display_id) {
        compositor.fb.update_active_index(active_index);
    }
    compositor.publish_present_request(sys);
}
