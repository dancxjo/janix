#![no_std]
#![no_main]

use abi::display_driver_protocol as drvproto;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::{sys as thingsys, ThingId};

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.display.Ramfb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn send_msg(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = drvproto::encode_message(&mut buf, msg_type, payload) {
        let _ = port_send(handle, &buf[..len]);
    }
}

fn parse_header(buf: &[u8]) -> Option<drvproto::DriverHeader> {
    if buf.len() < drvproto::HEADER_SIZE {
        return None;
    }

    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    let msg_type = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[8..12].try_into().ok()?);

    Some(drvproto::DriverHeader {
        magic,
        version,
        msg_type,
        payload_len,
    })
}

#[stem::main]
fn main(arg: usize) -> ! {
    let drv_req_read = unpack_handle(arg, 0);
    let drv_resp_write = unpack_handle(arg, 1);

    info!("display_ramfb: starting (drv_req_r={}, drv_resp_w={})", drv_req_read, drv_resp_write);

    let register = drvproto::RegisterPayload {
        driver_kind: drvproto::DRIVER_KIND_RAMFB,
        caps: 0,
    };
    let register_bytes = unsafe {
        core::slice::from_raw_parts(
            &register as *const _ as *const u8,
            core::mem::size_of::<drvproto::RegisterPayload>(),
        )
    };
    send_msg(drv_resp_write, drvproto::MSG_REGISTER, register_bytes);

    let mut buf = [0u8; 512];
    let mut rx_buf = [0u8; 1024];
    let mut rx_len = 0usize;
    let mut src_ptr: *const u8 = core::ptr::null();
    let mut dst_ptr: *mut u8 = core::ptr::null_mut();
    let mut stride = 0usize;
    let mut width = 0u32;
    let mut height = 0u32;

    loop {
        if let Ok(n) = port_recv(drv_req_read, &mut buf) {
            if n > 0 {
                if rx_len + n > rx_buf.len() {
                    rx_len = 0;
                }
                rx_buf[rx_len..rx_len + n].copy_from_slice(&buf[..n]);
                rx_len += n;
            }
        }

        while rx_len >= drvproto::HEADER_SIZE {
            let header = match parse_header(&rx_buf[..rx_len]) {
                Some(header) => header,
                None => {
                    rx_len = 0;
                    break;
                }
            };

            if header.magic != drvproto::DRIVER_MAGIC || header.version != drvproto::DRIVER_VERSION {
                rx_buf.copy_within(1..rx_len, 0);
                rx_len -= 1;
                continue;
            }

            let total = drvproto::HEADER_SIZE + (header.payload_len as usize);
            if rx_len < total {
                break;
            }

            let payload = &rx_buf[drvproto::HEADER_SIZE..total];
            match header.msg_type {
                drvproto::MSG_BIND => {
                    if payload.len() >= core::mem::size_of::<drvproto::BindPayload>() {
                        let bind: drvproto::BindPayload = unsafe {
                            core::ptr::read_unaligned(payload.as_ptr() as *const _)
                        };
                        let size = (bind.height as usize) * (bind.stride as usize);
                        match thingsys::bytespace_create(size, 0, 0) {
                            Ok(bs_id) => match thingsys::bytespace_map(bs_id) {
                                Ok(ptr) => {
                                    dst_ptr = ptr;
                                }
                                Err(e) => {
                                    info!("display_ramfb: dst bytespace_map failed: {:?}", e);
                                }
                            },
                            Err(e) => {
                                info!("display_ramfb: dst bytespace_create failed: {:?}", e);
                            }
                        }
                        match thingsys::bytespace_map(ThingId(bind.bytespace_id)) {
                            Ok(ptr) => {
                                src_ptr = ptr as *const u8;
                                stride = bind.stride as usize;
                                width = bind.width;
                                height = bind.height;
                                send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                                info!("display_ramfb: bound bytespace {}", bind.bytespace_id);
                            }
                            Err(e) => {
                                info!("display_ramfb: src bytespace_map failed: {:?}", e);
                                let err = drvproto::ErrResp { code: 2 };
                                let err_bytes = unsafe {
                                    core::slice::from_raw_parts(
                                        &err as *const _ as *const u8,
                                        core::mem::size_of::<drvproto::ErrResp>(),
                                    )
                                };
                                send_msg(drv_resp_write, drvproto::MSG_ERR, err_bytes);
                            }
                        }
                    }
                }
                drvproto::MSG_PRESENT => {
                    if !src_ptr.is_null() && !dst_ptr.is_null() {
                        let row_bytes = (width as usize) * 4;
                        for row in 0..height as usize {
                            unsafe {
                                core::ptr::copy_nonoverlapping(
                                    src_ptr.add(row * stride),
                                    dst_ptr.add(row * stride),
                                    row_bytes,
                                );
                            }
                        }
                        send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                    } else {
                        let err = drvproto::ErrResp { code: 1 };
                        let err_bytes = unsafe {
                            core::slice::from_raw_parts(
                                &err as *const _ as *const u8,
                                core::mem::size_of::<drvproto::ErrResp>(),
                            )
                        };
                        send_msg(drv_resp_write, drvproto::MSG_ERR, err_bytes);
                    }
                }
                _ => {}
            }

            if total < rx_len {
                rx_buf.copy_within(total..rx_len, 0);
            }
            rx_len -= total;
        }
        stem::yield_now();
    }
}
