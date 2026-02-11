//! ISO_CAT: Read files from ISO9660 filesystems via Tree Provider
//!
//! Demonstrates tree provider RPC protocol for filesystem navigation.

extern crate alloc;
extern crate stem;

use abi::schema;
use abi::tree_provider::*;
use abi::ThingId;
use alloc::string::String;
use alloc::vec::Vec;
use core::ptr;
use stem::info;
use stem::syscall::{exit, port_recv, port_send};
use stem::thing::query::query_nodes_by_kind;
use stem::thing::sys::prop_get;

const TEST_PATHS: &[&str] = &[
    "/BOOT/LIMINE.CFG",
    "/README",
    "/README.TXT",
    "/boot/limine.cfg",
    "/readme",
    "/readme.txt",
];

/// Find first ISO9660 CONTENT_SOURCE node and return its tree provider port handle
fn find_iso_provider() -> Option<(ThingId, u32)> {
    let mut nodes = [ThingId::default(); 16];
    let count = query_nodes_by_kind(schema::kinds::CONTENT_SOURCE, 16, &mut nodes).ok()?;

    for i in 0..count {
        let node = nodes[i];
        // Check if this is an ISO9660 source
        if let Ok(_kind_val) = prop_get(node, schema::keys::CONTENT_SOURCE_KIND) {
            // kind_val is the symbol ID for the kind string
            // We need to check if it matches "iso9660_disk" or similar
            // For now, just try to get the tree provider port from any content source
            // In a real impl, we'd check the kind string
            if let Ok(port_handle) = prop_get(node, schema::keys::WRITE_PORT_HANDLE) {
                info!(
                    "ISO_CAT: Found content source node {:?} with tree provider port {}",
                    node, port_handle
                );
                return Some((node, port_handle as u32));
            }
        }
    }

    None
}

/// Send a tree provider request and receive response
fn tree_rpc(port: u32, request_type: TreeProviderRequest, request_data: &[u8]) -> Option<Vec<u8>> {
    // Create request buffer: [request_type: u8, data...]
    let mut req_buf = Vec::new();
    req_buf.push(request_type as u8);
    req_buf.extend_from_slice(request_data);

    // Send request
    if port_send(port, &req_buf).is_err() {
        info!("ISO_CAT: port_send failed");
        return None;
    }

    // Receive response
    let mut resp_buf = alloc::vec![0u8; 8192];
    match port_recv(port, &mut resp_buf) {
        Ok(n) => {
            resp_buf.truncate(n);
            Some(resp_buf)
        }
        Err(_) => {
            info!("ISO_CAT: port_recv failed");
            None
        }
    }
}

/// Get root node ID from tree provider
fn get_root(port: u32) -> Option<u64> {
    let response = tree_rpc(port, TreeProviderRequest::Root, &[])?;

    if response.is_empty() {
        return None;
    }

    // Check response type
    let resp_type = response[0];
    if resp_type != TreeProviderResponse::Ok as u8 {
        info!("ISO_CAT: Root request failed with error");
        return None;
    }

    // Parse RootResponse
    if response.len() < 1 + core::mem::size_of::<RootResponse>() {
        return None;
    }

    let root_resp: RootResponse =
        unsafe { ptr::read_unaligned(response[1..].as_ptr() as *const RootResponse) };

    Some(root_resp.node_id)
}

/// List children of a node
fn list_children(port: u32, node_id: u64) -> Option<Vec<(u64, String, NodeKind, u64)>> {
    let req = ListRequest { node_id };
    let req_bytes = unsafe {
        core::slice::from_raw_parts(
            &req as *const _ as *const u8,
            core::mem::size_of::<ListRequest>(),
        )
    };

    let response = tree_rpc(port, TreeProviderRequest::List, req_bytes)?;

    if response.is_empty() {
        return None;
    }

    // Check response type
    let resp_type = response[0];
    if resp_type != TreeProviderResponse::Ok as u8 {
        info!("ISO_CAT: List request failed");
        return None;
    }

    // Parse ListResponseHeader
    if response.len() < 1 + core::mem::size_of::<ListResponseHeader>() {
        return None;
    }

    let header: ListResponseHeader =
        unsafe { ptr::read_unaligned(response[1..].as_ptr() as *const ListResponseHeader) };

    let mut children = Vec::new();
    let mut offset = 1 + core::mem::size_of::<ListResponseHeader>();

    for _ in 0..header.count {
        if offset + core::mem::size_of::<ChildEntry>() > response.len() {
            break;
        }

        let entry: ChildEntry =
            unsafe { ptr::read_unaligned(response[offset..].as_ptr() as *const ChildEntry) };

        offset += core::mem::size_of::<ChildEntry>();

        // Read name
        let name_len = entry.name_len as usize;
        if offset + name_len > response.len() {
            break;
        }

        let name_bytes = &response[offset..offset + name_len];
        let name = String::from_utf8_lossy(name_bytes).into_owned();
        offset += name_len;

        let kind = match entry.kind {
            0 => NodeKind::Directory,
            1 => NodeKind::File,
            2 => NodeKind::Symlink,
            _ => NodeKind::Other,
        };

        children.push((entry.node_id, name, kind, entry.size));
    }

    Some(children)
}

/// Navigate to a path by walking the tree
fn navigate_path(port: u32, root: u64, path: &str) -> Option<(u64, NodeKind, u64)> {
    let mut current = root;

    // Split path and filter empty segments
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if segments.is_empty() {
        return Some((root, NodeKind::Directory, 0));
    }

    let num_segments = segments.len();
    for (idx, segment) in segments.iter().enumerate() {
        let children = list_children(port, current)?;

        // Find matching child (case-insensitive)
        let mut found = None;
        for (child_id, child_name, child_kind, child_size) in children {
            if child_name.eq_ignore_ascii_case(segment) {
                found = Some((child_id, child_kind, child_size));
                break;
            }
        }

        if let Some((child_id, child_kind, child_size)) = found {
            current = child_id;

            // If this is the last segment, return it
            if idx == num_segments - 1 {
                return Some((current, child_kind, child_size));
            }
        } else {
            info!("ISO_CAT: Path segment '{}' not found", segment);
            return None;
        }
    }

    Some((current, NodeKind::Directory, 0))
}

/// Read data from a file node
fn read_file(port: u32, node_id: u64, offset: u64, length: u32) -> Option<Vec<u8>> {
    let req = ReadRequest {
        node_id,
        offset,
        length,
    };
    let req_bytes = unsafe {
        core::slice::from_raw_parts(
            &req as *const _ as *const u8,
            core::mem::size_of::<ReadRequest>(),
        )
    };

    let response = tree_rpc(port, TreeProviderRequest::Read, req_bytes)?;

    if response.is_empty() {
        return None;
    }

    // Check response type
    let resp_type = response[0];
    if resp_type != TreeProviderResponse::Ok as u8 {
        info!("ISO_CAT: Read request failed");
        return None;
    }

    // Parse ReadResponse
    if response.len() < 1 + core::mem::size_of::<ReadResponse>() {
        return None;
    }

    let read_resp: ReadResponse =
        unsafe { ptr::read_unaligned(response[1..].as_ptr() as *const ReadResponse) };

    let data_offset = 1 + core::mem::size_of::<ReadResponse>();
    let data_len = read_resp.data_len as usize;

    if data_offset + data_len > response.len() {
        return None;
    }

    Some(response[data_offset..data_offset + data_len].to_vec())
}

/// Print hex dump with ASCII preview
fn print_hexdump(data: &[u8], max_bytes: usize) {
    let len = core::cmp::min(data.len(), max_bytes);

    for offset in (0..len).step_by(16) {
        let mut hex_part = alloc::string::String::new();
        let mut ascii_part = alloc::string::String::new();

        for i in 0..16 {
            if offset + i < len {
                let byte = data[offset + i];
                use core::fmt::Write;
                let _ = write!(&mut hex_part, "{:02x} ", byte);

                if byte >= 0x20 && byte <= 0x7e {
                    ascii_part.push(byte as char);
                } else {
                    ascii_part.push('.');
                }
            } else {
                hex_part.push_str("   ");
            }
        }

        info!("{:08x}  {}  |{}|", offset, hex_part, ascii_part);
    }
}

/// List directory contents
fn list_directory(port: u32, node_id: u64) {
    if let Some(children) = list_children(port, node_id) {
        info!("ISO_CAT: Directory listing ({} entries):", children.len());
        for (_, name, kind, size) in children {
            let kind_str = match kind {
                NodeKind::Directory => "DIR ",
                NodeKind::File => "FILE",
                NodeKind::Symlink => "LINK",
                NodeKind::Other => "????",
            };
            info!("  {} {:>10} {}", kind_str, size, name);
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ISO_CAT: Starting");

    // Find ISO9660 mount
    let (node_id, port_handle) = match find_iso_provider() {
        Some(x) => x,
        None => {
            info!("ISO_CAT: No ISO9660 mount found");
            exit(1);
        }
    };

    info!(
        "ISO_CAT: Found ISO9660 mount (node={:?}, port={})",
        node_id, port_handle
    );

    // Get root node
    let root_id = match get_root(port_handle) {
        Some(id) => id,
        None => {
            info!("ISO_CAT: Failed to get root node");
            exit(1);
        }
    };

    info!("ISO_CAT: Root node ID = {}", root_id);

    // Try to read test files
    let mut found_file = false;
    for test_path in TEST_PATHS {
        info!("ISO_CAT: Trying to read {}", test_path);

        if let Some((file_id, kind, size)) = navigate_path(port_handle, root_id, test_path) {
            match kind {
                NodeKind::File => {
                    info!("ISO_CAT: Found file {} ({} bytes)", test_path, size);
                    found_file = true;

                    // Read first 512 bytes
                    let read_len = core::cmp::min(512, size as u32);
                    if let Some(data) = read_file(port_handle, file_id, 0, read_len) {
                        info!("ISO_CAT: Read {} bytes:", data.len());
                        print_hexdump(&data, 512);
                    } else {
                        info!("ISO_CAT: Failed to read file data");
                    }

                    break;
                }
                NodeKind::Directory => {
                    info!("ISO_CAT: {} is a directory", test_path);
                }
                _ => {
                    info!("ISO_CAT: {} is not a regular file", test_path);
                }
            }
        }
    }

    // If no file found, list root directory
    if !found_file {
        info!("ISO_CAT: No test files found, listing root directory:");
        list_directory(port_handle, root_id);
    }

    info!("ISO_CAT: Done");
    exit(0);
}
