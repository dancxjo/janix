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
pub mod key_overlay;
pub mod geometry; // Canonical geometry types
mod isa;      // Portable Render ISA types
mod logging;
mod lowered;
mod present;
mod raster;
mod reclaimer;
mod surface;
mod target;
mod target_cpu;

use abi::display_driver_protocol::BindPayload;
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
use alloc::collections::BTreeSet;

// use stem::stack::{Stack, StackSpec};

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
        if let Some(img) = ASSETS.load_wallpaper_from_graph(path) {
            log!("[wallpaper_loader] SUCCESS: loaded ({}x{})", img.width, img.height);
            ASSETS.publish_wallpaper(img);
            log!("[wallpaper_loader] published to pending");
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
    
    use abi::types::{WatchSpec, WatchEvent, WatchMode};
    use abi::query::{QueryStep, QueryOpKind};
    use abi::symbols::{SymbolRefWire, SYMBOL_REF_TAG_STR};
    use stem::syscall;
    use stem::thing::sys::{describe_thing, bytespace_info, prop_get};
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
    };
    
    let watch_id = match syscall::root_watch_open(&spec) {
        Ok(id) => {
            log!("[font_loader] watch opened (id={})", id);
            id
        }
        Err(e) => {
            log!("[font_loader] ERROR: watch open failed: {:?}", e);
            loop { stem::sleep_ms(10000); }
        }
    };

    let mut evt = WatchEvent::default();
    loop {
        match syscall::root_watch_next(watch_id, &mut evt) {
            Ok(1) => {
                let node_id = ThingId(evt.node_id);
                let mut buf = [0u8; 512];
                if let Ok(len) = describe_thing(node_id, &mut buf) {
                    let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
                    if desc.contains("name: \"") && (desc.contains(".ttf\"") || desc.contains(".otf\"") || desc.contains(".ttc\"")) {
                        let bs_id = prop_get(node_id, "bytespace").map(ThingId).ok();
                        let size = bs_id.and_then(|id| bytespace_info(id).ok());
                        
                        if let (Some(bs), Some(sz)) = (bs_id, size) {
                            if let Some(font) = AssetBank::load_font_from_node_id(bs, sz, desc) {
                                log!("[font_loader] SUCCESS: loaded '{}'", font.name);
                                ASSETS.publish_font(font);
                            }
                        }
                    }
                }
            }
            Ok(0) => {
                // Yield and wait
                stem::sleep_ms(100);
            }
            Err(e) => {
                log!("[font_loader] watch next error: {:?}", e);
                stem::sleep_ms(500);
            }
            _ => { stem::sleep_ms(100); }
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
        if let Some(cursor) = AssetBank::load_cursor_from_graph(path) {
            log!("[cursor_loader] SUCCESS: loaded cursor asset");
            ASSETS.publish_cursor(cursor);
            log!("[cursor_loader] published to pending");
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
    let bs_id = ThingId(arg_val as u64);

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

    log!("[bloom] starting (arg_req={} arg_resp={} bristle={} font_svc={})", arg_req, arg_resp, bristle_evt, svc_font_id);

    // Font Client is no longer used, as we load fonts locally now.

    use stem::stack::{Stack, StackSpec};

    // Spawn wallpaper loader thread
    let wallpaper_stack = Stack::alloc_growing_stack(StackSpec {
        reserve_bytes: 256 * 1024,
        initial_commit_bytes: 64 * 1024,
        ..StackSpec::default()
    }).expect("wallpaper stack");
    if let Err(e) = stem::thread::spawn_on(wallpaper_stack, wallpaper_loader_entry) {
        log!("[bloom] ERROR: failed to spawn wallpaper loader: {:?}", e);
    } else {
        log!("[bloom] spawned wallpaper_loader thread");
    }

    // Spawn cursor loader thread
    let cursor_stack = Stack::alloc_growing_stack(StackSpec {
        reserve_bytes: 256 * 1024,
        initial_commit_bytes: 64 * 1024,
        ..StackSpec::default()
    }).expect("cursor stack");
    if let Err(e) = stem::thread::spawn_on(cursor_stack, cursor_loader_entry) {
        log!("[bloom] ERROR: failed to spawn cursor loader: {:?}", e);
    } else {
        log!("[bloom] spawned cursor_loader thread");
    }

    // Spawn font loader thread
    let font_stack = Stack::alloc_growing_stack(StackSpec {
        reserve_bytes: 256 * 1024,
        initial_commit_bytes: 64 * 1024,
        ..StackSpec::default()
    }).expect("font stack");
    if let Err(e) = stem::thread::spawn_on(font_stack, font_loader_entry) {
        log!("[bloom] ERROR: failed to spawn font loader: {:?}", e);
    } else {
        log!("[bloom] spawned font_loader thread");
    }

    // 1. Discovery & Mapping
    log!("[bloom] discovering compositor target...");
    let target = match CompositorTarget::discover_and_map((arg_req, arg_resp), 2000) {
        Ok(t) => {
            log!("[bloom] compositor target: {}x{} @ {:p} backend={}", 
                t.width, t.height, t.ptr, t.backend.name());

            t
        },
        Err(e) => {
            log!("[bloom] ERROR: compositor discovery failed: {:?}", e);
            loop { stem::sleep_ms(1000); }
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
        log!("[bloom] presenter: driver (req={} resp={})", target.driver_req, target.driver_resp);
        let mut driver = DriverPresenter::new(target.driver_req, target.driver_resp);
        driver.wait_for_register();
        
        let bind = BindPayload {
            bytespace_id: target.bs_id.0,
            width: target.width,
            height: target.height,
            stride: target.stride_bytes,
            format: target.format,
        };
        driver.send_bind(&bind);
        driver.wait_for_bind();
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
            target.stride_bytes
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

    // Track held keys
    let mut keys = BTreeSet::new();
    let mut key_overlay = key_overlay::KeyOverlay::new();

    log!("[bloom] entering transactional frame loop (acquire -> build -> present)");
    log!("[bloom] reclaimer: budget={} bytes", reclaimer::memory_budget());

    // 4. Main Loop - Transactional Pattern
    loop {
        let _frame = loop_ctrl.next();

        // ═══════════════════════════════════════════════════════════════════
        // ACQUIRE: Promote pending assets, snapshot generation, get token
        // ═══════════════════════════════════════════════════════════════════
        let asset_gen = ASSETS.publish_pending();
        let token = presenter.acquire_frame(frame_spec.clone(), asset_gen);
        let frame_id = token.frame_id();

        // ═══════════════════════════════════════════════════════════════════
        // BUILD: Record ops and damage into the builder
        // ═══════════════════════════════════════════════════════════════════
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
                log!("[bloom] frame {}: wallpaper now visible (gen={})", frame_id, gen_snapshot.0);
                builder.mark_full_damage();
            }
        }

        // Check if cursor asset is ready (using generation-aware getter)
        if !cursor_loaded {
            if let Some(asset) = ASSETS.get_cursor_for_gen(gen_snapshot) {
                log!("[bloom] frame {}: cursor now visible (gen={})", frame_id, gen_snapshot.0);
                cursor.set_asset(asset);
                cursor_loaded = true;
                builder.add_damage(cursor.bbox());
            } else if frame_id % 60 == 0 {
                log!("[bloom] frame {}: cursor not ready yet", frame_id);
            }
        }
        // Check if fonts are ready
        if !font_loaded {
            if ASSETS.get_font_for_gen(gen_snapshot).is_some() {
                font_loaded = true;
                log!("[bloom] frame {}: fonts available", frame_id);
                builder.mark_full_damage();
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
        let old_cursor_bbox = cursor.bbox();
        if bristle_evt != 0 {
            bristle::poll_bristle(bristle_evt, &mut cursor, &mut keys, screen_w, screen_h);
        }
        
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
        if key_overlay.update(&keys, frame_id) {
             if let Some(rect) = key_overlay.damage_rect(screen_w, screen_h) {
                 builder.add_damage(rect);
             }
        }

        // Damage text regions (frame counter changes every frame)
        if font_loaded {
            builder.add_damage(Rect::new(20, 20, 200, 25)); // "thing-os"
            builder.add_damage(Rect::new(20, 45, 150, 25)); // "frame: N"
        }

        // Build Scene - record ops into the builder's DrawList
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
                 list.clear(geometry::Color::from_u32(0x00101010));
                 list.rect(10, 10, 20, 20, geometry::Color::from_u32(0xFF00FF00));
            }

            // Backend indicator: small box in top-right corner
            let indicator_size = 24;
            let indicator_x = screen_w - indicator_size - 8;
            let indicator_y = 8;
            list.rect(indicator_x, indicator_y, indicator_size, indicator_size, backend_indicator_color);

            // Demo text rendering if font is loaded
            if font_loaded {
                list.text("thing-os", 20, 20, 24.0, geometry::Color::from_u32(0xFFFFFF));
                list.text(&alloc::format!("frame: {}", frame_id), 20, 50, 16.0, geometry::Color::from_u32(0xCCCCCC));
                
                // Render Key Overlay
                key_overlay.render(list, screen_w, screen_h);
            }

            // Cursor
            cursor.emit_drawlist(list);
        }

        // Finish building - seal the token
        let token = builder.finish();
        
        // Get damage reference before consuming token
        let damage_for_raster = token.damage.clone();

        // ═══════════════════════════════════════════════════════════════════
        // PRESENT: Rasterize with damage, present to display
        // ═══════════════════════════════════════════════════════════════════
        
        // Rasterize using damage-aware rendering
        raster::execute_with_damage(&mut surface, &token.ops, &damage_for_raster);

        // Present (consumes token)
        let _stats = presenter.present_frame(token);
        presenter.pump();
        
        // Post-present overlay update
        key_overlay.post_present();

        // ═══════════════════════════════════════════════════════════════════
        // POST-PRESENT: Memory pressure check
        // ═══════════════════════════════════════════════════════════════════
        reclaimer::check_memory_pressure(&ASSETS);

        // Timing
        loop_ctrl.heartbeat(cursor.x, cursor.y);
        loop_ctrl.sleep();
    }
}
