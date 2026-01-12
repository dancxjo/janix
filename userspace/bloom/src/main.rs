#![no_std]
#![no_main]

use abi::display_protocol as dispproto;
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::sys as thingsys;

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn send_msg(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = dispproto::encode_message(&mut buf, msg_type, payload) {
        let _ = port_send(handle, &buf[..len]);
    }
}

fn recv_msg(handle: PortHandle, buf: &mut [u8]) -> (dispproto::DisplayHeader, usize) {
    loop {
        if let Ok(n) = port_recv(handle, buf) {
            if let Some((header, payload)) = dispproto::parse_message(&buf[..n]) {
                let payload_len = payload.len();
                return (header, payload_len + dispproto::HEADER_SIZE);
            }
        }
        stem::yield_now();
    }
}

fn draw_frame(ptr: *mut u32, width: u32, height: u32, stride: u32, frame: u32) {
    let stride_pixels = (stride / 4) as usize;
    let w = width as usize;
    let h = height as usize;

    for y in 0..h {
        for x in 0..w {
            let fx = ((x as u32 + frame) & 0xFF) as u32;
            let fy = ((y as u32 + frame) & 0xFF) as u32;
            let color = (fx << 16) | (fy << 8) | (0x40 + (frame & 0x3F));
            unsafe {
                core::ptr::write_volatile(ptr.add(y * stride_pixels + x), color);
            }
        }
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    let disp_req_write = unpack_handle(arg, 0);
    let disp_resp_read = unpack_handle(arg, 1);

    info!("bloom: starting (disp_req_w={}, disp_resp_r={})", disp_req_write, disp_resp_read);

    send_msg(disp_req_write, dispproto::MSG_HELLO, &[]);

    send_msg(disp_req_write, dispproto::MSG_INFO_REQ, &[]);
    let mut resp_buf = [0u8; 512];
    let (info_hdr, info_len) = recv_msg(disp_resp_read, &mut resp_buf);
    if info_hdr.msg_type != dispproto::MSG_INFO_RESP {
        info!("bloom: unexpected INFO response {}", info_hdr.msg_type);
    }
    let info_payload = &resp_buf[dispproto::HEADER_SIZE..info_len];
    let info: dispproto::InfoResp = unsafe {
        core::ptr::read_unaligned(info_payload.as_ptr() as *const dispproto::InfoResp)
    };

    send_msg(disp_req_write, dispproto::MSG_BUFFER_REQ, &[]);
    let (buf_hdr, buf_len) = recv_msg(disp_resp_read, &mut resp_buf);
    if buf_hdr.msg_type != dispproto::MSG_BUFFER_RESP {
        info!("bloom: unexpected BUFFER response");
    }
    let buf_payload = &resp_buf[dispproto::HEADER_SIZE..buf_len];
    let buffer: dispproto::BufferResp = unsafe {
        core::ptr::read_unaligned(buf_payload.as_ptr() as *const dispproto::BufferResp)
    };

    let bs_id = stem::thing::ThingId(buffer.bytespace_id);
    let map_ptr = match thingsys::bytespace_map(bs_id) {
        Ok(ptr) => ptr,
        Err(e) => {
            info!("bloom: bytespace_map failed: {:?}", e);
            loop { stem::yield_now(); }
        }
    };

    info!("bloom: mapped compositor bytespace {}", buffer.bytespace_id);

    let mut frame: u32 = 0;
    loop {
        draw_frame(map_ptr as *mut u32, info.width, info.height, info.stride, frame);
        send_msg(disp_req_write, dispproto::MSG_PRESENT, &[]);

        let (ack_hdr, ack_len) = recv_msg(disp_resp_read, &mut resp_buf);
        if ack_hdr.msg_type == dispproto::MSG_ERR {
            let err_payload = &resp_buf[dispproto::HEADER_SIZE..ack_len];
            let err = if err_payload.len() >= core::mem::size_of::<dispproto::ErrResp>() {
                unsafe {
                    Some(core::ptr::read_unaligned(
                        err_payload.as_ptr() as *const dispproto::ErrResp,
                    ))
                }
            } else {
                None
            };
            if let Some(err) = err {
                info!("bloom: present error code={}", err.code);
            } else {
                info!("bloom: present error (no payload)");
            }
        }

        frame = frame.wrapping_add(1);
        stem::syscall::sleep_ms(16);
    }
}
