#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod driver;
mod vfs_provider;

use abi::vfs_rpc::VFS_RPC_MAX_REQ;
use alloc::vec;
use stem::{info, warn};
use stem::syscall::{channel_create, channel_try_recv, vfs_mount};
use driver::BootFbDriver;
use vfs_provider::handle_vfs_rpc;

#[stem::main]
fn main(boot_fd: usize) -> ! {
    info!("display_bootfb: Starting VFS-native bootfb driver...");

    // 1. Map bootstrap memfd to get handles
    let mut drv_req_read = 0;
    let mut drv_resp_write = 0;
    let mut supervisor_port = 0;
    let mut bind_instance_id = 0u64;

    if boot_fd != 0 {
        use abi::vm::{VmBacking, VmMapReq, VmProt, VmMapFlags};
        let req = VmMapReq {
            addr_hint: 0,
            len: 4096,
            prot: VmProt::READ | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { fd: boot_fd as u32, offset: 0 },
        };
        if let Ok(resp) = stem::syscall::vm_map(&req) {
            let slice = unsafe { core::slice::from_raw_parts(resp.addr as *const u32, 1024) };
            
            // Layout from sprout/src/pipelines.rs:
            // slice[0]: drv_req_read
            // slice[1]: drv_resp_write
            // slice[2]: supervisor_port
            // slice[3..5]: bind_instance_id (u64)
            
            drv_req_read = slice[0];
            drv_resp_write = slice[1];
            supervisor_port = slice[2];
            
            let id_low = slice[3] as u64;
            let id_high = slice[4] as u64;
            bind_instance_id = id_low | (id_high << 32);

            info!("display_bootfb: Bootstrap handles: req_read={}, resp_write={}, svc={}, id={}", 
                drv_req_read, drv_resp_write, supervisor_port, bind_instance_id);
        } else {
            warn!("display_bootfb: Failed to map bootstrap memfd!");
        }
    } else {
        stem::info!("display_bootfb: ERROR: No bootstrap memfd arg provided");
    }

    if drv_req_read == 0 || drv_resp_write == 0 || supervisor_port == 0 || bind_instance_id == 0 {
        stem::info!("display_bootfb: ERROR: Invalid/Missing bootstrap components (req={}, resp={}, svc={}, id={})", 
            drv_req_read, drv_resp_write, supervisor_port, bind_instance_id);
        loop { stem::yield_now(); }
    }

    let mut driver = match BootFbDriver::new() {
        Some(d) => {
            info!("display_bootfb: Driver initialized successfully ({}x{})", d.fb.width, d.fb.height);
            info!("display_bootfb: Mapping framebuffer (backing fd={})...", boot_fd);
            d
        }
        None => {
            info!("display_bootfb: ERROR: Failed to acquire hardware framebuffer");
            loop {
                stem::yield_now();
            }
        }
    };

    // Create the VFS provider port pair.
    let (vfs_write, vfs_read) = match channel_create(VFS_RPC_MAX_REQ * 8) {
        Ok(handles) => handles,
        Err(e) => {
             info!("display_bootfb: ERROR: Failed to create provider port: {:?}", e);
             loop { stem::yield_now(); }
        }
    };

    // Sovereign Handshake
    use abi::supervisor_protocol::{self, classes};
    use abi::display_driver_protocol;

    let ready = supervisor_protocol::BindReadyPayload {
        bind_instance_id,
        class_mask: classes::DISPLAY_CARD | classes::FRAMEBUFFER,
        _reserved: 0,
    };
    let mut ready_bytes = [0u8; supervisor_protocol::BIND_READY_PAYLOAD_SIZE];
    if let Some(len) = supervisor_protocol::encode_bind_ready_le(&ready, &mut ready_bytes) {
        let mut buf = [0u8; 256];
        if let Some(total_len) = display_driver_protocol::encode_message(&mut buf, supervisor_protocol::MSG_BIND_READY, &ready_bytes[..len]) {
            info!("display_bootfb: Sending MSG_BIND_READY handshake...");
            // Send handle FIRST, then notify
            let _ = stem::syscall::channel_send_handle(supervisor_port, vfs_write);
            let _ = stem::syscall::channel_send_all(supervisor_port, &buf[..total_len]);
            info!("display_bootfb: Sent MSG_BIND_READY, waiting for MSG_BIND_ASSIGNED...");
        }
    }

    // Wait for MSG_BIND_ASSIGNED
    let mut wait_buf = [0u8; 512];
    loop {
        // Read from drv_req_read, NOT supervisor_port!
        if let Ok(n) = stem::syscall::channel_try_recv(drv_req_read, &mut wait_buf) {
            if let Some((header, payload)) = display_driver_protocol::parse_message(&wait_buf[..n]) {
                if header.msg_type == supervisor_protocol::MSG_BIND_ASSIGNED {
                    if let Some(assigned) = supervisor_protocol::decode_bind_assigned_le(payload) {
                        let path_len = assigned.primary_path.iter().position(|&b| b == 0).unwrap_or(64);
                        let path = core::str::from_utf8(&assigned.primary_path[..path_len]).unwrap_or("?");
                        info!("display_bootfb: Sovereign registration COMPLETE. Assigned: {}", path);
                        break;
                    }
                }
            }
        }
        stem::time::sleep_ms(10);
    }

    let mut req_buf = vec![0u8; VFS_RPC_MAX_REQ];
    loop {
        match channel_try_recv(vfs_read, &mut req_buf) {
            Ok(n) if n > 0 => {
                handle_vfs_rpc(&mut driver, &req_buf[..n]);
            }
            _ => {
                stem::time::sleep_ms(1);
            }
        }
    }
}
