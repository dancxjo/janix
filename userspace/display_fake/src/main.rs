#![feature(restricted_std)]
#![no_main]

use abi::display_driver_protocol as drvproto;
use abi::driver_frame::FrameReader;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::ThingId;

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.display.Fake\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

#[derive(Clone, Copy, Debug)]
struct FakeConfig {
    caps: u32,
    max_rects: u16,
    split_writes: bool,
    burst: bool,
}

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn parse_config(arg: usize) -> FakeConfig {
    // Config word layout in arg>>32 (for tests):
    // bits 0-1: caps mask (bit0=DIRTY_RECTS, bit1=FULLFRAME; 0 => default both)
    // bits 8-15: max_rects (0 => default 8)
    // bit 16: split_writes, bit 17: burst
    let cfg = (arg >> 32) as u32;
    let mut caps = 0u32;
    if cfg & 0x1 != 0 {
        caps |= drvproto::CAP_DIRTY_RECTS;
    }
    if cfg & 0x2 != 0 {
        caps |= drvproto::CAP_FULLFRAME;
    }
    if caps == 0 {
        caps = drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME;
    }

    let max_rects = ((cfg >> 8) & 0xFF) as u16;
    let max_rects = if max_rects == 0 { 8 } else { max_rects };

    let split_writes = cfg & (1 << 16) != 0;
    let burst = cfg & (1 << 17) != 0;

    FakeConfig {
        caps,
        max_rects,
        split_writes,
        burst,
    }
}

fn send_msg(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = drvproto::encode_message(&mut buf, msg_type, payload) {
        let _ = port_send(handle, &buf[..len]);
    }
}

fn send_msg_split(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = drvproto::encode_message(&mut buf, msg_type, payload) {
        if len < 3 {
            let _ = port_send(handle, &buf[..len]);
            return;
        }
        let part = len / 3;
        let mut offset = 0;
        for i in 0..3 {
            let remaining = len - offset;
            let chunk = if i == 2 { remaining } else { part.max(1) };
            if chunk == 0 {
                break;
            }
            let _ = port_send(handle, &buf[offset..offset + chunk]);
            offset += chunk;
        }
    }
}

fn send_ack(handle: PortHandle, burst: bool) {
    let mut buf = [0u8; 64];
    if let Some(len) = drvproto::encode_message(&mut buf, drvproto::MSG_ACK, &[]) {
        if burst {
            if let Some(len2) = drvproto::encode_message(&mut buf[len..], drvproto::MSG_ACK, &[]) {
                let total = len + len2;
                let _ = port_send(handle, &buf[..total]);
                return;
            }
        }
        let _ = port_send(handle, &buf[..len]);
    }
}

fn send_err(handle: PortHandle, code: u32) {
    let err = drvproto::ErrResp { code };
    let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
    if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
        send_msg(handle, drvproto::MSG_ERR, &err_bytes[..len]);
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    let drv_req_read = unpack_handle(arg, 0);
    let drv_resp_write = unpack_handle(arg, 1);
    let config = parse_config(arg);

    info!(
        "display_fake: starting (drv_req_r={}, drv_resp_w={}) caps=0x{:x} max_rects={} split_writes={} burst={}",
        drv_req_read,
        drv_resp_write,
        config.caps,
        config.max_rects,
        config.split_writes,
        config.burst
    );

    let mut buf = [0u8; 512];
    let mut frames = FrameReader::<4096>::new();
    let mut bound = false;
    let mut bound_bs: Option<ThingId> = None;

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
                    let welcome = drvproto::WelcomePayload {
                        proto_major: drvproto::PROTO_MAJOR,
                        proto_minor: drvproto::PROTO_MINOR,
                        have_caps: config.caps & want_caps,
                        max_rects: config.max_rects,
                        reserved: 0,
                    };
                    let mut welcome_bytes = [0u8; drvproto::WELCOME_PAYLOAD_WIRE_SIZE];
                    if let Some(len) =
                        drvproto::encode_welcome_payload_le(&welcome, &mut welcome_bytes)
                    {
                        if config.split_writes {
                            send_msg_split(
                                drv_resp_write,
                                drvproto::MSG_WELCOME,
                                &welcome_bytes[..len],
                            );
                        } else {
                            send_msg(drv_resp_write, drvproto::MSG_WELCOME, &welcome_bytes[..len]);
                        }
                    }
                }
                drvproto::MSG_BIND => {
                    if let Some(bind) = drvproto::decode_bind_payload_le(payload) {
                        bound = true;
                        bound_bs = Some(ThingId::from_u64(bind.bytespace_id));
                        send_ack(drv_resp_write, config.burst);
                    } else {
                        send_err(drv_resp_write, 2);
                    }
                }
                drvproto::MSG_PRESENT => {
                    if !bound || bound_bs.is_none() {
                        send_err(drv_resp_write, 1);
                        continue;
                    }

                    let present = drvproto::decode_present_header_le(payload);
                    if let Some(present) = present {
                        if (present._pad & drvproto::PRESENT_FLAG_FULLFRAME == 0)
                            && present.rect_count > config.max_rects as u32
                        {
                            send_err(drv_resp_write, 3);
                            continue;
                        }
                    }
                    send_ack(drv_resp_write, config.burst);
                }
                _ => {}
            }
        }
        stem::yield_now();
    }
}
