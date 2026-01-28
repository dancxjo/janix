#![no_std]
#![no_main]

extern crate alloc;

use abi::display_driver_protocol as drvproto;
use abi::schema::kinds;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::info;
use stem::syscall::{device_claim, device_map_mmio, port_recv, port_send, PortHandle};
use stem::thing::{sys as thingsys, ThingId};

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
    let mut buf = [ThingId(0); 1];
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

    info!("display_virtio_gpu: starting (drv_req_r={}, drv_resp_w={})", drv_req_read, drv_resp_write);

    // NOTE: VirtIO GPU driver is currently a stub.
    // Full implementation requires VirtIO PCI capability parsing
    // which is not yet implemented. For now, this just acks messages
    // but doesn't actually display anything.
    
    if let Some(gpu_id) = find_gpu() {
        if let Ok(claim) = device_claim(gpu_id.0) {
            let _ = device_map_mmio(claim, 0);
            info!("display_virtio_gpu: claimed gpu {} (stub driver - VirtIO not fully implemented)", gpu_id.0);
        }
    }

    let register = drvproto::RegisterPayload {
        driver_kind: drvproto::DRIVER_KIND_VIRTIO_GPU,
        caps: 0,
    };
    let mut register_bytes = [0u8; drvproto::REGISTER_PAYLOAD_WIRE_SIZE];
    if let Some(len) = drvproto::encode_register_payload_le(&register, &mut register_bytes) {
        send_msg(drv_resp_write, drvproto::MSG_REGISTER, &register_bytes[..len]);
    }

    let mut buf = [0u8; 512];
    let mut rx_buf = [0u8; 1024];
    let mut rx_len = 0usize;
    let mut bound = false;

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
            if let Some((header, payload)) = drvproto::parse_message(&rx_buf[..rx_len]) {
                match header.msg_type {
                drvproto::MSG_BIND => {
                    if let Some(bind) = drvproto::decode_bind_payload_le(payload) {
                        match thingsys::bytespace_map(ThingId(bind.bytespace_id)) {
                            Ok(_) => {
                                bound = true;
                                send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                                info!("display_virtio_gpu: bound bytespace {} (stub - not displaying)", bind.bytespace_id);
                            }
                            Err(e) => {
                                info!("display_virtio_gpu: bytespace_map failed: {:?}", e);
                                let err = drvproto::ErrResp { code: 2 };
                                let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                    send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                                }
                            }
                        }
                    }
                }
                drvproto::MSG_PRESENT => {
                    if bound {
                        // Stub: just ack without actually presenting
                        send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                    } else {
                        let err = drvproto::ErrResp { code: 1 };
                        let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                        if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                            send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                        }
                    }
                }
                _ => {}
            }

                let total = drvproto::HEADER_SIZE + payload.len();
                if total < rx_len {
                    rx_buf.copy_within(total..rx_len, 0);
                }
                rx_len -= total;
                continue;
            }

            if let Some(total) = drvproto::message_total_len(&rx_buf[..rx_len]) {
                if rx_len < total {
                    break;
                }
            } else {
                rx_buf.copy_within(1..rx_len, 0);
                rx_len -= 1;
            }
        }
        stem::yield_now();
    }
}
