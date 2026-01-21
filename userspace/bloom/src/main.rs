#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod asset;
mod bmp;
mod bristle;
mod compositor;
mod cursor;
mod damage;
mod drawlist;
mod frame;
mod frame_loop;
pub mod geometry; 
mod isa; 
pub mod key_overlay;
mod logging;
mod log_ratelimit;
mod lowered;
mod font_graph;
mod present;
mod raster;
mod reclaimer;
#[cfg(feature = "svg-cursors")]
mod svg;
mod surface;
#[cfg(feature = "svg-demo")]
mod demo;
pub mod ui;
pub mod perf;

use core::sync::atomic::{AtomicBool, Ordering};
use abi::hid::Key;
use stem::thing::{ThingId, HandleId};

pub static DISABLE_TEXT: AtomicBool = AtomicBool::new(false);
pub static DISABLE_WALLPAPER: AtomicBool = AtomicBool::new(false);
pub static FORCE_FULL_DAMAGE: AtomicBool = AtomicBool::new(false);

use abi::display_driver_protocol::BindPayload;
use abi::schema::keys;
use stem::syscall::PortHandle;

use crate::asset::AssetType;
use crate::compositor::{CompositorTarget, DisplayBackend};
use crate::cursor::CursorState;
use crate::frame::{FrameBuilder, FrameSpec};
use crate::frame_loop::FrameLoop;
use crate::present::{DriverPresenter, PresenterImpl};

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

use crate::asset::AssetBank;
pub static ASSETS: AssetBank = AssetBank::new();

extern "C" fn wallpaper_loader_entry() -> ! {
    stem::sleep_ms(200);
    let candidates = ["/assets/wallpapers/clouds.bmp", "wallpapers/clouds.bmp", "clouds.bmp"];
    for path in candidates.iter() {
        if ASSETS.probe_asset_exists(path) { ASSETS.enqueue_wallpaper_load(path); break; }
    }
    loop { stem::syscall::sleep_ms(10000); }
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
            if let Some(end) = rest.find('"') { &rest[..end] } else { continue; }
        } else { continue; };
        if mod_name.ends_with(".ttf") || mod_name.ends_with(".TTF") || mod_name.ends_with(".otf") || mod_name.ends_with(".OTF") {
            let bs_id = match prop_get(modules[i], "bytespace") { Ok(id) => ThingId::from_u64(id), Err(_) => continue };
            let size = match bytespace_info(bs_id) { Ok(s) => s, Err(_) => continue };
            ASSETS.enqueue_font_load(bs_id, size, mod_name);
        }
    }
    use abi::types::{WatchMode, WatchSpec};
    use abi::schema::rels;
    use stem::syscall;
    use stem::thing::sys::bytespace_read;
    use stem::root_watch;
    let spec = WatchSpec { mode: WatchMode::StreamOnly as u32, start_seq: 0, ..Default::default() };
    let watch_id = syscall::root_watch_open(&spec).expect("font watch open");
    let mut watch_buf = [0u8; 4096];
    let k_file = stem::thing::sys::intern(kinds::FONT_FILE).unwrap_or(0);
    let k_family = stem::thing::sys::intern(kinds::FONT_FAMILY).unwrap_or(0);
    let k_face = stem::thing::sys::intern(kinds::FONT_FACE).unwrap_or(0);
    let k_super = stem::thing::sys::intern(kinds::FONT_SUPERFAMILY).unwrap_or(0);
    let p_bs = stem::thing::sys::intern(keys::FONT_BYTESPACE).unwrap_or(0);
    let p_sz = stem::thing::sys::intern(keys::FONT_SIZE_BYTES).unwrap_or(0);
    let p_name = stem::thing::sys::intern(keys::FONT_NAME).unwrap_or(0);
    let dirty_keys = [p_name, stem::thing::sys::intern(keys::FONT_STYLE).unwrap_or(0), stem::thing::sys::intern(keys::FONT_WEIGHT).unwrap_or(0), stem::thing::sys::intern(keys::FONT_WIDTH).unwrap_or(0), stem::thing::sys::intern(keys::FONT_SLOPE).unwrap_or(0), stem::thing::sys::intern(keys::FONT_COVERAGE_RANGES).unwrap_or(0), stem::thing::sys::intern(rels::FONT_CONTAINS).unwrap_or(0), stem::thing::sys::intern(rels::FONT_COVERS).unwrap_or(0)];
    let process_payload = |buf: &[u8]| {
        let mut cursor = 0usize;
        while cursor < buf.len() {
            if let Ok((header, value)) = abi::watch::decode_event(&buf[cursor..]) {
                cursor += abi::watch::WATCH_EVENT_HEADER_LEN + value.len();
                let mut is_dirty = false;
                if header.predicate == abi::watch::WATCH_PRED_KIND && value.len() == 4 {
                    let kid = u32::from_le_bytes(value.try_into().unwrap());
                    if kid == k_file || kid == k_family || kid == k_face || kid == k_super { is_dirty = true; }
                } else {
                    let pred = header.predicate.to_u32_lossy();
                    for &k in &dirty_keys { if pred == k { is_dirty = true; break; } }
                    if pred == p_bs || pred == p_sz || pred == p_name { is_dirty = true; }
                }
                if is_dirty {
                    font_graph::mark_dirty();
                    let node_id = header.subject;
                    if let (Ok(bs), Ok(sz)) = (prop_get(node_id, keys::FONT_BYTESPACE), prop_get(node_id, keys::FONT_SIZE_BYTES)) {
                        let name = prop_get(node_id, keys::FONT_NAME).ok().and_then(|id| {
                            let bs_id = ThingId::from_u64(id);
                            let size = bytespace_info(bs_id).ok()?;
                            let mut b = alloc::vec![0u8; size];
                            let l = bytespace_read(bs_id, 0, &mut b).ok()?;
                            Some(alloc::string::String::from(core::str::from_utf8(&b[..l]).ok()?))
                        }).unwrap_or_else(|| "font.bin".into());
                        ASSETS.enqueue_font_load(ThingId::from_u64(bs), sz as usize, &name);
                    }
                }
            } else { break; }
        }
    };
    let _ = root_watch::watch_drain(watch_id, &mut watch_buf, |_seq, bytes| process_payload(bytes));
    let mut seq_out = 0u64;
    loop {
        match syscall::root_watch_next(watch_id, &mut seq_out, &mut watch_buf) {
            Ok(len) if len > 0 => process_payload(&watch_buf[..len]),
            Err(abi::errors::Errno::EAGAIN) => stem::sleep_ms(100),
            Err(abi::errors::Errno::EOVERFLOW) => { font_graph::mark_dirty(); }
            _ => stem::sleep_ms(1000),
        }
    }
}

extern "C" fn cursor_loader_entry() -> ! {
    stem::sleep_ms(300);
    #[cfg(feature = "svg-cursors")]
    let candidates = ["/assets/cursors/future/default.svg", "/assets/cursors/future/pointer.svg", "/assets/cursors/plain/Normal.cur"];
    #[cfg(not(feature = "svg-cursors"))]
    let candidates = ["/assets/cursors/plain/Normal.cur"];
    for path in candidates.iter() {
        if ASSETS.probe_asset_exists(path) { ASSETS.enqueue_cursor_load(path); break; }
    }
    loop { stem::syscall::sleep_ms(10000); }
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
        if slice[0] == 0xB100AA01 { arg_req = slice[1]; arg_resp = slice[2]; bristle_evt = slice[3]; }
        let _ = bytespace_unmap(bs_id, ptr);
    } else {
        arg_req = unpack_handle(arg, 0) as u32; arg_resp = unpack_handle(arg, 1) as u32; bristle_evt = unpack_handle(arg, 2) as u32;
    }

    use stem::stack::{Stack, StackSpec};
    let s_spec = StackSpec { reserve_bytes: 256 * 1024, initial_commit_bytes: 64 * 1024, ..StackSpec::default() };
    stem::thread::spawn_on(Stack::alloc_growing_stack(s_spec).unwrap(), wallpaper_loader_entry).ok();
    stem::thread::spawn_on(Stack::alloc_growing_stack(s_spec).unwrap(), cursor_loader_entry).ok();
    stem::thread::spawn_on(Stack::alloc_growing_stack(s_spec).unwrap(), font_loader_entry).ok();

    let target = CompositorTarget::discover_and_map((arg_req, arg_resp), 2000).expect("compositor discover");
    let indicator_color = match target.backend {
        DisplayBackend::VirtioGpu => geometry::Color::from_u32(0xFF00FF00),
        DisplayBackend::BootFB => geometry::Color::from_u32(0xFFFF0000),
        _ => geometry::Color::from_u32(0xFFFFFF00),
    };
    // #[cfg(feature = "svg-cursors")] cursor::set_target_color(indicator_color); // Removed: cursor is image-based now

    let mut presenter = if target.driver_req != 0 {
        let mut d = DriverPresenter::new(target.driver_req, target.driver_resp);
        d.send_bind(&BindPayload { bytespace_id: target.bs_id.to_u64_lossy(), width: target.width, height: target.height, stride: target.stride_bytes, format: target.format });
        PresenterImpl::Driver(d)
    } else { PresenterImpl::Null(present::NullPresenter) };

    let mut surface = unsafe { surface::Surface::new(target.ptr, target.size_bytes, target.width, target.height, target.stride_bytes) };
    
    #[cfg(feature = "svg-demo")]
    demo::run_svg_demo(unsafe { surface::Surface::new(target.ptr, target.size_bytes, target.width, target.height, target.stride_bytes) }, presenter, target.width, target.height, target.format);
    let mut cursor = CursorState::new((target.width as i32) / 2, (target.height as i32) / 2);
    let mut loop_ctrl = FrameLoop::new(60);
    let (mut wallpaper_loaded, mut cursor_loaded, mut font_loaded) = (false, false, false);
    let mut ui_watch_id = None;
    let mut ui_watch_buf = [0u8; 4096];
    let mut ui_force_damage = false;
    let mut ui_poll_deadline_ns = stem::monotonic_ns().saturating_add(1_000_000_000);

    let (screen_w, screen_h) = (target.width as i32, target.height as i32);
    let mut ui_pipeline = ui::UiPipeline::new();
    let ui_root = stem::ui::UiBuilder::create_root();
    ui_pipeline.set_root(ui_root);

    let ui_text_pred_id = stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0);
    if ui_text_pred_id != 0 {
        use abi::root::RootWatchFilter;
        use abi::types::{WatchMode, WatchSpec};
        let filter = RootWatchFilter::predicate(ui_text_pred_id);
        let spec = WatchSpec { mode: WatchMode::StreamOnly as u32, start_seq: 0, filter_ptr: &filter as *const _ as u64, filter_len: core::mem::size_of::<RootWatchFilter>() as u64, ..Default::default() };
        if let Ok(id) = stem::syscall::root_watch_open(&spec) {
            ui_watch_id = Some(id);
            let mut db = [0u8; 4096];
            let _ = stem::root_watch::watch_drain(id, &mut db, |_,_| { ui_force_damage = true; });
        }
    }

    let mut keys = alloc::collections::BTreeSet::new();
    let mut key_overlay = key_overlay::KeyOverlay::new();
    key_overlay.setup(ui_root);

    let mut prev_cursor_bbox: Option<crate::damage::Rect> = None;
    let mut first_frame = true; let mut first_frame_rendered = false;

    loop {
        let frame_start = stem::monotonic_ns();
        loop_ctrl.next();
        let asset_gen = ASSETS.publish_pending();
        let token = presenter.acquire_frame(crate::frame::FrameSpec::new(target.width, target.height, target.format), asset_gen);
        let frame_id = token.frame_id();

        trace_span!("build");
        let mut builder = FrameBuilder::new(token);
        let gen = builder.asset_generation();

        if first_frame { builder.mark_full_damage(); first_frame = false; }
        if !wallpaper_loaded && ASSETS.get_wallpaper_for_gen(gen).is_some() { wallpaper_loaded = true; builder.mark_full_damage(); }
        if !cursor_loaded { if let Some(a) = ASSETS.get_cursor_for_gen(gen) { cursor.set_asset(a); cursor_loaded = true; builder.add_damage(cursor.bbox()); } }
        if !font_loaded { if font_graph::has_fonts_ready() || !ASSETS.get_fonts().is_empty() { font_loaded = true; builder.mark_full_damage(); ui_pipeline.mark_dirty(); } }

        ASSETS.mark_reachable(AssetType::Wallpaper, wallpaper_loaded);
        ASSETS.mark_reachable(AssetType::Cursor, cursor_loaded);
        if wallpaper_loaded { ASSETS.mark_used(AssetType::Wallpaper, frame_id); }
        if cursor_loaded { ASSETS.mark_used(AssetType::Cursor, frame_id); }

        let old_bbox = cursor.bbox();
        if bristle_evt != 0 { bristle::poll_bristle(bristle_evt, &mut cursor, &mut keys, screen_w, screen_h); }
        let new_bbox = cursor.bbox();
        if let Some(prev) = prev_cursor_bbox { if prev != new_bbox { builder.add_damage(old_bbox); builder.add_damage(new_bbox); } }
        else { builder.add_damage(new_bbox); }
        prev_cursor_bbox = Some(new_bbox);

        if let Some(wid) = ui_watch_id {
            let mut seq = 0;
            loop {
                match stem::syscall::root_watch_next(wid, &mut seq, &mut ui_watch_buf) {
                    Ok(len) if len > 0 => {
                        let mut c = 0;
                        while c < len {
                            if let Ok((h, v)) = abi::watch::decode_event(&ui_watch_buf[c..len]) {
                                c += abi::watch::WATCH_EVENT_HEADER_LEN + v.len();
                                if ui_text_pred_id != 0 && h.predicate.to_u32_lossy() == ui_text_pred_id {
                                    let sid = h.subject;
                                    ui_pipeline.mark_node_dirty(sid); ui_force_damage = true;
                                    let now = crate::log_ratelimit::now_ms();
                                    if crate::log_ratelimit::log_every(2000, now) { log!("[bloom][uiwatch] UI_TEXT event: seq={} subj={}", seq, sid.to_u64_lossy()); }
                                }
                            } else { break; }
                        }
                    }
                    Ok(_) | Err(abi::errors::Errno::EAGAIN) => break,
                    Err(abi::errors::Errno::EOVERFLOW) => { ui_pipeline.mark_dirty(); ui_force_damage = true; break; }
                    Err(_) => break,
                }
            }
        }
        if frame_start >= ui_poll_deadline_ns { ui_pipeline.mark_dirty(); ui_force_damage = true; ui_poll_deadline_ns = frame_start.saturating_add(1_000_000_000); }
        if key_overlay.update(&keys, screen_w, screen_h) { ui_pipeline.mark_dirty(); }

        let (ui_changed, ui_dmg) = {
            let list = builder.ops();
            if !DISABLE_WALLPAPER.load(Ordering::Relaxed) {
                if let Some(clouds) = ASSETS.get_wallpaper_for_gen(gen) {
                    let (cw, ch) = (clouds.width as i32, clouds.height as i32);
                    for y in (0..screen_h).step_by(ch as usize) { for x in (0..screen_w).step_by(cw as usize) { list.blit_image(&clouds, x, y); } }
                } else { list.clear(geometry::Color::from_u32(0xFF002d44)); }
            }
            list.rect(screen_w - 32, 8, 24, 24, indicator_color);
            if font_loaded && !DISABLE_TEXT.load(Ordering::Relaxed) {
                list.text_font("thing-os", "NotoSerif-Regular.ttf", 20, 40, 24.0, geometry::Color::from_u32(0xFFFFFFFF));
                list.text_font(&alloc::format!("frame: {}", frame_id), "NotoSerif-Regular.ttf", 20, 70, 16.0, geometry::Color::from_u32(0xFFCCCCCC));
            }
            let res = ui_pipeline.run(screen_w, screen_h, list, &ASSETS);
            cursor.emit_drawlist(list);
            (res.changed, res.damage)
        };

        if keys.contains(&Key::LeftCtrl) && keys.contains(&Key::LeftAlt) && keys.contains(&Key::LeftShift) {
            if keys.contains(&Key::T) { DISABLE_TEXT.fetch_xor(true, Ordering::Relaxed); ui_pipeline.mark_dirty(); ui_force_damage = true; stem::sleep_ms(200); }
            if keys.contains(&Key::W) { DISABLE_WALLPAPER.fetch_xor(true, Ordering::Relaxed); ui_force_damage = true; stem::sleep_ms(200); }
            if keys.contains(&Key::P) { key_overlay.show_perf = !key_overlay.show_perf; ui_pipeline.mark_dirty(); ui_force_damage = true; stem::sleep_ms(200); }
        }

        if ui_changed || ui_force_damage || FORCE_FULL_DAMAGE.load(Ordering::Relaxed) {
            if ui_dmg.is_empty() || FORCE_FULL_DAMAGE.load(Ordering::Relaxed) { builder.mark_full_damage(); }
            else { for r in ui_dmg { builder.add_damage(r); } }
            ui_force_damage = false;
        }

        let token = builder.finish();
        let dmg = token.damage.clone();
        if !dmg.is_empty() {
            trace_span!("raster");
            raster::execute_with_damage(&mut surface, &token.ops, &dmg, ui_pipeline.solid_text);
        }
        { trace_span!("present"); presenter.present_frame(token); presenter.pump(); }
        key_overlay.post_present();
        if !first_frame_rendered && frame_id >= 1 { first_frame_rendered = true; log!("[CONTRACT] [bloom] First frame rendered"); }
        reclaimer::check_memory_pressure(&ASSETS);
        loop_ctrl.heartbeat(cursor.x, cursor.y);
        perf::add_counter("frame.work_ns", stem::monotonic_ns().saturating_sub(frame_start));
        perf::end_frame();
        loop_ctrl.sleep();
    }
}
