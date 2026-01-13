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
    // Load Clouds
    if let Some(img) = ASSETS.load_wallpaper_from_graph("wallpapers/clouds.bmp") {
         let w = img.width;
         let h = img.height;
         log!("loader: loaded clouds ({}x{})", w, h);
         
         // Publish (AssetBank publishes the Image directly if we change it? No, AssetBank::publish_clouds expects us to write to the buffer)
         // Wait, AssetBank is designed for "Init-Write-Publish".
         // load_wallpaper_from_graph returns an Image struct (Arc<[u32]>).
         // But ASSETS.pixels is the shared buffer.
         // We should probably just Update ASSETS to accept an Image?
         // Or copy the loaded image into the buffer?
         
         if let Some(buffer) = unsafe { ASSETS.get_wallpaper_write_access() } {
             // Copy logic
             // CAUTION: Buffer size (640x480) vs Image size?
             // If cloud is bigger or smaller?
             // For now, let's assume we copy what fits or resize?
             // Simplest: just copy row by row.
             
             // Check sizes
             let buf_len = buffer.len();
             let copy_w = w.min(640);
             let copy_h = h.min(480);
             
             for y in 0..copy_h {
                 let src_row = y as usize * w as usize;
                 let dst_row = y as usize * 640;
                 let len = copy_w as usize;
                 
                 if src_row + len <= img.pixels.len() && dst_row + len <= buf_len {
                      buffer[dst_row..dst_row+len].copy_from_slice(&img.pixels[src_row..src_row+len]);
                 }
             }
             ASSETS.publish_clouds();
         }
    } else {
        log!("loader: error - failed to load wallpaper");
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

