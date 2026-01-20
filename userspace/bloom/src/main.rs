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
pub mod geometry; // Canonical geometry types
mod isa; // Portable Render ISA types
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
// mod target;
// mod target_cpu;
pub mod ui;

use abi::display_driver_protocol::BindPayload;
use abi::ids::HandleId;
use abi::schema::keys;
use stem::syscall::PortHandle;

use crate::asset::AssetType;
use crate::compositor::{CompositorTarget, DisplayBackend};
use crate::cursor::CursorState;
use crate::damage::Rect;
use crate::frame::{FrameBuilder, FrameSpec};
use crate::frame_loop::FrameLoop;
use crate::present::{DriverPresenter, PresenterImpl};

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

use crate::asset::AssetBank;
pub static ASSETS: AssetBank = AssetBank::new();

/// Background thread for loading wallpaper
extern "C" fn wallpaper_loader_entry() -> ! {
    log!("[wallpaper_loader] thread started");

    stem::sleep_ms(200);
    log!("[wallpaper_loader] searching for wallpaper...");

    let candidates = [
        "/assets/wallpapers/clouds.bmp",
        "wallpapers/clouds.bmp",
        "clouds.bmp",
    ];

    for path in candidates.iter() {
        log!("[wallpaper_loader] trying: {}", path);
        if ASSETS.probe_asset_exists(path) {
            log!(
                "[wallpaper_loader] SUCCESS: found candidate '{}', enqueuing load",
                path
            );
            ASSETS.enqueue_wallpaper_load(path);
            break;
        } else {
            log!("[wallpaper_loader] not found: {}", path);
        }
    }

    log!("[wallpaper_loader] thread done, sleeping forever");
    loop {
        stem::syscall::sleep_ms(10000);
    }
}

/// Background thread for loading fonts
/// Background thread for loading fonts
extern "C" fn font_loader_entry() -> ! {
    log!("[font_loader] thread started");

    use abi::schema::kinds;
    use stem::thing::sys::{bytespace_info, bytespace_map, describe_thing, find, prop_get};
    use stem::thing::ThingId;

    // =========================================================================
    // PHASE 1: IMMEDIATE SCAN
    // Directly scan all boot modules for TTF files and load them immediately.
    // This ensures fonts are available as quickly as possible without waiting
    // for ingestd processing or watch mechanisms.
    // =========================================================================
    
    log!("[font_loader] scanning boot modules for fonts...");
    
    let mut modules = [ThingId::default(); 64];
    let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
    log!("[font_loader] found {} boot modules", count);
    
    for i in 0..count {
        let mod_id = modules[i];
        let mut buf = [0u8; 512];
        let len = match describe_thing(mod_id, &mut buf) {
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
        
        // Check if this looks like a font file
        let is_font = mod_name.ends_with(".ttf") 
            || mod_name.ends_with(".TTF")
            || mod_name.ends_with(".otf")
            || mod_name.ends_with(".OTF");
        
        if !is_font {
            continue;
        }
        
        log!("[font_loader] found font module: '{}'", mod_name);
        
        // Get bytespace
        let bs_id = match prop_get(mod_id, "bytespace") {
            Ok(id) => ThingId::from_u64(id),
            Err(_) => continue,
        };
        
        let size = match bytespace_info(bs_id) {
            Ok(s) => s,
            Err(_) => continue,
        };
        
        log!("[font_loader] enqueuing immediate font load: bs={} size={} name='{}'",
            bs_id.to_u64_lossy(), size, mod_name);
        ASSETS.enqueue_font_load(bs_id, size, mod_name);
    }
    
    log!("[font_loader] immediate scan complete, entering watch loop");

    // =========================================================================
    // PHASE 2: WATCH-BASED UPDATES
    // Continue watching for any new fonts added by ingestd or hot-loaded later.
    // =========================================================================
    
    use abi::types::{WatchMode, WatchSpec};
    use abi::schema::{keys, rels};
    use stem::syscall;
    use stem::thing::sys::bytespace_read;
    use stem::root_watch;
    use crate::font_graph;

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

    let spec = WatchSpec {
        mode: WatchMode::StreamOnly as u32,
        query_ptr: 0,
        query_len: 0,
        start_seq: 0,
        filter_ptr: 0,
        filter_len: 0,
        ..Default::default()
    };

    let watch_id = match syscall::root_watch_open(&spec) {
        Ok(id) => {
            log!("[font_loader] watch opened (id={})", id);
            id
        }
        Err(e) => {
            log!("[font_loader] ERROR: watch open failed: {:?}", e);
            loop {
                stem::sleep_ms(10000);
            }
        }
    };

    fn read_name_from_bytespace(id: ThingId) -> Option<alloc::string::String> {
        let size = bytespace_info(id).ok()?;
        let mut buf = alloc::vec![0u8; size];
        let len = bytespace_read(id, 0, &mut buf).ok()?;
        Some(alloc::string::String::from(core::str::from_utf8(&buf[..len]).unwrap_or("")))
    }

    let process_payload = |buf: &[u8]| {
        let mut cursor = 0usize;
        while cursor < buf.len() {
            match abi::watch::decode_event(&buf[cursor..]) {
                Ok((header, value)) => {
                    cursor += abi::watch::WATCH_EVENT_HEADER_LEN + value.len();
                    
                    if abi::watch::WatchOp::from_u8(header.op) != Some(abi::watch::WatchOp::Upsert) {
                        continue;
                    }

                    let mut is_dirty = false;
                    let mut is_file_update = false;

                    if header.predicate == abi::watch::WATCH_PRED_KIND {
                         if abi::watch::ValueEncoding::from_u8(header.value_encoding) 
                            == Some(abi::watch::ValueEncoding::Bytes) && value.len() == 4 
                         {
                             let kind_id = u32::from_le_bytes(value.try_into().unwrap());
                             if kind_id == k_file {
                                 is_dirty = true;
                                 is_file_update = true;
                             } else if kind_id == k_family || kind_id == k_face || kind_id == k_super {
                                 is_dirty = true;
                             }
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
                            if pred == p_bs || pred == p_sz {
                                is_file_update = true;
                            }
                        }
                    }

                    if is_dirty {
                        font_graph::mark_dirty();
                    }

                    if is_dirty || is_file_update {
                        let node_id = ThingId::from_u64(header.subject.to_u64_lossy());
                         if node_id.to_u64_lossy() != 0 {
                            let bs_id = prop_get(node_id, keys::FONT_BYTESPACE).map(ThingId::from_u64).ok();
                            let size = prop_get(node_id, keys::FONT_SIZE_BYTES).ok().map(|v| v as usize);
                            
                            if let (Some(bs), Some(sz)) = (bs_id, size) {
                                let name_id = prop_get(node_id, keys::FONT_NAME).map(ThingId::from_u64).ok();
                                let name = name_id.and_then(read_name_from_bytespace)
                                    .unwrap_or_else(|| "font.bin".into());

                                log!(
                                    "[font_loader] enqueuing font load: bs={} size={} name='{}'",
                                    bs.to_u64_lossy(),
                                    sz,
                                    name
                                );
                                ASSETS.enqueue_font_load(bs, sz, &name);
                            }
                         }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    };

    let mut watch_buf = [0u8; 4096];

    match root_watch::watch_drain(watch_id, &mut watch_buf, |_seq, bytes| {
        process_payload(bytes)
    }) {
        Ok(stats) => {
            if stats.batches > 0 || stats.overflows > 0 {
                log!("[font_loader] drain complete: payloads={} overflows={}", stats.batches, stats.overflows);
            }
        }
        Err(e) => {
            log!("[font_loader] ERROR: drain failed: {:?}", e);
        }
    }

    let mut seq_out = 0u64;
    loop {
        match syscall::root_watch_next(watch_id, &mut seq_out, &mut watch_buf) {
            Ok(len) if len > 0 => {
                process_payload(&watch_buf[..len]);
            }
            Ok(_) => {}
            Err(abi::errors::Errno::EAGAIN) => {
                stem::sleep_ms(100);
            }
            Err(abi::errors::Errno::EOVERFLOW) => {
                log!("[font_loader] watch overflow in steady state");
                font_graph::mark_dirty();
            }
            Err(e) => {
                log!("[font_loader] watch next error: {:?}", e);
                stem::sleep_ms(1000);
            }
        }
    }
}
/// Background thread for loading cursor
extern "C" fn cursor_loader_entry() -> ! {
    log!("[cursor_loader] thread started");

    stem::sleep_ms(300);
    log!("[cursor_loader] searching for cursor...");

    #[cfg(feature = "svg-cursors")]
    let candidates = [
        "/assets/cursors/future/default.svg",
        "/assets/cursors/future/pointer.svg",
        "/assets/cursors/plain/Normal.cur",
        "cursors/plain/Normal.cur",
        "Normal.cur",
    ];

    #[cfg(not(feature = "svg-cursors"))]
    let candidates = [
        "/assets/cursors/plain/Normal.cur",
        "cursors/plain/Normal.cur",
        "Normal.cur",
    ];

    for path in candidates.iter() {
        log!("[cursor_loader] trying: {}", path);
        if ASSETS.probe_asset_exists(path) {
            log!(
                "[cursor_loader] SUCCESS: found candidate '{}', enqueuing load",
                path
            );
            ASSETS.enqueue_cursor_load(path);
            break;
        } else {
            log!("[cursor_loader] not found: {}", path);
        }
    }

    log!("[cursor_loader] thread done, sleeping forever");
    loop {
        stem::syscall::sleep_ms(10000);
    }
}
#[cfg_attr(not(test), stem::main)]
fn main(arg: usize) -> ! {
    logging::init();

    let arg_val = arg;
    let mut arg_req = 0;
    let mut arg_resp = 0;
    let mut bristle_evt = 0;
    let mut svc_font_id = 0u64;

    // Try to map arg as Bytespace
    use stem::thing::sys::{bytespace_map, bytespace_unmap};
    use stem::thing::ThingId;
    let bs_id = ThingId::from_u64(arg_val as u64);

    let mapped = bytespace_map(bs_id);

    let mut valid_bs = false;
    if let Ok(ptr) = mapped {
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u32, 16) };
        if slice[0] == 0xB100AA01 {
            arg_req = slice[1];
            arg_resp = slice[2];
            bristle_evt = slice[3];
            svc_font_id = (slice[4] as u64) | ((slice[5] as u64) << 32);
            valid_bs = true;
            log!("[bloom] Bootstrapped via Bytespace ID={}", arg_val);
        }
        let _ = bytespace_unmap(bs_id, ptr);
    }

    if !valid_bs {
        // Fallback or Error?
        // Sprout was creating map. If failed, it might pass packed handles?
        // Sprout only passes packed handles if `boot_bs` fail.
        // Assuming Bytespace works.
        // If not, we might be running in old environment.
        // Try unpack legacy:
        log!("[bloom] WARN: Bootstrap BS failed/invalid, trying packed args...");
        arg_req = unpack_handle(arg_val, 0) as u32;
        arg_resp = unpack_handle(arg_val, 1) as u32;
        bristle_evt = unpack_handle(arg_val, 2) as u32;
    }

    log!(
        "[bloom] starting (arg_req={} arg_resp={} bristle={} font_svc={})",
        arg_req,
        arg_resp,
        bristle_evt,
        svc_font_id
    );

    // Font Client is no longer used, as we load fonts locally now.

    // Logic below to handle key input if needed (currently poll_bristle does it)
    use alloc::collections::BTreeSet;
    use stem::stack::{Stack, StackSpec};

    // Spawn wallpaper loader thread
    let wallpaper_stack = Stack::alloc_growing_stack(StackSpec {
        reserve_bytes: 256 * 1024,
        initial_commit_bytes: 64 * 1024,
        ..StackSpec::default()
    })
    .expect("wallpaper stack");
    match stem::thread::spawn_on(wallpaper_stack, wallpaper_loader_entry) {
        Ok(tid) => {
            // Loader threads enqueue quickly but should not outrank device drivers: Normal (2).
            let _ = stem::thread::set_priority(tid, 2);
            log!("[bloom] spawned wallpaper_loader thread (tid={})", tid);
        }
        Err(e) => {
            log!("[bloom] ERROR: failed to spawn wallpaper loader: {:?}", e);
        }
    }

    // Spawn cursor loader thread
    let cursor_stack = Stack::alloc_growing_stack(StackSpec {
        reserve_bytes: 256 * 1024,
        initial_commit_bytes: 64 * 1024,
        ..StackSpec::default()
    })
    .expect("cursor stack");
    match stem::thread::spawn_on(cursor_stack, cursor_loader_entry) {
        Ok(tid) => {
            let _ = stem::thread::set_priority(tid, 2);
            log!("[bloom] spawned cursor_loader thread (tid={})", tid);
        }
        Err(e) => {
            log!("[bloom] ERROR: failed to spawn cursor loader: {:?}", e);
        }
    }

    // Spawn font loader thread
    let font_stack = Stack::alloc_growing_stack(StackSpec {
        reserve_bytes: 256 * 1024,
        initial_commit_bytes: 64 * 1024,
        ..StackSpec::default()
    })
    .expect("font stack");
    match stem::thread::spawn_on(font_stack, font_loader_entry) {
        Ok(tid) => {
            let _ = stem::thread::set_priority(tid, 2);
            log!("[bloom] spawned font_loader thread (tid={})", tid);
        }
        Err(e) => {
            log!("[bloom] ERROR: failed to spawn font loader: {:?}", e);
        }
    }

    // 1. Discovery & Mapping
    log!("[bloom] discovering compositor target...");
    let target = match CompositorTarget::discover_and_map((arg_req, arg_resp), 2000) {
        Ok(t) => {
            log!(
                "[bloom] compositor target: {}x{} @ {:p} backend={}",
                t.width,
                t.height,
                t.ptr,
                t.backend.name()
            );

            t
        }
        Err(e) => {
            log!("[bloom] ERROR: compositor discovery failed: {:?}", e);
            loop {
                stem::sleep_ms(1000);
            }
        }
    };

    // Backend indicator color: green for VirtIO-GPU, red for BootFB
    let backend_indicator_color = match target.backend {
        DisplayBackend::VirtioGpu => geometry::Color::from_u32(0xFF00FF00), // Bright green
        DisplayBackend::BootFB => geometry::Color::from_u32(0xFFFF0000),    // Bright red
        DisplayBackend::Unknown => geometry::Color::from_u32(0xFFFFFF00),   // Yellow for unknown
    };

    #[cfg(feature = "svg-cursors")]
    cursor::set_target_color(backend_indicator_color);

    // 2. Presenter Setup
    let mut presenter = if target.driver_req != 0 && target.driver_resp != 0 {
        log!(
            "[bloom] presenter: driver (req={} resp={})",
            target.driver_req,
            target.driver_resp
        );
        let mut driver = DriverPresenter::new(target.driver_req, target.driver_resp);

        let bind = BindPayload {
            bytespace_id: target.bs_id.to_u64_lossy(),
            width: target.width,
            height: target.height,
            stride: target.stride_bytes,
            format: target.format,
        };
        driver.send_bind(&bind);
        PresenterImpl::Driver(driver)
    } else {
        log!("[bloom] presenter: null (headless/fallback)");
        PresenterImpl::Null(present::NullPresenter)
    };

    // 3. State Initialization
    let mut surface = unsafe {
        surface::Surface::new(
            target.ptr,
            target.size_bytes,
            target.width,
            target.height,
            target.stride_bytes,
        )
    };

    let mut cursor = CursorState::new((target.width as i32) / 2, (target.height as i32) / 2);
    let mut loop_ctrl = FrameLoop::new(60);
    let mut cursor_loaded = false;
    let mut wallpaper_loaded = false;
    let mut font_loaded = false;
    let mut ui_watch_id: Option<usize> = None;
    let ui_text_pred_id: u32;  // For userspace filtering
    let mut ui_watch_buf = [0u8; 4096];
    let mut ui_force_damage = false;
    let mut ui_poll_deadline_ns = stem::monotonic_ns().saturating_add(1_000_000_000);

    let screen_w = target.width as i32;
    let screen_h = target.height as i32;
    let frame_spec = FrameSpec::new(target.width, target.height, target.format);

    // Track previous cursor position for damage
    let mut prev_cursor_bbox: Option<Rect> = None;
    let mut first_frame = true;

    // 4. UI Pipeline Setup
    let mut ui_pipeline = ui::UiPipeline::new();
    let ui_root = stem::ui::UiBuilder::create_root();
    if ui_root.to_u64_lossy() == 0 {
        log!("[bloom] ERROR: failed to create UI root!");
    } else {
        log!("[bloom] created UI root node: {}", ui_root.to_u64_lossy());
    }
    ui_pipeline.set_root(ui_root);
    {
        use abi::root::RootWatchFilter;
        use abi::types::{WatchMode, WatchSpec, WATCH_START_LATEST};

        let ui_text_pred = stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0);
            ui_text_pred_id = ui_text_pred;
        if ui_text_pred != 0 {
            // WORKAROUND: unfiltered watch
            // let filter = RootWatchFilter::predicate(ui_text_pred);
            let spec = WatchSpec {
                mode: WatchMode::StreamOnly as u32,
                start_seq: 0, // Catch-up mode
                filter_ptr: 0,
                filter_len: 0,
                ..Default::default()
            };
            match stem::syscall::root_watch_open(&spec) {
                Ok(id) => {
                    ui_watch_id = Some(id);
                    log!("[bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id={})", ui_text_pred);

                    // Drain initial events to catch up
                    let mut drain_buf = [0u8; 4096];
                    match stem::root_watch::watch_drain(id, &mut drain_buf, |_seq, _payload| {
                         // We don't need to parse payload because we do a full snapshot anyway
                    }) {
                        Ok(stats) => {
                            if stats.batches > 0 {
                                log!("[bloom] ui watch drained: {} batches", stats.batches);
                                // Ensure we check UI on first frame
                                ui_force_damage = true;
                            }
                        }
                        Err(e) => {
                            log!("[bloom] ui watch drain failed: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    log!("[bloom] ui watch open failed: {:?}", e);
                }
            }
        }
    }

    // Track held keys
    let mut keys = BTreeSet::new();
    let mut key_overlay = key_overlay::KeyOverlay::new();
    key_overlay.setup(ui_root);

    log!("[bloom] entering transactional frame loop (acquire -> build -> present)");
    log!(
        "[bloom] reclaimer: budget={} bytes",
        reclaimer::memory_budget()
    );

    // 4. Main Loop - Transactional Pattern
    let mut perf = PerfStats::default();

    loop {
        let frame_start = stem::monotonic_ns();
        let _frame_id_val = loop_ctrl.next();

        // ═══════════════════════════════════════════════════════════════════
        // ACQUIRE: Promote pending assets, snapshot generation, get token
        // ═══════════════════════════════════════════════════════════════════
        let asset_gen = ASSETS.publish_pending();
        let token = presenter.acquire_frame(frame_spec.clone(), asset_gen);
        let frame_id = token.frame_id();

        // ═══════════════════════════════════════════════════════════════════
        // BUILD: Record ops and damage into the builder
        // ═══════════════════════════════════════════════════════════════════
        let build_start = stem::monotonic_ns();
        let mut builder = FrameBuilder::new(token);

        // Snapshot the asset generation early (before any mutable borrows)
        let gen_snapshot = builder.asset_generation();

        // First frame requires full redraw
        if first_frame {
            builder.mark_full_damage();
            first_frame = false;
        }

        // Check if wallpaper asset is ready (log once)
        if !wallpaper_loaded {
            if ASSETS.get_wallpaper_for_gen(gen_snapshot).is_some() {
                wallpaper_loaded = true;
                log!(
                    "[bloom] frame {}: wallpaper now visible (gen={})",
                    frame_id,
                    gen_snapshot.0
                );
                builder.mark_full_damage();
            }
        }

        // Check if cursor asset is ready (using generation-aware getter)
        if !cursor_loaded {
            if let Some(asset) = ASSETS.get_cursor_for_gen(gen_snapshot) {
                log!(
                    "[bloom] frame {}: cursor now visible (gen={})",
                    frame_id,
                    gen_snapshot.0
                );
                cursor.set_asset(asset);
                cursor_loaded = true;
                builder.add_damage(cursor.bbox());
            }
        }
        // Check if fonts are ready via font graph OR legacy
        if !font_loaded {
            let ready_graph = font_graph::with_graph(|graph| graph.has_fonts());
            let ready_legacy = !ASSETS.get_fonts().is_empty();

            if ready_graph || ready_legacy {
                font_loaded = true;
                let mode = if ready_graph { "graph" } else { "legacy" };
                log!("[bloom] frame {}: fonts available (mode={})", frame_id, mode);
                builder.mark_full_damage();
                ui_pipeline.mark_dirty();
            }
        }

        // Mark assets as reachable (in scene graph)
        ASSETS.mark_reachable(AssetType::Wallpaper, wallpaper_loaded);
        ASSETS.mark_reachable(AssetType::Cursor, cursor_loaded);

        // Mark assets as used this frame
        if wallpaper_loaded {
            ASSETS.mark_used(AssetType::Wallpaper, frame_id);
        }
        if cursor_loaded {
            ASSETS.mark_used(AssetType::Cursor, frame_id);
        }

        // Input - capture cursor position before input
        let input_start = stem::monotonic_ns();
        let old_cursor_bbox = cursor.bbox();
        if bristle_evt != 0 {
            bristle::poll_bristle(bristle_evt, &mut cursor, &mut keys, screen_w, screen_h);
        }
        perf.input_ns += stem::monotonic_ns().saturating_sub(input_start);

        // Track cursor movement damage
        let new_cursor_bbox = cursor.bbox();
        if let Some(prev) = prev_cursor_bbox {
            if prev != new_cursor_bbox {
                // Cursor moved: damage both old and new positions
                builder.add_damage(old_cursor_bbox);
                builder.add_damage(new_cursor_bbox);
            }
        } else {
            // First frame - damage cursor area
            builder.add_damage(new_cursor_bbox);
        }
        prev_cursor_bbox = Some(new_cursor_bbox);

        // UI Watch Update
        if let Some(watch_id) = ui_watch_id {
            let mut seq: u64 = 0;
            match stem::syscall::root_watch_next(watch_id, &mut seq, &mut ui_watch_buf) {
                Ok(len) if len > 0 => {
                    // Parse ALL events in payload and mark dirty if ANY match UI_TEXT
                    let mut found_ui_text = false;
                    let mut cursor = 0usize;
                    
                    while cursor < len {
                        match abi::watch::decode_event(&ui_watch_buf[cursor..len]) {
                            Ok((header, value)) => {
                                cursor += abi::watch::WATCH_EVENT_HEADER_LEN + value.len();
                                
                                // USERSPACE FILTER: check if this event is for UI_TEXT predicate
                                if ui_text_pred_id != 0 && header.predicate.to_u32_lossy() == ui_text_pred_id {
                                    found_ui_text = true;
                                    
                                    // Rate-limited diagnostic log
                                    let now_ms = crate::log_ratelimit::now_ms();
                                    if crate::log_ratelimit::log_every(1000, now_ms) {
                                        crate::log!("[bloom][uiwatch] UI_TEXT event: seq={} subj={} pred={}", 
                                            seq, 
                                            header.subject.to_u64_lossy(),
                                            header.predicate.to_u32_lossy());
                                    }
                                }
                            }
                            Err(_) => break,
                        }
                    }
                    
                    // Only mark dirty if we found a UI_TEXT event
                    if found_ui_text {
                        ui_pipeline.mark_dirty();
                        ui_force_damage = true;
                        crate::log!("[bloom][ui] DIRTY reason=ui_text watch seq={}", seq);
                    }
                }
                Ok(_) => {}
                Err(abi::errors::Errno::EAGAIN) => {}
                Err(abi::errors::Errno::EOVERFLOW) => {
                    ui_pipeline.mark_dirty();
                    ui_force_damage = true;
                }
                Err(e) => {
                    log!("[bloom] ui watch error: {:?}", e);
                }
            }
        }
        if frame_start >= ui_poll_deadline_ns {
            ui_pipeline.mark_dirty();
            ui_force_damage = true;
            ui_poll_deadline_ns = frame_start.saturating_add(1_000_000_000);
        }

        // Key Overlay Update
        if key_overlay.update(&keys, screen_w, screen_h) {
            ui_pipeline.mark_dirty();
        }

        // Damage text regions (frame counter changes every frame)
        if font_loaded {
            builder.add_damage(Rect::new(20, 20, 300, 100)); // Covers "thing-os" and "frame: N"
        }

        // Build Scene - record ops into the builder's DrawList
        let ui_changed;
        let ui_damage;
        {
            let list = builder.ops();

            // Background / Wallpaper (use generation-aware getter)
            if let Some(clouds) = ASSETS.get_wallpaper_for_gen(gen_snapshot) {
                let cw = clouds.width as i32;
                let ch = clouds.height as i32;
                for y in (0..screen_h).step_by(ch as usize) {
                    for x in (0..screen_w).step_by(cw as usize) {
                        list.blit_image(&clouds, x, y);
                    }
                }
            } else {
                // Aesthetic fallback: deep "Thing-OS" blue
                list.clear(geometry::Color::from_u32(0xFF002d44));
            }

            // Backend indicator: small box in top-right corner
            let indicator_size = 24;
            let indicator_x = screen_w - indicator_size - 8;
            let indicator_y = 8;
            list.rect(
                indicator_x,
                indicator_y,
                indicator_size,
                indicator_size,
                backend_indicator_color,
            );

            // Demo text rendering if font is loaded
            if font_loaded {
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

            // Run UI Pipeline (UNIFEROUS - runs regardless of font_loaded now)
            let ui_start = stem::monotonic_ns();
            let ui_result = ui_pipeline.run(screen_w, screen_h, list, &ASSETS);
            ui_changed = ui_result.changed;
            ui_damage = ui_result.damage;
            perf.ui_ns += stem::monotonic_ns().saturating_sub(ui_start);

            // Cursor
            cursor.emit_drawlist(list);
        }

        if ui_changed || ui_force_damage {
            if ui_damage.is_empty() {
                builder.mark_full_damage();
            } else {
                for rect in ui_damage {
                    builder.add_damage(rect);
                }
            }
            ui_force_damage = false;
        }

        // Finish building - seal the token
        let token = builder.finish();
        perf.build_ns += stem::monotonic_ns().saturating_sub(build_start);

        // Get damage reference before consuming token
        let damage_for_raster = token.damage.clone();

        // ═══════════════════════════════════════════════════════════════════
        // PRESENT: Rasterize with damage, present to display
        // ═══════════════════════════════════════════════════════════════════

        // Rate-limited damage logging
        let now_ms = crate::log_ratelimit::now_ms();
        if crate::log_ratelimit::log_every(1000, now_ms) {
            crate::log!("[bloom][damage] rects={} frame={}", damage_for_raster.rect_count(), frame_id);
        }
        
        // Rasterize using damage-aware rendering
        if !damage_for_raster.is_empty() {
            let raster_start = stem::monotonic_ns();
            raster::execute_with_damage(&mut surface, &token.ops, &damage_for_raster);
            perf.raster_ns += stem::monotonic_ns().saturating_sub(raster_start);
        }

        // Present (consumes token)
        let present_start = stem::monotonic_ns();
        let _stats = presenter.present_frame(token);
        presenter.pump();
        perf.present_ns += stem::monotonic_ns().saturating_sub(present_start);

        // Post-present overlay update
        key_overlay.post_present();

        // ═══════════════════════════════════════════════════════════════════
        // POST-PRESENT: Memory pressure check
        // ═══════════════════════════════════════════════════════════════════
        reclaimer::check_memory_pressure(&ASSETS);

        // Timing
        loop_ctrl.heartbeat(cursor.x, cursor.y);

        perf.count += 1;
        perf.frame_ns += stem::monotonic_ns().saturating_sub(frame_start);

        if perf.count >= 120 {
            log!("[bloom] PERF: 120 frames avg: total={:.2}ms build={:.2}ms (ui={:.2}ms) raster={:.2}ms present={:.2}ms input={:.2}ms",
                (perf.frame_ns as f64 / 120.0) / 1_000_000.0,
                (perf.build_ns as f64 / 120.0) / 1_000_000.0,
                (perf.ui_ns as f64 / 120.0) / 1_000_000.0,
                (perf.raster_ns as f64 / 120.0) / 1_000_000.0,
                (perf.present_ns as f64 / 120.0) / 1_000_000.0,
                (perf.input_ns as f64 / 120.0) / 1_000_000.0,
            );
            perf = PerfStats::default();
        }

        loop_ctrl.sleep();
    }
}

#[derive(Default)]
struct PerfStats {
    count: u64,
    frame_ns: u64,
    input_ns: u64,
    ui_ns: u64,
    build_ns: u64,
    raster_ns: u64,
    present_ns: u64,
}
