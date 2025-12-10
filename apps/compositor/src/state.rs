use userland::prelude::*;
use userland_std::thing_models::{DisplayPresentRequest, MousePacketEvent};
use userland_std::MODE_INDEX_CONSOLE;

use crate::config::FRAME_INTERVAL_NS;
use crate::graph::{
    active_framebuffer, collect_surfaces_for_windows, collect_windows_for_place,
    console_mode_active, current_mode, handle_mode_switches, layout_policy_for_mode,
};
use crate::layout::{self, LayoutPolicy, StackedWindow};
use crate::model::Compositor;
use crate::render::{build_display_list, render_display_list};

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

    loop {
        tick_once(sys, &mut compositor);
        sys.sleep_for_ns(FRAME_INTERVAL_NS);
    }
}

fn tick_once<S: Sys>(sys: &mut S, compositor: &mut Compositor) {
    compositor.ensure_display_contracts(sys);
    handle_mode_switches(sys);

    if console_mode_active(sys) {
        compositor.process_mouse_packets(sys, &[]);
        compositor.drag = None;
        return;
    }

    let mode = match current_mode(sys) {
        Some(mode) => mode,
        None => {
            compositor.process_mouse_packets(sys, &[]);
            return;
        }
    };

    if mode.index == MODE_INDEX_CONSOLE {
        compositor.process_mouse_packets(sys, &[]);
        compositor.drag = None;
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
    let ops = build_display_list(compositor, &stacked, &windows, &surface_map);
    render_display_list(compositor, &ops);
    compositor.publish_present_request(sys);
}
