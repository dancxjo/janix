#![no_std]
#![no_main]

extern crate alloc;

mod bristle;
mod compositor;
mod cursor;
mod drawlist;
mod frame_loop;
mod logging;
mod present;
mod raster;
mod scene;
mod surface;

use abi::display_driver_protocol::BindPayload;
use stem::syscall::PortHandle;

use crate::compositor::CompositorTarget;
use crate::cursor::CursorState;
use crate::frame_loop::FrameLoop;
use crate::present::{DriverPresenter, PresenterImpl};
use crate::scene::{Fb32, draw};

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

#[stem::main]
fn main(arg: usize) -> ! {
    logging::init();

    let arg_req = unpack_handle(arg, 0);
    let arg_resp = unpack_handle(arg, 1);
    let bristle_evt = unpack_handle(arg, 2);

    log!("starting (arg_req={} arg_resp={} bristle={})", arg_req, arg_resp, bristle_evt);

    // 1. Discovery & Mapping
    // Wait up to 2 seconds for a valid compositor bytespace
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
        // Note: We ignore the ACK for proof-of-life simplicity, as DriverPresenter handles state inside
        PresenterImpl::Driver(driver)
    } else {
        log!("presenter: null (headless/fallback)");
        PresenterImpl::Null(present::NullPresenter)
    };

    // 3. State Initialization
    let mut cursor = CursorState::new((target.width as i32) / 2, (target.height as i32) / 2);
    let mut loop_ctrl = FrameLoop::new(60);

    log!("entering frame loop");

    // 4. Main Loop
    loop {
        let frame = loop_ctrl.next();

        // Input
        if bristle_evt != 0 {
            bristle::poll_bristle(bristle_evt, &mut cursor, target.width as i32, target.height as i32);
        }

        // Draw
        // Create a safe slice from the raw pointer
        // SAFETY: Mapped pointer is valid for `size_bytes`.
        let pixels_len = target.size_bytes / 4;
        let pixels = unsafe { 
            core::slice::from_raw_parts_mut(target.ptr as *mut u32, pixels_len) 
        };

        if let Ok(mut fb) = Fb32::new(pixels, target.width as usize, target.height as usize, target.stride_bytes as usize / 4) {
             draw(&mut fb, frame, &cursor);
        } else {
             // This should ideally never happen after successful discovery
             log!("error: fb32 creation failed");
        }

        // Present
        presenter.present();
        presenter.pump();

        // Timing & Diagnostics
        loop_ctrl.heartbeat(cursor.x, cursor.y);
        loop_ctrl.sleep();
    }
}
