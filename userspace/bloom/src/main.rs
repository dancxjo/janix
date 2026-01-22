#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod asset;
mod bmp;
mod bristle;
mod compositor;
mod cursor;
mod damage;
#[cfg(feature = "svg-demo")]
mod demo;
mod drawlist;
mod font_graph;
mod frame;
mod frame_loop;
pub mod geometry;
mod isa;
pub mod key_overlay;
mod log_ratelimit;
mod logging;
mod lowered;
pub mod perf;
mod present;
mod raster;
mod reclaimer;
mod render_state;
mod surface;
mod svg;
pub mod ui;

use abi::hid::Key;
use abi::root::RootWatchFilter;
use abi::types::{WatchMode, WatchSpec};
use core::sync::atomic::{AtomicBool, Ordering};
use stem::thing::{HandleId, ThingId};

pub static DISABLE_TEXT: AtomicBool = AtomicBool::new(false);
pub static DISABLE_WALLPAPER: AtomicBool = AtomicBool::new(false);
pub static FORCE_FULL_DAMAGE: AtomicBool = AtomicBool::new(false);

const UI_WATCH_MAX_EVENTS: u64 = 1024;
const UI_WATCH_BUDGET_NS: u64 = 2_000_000;

const WATCH_OVERFLOW_COUNTERS: [&str; 32] = [
    "watch_overflows.0",
    "watch_overflows.1",
    "watch_overflows.2",
    "watch_overflows.3",
    "watch_overflows.4",
    "watch_overflows.5",
    "watch_overflows.6",
    "watch_overflows.7",
    "watch_overflows.8",
    "watch_overflows.9",
    "watch_overflows.10",
    "watch_overflows.11",
    "watch_overflows.12",
    "watch_overflows.13",
    "watch_overflows.14",
    "watch_overflows.15",
    "watch_overflows.16",
    "watch_overflows.17",
    "watch_overflows.18",
    "watch_overflows.19",
    "watch_overflows.20",
    "watch_overflows.21",
    "watch_overflows.22",
    "watch_overflows.23",
    "watch_overflows.24",
    "watch_overflows.25",
    "watch_overflows.26",
    "watch_overflows.27",
    "watch_overflows.28",
    "watch_overflows.29",
    "watch_overflows.30",
    "watch_overflows.31",
];

use abi::display_driver_protocol::BindPayload;
use abi::schema::keys;
use stem::syscall::PortHandle;

use crate::asset::AssetType;
use crate::compositor::{CompositorTarget, DisplayBackend};
use crate::cursor::CursorState;
use crate::frame::{FrameBuilder, FrameSpec};
use crate::frame_loop::FrameLoop;
use crate::present::{DriverPresenter, PresenterImpl};
use crate::ui::FullRefreshReason;

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

use crate::asset::AssetBank;
pub static ASSETS: AssetBank = AssetBank::new();

extern "C" fn wallpaper_loader_entry() -> ! {
    stem::sleep_ms(200);
    let candidates = [
        "/assets/wallpapers/clouds.bmp",
        "wallpapers/clouds.bmp",
        "clouds.bmp",
    ];
    for path in candidates.iter() {
        if ASSETS.probe_asset_exists(path) {
            ASSETS.enqueue_wallpaper_load(path);
            break;
        }
    }
    loop {
        stem::syscall::sleep_ms(10000);
    }
}

extern "C" fn font_loader_entry() -> ! {
    use abi::schema::kinds;
    use stem::thing::sys::{bytespace_info, describe_thing, find, prop_get};
    let mut modules = [ThingId::default(); 64];
    let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
    for i in 0..count {
        let mut buf = [0u8; 512];
        let len = match describe_thing(modules[i], &mut buf) {
            Ok(l) => l,
            Err(_) => continue,
        };
        let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
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
        if mod_name.ends_with(".ttf")
            || mod_name.ends_with(".TTF")
            || mod_name.ends_with(".otf")
            || mod_name.ends_with(".OTF")
        {
            let bs_id = match prop_get(modules[i], "bytespace") {
                Ok(id) => ThingId::from_u64(id),
                Err(_) => continue,
            };
            let size = match bytespace_info(bs_id) {
                Ok(s) => s,
                Err(_) => continue,
            };
            ASSETS.enqueue_font_load(bs_id, size, mod_name);
        }
    }
    use abi::schema::rels;
    use abi::types::{WatchMode, WatchSpec};
    use stem::root_watch;
    use stem::syscall;
    use stem::thing::sys::bytespace_read;
    let k_file = stem::thing::sys::intern(kinds::FONT_FILE).unwrap_or(0);
    let k_family = stem::thing::sys::intern(kinds::FONT_FAMILY).unwrap_or(0);
    let k_face = stem::thing::sys::intern(kinds::FONT_FACE).unwrap_or(0);
    let k_super = stem::thing::sys::intern(kinds::FONT_SUPERFAMILY).unwrap_or(0);
    let p_bs = stem::thing::sys::intern(keys::FONT_BYTESPACE).unwrap_or(0);
    let p_sz = stem::thing::sys::intern(keys::FONT_SIZE_BYTES).unwrap_or(0);
    let p_name = stem::thing::sys::intern(keys::FONT_NAME).unwrap_or(0);
    let dirty_keys = [
        p_name,
        stem::thing::sys::intern(keys::FONT_STYLE).unwrap_or(0),
        stem::thing::sys::intern(keys::FONT_WEIGHT).unwrap_or(0),
        stem::thing::sys::intern(keys::FONT_WIDTH).unwrap_or(0),
        stem::thing::sys::intern(keys::FONT_SLOPE).unwrap_or(0),
        stem::thing::sys::intern(keys::FONT_COVERAGE_RANGES).unwrap_or(0),
        stem::thing::sys::intern(rels::FONT_CONTAINS).unwrap_or(0),
        stem::thing::sys::intern(rels::FONT_COVERS).unwrap_or(0),
    ];
    let watch_kinds = [k_file, k_family, k_face, k_super];
    let mut watch_ids: alloc::vec::Vec<usize> = alloc::vec::Vec::new();
    let mut watch_bufs: alloc::vec::Vec<[u8; 4096]> = alloc::vec::Vec::new();
    let mut watch_seq: alloc::vec::Vec<u64> = alloc::vec::Vec::new();

    let process_payload = |buf: &[u8]| {
        let mut cursor = 0usize;
        while cursor < buf.len() {
            if let Ok((header, value)) = abi::watch::decode_event(&buf[cursor..]) {
                cursor += abi::watch::WATCH_EVENT_HEADER_LEN + value.len();
                let mut is_dirty = false;
                if header.predicate == abi::watch::WATCH_PRED_KIND && value.len() == 4 {
                    let kid = u32::from_le_bytes(value.try_into().unwrap());
                    if kid == k_file || kid == k_family || kid == k_face || kid == k_super {
                        is_dirty = true;
                    }
                } else {
                    let pred = header.predicate.to_u32_lossy();
                    for &k in &dirty_keys {
                        if pred == k {
                            is_dirty = true;
                            break;
                        }
                    }
                    if pred == p_bs || pred == p_sz || pred == p_name {
                        is_dirty = true;
                    }
                }
                if is_dirty {
                    font_graph::mark_dirty();
                    let node_id = header.subject;
                    if let (Ok(bs), Ok(sz)) = (
                        prop_get(node_id, keys::FONT_BYTESPACE),
                        prop_get(node_id, keys::FONT_SIZE_BYTES),
                    ) {
                        let name = prop_get(node_id, keys::FONT_NAME)
                            .ok()
                            .and_then(|id| {
                                let bs_id = ThingId::from_u64(id);
                                let size = bytespace_info(bs_id).ok()?;
                                let mut b = alloc::vec![0u8; size];
                                let l = bytespace_read(bs_id, 0, &mut b).ok()?;
                                Some(alloc::string::String::from(
                                    core::str::from_utf8(&b[..l]).ok()?,
                                ))
                            })
                            .unwrap_or_else(|| "font.bin".into());
                        ASSETS.enqueue_font_load(ThingId::from_u64(bs), sz as usize, &name);
                    }
                }
            } else {
                break;
            }
        }
    };

    for kid in watch_kinds.into_iter().filter(|k| *k != 0) {
        let filter = RootWatchFilter::kind(kid);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            start_seq: 0,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(wid) = syscall::root_watch_open(&spec) {
            crate::perf::add_counter("bloom.watch_open_count", 1);
            crate::perf::add_counter("bloom.watch_reopen_reason.font_graph", 1);
            crate::perf::log_event("bloom.watch_reopen_reason", "font_graph");
            watch_ids.push(wid);
            watch_bufs.push([0u8; 4096]);
            watch_seq.push(0);
            let _ = root_watch::watch_drain(wid, watch_bufs.last_mut().unwrap(), |_, bytes| {
                process_payload(bytes)
            });
        }
    }
    loop {
        let mut any_activity = false;
        for idx in 0..watch_ids.len() {
            match syscall::root_watch_next(
                watch_ids[idx],
                &mut watch_seq[idx],
                &mut watch_bufs[idx],
            ) {
                Ok(len) if len > 0 => {
                    any_activity = true;
                    process_payload(&watch_bufs[idx][..len]);
                }
                Err(abi::errors::Errno::EAGAIN) => {}
                Err(abi::errors::Errno::EOVERFLOW) => {
                    font_graph::mark_dirty();
                }
                _ => {}
            }
        }
        if !any_activity {
            stem::sleep_ms(100);
        }
    }
}

extern "C" fn cursor_loader_entry() -> ! {
    stem::sleep_ms(300);
    #[cfg(feature = "svg-cursors")]
    let candidates = [
        "/assets/cursors/future/default.svg",
        "/assets/cursors/future/pointer.svg",
        "/assets/cursors/plain/Normal.cur",
    ];
    #[cfg(not(feature = "svg-cursors"))]
    let candidates = ["/assets/cursors/plain/Normal.cur"];
    for path in candidates.iter() {
        if ASSETS.probe_asset_exists(path) {
            ASSETS.enqueue_cursor_load(path);
            break;
        }
    }
    loop {
        stem::syscall::sleep_ms(10000);
    }
}

#[cfg_attr(not(test), stem::main)]
fn main(arg: usize) -> ! {
    logging::init();
    perf::init();
    use stem::thing::sys::{bytespace_map, bytespace_unmap};
    let bs_id = ThingId::from_u64(arg as u64);
    let (mut arg_req, mut arg_resp, mut bristle_evt) = (0, 0, 0);
    if let Ok(ptr) = bytespace_map(bs_id) {
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u32, 16) };
        if slice[0] == 0xB100AA01 {
            arg_req = slice[1];
            arg_resp = slice[2];
            bristle_evt = slice[3];
        }
        let _ = bytespace_unmap(bs_id, ptr);
    } else {
        arg_req = unpack_handle(arg, 0) as u32;
        arg_resp = unpack_handle(arg, 1) as u32;
        bristle_evt = unpack_handle(arg, 2) as u32;
    }

    use stem::stack::{Stack, StackSpec};
    let s_spec = StackSpec {
        reserve_bytes: 256 * 1024,
        initial_commit_bytes: 64 * 1024,
        ..StackSpec::default()
    };
    stem::thread::spawn_on(
        Stack::alloc_growing_stack(s_spec).unwrap(),
        wallpaper_loader_entry,
    )
    .ok();
    stem::thread::spawn_on(
        Stack::alloc_growing_stack(s_spec).unwrap(),
        cursor_loader_entry,
    )
    .ok();
    stem::thread::spawn_on(
        Stack::alloc_growing_stack(s_spec).unwrap(),
        font_loader_entry,
    )
    .ok();

    let target =
        CompositorTarget::discover_and_map((arg_req, arg_resp), 2000).expect("compositor discover");
    let indicator_color = match target.backend {
        DisplayBackend::VirtioGpu => geometry::Color::from_u32(0xFF00FF00),
        DisplayBackend::BootFB => geometry::Color::from_u32(0xFFFF0000),
        _ => geometry::Color::from_u32(0xFFFFFF00),
    };
    // #[cfg(feature = "svg-cursors")] cursor::set_target_color(indicator_color); // Removed: cursor is image-based now

    let mut presenter = if target.driver_req != 0 {
        let mut d = DriverPresenter::new(target.driver_req, target.driver_resp);
        d.send_bind(&BindPayload {
            bytespace_id: target.bs_id.to_u64_lossy(),
            width: target.width,
            height: target.height,
            stride: target.stride_bytes,
            format: target.format,
        });
        PresenterImpl::Driver(d)
    } else {
        PresenterImpl::Null(present::NullPresenter)
    };

    let mut surface = unsafe {
        surface::Surface::new(
            target.ptr,
            target.size_bytes,
            target.width,
            target.height,
            target.stride_bytes,
        )
    };

    #[cfg(feature = "svg-demo")]
    demo::run_svg_demo(
        unsafe {
            surface::Surface::new(
                target.ptr,
                target.size_bytes,
                target.width,
                target.height,
                target.stride_bytes,
            )
        },
        presenter,
        target.width,
        target.height,
        target.format,
    );
    let mut cursor = CursorState::new((target.width as i32) / 2, (target.height as i32) / 2);
    let mut prev_buttons: u32 = 0;
    let mut loop_ctrl = FrameLoop::new(60);
    let (mut wallpaper_loaded, mut cursor_loaded, mut font_loaded) = (false, false, false);
    let mut ui_watch_handles: alloc::vec::Vec<usize> = alloc::vec::Vec::new();
    let mut ui_watch_bufs: alloc::vec::Vec<[u8; 4096]> = alloc::vec::Vec::new();
    let mut ui_watch_seq: alloc::vec::Vec<u64> = alloc::vec::Vec::new();
    let mut ui_watch_pending = false;
    let mut ui_force_damage = false;
    let mut ui_poll_deadline_ns = stem::monotonic_ns().saturating_add(1_000_000_000);

    let (screen_w, screen_h) = (target.width as i32, target.height as i32);
    let mut ui_pipeline = ui::UiPipeline::new();
    let ui_root = stem::ui::UiBuilder::create_root();
    ui_pipeline.set_root(ui_root);

    // Subscribe to UI updates by predicate (props + edges), avoiding the global firehose.
    let ui_keys = ui_pipeline.ui_keys();
    let mut watch_predicates = alloc::collections::BTreeSet::new();
    for key in ui_keys.all_keys().into_iter().filter(|k| *k != 0) {
        watch_predicates.insert(key);
    }
    if ui_keys.has_child != 0 {
        watch_predicates.insert(ui_keys.has_child);
    }

    for predicate in watch_predicates.into_iter() {
        let filter = RootWatchFilter::predicate(predicate);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            start_seq: 0,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(id) = stem::syscall::root_watch_open(&spec) {
            crate::perf::add_counter("bloom.watch_open_count", 1);
            crate::perf::add_counter("bloom.watch_reopen_reason.ui_predicate", 1);
            crate::perf::log_event("bloom.watch_reopen_reason", "ui_predicate");
            ui_watch_handles.push(id);
            ui_watch_bufs.push([0u8; 4096]);
            ui_watch_seq.push(0);
            let _ = stem::root_watch::watch_drain(id, ui_watch_bufs.last_mut().unwrap(), |_, _| {
                ui_force_damage = true;
            });
        }
    }

    let mut keys = alloc::collections::BTreeSet::new();
    let mut key_overlay = key_overlay::KeyOverlay::new();
    key_overlay.setup(ui_root);

    let mut prev_cursor_bbox: Option<crate::damage::Rect> = None;
    let mut accel_cfg = bristle::MouseAccelConfig::default();
    let mut accel_state = bristle::MouseAccelState::default();
    let mut first_frame = true;
    let mut first_frame_rendered = false;

    loop {
        let frame_start = stem::monotonic_ns();
        loop_ctrl.next();
        let asset_gen = ASSETS.publish_pending();
        let token = presenter.acquire_frame(
            crate::frame::FrameSpec::new(target.width, target.height, target.format),
            asset_gen,
        );
        let frame_id = token.frame_id();

        trace_span!("build");
        let mut builder = FrameBuilder::new(token);
        let gen = builder.asset_generation();

        if first_frame {
            builder.mark_full_damage();
            first_frame = false;
        }
        if !wallpaper_loaded && ASSETS.get_wallpaper_for_gen(gen).is_some() {
            wallpaper_loaded = true;
            builder.mark_full_damage();
        }
        if !cursor_loaded {
            if let Some(a) = ASSETS.get_cursor_for_gen(gen) {
                cursor.set_asset(a);
                cursor_loaded = true;
                builder.add_damage(cursor.bbox());
            }
        }
        if !font_loaded {
            if font_graph::has_fonts_ready() || !ASSETS.get_fonts().is_empty() {
                font_loaded = true;
                builder.mark_full_damage();
                ui_pipeline.mark_dirty_full_with_reason(FullRefreshReason::CacheInvalidated);
            }
        }

        ASSETS.mark_reachable(AssetType::Wallpaper, wallpaper_loaded);
        ASSETS.mark_reachable(AssetType::Cursor, cursor_loaded);
        if wallpaper_loaded {
            ASSETS.mark_used(AssetType::Wallpaper, frame_id);
        }
        if cursor_loaded {
            ASSETS.mark_used(AssetType::Cursor, frame_id);
        }

        let old_bbox = cursor.bbox();
        if bristle_evt != 0 {
            bristle::poll_bristle(
                bristle_evt,
                &mut cursor,
                &mut keys,
                &accel_cfg,
                &mut accel_state,
                screen_w,
                screen_h,
            );
        }
        let new_bbox = cursor.bbox();
        if let Some(prev) = prev_cursor_bbox {
            if prev != new_bbox {
                builder.add_damage(old_bbox);
                builder.add_damage(new_bbox);
            }
        } else {
            builder.add_damage(new_bbox);
        }
        prev_cursor_bbox = Some(new_bbox);

        let buttons = cursor.buttons();
        let left_pressed = buttons & 0x1 != 0 && prev_buttons & 0x1 == 0;
        prev_buttons = buttons;

        {
            let drain_start = stem::monotonic_ns();
            let mut drained_events = 0u64;
            let mut drained_bytes = 0u64;
            let mut budget_exhausted = false;
            crate::trace_span!("ui.watch_drain");
            for (idx, wid) in ui_watch_handles.iter().enumerate() {
                loop {
                    if drained_events >= UI_WATCH_MAX_EVENTS
                        || stem::monotonic_ns().saturating_sub(drain_start) >= UI_WATCH_BUDGET_NS
                    {
                        budget_exhausted = true;
                        break;
                    }
                    match stem::syscall::root_watch_next(
                        *wid,
                        &mut ui_watch_seq[idx],
                        &mut ui_watch_bufs[idx],
                    ) {
                        Ok(len) if len > 0 => {
                            drained_bytes = drained_bytes.saturating_add(len as u64);
                            let mut c = 0;
                            while c < len {
                                if let Ok((h, v)) =
                                    abi::watch::decode_event(&ui_watch_bufs[idx][c..len])
                                {
                                    c += abi::watch::WATCH_EVENT_HEADER_LEN + v.len();
                                    let pred = h.predicate.to_u32_lossy();
                                    if pred == ui_keys.has_child {
                                        ui_pipeline.mark_node_edges_dirty(h.subject);
                                    } else {
                                        ui_pipeline.mark_node_dirty(h.subject);
                                    }
                                    ui_force_damage = true;
                                    drained_events = drained_events.saturating_add(1);
                                    if drained_events >= UI_WATCH_MAX_EVENTS {
                                        budget_exhausted = true;
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                        Ok(_) | Err(abi::errors::Errno::EAGAIN) => break,
                        Err(abi::errors::Errno::EOVERFLOW) => {
                            crate::perf::add_counter("ui.watch.overflows", 1);
                            crate::perf::add_counter("watch_overflows", 1);
                            if let Some(counter) = WATCH_OVERFLOW_COUNTERS.get(idx) {
                                crate::perf::add_counter(counter, 1);
                            } else {
                                crate::perf::add_counter("watch_overflows.other", 1);
                            }
                            ui_pipeline.mark_dirty_full_with_reason(FullRefreshReason::WatchOverflow);
                            ui_force_damage = true;
                            break;
                        }
                        Err(_) => break,
                    }
                    if budget_exhausted {
                        break;
                    }
                }
                if budget_exhausted {
                    break;
                }
            }
            let drain_ns = stem::monotonic_ns().saturating_sub(drain_start);
            crate::perf::add_counter("ui.watch.drain_ns", drain_ns);
            if drained_events > 0 {
                crate::perf::add_counter("ui.watch.events", drained_events);
                crate::perf::add_counter("ui.watch.event_ns", drain_ns / drained_events);
            }
            if drained_bytes > 0 {
                crate::perf::add_counter("ui.watch.bytes", drained_bytes);
            }
            if budget_exhausted {
                crate::perf::add_counter("ui.watch.budget_exhausted", 1);
            }
            ui_watch_pending = budget_exhausted;
            if ui_watch_pending {
                crate::perf::add_counter("ui.watch.pending", 1);
            }
        }
        if frame_start >= ui_poll_deadline_ns {
            ui_pipeline.mark_dirty();
            ui_force_damage = true;
            ui_poll_deadline_ns = frame_start.saturating_add(1_000_000_000);
        }
        if key_overlay.update(&keys, screen_w, screen_h) {
            ui_pipeline.mark_dirty();
        }

        let (ui_changed, ui_dmg) = {
            let list = builder.ops();
            if !DISABLE_WALLPAPER.load(Ordering::Relaxed) {
                if let Some(clouds) = ASSETS.get_wallpaper_for_gen(gen) {
                    let (cw, ch) = (clouds.width as i32, clouds.height as i32);
                    for y in (0..screen_h).step_by(ch as usize) {
                        for x in (0..screen_w).step_by(cw as usize) {
                            list.blit_image(&clouds, x, y);
                        }
                    }
                } else {
                    list.clear(geometry::Color::from_u32(0xFF002d44));
                }
            }
            list.rect(screen_w - 32, 8, 24, 24, indicator_color);
            if font_loaded && !DISABLE_TEXT.load(Ordering::Relaxed) {
                list.text_font(
                    "thing-os",
                    "NotoSerif-Regular.ttf",
                    20,
                    40,
                    24.0,
                    geometry::Color::from_u32(0xFFFFFFFF),
                );
                list.text_font(
                    &alloc::format!("frame: {}", frame_id),
                    "NotoSerif-Regular.ttf",
                    20,
                    70,
                    16.0,
                    geometry::Color::from_u32(0xFFCCCCCC),
                );
            }
            let res = ui_pipeline.run(screen_w, screen_h, list, &ASSETS);
            cursor.emit_drawlist(list);
            (res.changed, res.damage)
        };

        if left_pressed {
            if let Some(win) = ui_pipeline.hit_test_shade_button(cursor.x, cursor.y) {
                if ui_pipeline.toggle_window_shade(win) {
                    ui_force_damage = true;
                }
            }
        }

        if keys.contains(&Key::LeftCtrl)
            && keys.contains(&Key::LeftAlt)
            && keys.contains(&Key::LeftShift)
        {
            if keys.contains(&Key::T) {
                DISABLE_TEXT.fetch_xor(true, Ordering::Relaxed);
                ui_pipeline.mark_dirty();
                ui_force_damage = true;
                stem::sleep_ms(200);
            }
            if keys.contains(&Key::W) {
                DISABLE_WALLPAPER.fetch_xor(true, Ordering::Relaxed);
                ui_force_damage = true;
                stem::sleep_ms(200);
            }
            if keys.contains(&Key::P) {
                key_overlay.show_perf = !key_overlay.show_perf;
                ui_pipeline.mark_dirty();
                ui_force_damage = true;
                stem::sleep_ms(200);
            }
        }

        if ui_changed || ui_force_damage || FORCE_FULL_DAMAGE.load(Ordering::Relaxed) {
            if ui_dmg.is_empty() || FORCE_FULL_DAMAGE.load(Ordering::Relaxed) {
                builder.mark_full_damage();
            } else {
                for r in ui_dmg {
                    builder.add_damage(r);
                }
            }
            ui_force_damage = false;
        }

        let token = builder.finish();
        let dmg = token.damage.clone();
        crate::perf::add_counter("damage_rect_count", dmg.rect_count() as u64);
        let damage_total_area: i64 = dmg.iter().map(|rect| rect.area()).sum();
        crate::perf::add_counter("damage_total_area", damage_total_area.max(0) as u64);
        if !dmg.is_empty() {
            raster::execute_with_damage(&mut surface, &token.ops, &dmg, ui_pipeline.solid_text);
        }
        {
            trace_span!("present");
            presenter.present_frame(token);
            presenter.pump();
        }
        key_overlay.post_present();
        if !first_frame_rendered && frame_id >= 1 {
            first_frame_rendered = true;
            log!("[CONTRACT] [bloom] First frame rendered");
        }
        reclaimer::check_memory_pressure(&ASSETS);
        loop_ctrl.heartbeat(cursor.x, cursor.y);
        perf::add_counter(
            "frame.work_ns",
            stem::monotonic_ns().saturating_sub(frame_start),
        );
        perf::end_frame();
        loop_ctrl.sleep();
    }
}
