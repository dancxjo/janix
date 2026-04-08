//! Disk Probe VFS Provider
//!
//! Exposes a virtual filesystem at `/dev/disks` where each block device
//! discovered in the system is represented as a file (e.g. `/dev/disks/0`).
//! Reading the file outputs its properties.

#![feature(restricted_std)]
#![no_main]

extern crate alloc;

use abi::vfs_rpc::{VfsRpcOp, VfsRpcReqHeader, VFS_RPC_MAX_REQ};
use alloc::format;
use alloc::vec::Vec;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::info;
use stem::syscall::{port_create, port_recv, port_send, port_wait, vfs_mount, PortHandle};
use stem::thing::sys::{find, prop_get};
use stem::thing::ThingId;

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Service,
    device_kind: {
        let mut arr = [0; 64];
        let id = b"svc.vfs.disk_probe";
        let mut i = 0;
        while i < id.len() {
            arr[i] = id[i];
            i += 1;
        }
        arr
    },
    version: 1,
    _reserved: 0,
};

/// File type bits (same as kernel VfsStat::S_IFxxx).
const S_IFDIR: u32 = 0o040000;
const S_IFREG: u32 = 0o100000;

/// Errno values
const E_OK: u8 = 0;
const E_NOENT: u8 = 2;
const E_INVAL: u8 = 22;
const E_ROFS: u8 = 30;
const E_NOTSUP: u8 = 38;

fn send_resp(resp_port: PortHandle, data: &[u8]) {
    let _ = port_send(resp_port, data);
}

fn send_err(resp_port: PortHandle, errno: u8) {
    send_resp(resp_port, &[errno]);
}

fn render_disk_info(index: usize) -> Option<alloc::string::String> {
    let mut disks = [ThingId::default(); 16];
    let count = find("dev.storage.Disk", &mut disks).unwrap_or(0);
    if index >= count {
        return None;
    }
    let disk_id = disks[index];

    let mut out = format!("Disk {} (Thing {:?})\n", index, disk_id.0);

    match prop_get(disk_id, "sector_count") {
        Ok(sectors) => {
            let size_mb = (sectors * 512) / (1024 * 1024);
            out.push_str(&format!("Sector count: {} ({} MB)\n", sectors, size_mb));
        }
        Err(_) => out.push_str("Sector count: (unavailable)\n"),
    }

    match prop_get(disk_id, "sector_size") {
        Ok(size) => {
            out.push_str(&format!("Sector size: {} bytes\n", size));
        }
        Err(_) => out.push_str("Sector size: (unavailable)\n"),
    }

    match prop_get(disk_id, "lba48") {
        Ok(lba48) => {
            let support = if lba48 != 0 { "yes" } else { "no" };
            out.push_str(&format!("LBA48 support: {}\n", support));
        }
        Err(_) => out.push_str("LBA48 support: (unavailable)\n"),
    }

    Some(out)
}

fn handle_vfs_rpc(buf: &[u8]) {
    if buf.len() < core::mem::size_of::<VfsRpcReqHeader>() {
        return;
    }

    let hdr_size = core::mem::size_of::<VfsRpcReqHeader>();
    let resp_port = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as PortHandle;
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
        VfsRpcOp::Read => handle_read(resp_port, payload),
        VfsRpcOp::Write => send_err(resp_port, E_ROFS),
        VfsRpcOp::Readdir => handle_readdir(resp_port, payload),
        VfsRpcOp::Stat => handle_stat(resp_port, payload),
        VfsRpcOp::Close => send_resp(resp_port, &[E_OK]),
        VfsRpcOp::Poll => {
            let mut r = [0u8; 5];
            r[0] = E_OK;
            r[1..5].copy_from_slice(&1u32.to_le_bytes()); // POLLIN
            send_resp(resp_port, &r);
        }
    }
}

fn handle_lookup(resp_port: PortHandle, payload: &[u8]) {
    if payload.len() < 4 {
        send_err(resp_port, E_INVAL);
        return;
    }
    let path_len = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
    if payload.len() < 4 + path_len {
        send_err(resp_port, E_INVAL);
        return;
    }
    let path_bytes = &payload[4..4 + path_len];
    let path = match core::str::from_utf8(path_bytes) {
        Ok(p) => p,
        Err(_) => {
            send_err(resp_port, E_INVAL);
            return;
        }
    };

    let handle: u64 = if path.is_empty() || path == "/" {
        0
    } else {
        match path.parse::<usize>() {
            Ok(idx) if render_disk_info(idx).is_some() => (idx as u64) + 1,
            _ => {
                send_err(resp_port, E_NOENT);
                return;
            }
        }
    };

    let mut resp = [0u8; 9];
    resp[0] = E_OK;
    resp[1..9].copy_from_slice(&handle.to_le_bytes());
    send_resp(resp_port, &resp);
}

fn handle_read(resp_port: PortHandle, payload: &[u8]) {
    if payload.len() < 20 {
        send_err(resp_port, E_INVAL);
        return;
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);
    let offset = u64::from_le_bytes([
        payload[8],
        payload[9],
        payload[10],
        payload[11],
        payload[12],
        payload[13],
        payload[14],
        payload[15],
    ]) as usize;
    let len = u32::from_le_bytes([payload[16], payload[17], payload[18], payload[19]]) as usize;

    if handle == 0 {
        // Can't read a directory
        send_err(resp_port, E_INVAL);
        return;
    }

    let idx = (handle - 1) as usize;
    if let Some(text) = render_disk_info(idx) {
        let bytes = text.as_bytes();
        let size = bytes.len();

        if offset >= size {
            let mut resp = [0u8; 5];
            resp[0] = E_OK; // bytes_read = 0 (pad is 0s)
            send_resp(resp_port, &resp);
            return;
        }

        let clamped_len = len.min(size - offset);
        let mut resp = Vec::with_capacity(5 + clamped_len);
        resp.push(E_OK);
        resp.extend_from_slice(&(clamped_len as u32).to_le_bytes());
        resp.extend_from_slice(&bytes[offset..offset + clamped_len]);
        send_resp(resp_port, &resp);
    } else {
        send_err(resp_port, E_NOENT);
    }
}

fn handle_readdir(resp_port: PortHandle, payload: &[u8]) {
    if payload.len() < 20 {
        send_err(resp_port, E_INVAL);
        return;
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);
    let offset = u64::from_le_bytes([
        payload[8],
        payload[9],
        payload[10],
        payload[11],
        payload[12],
        payload[13],
        payload[14],
        payload[15],
    ]) as usize;
    let max_bytes =
        u32::from_le_bytes([payload[16], payload[17], payload[18], payload[19]]) as usize;

    if handle != 0 {
        // Not a directory
        send_err(resp_port, E_INVAL);
        return;
    }

    let mut disks = [ThingId::default(); 16];
    let count = find("dev.storage.Disk", &mut disks).unwrap_or(0);

    let start_idx = offset;
    let mut out: Vec<u8> = Vec::new();

    for i in start_idx..count {
        let name_str = format!("{}", i);
        let name_bytes = name_str.as_bytes();
        let name_len = name_bytes.len() as u8;
        let file_type = 8; // DT_REG
        let ino = (i as u64) + 1;

        let entry_size = 10 + name_len as usize;
        if out.len() + entry_size > max_bytes {
            break;
        }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(file_type);
        out.push(name_len);
        out.extend_from_slice(name_bytes);
    }

    let mut resp = Vec::with_capacity(5 + out.len());
    resp.push(E_OK);
    resp.extend_from_slice(&(out.len() as u32).to_le_bytes());
    resp.extend_from_slice(&out);
    send_resp(resp_port, &resp);
}

fn handle_stat(resp_port: PortHandle, payload: &[u8]) {
    if payload.len() < 8 {
        send_err(resp_port, E_INVAL);
        return;
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);

    let mode;
    let size;
    let ino = handle;

    if handle == 0 {
        mode = S_IFDIR | 0o555;
        size = 0;
    } else {
        let idx = (handle - 1) as usize;
        if let Some(text) = render_disk_info(idx) {
            mode = S_IFREG | 0o444;
            size = text.len() as u64;
        } else {
            send_err(resp_port, E_NOENT);
            return;
        }
    }

    let mut resp = [0u8; 21];
    resp[0] = E_OK;
    resp[1..5].copy_from_slice(&mode.to_le_bytes());
    resp[5..13].copy_from_slice(&size.to_le_bytes());
    resp[13..21].copy_from_slice(&ino.to_le_bytes());
    send_resp(resp_port, &resp);
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("disk_probe: starting VFS provider");

    // Wait a brief moment to let storage drivers populate graph
    stem::sleep(core::time::Duration::from_millis(500));

    let (req_write, req_read) = match port_create(VFS_RPC_MAX_REQ * 8) {
        Ok(p) => p,
        Err(e) => {
            info!("disk_probe: failed to create provider port: {:?}", e);
            loop {
                stem::sleep(core::time::Duration::from_secs(60));
            }
        }
    };

    match vfs_mount(req_write, "/dev/disks") {
        Ok(()) => {
            info!(
                "disk_probe: mounted at /dev/disks (provider port w={} r={})",
                req_write, req_read
            );
        }
        Err(e) => {
            info!("disk_probe: vfs_mount failed: {:?}", e);
            loop {
                stem::sleep(core::time::Duration::from_secs(60));
            }
        }
    }

    let mut req_buf = alloc::vec![0u8; VFS_RPC_MAX_REQ];

    loop {
        match port_wait(&[req_read], abi::syscall::port_wait::READABLE) {
            Ok(_) => {}
            Err(_) => {
                stem::sleep(core::time::Duration::from_millis(10));
                continue;
            }
        }

        match port_recv(req_read, &mut req_buf) {
            Ok(n) if n > 0 => {
                handle_vfs_rpc(&req_buf[..n]);
            }
            _ => {}
        }
    }
}
