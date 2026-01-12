#![no_std]
#![no_main]

use abi::display_driver_protocol as drvproto;
use abi::display_protocol as dispproto;
use abi::schema::{keys, kinds};
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::{sys as thingsys, ThingId};

const DISPLAY_ROLE_KEY: &str = "display_role";
const DISPLAY_ROLE_COMPOSITOR: &str = "display.compositor";

struct DisplaySurface {
    bytespace_id: ThingId,
    width: u32,
    height: u32,
    stride: u32,
    format: u32,
}

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn find_compositor_surface() -> Option<DisplaySurface> {
    let role_sym = thingsys::intern(DISPLAY_ROLE_COMPOSITOR).ok()?;

    let mut buf = [ThingId(0); 64];
    let count = thingsys::find(kinds::BYTESPACE, &mut buf).ok()?;
    for i in 0..core::cmp::min(count, buf.len()) {
        let bs = buf[i];
        if bs.0 == 0 {
            continue;
        }
        let role = match thingsys::prop_get(bs, DISPLAY_ROLE_KEY) {
            Ok(val) => val,
            Err(_) => continue,
        };
        if role != role_sym as u64 {
            continue;
        }

        let width = thingsys::prop_get(bs, keys::WIDTH).unwrap_or(0) as u32;
        let height = thingsys::prop_get(bs, keys::HEIGHT).unwrap_or(0) as u32;
        let stride = thingsys::prop_get(bs, keys::STRIDE).unwrap_or(0) as u32;
        let format = thingsys::prop_get(bs, keys::FORMAT).unwrap_or(0) as u32;
        return Some(DisplaySurface {
            bytespace_id: bs,
            width,
            height,
            stride,
            format,
        });
    }

    None
}

fn send_display_msg(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = dispproto::encode_message(&mut buf, msg_type, payload) {
        let _ = port_send(handle, &buf[..len]);
    }
}

fn send_driver_msg(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = drvproto::encode_message(&mut buf, msg_type, payload) {
        let _ = port_send(handle, &buf[..len]);
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    let disp_req_read = unpack_handle(arg, 0);
    let disp_resp_write = unpack_handle(arg, 1);
    let drv_req_write = unpack_handle(arg, 2);
    let drv_resp_read = unpack_handle(arg, 3);

    info!("blossom: starting (disp_req_r={}, disp_resp_w={}, drv_req_w={}, drv_resp_r={})",
        disp_req_read, disp_resp_write, drv_req_write, drv_resp_read);

    let surface = match find_compositor_surface() {
        Some(surface) => surface,
        None => {
            info!("blossom: compositor bytespace not found");
            loop {
                stem::yield_now();
            }
        }
    };

    let mut driver_ready = false;
    let mut driver_bound = false;
    let mut pending_present = false;

    let mut disp_buf = [0u8; 512];
    let mut drv_buf = [0u8; 512];

    loop {
        if let Ok(n) = port_recv(drv_resp_read, &mut drv_buf) {
            if let Some((header, payload)) = drvproto::parse_message(&drv_buf[..n]) {
                match header.msg_type {
                    drvproto::MSG_REGISTER => {
                        if payload.len() >= core::mem::size_of::<drvproto::RegisterPayload>() {
                            driver_ready = true;
                            send_driver_msg(drv_req_write, drvproto::MSG_ACK, &[]);

                            let bind = drvproto::BindPayload {
                                bytespace_id: surface.bytespace_id.0,
                                width: surface.width,
                                height: surface.height,
                                stride: surface.stride,
                                format: surface.format,
                            };
                            let bind_bytes = unsafe {
                                core::slice::from_raw_parts(
                                    &bind as *const _ as *const u8,
                                    core::mem::size_of::<drvproto::BindPayload>(),
                                )
                            };
                            send_driver_msg(drv_req_write, drvproto::MSG_BIND, bind_bytes);
                            info!("blossom: driver registered, bind sent");
                        }
                    }
                    drvproto::MSG_ACK => {
                        if !driver_bound {
                            driver_bound = true;
                            info!("blossom: driver bound");
                        }
                        if pending_present {
                            send_display_msg(disp_resp_write, dispproto::MSG_ACK, &[]);
                            pending_present = false;
                        }
                    }
                    drvproto::MSG_ERR => {
                        if !driver_bound {
                            info!("blossom: driver bind error");
                        }
                        if pending_present {
                            send_display_msg(disp_resp_write, dispproto::MSG_ERR, payload);
                            pending_present = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Ok(n) = port_recv(disp_req_read, &mut disp_buf) {
            if let Some((header, payload)) = dispproto::parse_message(&disp_buf[..n]) {
                match header.msg_type {
                    dispproto::MSG_HELLO => {
                        send_display_msg(disp_resp_write, dispproto::MSG_ACK, &[]);
                    }
                    dispproto::MSG_INFO_REQ => {
                        let info = dispproto::InfoResp {
                            width: surface.width,
                            height: surface.height,
                            stride: surface.stride,
                            format: surface.format,
                        };
                        let info_bytes = unsafe {
                            core::slice::from_raw_parts(
                                &info as *const _ as *const u8,
                                core::mem::size_of::<dispproto::InfoResp>(),
                            )
                        };
                        send_display_msg(disp_resp_write, dispproto::MSG_INFO_RESP, info_bytes);
                    }
                    dispproto::MSG_BUFFER_REQ => {
                        let size = (surface.height as u64) * (surface.stride as u64);
                        let resp = dispproto::BufferResp {
                            bytespace_id: surface.bytespace_id.0,
                            size,
                            stride: surface.stride,
                            format: surface.format,
                        };
                        let resp_bytes = unsafe {
                            core::slice::from_raw_parts(
                                &resp as *const _ as *const u8,
                                core::mem::size_of::<dispproto::BufferResp>(),
                            )
                        };
                        send_display_msg(disp_resp_write, dispproto::MSG_BUFFER_RESP, resp_bytes);
                    }
                    dispproto::MSG_PRESENT => {
                        if !driver_ready {
                            let err = dispproto::ErrResp { code: 1 };
                            let err_bytes = unsafe {
                                core::slice::from_raw_parts(
                                    &err as *const _ as *const u8,
                                    core::mem::size_of::<dispproto::ErrResp>(),
                                )
                            };
                            send_display_msg(disp_resp_write, dispproto::MSG_ERR, err_bytes);
                        } else if !driver_bound {
                            let err = dispproto::ErrResp { code: 2 };
                            let err_bytes = unsafe {
                                core::slice::from_raw_parts(
                                    &err as *const _ as *const u8,
                                    core::mem::size_of::<dispproto::ErrResp>(),
                                )
                            };
                            send_display_msg(disp_resp_write, dispproto::MSG_ERR, err_bytes);
                        } else {
                            pending_present = true;
                            send_driver_msg(drv_req_write, drvproto::MSG_PRESENT, payload);
                        }
                    }
                    _ => {}
                }
            }
        }

        stem::yield_now();
    }
}
