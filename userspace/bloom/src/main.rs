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
mod lowered;
mod present;
mod raster;
mod reclaimer;
mod surface;
// mod target;
// mod target_cpu;
pub mod ui;

use abi::display_driver_protocol::BindPayload;
use abi::ids::HandleId;
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
extern "C" fn font_loader_entry() -> ! {
    log!("[font_loader] thread started");

    use abi::query::{QueryOpKind, QueryStep};
    use abi::symbols::{SymbolRefWire, SYMBOL_REF_TAG_STR};
    use abi::types::{WatchEvent, WatchMode, WatchSpec};
    use stem::syscall;
    use stem::thing::sys::{bytespace_info, describe_thing, prop_get};
    use stem::thing::ThingId;

    // Phase 1: Scan for boot.Modules that look like fonts
    let kind_str = "boot.Module";
    let symbol = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: kind_str.as_ptr() as u64,
        len: kind_str.len() as u64,
    };

    let steps = [QueryStep {
        op: QueryOpKind::Scan as u64,
        arg1: 256, // limit
        arg2: 0,
        symbol,
    }];

    let spec = WatchSpec {
        mode: WatchMode::QueryThenStream as u32,
        query_ptr: steps.as_ptr() as u64,
        query_len: steps.len() as u64,
        start_seq: 0,
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

    let mut seq_out = 0u64;
    let mut watch_buf = [0u8; 4096];
    loop {
        match syscall::root_watch_next(watch_id, &mut seq_out, &mut watch_buf) {
            Ok(len) if len > 0 => {
                // Parse WatchEvent from the returned batch payload
                // For now just extract node_id from the THRT batch format
                if len >= 8 + 1 + 16 {
                    // Skip 8-byte header + 1-byte op tag, read 16-byte kind and node_id
                    // The actual batch payload format: THRT header (8) + op tag (1) + kind symbol (16) + result_id (8)
                    // Actually the result_id comes from reply, let's check commit data
                    // For CreateNode: the node_id needs to be extracted properly
                    // But root_watch delivers raw batch payloads, we need to get node from events
                }
                
                // For now, parse as WatchEvent if kernel populated it that way
                // This path needs better batch parsing, but the syscall fix is the key change
                if len >= core::mem::size_of::<abi::types::WatchEvent>() {
                    let evt: abi::types::WatchEvent = unsafe { 
                        core::ptr::read_unaligned(watch_buf.as_ptr() as *const _) 
                    };
                    let node_id = ThingId::from_u64(evt.node_id);
                    let mut buf = [0u8; 512];
                    if let Ok(desc_len) = describe_thing(node_id, &mut buf) {
                        let desc = core::str::from_utf8(&buf[..desc_len]).unwrap_or("");
                        if desc.contains("name: \"")
                            && (desc.contains(".ttf\"")
                                || desc.contains(".otf\"")
                                || desc.contains(".ttc\""))
                        {
                            log!("[font_loader] found font candidate: '{}'", desc);
                            let bs_id = prop_get(node_id, "bytespace").map(ThingId::from_u64).ok();
                            let size = bs_id.and_then(|id| bytespace_info(id).ok());

                            if let (Some(bs), Some(sz)) = (bs_id, size) {
                                log!(
                                    "[font_loader] enqueuing font load: bs={} size={} name='{}'",
                                    bs.to_u64_lossy(),
                                    sz,
                                    desc
                                );
                                ASSETS.enqueue_font_load(bs, sz, desc);
                            } else {
                                log!(
                                    "[font_loader] WARN: could not get bytespace/size for font '{}'",
                                    desc
                                );
                            }
                        }
                    }
                }
            }
            Ok(0) => {
                // No data yet
                stem::sleep_ms(100);
            }
            Err(e) => {
                log!("[font_loader] watch next error: {:?}", e);
                stem::sleep_ms(500);
            }
            _ => {
                stem::sleep_ms(100);
            }
        }
    }
}

/// Background thread for loading cursor
extern "C" fn cursor_loader_entry() -> ! {
    log!("[cursor_loader] thread started");

    stem::sleep_ms(300);
    log!("[cursor_loader] searching for cursor...");

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

    let screen_w = target.width as i32;
    let screen_h = target.height as i32;
    let frame_spec = FrameSpec::new(target.width, target.height, target.format);

    // Track previous cursor position for damage
    let mut prev_cursor_bbox: Option<Rect> = None;
    let mut first_frame = true;

    // 4. UI Pipeline Setup
    let mut ui_pipeline = ui::UiPipeline::new();
    let ui_root = stem::ui::UiBuilder::create_root();
    ui_pipeline.set_root(ui_root);

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
        // Check if fonts are ready
        if !font_loaded {
            if ASSETS.get_font_for_gen(gen_snapshot).is_some() {
                font_loaded = true;
                log!("[bloom] frame {}: fonts available", frame_id);
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

        // Key Overlay Update
        if key_overlay.update(&keys, screen_w, screen_h) {
            ui_pipeline.mark_dirty();
        }

        // Damage text regions (frame counter changes every frame)
        if font_loaded {
            builder.add_damage(Rect::new(20, 20, 300, 100)); // Covers "thing-os" and "frame: N"
        }

        // Build Scene - record ops into the builder's DrawList
        let mut ui_changed = false;
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

                // Run UI Pipeline
                let ui_start = stem::monotonic_ns();
                ui_changed = ui_pipeline.run(screen_w, screen_h, list, &ASSETS);
                perf.ui_ns += stem::monotonic_ns().saturating_sub(ui_start);
            }

            // Cursor
            cursor.emit_drawlist(list);
        }

        if ui_changed {
            builder.mark_full_damage();
        }

        // Finish building - seal the token
        let token = builder.finish();
        perf.build_ns += stem::monotonic_ns().saturating_sub(build_start);

        // Get damage reference before consuming token
        let damage_for_raster = token.damage.clone();

        // ═══════════════════════════════════════════════════════════════════
        // PRESENT: Rasterize with damage, present to display
        // ═══════════════════════════════════════════════════════════════════

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
