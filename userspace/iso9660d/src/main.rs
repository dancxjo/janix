//! ISO9660 Mount Service (iso9660d)
//!
//! Discovers block devices in the graph, probes them for ISO9660 filesystems,
//! and exposes mounted ISOs via the Tree Provider RPC protocol.
//!
//! ## Architecture
//!
//! 1. **Discovery**: Scans graph for DEV_STORAGE_BLOCK_DEVICE nodes
//! 2. **Probing**: Reads each block device's WRITE_PORT_HANDLE and probes for ISO9660
//! 3. **Mounting**: Creates CONTENT_SOURCE nodes for discovered filesystems
//! 4. **Service**: Exposes tree provider ports for file navigation and reading
//!
//! ## Node ID Encoding
//!
//! Tree provider node IDs encode directory/file extent information:
//! - `node_id = (extent_lba << 32) | extent_size`
//! - This allows O(1) lookup without maintaining a separate ID-to-extent map

#![no_std]
#![no_main]

extern crate alloc;

use abi::block_device_protocol::{
    BlockDeviceError, BlockDeviceRequest, BlockDeviceResponse, ReadRequest, ReadResponse,
};
use abi::schema::{keys, kinds};
use abi::tree_provider::{
    ErrorResponse, GetMetadataRequest, ListRequest, NodeKind, ReadRequest as TreeReadRequest,
    RootResponse, TreeProviderError, TreeProviderRequest, TreeProviderResponse,
    MAX_CHILDREN_PER_LIST, MAX_NAME_LEN, MAX_READ_SIZE,
};
use alloc::vec::Vec;
use iso9660::{IsoFs, ISO_SECTOR_SIZE};
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::block::{BlockDevice, BlockError};
use stem::syscall::{port_create, port_recv, port_send, port_wait, PortHandle};
use stem::thing::sys::{create_node, find, intern, prop_get, prop_set};
use stem::thing::ThingId;
use stem::{info, warn};

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Service,
    device_kind: *b"svc.iso9660.Mount\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

/// BlockDevice adapter for port-based RPC
struct PortBlockDevice {
    port: PortHandle,
    sector_size: u32,
    sector_count: u64,
    resp_w: PortHandle,
    resp_r: PortHandle,
}

impl PortBlockDevice {
    fn new(port: PortHandle) -> Option<Self> {
        // Create a response port pair for this device
        // Use a large buffer to accommodate multi-sector reads
        let (resp_w, resp_r) = port_create(128 * 1024).ok()?;

        // For ISO9660, we expect 2048-byte sectors
        // We don't need to identify the device since ISO9660 has fixed sector size
        Some(Self {
            port,
            sector_size: ISO_SECTOR_SIZE as u32,
            sector_count: 0, // Unknown, not needed for ISO9660
            resp_w,
            resp_r,
        })
    }
}

impl BlockDevice for PortBlockDevice {
    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        // Build read request: [4: resp_w][1: request_type][payload...]
        let mut req = [0u8; 4 + 1 + core::mem::size_of::<ReadRequest>()];
        req[0..4].copy_from_slice(&(self.resp_w as u32).to_le_bytes());
        req[4] = BlockDeviceRequest::Read as u8;

        let read_req = ReadRequest {
            lba,
            sector_count: count as u32,
        };

        // Safety: ReadRequest is repr(C) and POD
        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &read_req as *const ReadRequest as *const u8,
                core::mem::size_of::<ReadRequest>(),
            )
        };
        req[5..].copy_from_slice(req_bytes);

        // Send request
        if port_send(self.port, &req).is_err() {
            return Err(BlockError::IoError);
        }

        // Receive response from our private response port
        let expected_size =
            core::mem::size_of::<ReadResponse>() + (count as usize * self.sector_size as usize) + 1;
        let mut resp_buf = alloc::vec![0u8; expected_size];

        let n = port_recv(self.resp_r, &mut resp_buf).map_err(|_| BlockError::IoError)?;
        if n < core::mem::size_of::<ReadResponse>() + 1 {
            return Err(BlockError::IoError);
        }

        // Parse response header
        let resp_type = resp_buf[0];
        if resp_type != BlockDeviceResponse::Ok as u8 {
            // Error response
            if resp_type == BlockDeviceResponse::Error as u8 && n >= 5 {
                let error_code = resp_buf[1];
                return Err(match error_code {
                    x if x == BlockDeviceError::InvalidParam as u8 => BlockError::InvalidParam,
                    x if x == BlockDeviceError::IoError as u8 => BlockError::IoError,
                    x if x == BlockDeviceError::NotReady as u8 => BlockError::NotReady,
                    x if x == BlockDeviceError::OutOfRange as u8 => BlockError::OutOfRange,
                    _ => BlockError::NotSupported,
                });
            }
            return Err(BlockError::IoError);
        }

        // Read header (skip response type byte)
        let header_bytes = &resp_buf[1..1 + core::mem::size_of::<ReadResponse>()];
        let data_len = u32::from_le_bytes([
            header_bytes[0],
            header_bytes[1],
            header_bytes[2],
            header_bytes[3],
        ]);

        let data_start = 1 + core::mem::size_of::<ReadResponse>();
        let data_end = data_start + data_len as usize;

        if data_end > n || data_len as usize > buf.len() {
            return Err(BlockError::InvalidParam);
        }

        buf[..data_len as usize].copy_from_slice(&resp_buf[data_start..data_end]);
        Ok(())
    }

    fn sector_size(&self) -> u64 {
        self.sector_size as u64
    }

    fn sector_count(&self) -> Option<u64> {
        if self.sector_count > 0 {
            Some(self.sector_count)
        } else {
            None
        }
    }
}

/// A mounted ISO9660 filesystem
struct Mount {
    fs: IsoFs,
    device: PortBlockDevice,
    tree_port_read: PortHandle,
    tree_port_write: PortHandle,
    mount_node: ThingId,
}

/// Decode node_id into (extent_lba, extent_size)
fn decode_node_id(node_id: u64) -> (u32, u32) {
    let extent_lba = (node_id >> 32) as u32;
    let extent_size = (node_id & 0xFFFFFFFF) as u32;
    (extent_lba, extent_size)
}

/// Encode (extent_lba, extent_size) into node_id
fn encode_node_id(extent_lba: u32, extent_size: u32) -> u64 {
    ((extent_lba as u64) << 32) | (extent_size as u64)
}

/// Send error response
fn send_error(port: PortHandle, error_code: TreeProviderError) {
    let mut resp = [0u8; 1 + core::mem::size_of::<ErrorResponse>()];
    resp[0] = TreeProviderResponse::Error as u8;
    resp[1] = error_code as u8;
    let _ = port_send(port, &resp);
}

/// Handle tree provider requests for a mount
fn handle_tree_request(mount: &Mount, req_buf: &[u8]) {
    if req_buf.is_empty() {
        send_error(mount.tree_port_write, TreeProviderError::InvalidParam);
        return;
    }

    let req_type = req_buf[0];

    match req_type {
        x if x == TreeProviderRequest::Root as u8 => {
            // Return root directory node ID
            let root_node_id =
                encode_node_id(mount.fs.pvd.root_dir_extent, mount.fs.pvd.root_dir_size);

            let mut resp = [0u8; 1 + core::mem::size_of::<RootResponse>()];
            resp[0] = TreeProviderResponse::Ok as u8;

            let root_resp = RootResponse {
                node_id: root_node_id,
            };

            let resp_bytes = unsafe {
                core::slice::from_raw_parts(
                    &root_resp as *const RootResponse as *const u8,
                    core::mem::size_of::<RootResponse>(),
                )
            };
            resp[1..].copy_from_slice(resp_bytes);

            let _ = port_send(mount.tree_port_write, &resp);
        }

        x if x == TreeProviderRequest::List as u8 => {
            if req_buf.len() < 1 + core::mem::size_of::<ListRequest>() {
                send_error(mount.tree_port_write, TreeProviderError::InvalidParam);
                return;
            }

            let req_bytes = &req_buf[1..];
            let node_id = u64::from_le_bytes([
                req_bytes[0],
                req_bytes[1],
                req_bytes[2],
                req_bytes[3],
                req_bytes[4],
                req_bytes[5],
                req_bytes[6],
                req_bytes[7],
            ]);

            let (extent_lba, extent_size) = decode_node_id(node_id);

            // List directory entries
            let entries = mount.fs.list_dir(&mount.device, extent_lba, extent_size);

            // Build response
            let mut resp_data = Vec::new();

            // Response type
            resp_data.push(TreeProviderResponse::Ok as u8);

            // Header
            let count = core::cmp::min(entries.len(), MAX_CHILDREN_PER_LIST) as u32;
            resp_data.extend_from_slice(&count.to_le_bytes());

            // Entries
            for entry in entries.iter().take(MAX_CHILDREN_PER_LIST) {
                let child_node_id = encode_node_id(entry.extent_lba, entry.size);
                let kind = if entry.is_directory {
                    NodeKind::Directory
                } else {
                    NodeKind::File
                } as u8;

                let name_bytes = entry.name.as_bytes();
                let name_len = core::cmp::min(name_bytes.len(), MAX_NAME_LEN) as u8;

                // Encode ChildEntry
                resp_data.extend_from_slice(&child_node_id.to_le_bytes());
                resp_data.push(kind);
                resp_data.push(name_len);
                resp_data.extend_from_slice(&entry.size.to_le_bytes());
                resp_data.extend_from_slice(&(0u32.to_le_bytes())); // _reserved[0..4]
                resp_data.extend_from_slice(&[0u8, 0u8]); // _reserved[4..6]

                // Name data
                resp_data.extend_from_slice(&name_bytes[..name_len as usize]);
            }

            let _ = port_send(mount.tree_port_write, &resp_data);
        }

        x if x == TreeProviderRequest::Read as u8 => {
            if req_buf.len() < 1 + core::mem::size_of::<TreeReadRequest>() {
                send_error(mount.tree_port_write, TreeProviderError::InvalidParam);
                return;
            }

            let req_bytes = &req_buf[1..];
            let node_id = u64::from_le_bytes([
                req_bytes[0],
                req_bytes[1],
                req_bytes[2],
                req_bytes[3],
                req_bytes[4],
                req_bytes[5],
                req_bytes[6],
                req_bytes[7],
            ]);
            let offset = u64::from_le_bytes([
                req_bytes[8],
                req_bytes[9],
                req_bytes[10],
                req_bytes[11],
                req_bytes[12],
                req_bytes[13],
                req_bytes[14],
                req_bytes[15],
            ]);
            let length =
                u32::from_le_bytes([req_bytes[16], req_bytes[17], req_bytes[18], req_bytes[19]])
                    as usize;

            let (extent_lba, extent_size) = decode_node_id(node_id);

            // Validate length
            if length > MAX_READ_SIZE {
                send_error(mount.tree_port_write, TreeProviderError::InvalidParam);
                return;
            }

            // Validate offset
            if offset >= extent_size as u64 {
                send_error(mount.tree_port_write, TreeProviderError::OutOfRange);
                return;
            }

            // Read file data using IsoFile
            let iso_file = iso9660::IsoFile {
                extent_lba,
                size: extent_size,
            };

            match iso_file.read_range(&mount.device, offset, length) {
                Ok(data) => {
                    let mut resp_data = Vec::with_capacity(1 + 4 + data.len());
                    resp_data.push(TreeProviderResponse::Ok as u8);
                    resp_data.extend_from_slice(&(data.len() as u32).to_le_bytes());
                    resp_data.extend_from_slice(&data);

                    let _ = port_send(mount.tree_port_write, &resp_data);
                }
                Err(_) => {
                    send_error(mount.tree_port_write, TreeProviderError::IoError);
                }
            }
        }

        x if x == TreeProviderRequest::GetMetadata as u8 => {
            if req_buf.len() < 1 + core::mem::size_of::<GetMetadataRequest>() {
                send_error(mount.tree_port_write, TreeProviderError::InvalidParam);
                return;
            }

            let req_bytes = &req_buf[1..];
            let node_id = u64::from_le_bytes([
                req_bytes[0],
                req_bytes[1],
                req_bytes[2],
                req_bytes[3],
                req_bytes[4],
                req_bytes[5],
                req_bytes[6],
                req_bytes[7],
            ]);

            let (extent_lba, extent_size) = decode_node_id(node_id);

            // For root directory, return metadata directly
            if extent_lba == mount.fs.pvd.root_dir_extent
                && extent_size == mount.fs.pvd.root_dir_size
            {
                let mut resp_data = Vec::new();
                resp_data.push(TreeProviderResponse::Ok as u8);
                resp_data.push(NodeKind::Directory as u8);
                resp_data.push(0); // name_len = 0 for root
                resp_data.extend_from_slice(&[0u8; 6]); // _reserved
                resp_data.extend_from_slice(&extent_size.to_le_bytes());
                resp_data.extend_from_slice(&(0u64.to_le_bytes())); // mtime = 0

                let _ = port_send(mount.tree_port_write, &resp_data);
                return;
            }

            // For other nodes, we need to find them by traversing the parent
            // This is a simplified implementation - a full implementation would cache metadata
            send_error(mount.tree_port_write, TreeProviderError::NotSupported);
        }

        _ => {
            send_error(mount.tree_port_write, TreeProviderError::NotSupported);
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ISO9660D: Starting ISO9660 mount service...");

    let mut mounts = Vec::new();

    // 1. Find all block devices in the graph
    let mut devices = [ThingId::default(); 32];
    let count = match find(kinds::DEV_STORAGE_BLOCK_DEVICE, &mut devices) {
        Ok(n) => n,
        Err(_) => {
            warn!("ISO9660D: Failed to find block devices");
            0
        }
    };

    info!("ISO9660D: Found {} block devices", count);

    // 2. Probe each device for ISO9660
    for &dev_id in &devices[..count] {
        // Get the WRITE_PORT_HANDLE property
        let port_handle = match prop_get(dev_id, keys::WRITE_PORT_HANDLE) {
            Ok(handle) => handle as PortHandle,
            Err(_) => {
                warn!("ISO9660D: Device {:?} has no WRITE_PORT_HANDLE", dev_id);
                continue;
            }
        };

        // Create block device adapter
        let block_dev = match PortBlockDevice::new(port_handle) {
            Some(dev) => dev,
            None => continue,
        };

        // Probe for ISO9660
        match IsoFs::probe(&block_dev) {
            Some(fs) => {
                info!("ISO9660D: Found ISO9660 filesystem on device {:?}", dev_id);

                // Create mount node
                let mount_node = match create_node(kinds::CONTENT_SOURCE) {
                    Ok(node) => node,
                    Err(_) => {
                        warn!("ISO9660D: Failed to create mount node");
                        continue;
                    }
                };

                // Set mount properties
                let kind_sym = intern("iso9660").unwrap_or(0);
                let state_sym = intern("ready").unwrap_or(0);
                let _ = prop_set(mount_node, keys::CONTENT_SOURCE_KIND, kind_sym as u64);
                let _ = prop_set(mount_node, keys::CONTENT_SOURCE_STATE, state_sym as u64);
                let _ = prop_set(mount_node, "device", dev_id.to_u64_lossy());

                // Create tree provider port
                let (tree_write, tree_read) = match port_create(8192) {
                    Ok((w, r)) => (w, r),
                    Err(_) => {
                        warn!("ISO9660D: Failed to create tree provider port");
                        continue;
                    }
                };

                // Publish port handle
                let _ = prop_set(mount_node, keys::WRITE_PORT_HANDLE, tree_read as u64);

                info!(
                    "ISO9660D: Mounted ISO9660 at node {:?}, tree port {}",
                    mount_node, tree_read
                );

                mounts.push(Mount {
                    fs,
                    device: block_dev,
                    tree_port_read: tree_read,
                    tree_port_write: tree_write,
                    mount_node,
                });
            }
            None => {
                // Not an ISO9660 filesystem
            }
        }
    }

    if mounts.is_empty() {
        info!("ISO9660D: No ISO9660 filesystems found");
        // Keep running anyway - devices might appear later
    }

    // 3. Service loop - wait on all tree provider ports
    info!(
        "ISO9660D: Entering service loop with {} mounts",
        mounts.len()
    );

    let mut req_buf = [0u8; 8192];

    loop {
        if mounts.is_empty() {
            // No mounts, just sleep
            stem::sleep(core::time::Duration::from_secs(1));
            continue;
        }

        // Collect all read handles
        let port_handles: Vec<PortHandle> = mounts.iter().map(|m| m.tree_port_read).collect();

        // Wait for any port to become readable
        match port_wait(&port_handles, abi::syscall::port_wait::READABLE) {
            Ok(ready_port) => {
                // Find which mount this port belongs to
                if let Some(mount) = mounts.iter().find(|m| m.tree_port_read == ready_port) {
                    // Read request
                    match port_recv(ready_port, &mut req_buf) {
                        Ok(n) if n > 0 => {
                            handle_tree_request(mount, &req_buf[..n]);
                        }
                        Ok(_) => {
                            // Empty message, ignore
                        }
                        Err(_) => {
                            warn!("ISO9660D: Failed to receive request on port {}", ready_port);
                        }
                    }
                }
            }
            Err(_) => {
                // port_wait failed, sleep briefly and retry
                stem::sleep(core::time::Duration::from_millis(100));
            }
        }
    }
}
