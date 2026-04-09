//! ISO9660 VFS Provider (iso9660d)
//!
//! Discovers block devices in the system graph, probes them for ISO9660
//! filesystems, and mounts the first one found as a userland VFS provider at
//! `/mnt/iso` using the janix Act V VFS provider mechanism.
//!
//! ## How it works
//!
//! 1. **Discovery** — scans the graph for DEV_STORAGE_BLOCK_DEVICE nodes.
//! 2. **Probing** — reads each block device and looks for a valid ISO9660 PVD.
//! 3. **Mounting** — calls `SYS_FS_MOUNT(provider_port, "/mnt/iso")` so the
//!    kernel routes VFS operations here.
//! 4. **Service loop** — waits for VFS RPC messages on its request port,
//!    dispatches them to the [`IsoFs`] library, and sends responses back
//!    through the response port handle embedded in each request header.
//!
//! ## VFS RPC protocol
//!
//! See [`abi::vfs_rpc`] for the full wire format.  This service implements:
//! - `Lookup` — resolve a path to a provider handle (encoded as `(lba << 32) | size`)
//! - `Read`   — read file bytes from a handle
//! - `Readdir`— list directory entries for a handle
//! - `Stat`   — return mode/size/ino for a handle
//! - `Close`  — no-op (handles are stateless in this implementation)
//! - `Write`  — returns `EROFS` (ISO9660 is read-only)
//! - `Poll`   — returns `POLLIN` always

#![feature(restricted_std)]
#![no_main]

extern crate alloc;

use abi::block_device_protocol::{
    BlockDeviceError, BlockDeviceRequest, BlockDeviceResponse, ReadRequest, ReadResponse,
};
use abi::schema::{keys, kinds};
use abi::vfs_rpc::{VfsRpcOp, VfsRpcReqHeader, VFS_RPC_MAX_REQ};
use alloc::vec::Vec;
use iso9660::{IsoFs, ISO_SECTOR_SIZE};
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::block::{BlockDevice, BlockError};
use stem::syscall::{
    channel_create, channel_recv, channel_send, channel_wait, vfs_mount, ChannelHandle,
};
use stem::syscall::vfs::{vfs_close, vfs_mkdir, vfs_open, vfs_read, vfs_readdir};
use stem::{info, warn};

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Service,
    device_kind: *b"svc.vfs.iso9660\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

// ── Block device adapter ─────────────────────────────────────────────────────

/// [`BlockDevice`] implementation that talks to a virtio/ATA driver via ports.
struct PortBlockDevice {
    port: ChannelHandle,
    resp_w: ChannelHandle,
    resp_r: ChannelHandle,
}

impl PortBlockDevice {
    fn new(port: ChannelHandle) -> Option<Self> {
        let (resp_w, resp_r) = channel_create(256 * 1024).ok()?;
        Some(Self {
            port,
            resp_w,
            resp_r,
        })
    }
}

impl BlockDevice for PortBlockDevice {
    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        let mut req = [0u8; 4 + 1 + core::mem::size_of::<ReadRequest>()];
        req[0..4].copy_from_slice(&(self.resp_w as u32).to_le_bytes());
        req[4] = BlockDeviceRequest::Read as u8;
        let read_req = ReadRequest {
            lba,
            sector_count: count as u32,
        };
        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &read_req as *const ReadRequest as *const u8,
                core::mem::size_of::<ReadRequest>(),
            )
        };
        req[5..].copy_from_slice(req_bytes);
        channel_send(self.port, &req).map_err(|_| BlockError::IoError)?;

        let expected =
            core::mem::size_of::<ReadResponse>() + (count as usize * ISO_SECTOR_SIZE as usize) + 1;
        let mut resp_buf = alloc::vec![0u8; expected];
        let n = channel_recv(self.resp_r, &mut resp_buf).map_err(|_| BlockError::IoError)?;
        if n < core::mem::size_of::<ReadResponse>() + 1 {
            return Err(BlockError::IoError);
        }
        let resp_type = resp_buf[0];
        if resp_type != BlockDeviceResponse::Ok as u8 {
            if resp_type == BlockDeviceResponse::Error as u8 && n >= 5 {
                return Err(match resp_buf[1] {
                    x if x == BlockDeviceError::InvalidParam as u8 => BlockError::InvalidParam,
                    x if x == BlockDeviceError::IoError as u8 => BlockError::IoError,
                    x if x == BlockDeviceError::NotReady as u8 => BlockError::NotReady,
                    x if x == BlockDeviceError::OutOfRange as u8 => BlockError::OutOfRange,
                    _ => BlockError::NotSupported,
                });
            }
            return Err(BlockError::IoError);
        }
        let header_bytes = &resp_buf[1..1 + core::mem::size_of::<ReadResponse>()];
        let data_len = u32::from_le_bytes([
            header_bytes[0],
            header_bytes[1],
            header_bytes[2],
            header_bytes[3],
        ]) as usize;
        let data_start = 1 + core::mem::size_of::<ReadResponse>();
        let data_end = data_start + data_len;
        if data_end > n || data_len > buf.len() {
            return Err(BlockError::InvalidParam);
        }
        buf[..data_len].copy_from_slice(&resp_buf[data_start..data_end]);
        Ok(())
    }

    fn sector_size(&self) -> u64 {
        ISO_SECTOR_SIZE
    }

    fn sector_count(&self) -> Option<u64> {
        None
    }
}

// ── Handle encoding ───────────────────────────────────────────────────────────

/// Encode `(lba, size)` into a 64-bit provider handle.
///
/// The high 32 bits hold the LBA, the low 32 bits hold the extent size in
/// bytes.  This encoding is identical to the one used by the old Tree Provider
/// implementation and lets us reconstruct extent info from the handle without
/// any additional state.
fn encode_handle(lba: u32, size: u32) -> u64 {
    ((lba as u64) << 32) | (size as u64)
}

fn decode_handle(h: u64) -> (u32, u32) {
    ((h >> 32) as u32, (h & 0xFFFF_FFFF) as u32)
}

// ── VFS RPC dispatch ─────────────────────────────────────────────────────────

/// File type bits (same as kernel VfsStat::S_IFxxx).
const S_IFDIR: u32 = 0o040000;
const S_IFREG: u32 = 0o100000;

/// Errno values we return in VFS RPC responses.
const E_OK: u8 = 0;
const E_NOENT: u8 = 2;
const E_IO: u8 = 5;
const E_ROFS: u8 = 30;
const E_INVAL: u8 = 22;
const E_NOTSUP: u8 = 38; // ENOSYS — used as "not supported"

/// Send a VFS RPC response to `resp_port`.
fn send_resp(resp_port: ChannelHandle, data: &[u8]) {
    let _ = channel_send(resp_port, data);
}

/// Send an error response.
fn send_err(resp_port: ChannelHandle, errno: u8) {
    send_resp(resp_port, &[errno]);
}

/// Handle one VFS RPC request for a mounted ISO9660 filesystem.
fn handle_vfs_rpc(fs: &IsoFs, dev: &PortBlockDevice, buf: &[u8]) {
    if buf.len() < core::mem::size_of::<VfsRpcReqHeader>() {
        return;
    }

    // Parse header.
    let hdr_size = core::mem::size_of::<VfsRpcReqHeader>();
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
        VfsRpcOp::Lookup => handle_lookup(fs, dev, resp_port, payload),
        VfsRpcOp::Read => handle_read(fs, dev, resp_port, payload),
        VfsRpcOp::Write => send_err(resp_port, E_ROFS),
        VfsRpcOp::Readdir => handle_readdir(fs, dev, resp_port, payload),
        VfsRpcOp::Stat => handle_stat(fs, dev, resp_port, payload),
        VfsRpcOp::Close => send_resp(resp_port, &[E_OK]),
        VfsRpcOp::Poll => {
            // Always report readable.
            let mut r = [0u8; 5];
            r[0] = E_OK;
            r[1..5].copy_from_slice(&1u32.to_le_bytes()); // POLLIN
            send_resp(resp_port, &r);
        }
        VfsRpcOp::DeviceCall => send_err(resp_port, E_NOTSUP),
    }
}

/// LOOKUP: resolve a path within the ISO and return a handle.
fn handle_lookup(fs: &IsoFs, dev: &PortBlockDevice, resp_port: ChannelHandle, payload: &[u8]) {
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

    // Empty path means the mount-point root directory.
    let (lba, size, is_dir) = if path.is_empty() || path == "/" {
        (fs.pvd.root_dir_extent, fs.pvd.root_dir_size, true)
    } else {
        match fs.lookup_path(dev, path) {
            Some(entry) => (entry.extent_lba, entry.size, entry.is_directory),
            None => {
                send_err(resp_port, E_NOENT);
                return;
            }
        }
    };

    let handle = encode_handle(lba, size);
    let mut resp = [0u8; 9];
    resp[0] = E_OK;
    resp[1..9].copy_from_slice(&handle.to_le_bytes());
    // Embed the directory flag in handle bit 63 (high bit of ino is unused).
    // Actually we store it separately in the stat — here we just return handle.
    let _ = is_dir; // used in stat/readdir
    send_resp(resp_port, &resp);
}

/// READ: read file data.
fn handle_read(_fs: &IsoFs, dev: &PortBlockDevice, resp_port: ChannelHandle, payload: &[u8]) {
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
    ]);
    let len = u32::from_le_bytes([payload[16], payload[17], payload[18], payload[19]]) as usize;

    let (lba, size) = decode_handle(handle);

    if offset >= size as u64 {
        // Past EOF — return 0 bytes.
        let mut resp = [0u8; 5];
        resp[0] = E_OK;
        // bytes_read = 0
        send_resp(resp_port, &resp);
        return;
    }

    let iso_file = iso9660::IsoFile {
        extent_lba: lba,
        size,
    };
    let clamped_len = len.min((size as u64 - offset) as usize);

    match iso_file.read_range(dev, offset, clamped_len) {
        Ok(data) => {
            let mut resp = Vec::with_capacity(5 + data.len());
            resp.push(E_OK);
            resp.extend_from_slice(&(data.len() as u32).to_le_bytes());
            resp.extend_from_slice(&data);
            send_resp(resp_port, &resp);
        }
        Err(_) => send_err(resp_port, E_IO),
    }
}

/// READDIR: list directory entries.
///
/// The response data is a sequence of packed `DirentWire` structs followed by
/// the name bytes.  The `offset` parameter is used as an entry index (not a
/// byte offset) for simplicity.
fn handle_readdir(fs: &IsoFs, dev: &PortBlockDevice, resp_port: ChannelHandle, payload: &[u8]) {
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
    ]);
    let max_bytes =
        u32::from_le_bytes([payload[16], payload[17], payload[18], payload[19]]) as usize;

    let (lba, size) = decode_handle(handle);
    let entries = fs.list_dir(dev, lba, size);

    let start_idx = offset as usize;
    let mut out: Vec<u8> = Vec::new();

    for entry in entries.iter().skip(start_idx) {
        let name_bytes = entry.name.as_bytes();
        let name_len = name_bytes.len().min(255) as u8;
        let file_type: u8 = if entry.is_directory { 4 } else { 8 }; // DT_DIR=4, DT_REG=8
        let ino = encode_handle(entry.extent_lba, entry.size);

        // DirentWire: [ino: u64][file_type: u8][name_len: u8][name...]
        let entry_size = 10 + name_len as usize;
        if out.len() + entry_size > max_bytes {
            break;
        }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(file_type);
        out.push(name_len);
        out.extend_from_slice(&name_bytes[..name_len as usize]);
    }

    let mut resp = Vec::with_capacity(5 + out.len());
    resp.push(E_OK);
    resp.extend_from_slice(&(out.len() as u32).to_le_bytes());
    resp.extend_from_slice(&out);
    send_resp(resp_port, &resp);
}

/// STAT: return metadata for a handle.
fn handle_stat(fs: &IsoFs, dev: &PortBlockDevice, resp_port: ChannelHandle, payload: &[u8]) {
    if payload.len() < 8 {
        send_err(resp_port, E_INVAL);
        return;
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);
    let (lba, size) = decode_handle(handle);

    // Determine if this is a directory by checking if the root dir matches or
    // trying to parse the first few bytes of the extent.
    let is_dir = if lba == fs.pvd.root_dir_extent {
        true
    } else {
        // Peek at the extent to see if it looks like a directory (has valid
        // directory records).  For simplicity we check by listing — if we can
        // get entries it's a directory.
        let entries = fs.list_dir(dev, lba, size);
        !entries.is_empty()
    };

    let mode = if is_dir {
        S_IFDIR | 0o555
    } else {
        S_IFREG | 0o444
    };
    let ino = handle; // reuse handle as inode number

    let mut resp = [0u8; 21]; // 1 + 4 + 8 + 8
    resp[0] = E_OK;
    resp[1..5].copy_from_slice(&mode.to_le_bytes());
    resp[5..13].copy_from_slice(&(size as u64).to_le_bytes());
    resp[13..21].copy_from_slice(&ino.to_le_bytes());
    send_resp(resp_port, &resp);
}

// ── Main ─────────────────────────────────────────────────────────────────────

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("iso9660d: starting ISO9660 VFS provider");

    // 1. Find block devices via VFS.
    let mut mounted: Option<(IsoFs, PortBlockDevice, ChannelHandle, ChannelHandle)> = None;

    if let Ok(fd) = vfs_open("/services/storage", abi::syscall::vfs_flags::O_RDONLY) {
        let mut buf = [0u8; 4096];
        if let Ok(n) = vfs_readdir(fd, &mut buf) {
            let mut offset = 0;
            while offset < n {
                let mut end = offset;
                while end < n && buf[end] != 0 {
                    end += 1;
                }
                if end > offset {
                    if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                        let path = alloc::format!("/services/storage/{}", name);
                        if let Ok(h_fd) = vfs_open(&path, abi::syscall::vfs_flags::O_RDONLY) {
                            let mut h_buf = [0u8; 32];
                            if let Ok(h_n) = vfs_read(h_fd, &mut h_buf) {
                                let h_str = core::str::from_utf8(&h_buf[..h_n]).unwrap_or("");
                                if let Ok(port_handle) = h_str.trim().parse::<u32>() {
                                    let block_dev = match PortBlockDevice::new(port_handle as ChannelHandle) {
                                        Some(d) => d,
                                        None => {
                                            let _ = vfs_close(h_fd);
                                            offset = end + 1;
                                            continue;
                                        }
                                    };

                                    if let Some(fs) = IsoFs::probe(&block_dev) {
                                        info!("iso9660d: found ISO9660 on device {}", name);

                                        // 3. Create the provider port pair.
                                        let (req_write, req_read) = match channel_create(VFS_RPC_MAX_REQ * 8) {
                                            Ok(p) => p,
                                            Err(e) => {
                                                warn!("iso9660d: failed to create provider port: {:?}", e);
                                                let _ = vfs_close(h_fd);
                                                offset = end + 1;
                                                continue;
                                            }
                                        };

                                        // 4. Mount via SYS_FS_MOUNT.
                                        match vfs_mount(req_write, "/mnt/iso") {
                                            Ok(()) => {
                                                info!("iso9660d: mounted at /mnt/iso (provider port w={} r={})", req_write, req_read);
                                                mounted = Some((fs, block_dev, req_write, req_read));
                                                let _ = vfs_close(h_fd);
                                                break;
                                            }
                                            Err(e) => {
                                                warn!("iso9660d: vfs_mount failed: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                            let _ = vfs_close(h_fd);
                        }
                    }
                }
                offset = end + 1;
            }
        }
        let _ = vfs_close(fd);
    }

    let (fs, dev, _req_write, req_read) = match mounted {
        Some(m) => m,
        None => {
            info!("iso9660d: no ISO9660 filesystem found — sleeping");
            loop {
                stem::sleep(core::time::Duration::from_secs(60));
            }
        }
    };

    // 5. Service loop.
    info!("iso9660d: entering VFS RPC service loop");
    let mut req_buf = alloc::vec![0u8; VFS_RPC_MAX_REQ];

    loop {
        match channel_wait(&[req_read], abi::syscall::channel_wait::READABLE) {
            Ok(_) => {}
            Err(_) => {
                stem::sleep(core::time::Duration::from_millis(10));
                continue;
            }
        }

        match channel_recv(req_read, &mut req_buf) {
            Ok(n) if n > 0 => {
                handle_vfs_rpc(&fs, &dev, &req_buf[..n]);
            }
            _ => {}
        }
    }
}
