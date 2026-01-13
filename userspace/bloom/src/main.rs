#![no_std]
#![no_main]

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
mod logging;
mod lowered;
mod present;
mod raster;
mod surface;
mod target;
mod target_cpu;

use abi::display_driver_protocol::BindPayload;
use stem::syscall::PortHandle;

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

static ASSETS: AssetBank = AssetBank::new();

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

/// Background thread for loading fonts
extern "C" fn font_loader_entry() -> ! {
    log!("[font_loader] thread started");
    
    stem::sleep_ms(400); 
    log!("[font_loader] searching for font...");

    let candidates = [
        "/assets/fonts/Hack-Regular.ttf",
        "/assets/fonts/NotoSans-Regular.ttf",
        "fonts/Hack-Regular.ttf",
        "Hack-Regular.ttf",
    ];

    for path in candidates.iter() {
        log!("[font_loader] trying: {}", path);
        if let Some(font) = AssetBank::load_font_from_graph(path) {
            log!("[font_loader] SUCCESS: loaded font '{}'", font.name);
            ASSETS.publish_font(font);
            log!("[font_loader] published to pending");
            break;
        } else {
            log!("[font_loader] not found: {}", path);
        }
    }
    
    log!("[font_loader] thread done, sleeping forever");
    loop {
        stem::syscall::sleep_ms(10000);
    }
}


#[stem::main]
fn main(arg: usize) -> ! {
    logging::init();

    let arg_req = unpack_handle(arg, 0);
    let arg_resp = unpack_handle(arg, 1);
    let bristle_evt = unpack_handle(arg, 2);

    log!("[bloom] starting (arg_req={} arg_resp={} bristle={})", arg_req, arg_resp, bristle_evt);

    // Spawn wallpaper loader thread
    if let Err(e) = stem::thread::spawn(wallpaper_loader_entry) {
        log!("[bloom] ERROR: failed to spawn wallpaper loader: {:?}", e);
    } else {
        log!("[bloom] spawned wallpaper_loader thread");
    }

    // Spawn cursor loader thread
    if let Err(e) = stem::thread::spawn(cursor_loader_entry) {
        log!("[bloom] ERROR: failed to spawn cursor loader: {:?}", e);
    } else {
        log!("[bloom] spawned cursor_loader thread");
    }

    // Spawn font loader thread
    if let Err(e) = stem::thread::spawn(font_loader_entry) {
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
        DisplayBackend::VirtioGpu => 0xFF00FF00, // Bright green
        DisplayBackend::BootFB => 0xFFFF0000,    // Bright red
        DisplayBackend::Unknown => 0xFFFFFF00,   // Yellow for unknown
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

    log!("[bloom] entering transactional frame loop (acquire -> build -> present)");

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

        // Check if font asset is ready (log once)
        if !font_loaded {
            if ASSETS.get_font_for_gen(gen_snapshot).is_some() {
                font_loaded = true;
                log!("[bloom] frame {}: font now visible (gen={})", frame_id, gen_snapshot.0);
                builder.mark_full_damage();
            }
        }

        // Input - capture cursor position before input
        let old_cursor_bbox = cursor.bbox();
        if bristle_evt != 0 {
            bristle::poll_bristle(bristle_evt, &mut cursor, screen_w, screen_h);
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
                 list.clear(0x00101010);
                 list.rect(10, 10, 20, 20, 0xFF00FF00);
            }

            // Backend indicator: small box in top-right corner
            let indicator_size = 24;
            let indicator_x = screen_w - indicator_size - 8;
            let indicator_y = 8;
            list.rect(indicator_x, indicator_y, indicator_size, indicator_size, backend_indicator_color);

            // Demo text rendering if font is loaded
            if font_loaded {
                list.text("thing-os", 20, 20, 24.0, 0xFFFFFF);
                list.text(&alloc::format!("frame: {}", frame_id), 20, 50, 16.0, 0xCCCCCC);
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

        // Timing
        loop_ctrl.heartbeat(cursor.x, cursor.y);
        loop_ctrl.sleep();
    }
}
