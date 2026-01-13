#![no_std]
#![no_main]

extern crate alloc;

mod asset;
mod bmp;
mod bristle;
mod compositor;
mod cursor;
mod drawlist;
mod frame_loop;
mod logging;
mod present;
mod raster;

mod surface;

use abi::display_driver_protocol::BindPayload;
use stem::syscall::PortHandle;

use crate::compositor::CompositorTarget;
use crate::cursor::CursorState;
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
            log!("[wallpaper_loader] published to asset bank");
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

    // Note: limine.conf uses /assets/cursors/plain/Normal.cur
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
            log!("[cursor_loader] published to asset bank");
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

    // 1. Discovery & Mapping
    log!("[bloom] discovering compositor target...");
    let target = match CompositorTarget::discover_and_map((arg_req, arg_resp), 2000) {
        Ok(t) => {
            log!("[bloom] compositor target: {}x{} @ {:p}", t.width, t.height, t.ptr);
            t
        },
        Err(e) => {
            log!("[bloom] ERROR: compositor discovery failed: {:?}", e);
            loop { stem::sleep_ms(1000); }
        }
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
    let mut frame_count: u64 = 0;

    log!("[bloom] entering frame loop");

    // 4. Main Loop
    loop {
        let _frame = loop_ctrl.next();
        frame_count += 1;

        // Check if wallpaper asset is ready (log once)
        if !wallpaper_loaded {
            if ASSETS.get_wallpaper().is_some() {
                wallpaper_loaded = true;
                log!("[bloom] frame {}: wallpaper now available", frame_count);
            }
        }

        // Check if cursor asset is ready (only check every 30 frames to reduce noise)
        if !cursor_loaded {
            if let Some(asset) = ASSETS.get_cursor() {
                log!("[bloom] frame {}: GOT cursor asset, applying to CursorState", frame_count);
                cursor.set_asset(asset);
                cursor_loaded = true;
                log!("[bloom] frame {}: cursor asset applied successfully", frame_count);
            } else if frame_count % 60 == 0 {
                log!("[bloom] frame {}: cursor not ready yet", frame_count);
            }
        }

        // Input
        if bristle_evt != 0 {
            bristle::poll_bristle(bristle_evt, &mut cursor, target.width as i32, target.height as i32);
        }

        // Build Scene
        let mut list = drawlist::DrawList::new();
        
        // Background / Wallpaper
        if let Some(clouds) = ASSETS.get_wallpaper() {
             let cw = clouds.width as i32;
             let ch = clouds.height as i32;
             for y in (0..target.height as i32).step_by(ch as usize) {
                 for x in (0..target.width as i32).step_by(cw as usize) {
                     list.blit_image(&clouds, x, y);
                 }
             }
        } else {
             list.clear(0x00101010);
             list.rect(10, 10, 20, 20, 0xFF00FF00);
        }

        // Cursor
        cursor.emit_drawlist(&mut list);

        // Rasterize
        raster::execute(&mut surface, &list);

        // Present
        presenter.present();
        presenter.pump();

        // Timing
        loop_ctrl.heartbeat(cursor.x, cursor.y);
        loop_ctrl.sleep();
    }
}
