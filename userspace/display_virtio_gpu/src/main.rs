#![no_std]
#![no_main]

extern crate alloc;

use abi::display_driver_protocol as drvproto;
use abi::driver_frame::FrameReader;
use abi::ids::HandleId;
use abi::schema::kinds;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::{sys as thingsys, ThingId};
use virtio_gpu::{Rect, VirtioGpu};

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.display.Gpu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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

fn find_gpu() -> Option<ThingId> {
    let mut buf = [ThingId::default(); 1];
    let count = thingsys::find(kinds::DEV_DISPLAY_GPU, &mut buf).ok()?;
    if count == 0 {
        return None;
    }
    Some(buf[0])
}

#[stem::main]
fn main(arg: usize) -> ! {
    let drv_req_read = unpack_handle(arg, 0);
    let drv_resp_write = unpack_handle(arg, 1);

    info!(
        "display_virtio_gpu: starting (drv_req_r={}, drv_resp_w={})",
        drv_req_read, drv_resp_write
    );

    // Find and initialize GPU
    let gpu_id = match find_gpu() {
        Some(id) => id,
        None => {
            info!("display_virtio_gpu: GPU device not found");
            loop {
                stem::yield_now();
            }
        }
    };

    let mut gpu = match VirtioGpu::new(gpu_id.to_u64_lossy()) {
        Ok(g) => g,
        Err(e) => {
            info!("display_virtio_gpu: Failed to initialize GPU: {:?}", e);
            loop {
                stem::yield_now();
            }
        }
    };

    if let Err(e) = gpu.init_virtio() {
        info!("display_virtio_gpu: Virtio init failed: {}", e);
        loop {
            stem::yield_now();
        }
    }

    info!("display_virtio_gpu: GPU initialized successfully");

    // Send MSG_REGISTER
    let register = drvproto::RegisterPayload {
        driver_kind: drvproto::DRIVER_KIND_VIRTIO_GPU,
        caps: drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME,
    };
    let mut register_bytes = [0u8; drvproto::REGISTER_PAYLOAD_WIRE_SIZE];
    if let Some(len) = drvproto::encode_register_payload_le(&register, &mut register_bytes) {
        send_msg(
            drv_resp_write,
            drvproto::MSG_REGISTER,
            &register_bytes[..len],
        );
    }

    let mut buf = [0u8; 512];
    let mut frames = FrameReader::<4096>::new();
    let mut bound_bytespace: Option<ThingId> = None;
    let mut src_ptr: *const u8 = core::ptr::null();
    let mut src_width = 0u32;
    let mut src_height = 0u32;
    let mut resource_created = false;

    loop {
        if let Ok(n) = port_recv(drv_req_read, &mut buf) {
            if n > 0 {
                frames.push(&buf[..n]);
            }
        }

        while let Some((header, payload)) = frames.next_message() {
            match header.msg_type {
                drvproto::MSG_HELLO => {
                    let want_caps = drvproto::decode_hello_payload_le(payload)
                        .map(|hello| hello.want_caps)
                        .unwrap_or(0);
                    let supported_caps = drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME;
                    let welcome = drvproto::WelcomePayload {
                        proto_major: drvproto::PROTO_MAJOR,
                        proto_minor: drvproto::PROTO_MINOR,
                        have_caps: supported_caps & want_caps,
                        max_rects: 8,
                        reserved: 0,
                    };
                    let mut welcome_bytes = [0u8; drvproto::WELCOME_PAYLOAD_WIRE_SIZE];
                    if let Some(len) =
                        drvproto::encode_welcome_payload_le(&welcome, &mut welcome_bytes)
                    {
                        send_msg(drv_resp_write, drvproto::MSG_WELCOME, &welcome_bytes[..len]);
                    }
                }
                drvproto::MSG_BIND => {
                    if let Some(bind) = drvproto::decode_bind_payload_le(payload) {
                        let bs_id = ThingId::from_u64(bind.bytespace_id);
                        
                        // Map bytespace
                        match thingsys::bytespace_map(bs_id) {
                            Ok(ptr) => {
                                // Get physical address for GPU backing
                                match thingsys::bytespace_phys(bs_id) {
                                    Ok(phys_addr) => {
                                        bound_bytespace = Some(bs_id);
                                        src_ptr = ptr as *const u8;
                                        src_width = bind.width;
                                        src_height = bind.height;

                                        let size = (bind.height as usize) * (bind.stride as usize);

                                        // Set GPU dimensions
                                        gpu.set_dimensions(bind.width, bind.height);

                                        // Create resource if not created yet or dimensions changed
                                        if !resource_created {
                                            if let Err(e) = gpu.create_resource_2d() {
                                                info!("display_virtio_gpu: create_resource_2d failed: {}", e);
                                                let err = drvproto::ErrResp { code: 3 };
                                                let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                                if let Some(len) =
                                                    drvproto::encode_err_resp_le(&err, &mut err_bytes)
                                                {
                                                    send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                                }
                                                continue;
                                            }
                                            resource_created = true;
                                        }

                                        // Attach bytespace as backing memory
                                        if let Err(e) = gpu.attach_backing(phys_addr, size) {
                                            info!("display_virtio_gpu: attach_backing failed: {}", e);
                                            let err = drvproto::ErrResp { code: 4 };
                                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                            if let Some(len) =
                                                drvproto::encode_err_resp_le(&err, &mut err_bytes)
                                            {
                                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                            }
                                            continue;
                                        }

                                        // Set scanout
                                        if let Err(e) = gpu.set_scanout(bind.width, bind.height) {
                                            info!("display_virtio_gpu: set_scanout failed: {}", e);
                                            let err = drvproto::ErrResp { code: 5 };
                                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                            if let Some(len) =
                                                drvproto::encode_err_resp_le(&err, &mut err_bytes)
                                            {
                                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                            }
                                            continue;
                                        }

                                        send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                                        info!("display_virtio_gpu: bound bytespace {}", bind.bytespace_id);
                                    }
                                    Err(e) => {
                                        info!("display_virtio_gpu: bytespace_phys failed: {:?}", e);
                                        let err = drvproto::ErrResp { code: 6 };
                                        let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                        if let Some(len) =
                                            drvproto::encode_err_resp_le(&err, &mut err_bytes)
                                        {
                                            send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                info!("display_virtio_gpu: bytespace_map failed: {:?}", e);
                                let err = drvproto::ErrResp { code: 2 };
                                let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                if let Some(len) =
                                    drvproto::encode_err_resp_le(&err, &mut err_bytes)
                                {
                                    send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                }
                            }
                        }
                    }
                }
                drvproto::MSG_PRESENT => {
                    if bound_bytespace.is_none() {
                        let err = drvproto::ErrResp { code: 1 };
                        let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                        if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                            send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                        }
                        continue;
                    }

                    if let Some(present) = drvproto::decode_present_header_le(payload) {
                        let rects_payload = &payload[drvproto::PRESENT_HEADER_WIRE_SIZE..];
                        
                        // Handle full-frame present
                        if present.rect_count == 0
                            || (present._pad & drvproto::PRESENT_FLAG_FULLFRAME != 0)
                        {
                            let full_rect = Rect {
                                x: 0,
                                y: 0,
                                w: src_width,
                                h: src_height,
                            };
                            let _ = gpu.present_rect(full_rect);
                        } else {
                            // Handle dirty rects
                            let rect_size = drvproto::RECT_WIRE_SIZE;
                            for i in 0..present.rect_count as usize {
                                let off = i * rect_size;
                                if rects_payload.len() < off + rect_size {
                                    break;
                                }
                                if let Some(rect) =
                                    drvproto::decode_rect_le(&rects_payload[off..off + rect_size])
                                {
                                    let gpu_rect = Rect {
                                        x: rect.x,
                                        y: rect.y,
                                        w: rect.w,
                                        h: rect.h,
                                    };
                                    let _ = gpu.present_rect(gpu_rect);
                                }
                            }
                        }
                    }
                    send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                }
                _ => {}
            }
        }
        stem::yield_now();
    }
}
