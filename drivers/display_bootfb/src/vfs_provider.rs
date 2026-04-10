use crate::driver::BootFbDriver;
use abi::device::DeviceCall;
use abi::display::{
    BufferHandle, BufferId, CommitRequest, DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO,
    DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_RELEASE_BUFFER,
};
use abi::errors::Errno;
use abi::vfs_rpc::{VfsRpcOp, VfsRpcReqHeader};
use alloc::vec::Vec;
use stem::info;
use stem::syscall::{channel_send, ChannelHandle};

// Handle IDs for this driver.
pub const HANDLE_ROOT: u64 = 0;
pub const HANDLE_CARD: u64 = 1;

const E_OK: u8 = 0;
const E_NOENT: u8 = 2;
const E_IO: u8 = 5;
const E_INVAL: u8 = 22;
const E_NOTSUP: u8 = 38;

const S_IFDIR: u32 = 0o040000;
const S_IFCHR: u32 = 0o020000;

fn send_resp(resp_port: ChannelHandle, data: &[u8]) {
    let _ = channel_send(resp_port, data);
}

fn send_err(resp_port: ChannelHandle, errno: u8) {
    send_resp(resp_port, &[errno]);
}

fn send_ok_data(resp_port: ChannelHandle, data: &[u8]) {
    let mut resp = Vec::with_capacity(1 + data.len());
    resp.push(E_OK);
    resp.extend_from_slice(data);
    send_resp(resp_port, &resp);
}

/// Send OK + [ret_val: u32][out_data_len: u32][out_data...]
fn send_ok_device_call(resp_port: ChannelHandle, ret_val: u32, out_data: &[u8]) {
    let mut resp = Vec::with_capacity(9 + out_data.len());
    resp.push(E_OK);
    resp.extend_from_slice(&ret_val.to_le_bytes());
    resp.extend_from_slice(&(out_data.len() as u32).to_le_bytes());
    resp.extend_from_slice(out_data);
    send_resp(resp_port, &resp);
}

pub fn handle_vfs_rpc(driver: &mut BootFbDriver, buf: &[u8]) {
    let hdr_size = core::mem::size_of::<VfsRpcReqHeader>();
    if buf.len() < hdr_size {
        return;
    }

    let resp_port = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as ChannelHandle;
    let op_byte = buf[4];
    let payload = &buf[hdr_size..];

    let op = match VfsRpcOp::from_u8(op_byte) {
        Some(o) => o,
        None => {
            send_err(resp_port, E_NOTSUP);
            return;
        }
    };

    match op {
        VfsRpcOp::Lookup => handle_lookup(resp_port, payload),
        VfsRpcOp::Stat => handle_stat(resp_port, payload),
        VfsRpcOp::Close => send_resp(resp_port, &[E_OK]),
        VfsRpcOp::DeviceCall => handle_device_call(driver, resp_port, payload),
        VfsRpcOp::Rename => send_err(resp_port, E_NOTSUP),
        VfsRpcOp::SubscribeReady => send_resp(resp_port, &[E_OK]),
        VfsRpcOp::UnsubscribeReady => send_resp(resp_port, &[E_OK]),
        _ => send_err(resp_port, E_NOTSUP),
        _ => send_err(resp_port, E_NOTSUP),
    }
}

fn handle_lookup(resp_port: ChannelHandle, payload: &[u8]) {
    if payload.len() < 4 {
        send_err(resp_port, E_INVAL);
        return;
    }
    let path_len = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
    if payload.len() < 4 + path_len {
        send_err(resp_port, E_INVAL);
        return;
    }
    let path = match core::str::from_utf8(&payload[4..4 + path_len]) {
        Ok(s) => s,
        Err(_) => {
            send_err(resp_port, E_INVAL);
            return;
        }
    };
    let path = path.trim_matches('/');

    let handle: u64 = match path {
        "" => HANDLE_ROOT,
        "card0" => HANDLE_CARD,
        _ => {
            send_err(resp_port, E_NOENT);
            return;
        }
    };

    let mut resp = [0u8; 9];
    resp[0] = E_OK;
    resp[1..9].copy_from_slice(&handle.to_le_bytes());
    send_resp(resp_port, &resp);
}

fn handle_stat(resp_port: ChannelHandle, payload: &[u8]) {
    if payload.len() < 8 {
        send_err(resp_port, E_INVAL);
        return;
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);

    let (mode, size): (u32, u64) = match handle {
        HANDLE_ROOT => (S_IFDIR | 0o755, 0),
        HANDLE_CARD => (S_IFCHR | 0o666, 0),
        _ => {
            send_err(resp_port, E_NOENT);
            return;
        }
    };

    let mut resp = [0u8; 21];
    resp[0] = E_OK;
    resp[1..5].copy_from_slice(&mode.to_le_bytes());
    resp[5..13].copy_from_slice(&size.to_le_bytes());
    resp[13..21].copy_from_slice(&handle.to_le_bytes());
    send_resp(resp_port, &resp);
}

fn handle_device_call(driver: &mut BootFbDriver, resp_port: ChannelHandle, payload: &[u8]) {
    if payload.len() < 8 + core::mem::size_of::<DeviceCall>() {
        send_err(resp_port, E_INVAL);
        return;
    }

    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);

    if handle != HANDLE_CARD {
        send_err(resp_port, E_INVAL);
        return;
    }

    let call: DeviceCall = unsafe {
        core::ptr::read_unaligned(
            payload[8..8 + core::mem::size_of::<DeviceCall>()].as_ptr() as *const _
        )
    };

    let call_payload = &payload[8 + core::mem::size_of::<DeviceCall>()..];

    match call.op {
        DISPLAY_OP_GET_INFO => {
            let info = driver.get_info();
            let out_bytes = unsafe {
                core::slice::from_raw_parts(
                    &info as *const _ as *const u8,
                    core::mem::size_of::<abi::display::DisplayInfo>(),
                )
            };
            send_ok_device_call(resp_port, 0, out_bytes);
        }
        DISPLAY_OP_IMPORT_BUFFER => {
            if call_payload.len() < core::mem::size_of::<BufferHandle>() {
                send_err(resp_port, E_INVAL);
                return;
            }
            let buffer_handle: BufferHandle =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };
            match driver.import_buffer(&buffer_handle) {
                Ok(id) => {
                    send_ok_device_call(resp_port, id.0, &[]);
                }
                Err(e) => send_err(resp_port, e as u8),
            }
        }
        DISPLAY_OP_RELEASE_BUFFER => {
            if call_payload.len() < 4 {
                send_err(resp_port, E_INVAL);
                return;
            }
            let id = BufferId(u32::from_le_bytes([
                call_payload[0],
                call_payload[1],
                call_payload[2],
                call_payload[3],
            ]));
            match driver.release_buffer(id) {
                Ok(()) => send_ok_device_call(resp_port, 0, &[]),
                Err(e) => send_err(resp_port, e as u8),
            }
        }
        DISPLAY_OP_COMMIT => {
            if call_payload.len() < 4 {
                // At least plane count
                send_err(resp_port, E_INVAL);
                return;
            }
            let req = unsafe { &*(call_payload.as_ptr() as *const CommitRequest) };
            // Note: Since req has variable planes, we should be careful.
            // CommitRequest is repr(C) and has planes: [PlaneCommit; 0].
            // The actual planes follow it.

            match driver.commit(req) {
                Ok(()) => send_ok_device_call(resp_port, 0, &[]),
                Err(e) => send_err(resp_port, e as u8),
            }
        }
        _ => send_err(resp_port, E_NOTSUP),
    }
}
