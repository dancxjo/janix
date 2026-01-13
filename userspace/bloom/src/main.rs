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
    
    stem::sleep_ms(500); 

    let candidates = [
        "/assets/wallpapers/clouds.bmp",
        "wallpapers/clouds.bmp",
        "clouds.bmp",
    ];
    
    for path in candidates.iter() {
        if let Some(img) = ASSETS.load_wallpaper_from_graph(path) {
            log!("loader: loaded clouds ({}x{})", img.width, img.height);
            ASSETS.publish_image(img);
            break;
        }
    }
    
    log!("loader: done");
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
