#![no_std]
#![no_main]

extern crate alloc;

mod bristle;
mod cursor;
mod drawlist;
mod present;
mod raster;
mod surface;

use abi::display_driver_protocol::BindPayload;
use abi::display_protocol::FORMAT_XRGB8888;
use abi::schema::{keys, kinds};
use stem::info;
use stem::syscall::PortHandle;
use stem::thing::sys as thingsys;
use stem::thing::ThingId;

use crate::cursor::CursorState;
use crate::drawlist::DrawList;
use crate::present::{DriverPresenter, PresenterImpl};
use crate::surface::Surface;

const BACKGROUND_COLOR: u32 = 0x00101010;
const RECT_COLOR: u32 = 0x00306090;

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn find_compositor_bytespace() -> Option<(ThingId, u32, u32, u32, u32)> {
    let role_sym = thingsys::intern("display.compositor").ok()? as u64;
    let mut buf = [ThingId(0); 16];
    let count = thingsys::find(kinds::BYTESPACE, &mut buf).ok()?;
    for id in buf.iter().take(count) {
        let role = thingsys::prop_get(*id, "display_role").unwrap_or(0);
        if role != role_sym {
            continue;
        }
        let width = thingsys::prop_get(*id, keys::WIDTH).unwrap_or(0) as u32;
        let height = thingsys::prop_get(*id, keys::HEIGHT).unwrap_or(0) as u32;
        let stride = thingsys::prop_get(*id, keys::STRIDE).unwrap_or(0) as u32;
        let format = thingsys::prop_get(*id, keys::FORMAT).unwrap_or(0) as u32;
        return Some((*id, width, height, stride, format));
    }
    None
}

fn build_scene(list: &mut DrawList, width: i32, height: i32, cursor: &CursorState) {
    list.clear(BACKGROUND_COLOR);

    let rect_w = (width / 3).max(1);
    let rect_h = (height / 3).max(1);
    let rect_x = (width - rect_w) / 2;
    let rect_y = (height - rect_h) / 2;
    list.rect(rect_x, rect_y, rect_w, rect_h, RECT_COLOR);

    cursor.emit_drawlist(list);
}

#[stem::main]
fn main(arg: usize) -> ! {
    let drv_req_write = unpack_handle(arg, 0);
    let drv_resp_read = unpack_handle(arg, 1);
    let bristle_evt_read = unpack_handle(arg, 2);

    info!(
        "bloom: arg=0x{:x} handles req_w={} resp_r={} bristle_r={}",
        arg, drv_req_write, drv_resp_read, bristle_evt_read
    );
    if bristle_evt_read == 0 {
        info!("bloom: bristle event handle is 0; input may be unavailable");
    }

    let (bs_id, width, height, stride, format) = loop {
        if let Some(found) = find_compositor_bytespace() {
            break found;
        }
        stem::yield_now();
        stem::sleep_ms(50);
    };

    info!(
        "bloom: compositor bytespace {} ({}x{} stride={} format={})",
        bs_id.0, width, height, stride, format
    );

    if format != FORMAT_XRGB8888 {
        info!("bloom: unexpected format {}, continuing anyway", format);
    }

    let fallback_size = height as usize * stride as usize;
    let info_size = thingsys::bytespace_info(bs_id).unwrap_or(0);
    let size = if info_size == 0 { fallback_size } else { info_size };
    if info_size == 0 {
        info!("bloom: bytespace_info returned 0; using fallback size {}", fallback_size);
    }
    let ptr = match thingsys::bytespace_map(bs_id) {
        Ok(ptr) => ptr,
        Err(e) => {
            info!("bloom: bytespace_map failed: {:?}", e);
            loop {
                stem::yield_now();
            }
        }
    };

    if ptr.is_null() {
        info!("bloom: bytespace_map returned null pointer");
        loop {
            stem::yield_now();
        }
    }

    info!(
        "bloom: mapped bytespace at {:p} (size={})",
        ptr, size
    );
    info!(
        "bloom: bristle port handle {} (listening)",
        bristle_evt_read
    );

    let mut presenter = if !(drv_req_write == 0 && drv_resp_read == 0) {
        info!(
            "bloom: driver ports req_w={} resp_r={}",
            drv_req_write, drv_resp_read
        );
        let mut driver = DriverPresenter::new(drv_req_write, drv_resp_read);
        driver.wait_for_register();
        let bind = BindPayload {
            bytespace_id: bs_id.0,
            width,
            height,
            stride,
            format,
        };
        driver.send_bind(&bind);
        PresenterImpl::Driver(driver)
    } else {
        info!("bloom: no driver ports; rendering only");
        PresenterImpl::Null(present::NullPresenter)
    };

    let mut surface = unsafe { Surface::new(ptr, size, width, height, stride) };
    let mut cursor = CursorState::new((width as i32) / 2, (height as i32) / 2);

    info!("bloom: frame loop started");

    loop {
        bristle::poll_bristle(bristle_evt_read, &mut cursor, surface.width(), surface.height());

        let mut list = DrawList::new();
        build_scene(&mut list, surface.width(), surface.height(), &cursor);
        raster::execute(&mut surface, &list);

        presenter.present();
        presenter.pump();

        stem::sleep_ms(16);
    }
}
