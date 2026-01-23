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
mod ui;

use stem::thing::{ThingId, HandleId};

use abi::display_driver_protocol::BindPayload;
use abi::schema::keys;
use stem::syscall::PortHandle;

use crate::compositor::CompositorTarget;
use crate::frame::FrameBuilder;
use crate::frame_loop::FrameLoop;
use crate::present::{DriverPresenter, PresenterImpl};
use crate::bristle::{poll_bristle, MouseAccelConfig, MouseAccelState};
use crate::cursor::CursorState;
use crate::ui::FullRefreshReason;
use alloc::collections::BTreeSet;

fn clear_surface(surface: &mut surface::Surface, color: u32) {
    let w = surface.width();
    let h = surface.height();
    for y in 0..h {
        for x in 0..w {
            surface.put_px(x, y, color);
        }
    }
}

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn build_ui_watch_predicates(keys: &ui::snapshot::UiKeys) -> alloc::vec::Vec<u32> {
    let mut preds = BTreeSet::new();
    for &key in keys.all_keys().iter().chain(core::iter::once(&keys.has_child)) {
        if key != 0 {
            preds.insert(key);
        }
    }
    preds.into_iter().collect()
}

fn is_bytespace_prop(pred: u32, keys: &ui::snapshot::UiKeys) -> bool {
    pred == keys.text
        || pred == keys.font
        || pred == keys.font_stack
        || pred == keys.title
        || pred == keys.svg_bytes
        || pred == keys.window_icon
        || pred == keys.tile_asset
}

fn process_ui_watch_payload(
    buf: &[u8],
    ui_pipeline: &mut ui::UiPipeline,
    keys: &ui::snapshot::UiKeys,
) {
    let mut cursor = 0usize;
    while cursor < buf.len() {
        let Ok((header, value)) = abi::watch::decode_event(&buf[cursor..]) else {
            break;
        };
        cursor += abi::watch::WATCH_EVENT_HEADER_LEN + value.len();

        let subject = ThingId(header.subject.0);
        let pred = header.predicate.to_u32_lossy();
        if pred == keys.has_child {
            ui_pipeline.mark_node_edges_dirty(subject);
        } else {
            ui_pipeline.mark_node_dirty(subject);
        }

        if is_bytespace_prop(pred, keys)
            && header.value_encoding == abi::watch::ValueEncoding::U64LE as u8
            && value.len() == 8
        {
            let bs_id = u64::from_le_bytes(value.try_into().unwrap());
            if bs_id != 0 {
                ui_pipeline.invalidate_asset(bs_id);
            }
        }
    }
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
    use abi::root::RootWatchFilter;
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
    stem::info!("bloom cursor_loader started");
    #[cfg(feature = "svg-cursors")]
    let candidates = [
        "/assets/cursors/future/default.svg",
        "/assets/cursors/future/pointer.svg",
        "/assets/cursors/plain/Normal.cur",
    ];
    #[cfg(not(feature = "svg-cursors"))]
    let candidates = ["/assets/cursors/plain/Normal.cur"];
    let mut found = false;
    for path in candidates.iter() {
        if ASSETS.probe_asset_exists(path) {
            stem::info!("bloom cursor_loader found asset: {}", path);
            ASSETS.enqueue_cursor_load(path);
            found = true;
            break;
        }
    }
    if !found {
        stem::info!("bloom cursor_loader: no cursor asset found!");
    }
    loop {
        stem::syscall::sleep_ms(10000);
    }
}

/// Draw cursor overlay onto surface
fn draw_cursor_overlay(surface: &mut surface::Surface, cursor: &CursorState) {
    let bbox = cursor.bbox();
    let screen_w = surface.width() as i32;
    let screen_h = surface.height() as i32;
    
    // Get cursor frame data
    let mut list = drawlist::DrawList::new();
    cursor.emit_drawlist(&mut list);
    
    for cmd in list.iter() {
        if let drawlist::DrawCmd::Cursor { frame, position } = cmd {
            let dx = position.x - frame.hotspot_x as i32;
            let dy = position.y - frame.hotspot_y as i32;
            
            // Blit cursor bitmap with alpha blending
            for py in 0..frame.image.height as i32 {
                for px in 0..frame.image.width as i32 {
                    let sx = dx + px;
                    let sy = dy + py;
                    if sx < 0 || sy < 0 || sx >= screen_w || sy >= screen_h {
                        continue;
                    }
                    let idx = (py as usize * frame.image.width as usize + px as usize);
                    if idx >= frame.image.pixels.len() {
                        continue;
                    }
                    let rgba = frame.image.pixels[idx];
                    let a = (rgba >> 24) & 0xFF;
                    if a == 0 { continue; }
                    if a == 0xFF {
                        surface.put_px(sx, sy, rgba);
                    } else {
                        // Alpha blend
                        let dst = surface.get_px(sx, sy);
                        let sr = (rgba >> 16) & 0xFF;
                        let sg = (rgba >> 8) & 0xFF;
                        let sb = rgba & 0xFF;
                        let dr = (dst >> 16) & 0xFF;
                        let dg = (dst >> 8) & 0xFF;
                        let db = dst & 0xFF;
                        let inv = 255 - a;
                        let r = (sr * a + dr * inv) / 255;
                        let g = (sg * a + dg * inv) / 255;
                        let b = (sb * a + db * inv) / 255;
                        surface.put_px(sx, sy, 0xFF000000 | (r << 16) | (g << 8) | b);
                    }
                }
            }
        } else {
            // Fallback: procedural crosshair
            let cx = cursor.x;
            let cy = cursor.y;
            for dx in -5..=5 {
                if cx + dx >= 0 && cx + dx < screen_w {
                    surface.put_px(cx + dx, cy, 0xFFFFFFFF);
                }
            }
            for dy in -5..=5 {
                if cy + dy >= 0 && cy + dy < screen_h {
                    surface.put_px(cx, cy + dy, 0xFFFFFFFF);
                }
            }
        }
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
    let _ = target.backend;

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
    let mut loop_ctrl = FrameLoop::new(60);
    let (screen_w, screen_h) = (target.width as i32, target.height as i32);
    
    // Cursor state
    let bristle_evt_handle = bristle_evt as PortHandle;
    let mut cursor = CursorState::new(screen_w / 2, screen_h / 2);
    let mut pressed_keys: BTreeSet<abi::hid::Key> = BTreeSet::new();
    let accel_cfg = MouseAccelConfig::default();
    let mut accel_state = MouseAccelState::default();
    let mut cursor_logged = false;
    
    // Load cursor asset if available
    if let Some(asset) = ASSETS.get_cursor() {
        cursor.set_asset(asset);
    }
    
    let ui_root = {
        let mut roots = [ThingId::default(); 1];
        match stem::thing::sys::find(abi::schema::kinds::UI_ROOT, &mut roots) {
            Ok(count) if count > 0 => roots[0],
            _ => stem::ui::UiBuilder::create_root(),
        }
    };
    let mut ui_pipeline = ui::UiPipeline::new();
    ui_pipeline.set_root(ui_root);
    let ui_keys = ui_pipeline.ui_keys();
    let ui_watch_preds = build_ui_watch_predicates(&ui_keys);
    let mut ui_watch_ids: alloc::vec::Vec<usize> = alloc::vec::Vec::new();
    let mut ui_watch_bufs: alloc::vec::Vec<[u8; 4096]> = alloc::vec::Vec::new();
    let mut ui_watch_seq: alloc::vec::Vec<u64> = alloc::vec::Vec::new();
    for pred in ui_watch_preds {
        let filter = abi::root::RootWatchFilter::predicate(pred);
        let spec = abi::types::WatchSpec {
            mode: abi::types::WatchMode::StreamOnly as u32,
            start_seq: 0,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<abi::root::RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(wid) = stem::syscall::root_watch_open(&spec) {
            ui_watch_ids.push(wid);
            ui_watch_bufs.push([0u8; 4096]);
            ui_watch_seq.push(0);
            let _ = stem::root_watch::watch_drain(
                wid,
                ui_watch_bufs.last_mut().unwrap(),
                |_, bytes| process_ui_watch_payload(bytes, &mut ui_pipeline, &ui_keys),
            );
        }
    }

    loop {
        loop_ctrl.next();
        
        // Promote pending assets (cursor, fonts, wallpaper) to ready
        ASSETS.publish_pending();
        
        let token = presenter.acquire_frame(
            crate::frame::FrameSpec::new(target.width, target.height, target.format),
            crate::frame::AssetGeneration::ZERO,
        );
        let mut builder = FrameBuilder::new(token);
        builder.mark_full_damage();
        clear_surface(&mut surface, 0xFF101018);
        for idx in 0..ui_watch_ids.len() {
            match stem::syscall::root_watch_next(
                ui_watch_ids[idx],
                &mut ui_watch_seq[idx],
                &mut ui_watch_bufs[idx],
            ) {
                Ok(len) if len > 0 => {
                    process_ui_watch_payload(
                        &ui_watch_bufs[idx][..len],
                        &mut ui_pipeline,
                        &ui_keys,
                    );
                }
                Err(abi::errors::Errno::EOVERFLOW) => {
                    crate::perf::add_counter("watch_overflows", 1);
                    ui_pipeline.mark_dirty_full_with_reason(FullRefreshReason::WatchOverflow);
                }
                Ok(_) | Err(abi::errors::Errno::EAGAIN) => {}
                Err(_) => {}
            }
        }
        let mut list = drawlist::DrawList::new();
        let ui_result = ui_pipeline.run(screen_w, screen_h, &mut list, &ASSETS);
        raster::execute(&mut surface, &list, ui_result.solid_text);
        
        // Poll input and update cursor
        if bristle_evt_handle != 0 {
            poll_bristle(
                bristle_evt_handle, 
                &mut cursor, 
                &mut pressed_keys, 
                &accel_cfg, 
                &mut accel_state, 
                screen_w, 
                screen_h
            );
        }
        
        // Update cursor asset if newly loaded
        if let Some(asset) = ASSETS.get_cursor() {
            cursor.set_asset(asset);
        }
        
        // Draw cursor overlay (always on top)
        draw_cursor_overlay(&mut surface, &cursor);
        
        if !cursor_logged {
            stem::info!("[CONTRACT] bloom cursor rendered at ({}, {})", cursor.x, cursor.y);
            cursor_logged = true;
        }

        let token = builder.finish();
        presenter.present_frame(token);
        presenter.pump();
        loop_ctrl.sleep();
    }
}
