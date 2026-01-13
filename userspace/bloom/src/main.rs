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

extern "C" fn loader_entry() -> ! {
    log!("loader: started");
    
    // Simulate decode delay
    stem::sleep_ms(1000); 

    // Get write access to the pre-allocated buffer
    // Safety: ASSETS initialized in main.
    if let Some(buffer) = unsafe { ASSETS.get_wallpaper_write_access() } {
         let w = 640;
         let h = 480;
         // Ensure buffer is large enough (should be, we init with 640x480)
         // Generate pattern (No allocation!)
         for y in 0..h {
            let r = (y as f32 / h as f32 * 255.0) as u32;
            for x in 0..w {
                let idx = y as usize * w as usize + x as usize;
                if idx < buffer.len() {
                    let b = (x as f32 / w as f32 * 255.0) as u32;
                    buffer[idx] = 0xFF000000 | (r << 16) | b;
                }
            }
        }
        ASSETS.publish_clouds();
        log!("loader: published clouds (generated {}x{})", w, h);
    } else {
        log!("loader: error - no buffer access");
    }
    
    log!("loader: done, sleeping");
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

    log!("starting (arg_req={} arg_resp={} bristle={})", arg_req, arg_resp, bristle_evt);

    // 0. Init Assets & Spawn Loader
    ASSETS.init_wallpaper_buffer(640, 480);

    if let Err(e) = stem::thread::spawn(loader_entry) {
        log!("error: failed to spawn loader: {:?}", e);
    } else {
        log!("loader: thread spawned");
    }

    // 1. Discovery & Mapping
    let target = match CompositorTarget::discover_and_map((arg_req, arg_resp), 2000) {
        Ok(t) => t,
        Err(e) => {
            log!("error: compositor discovery failed: {:?}", e);
            loop { stem::sleep_ms(1000); }
        }
    };

    // 2. Presenter Setup
    let mut presenter = if target.driver_req != 0 && target.driver_resp != 0 {
        log!("presenter: driver (req={} resp={})", target.driver_req, target.driver_resp);
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
        log!("presenter: null (headless/fallback)");
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


    log!("entering frame loop");

    // 4. Main Loop
    loop {
        let _frame = loop_ctrl.next();

        // Input
        if bristle_evt != 0 {
            bristle::poll_bristle(bristle_evt, &mut cursor, target.width as i32, target.height as i32);
        }

        // Build Scene

        let mut list = drawlist::DrawList::new();
        
        // Background / Wallpaper
        if let Some(clouds) = ASSETS.get_clouds() {
             // Tile the clouds
             let cw = clouds.width as i32;
             let ch = clouds.height as i32;
             // Simple tile logic
             for y in (0..target.height as i32).step_by(ch as usize) {
                 for x in (0..target.width as i32).step_by(cw as usize) {
                     list.blit_image(&clouds, x, y);
                 }
             }
        } else {
             // Fallback
             list.clear(0x00101010); // Dark Gray
             // Loading indicator?
             list.rect(10, 10, 20, 20, 0xFF00FF00); // Tiny loading green dot
        }

        // Centered Rect (App Window)
        let rw = 200;
        let rh = 150;
        let rx = (target.width as i32 - rw) / 2;
        let ry = (target.height as i32 - rh) / 2;
        list.rect(rx, ry, rw, rh, 0x00306090); // Nice Blue

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
