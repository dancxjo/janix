//! VirtIO-NET userspace driver
//!
//! This service owns the VirtIO-NET device hardware and exposes it as a VFS
//! provider mounted at `/dev/net/virtio0/`.
//!
//! Architecture:
//! - virtio_netd: Hardware driver (RX/TX queues, DMA buffers, interrupts)
//!   Exposes files: ctl, status, mac, mtu, rx, tx, features, events
//! - netd / other consumers: talk to the driver purely through file paths

#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod driver;
mod vfs_provider;

use abi::vfs_rpc::VFS_RPC_MAX_REQ;
use alloc::vec;
use driver::VirtioNetDriver;
use stem::syscall::port::{port_create, port_try_recv};
use stem::syscall::vfs_mount;
use stem::{error, info, warn};
use vfs_provider::{handle_vfs_rpc, NetVfsState};

#[stem::main]
fn main(arg: usize) -> ! {
    info!("VIRTIO_NETD: Starting VirtIO-NET driver service...");

    // Initialize VirtIO-NET driver.
    let mut driver = match if arg != 0 {
        VirtioNetDriver::claim_device(arg as u64)
    } else {
        VirtioNetDriver::find_and_claim()
    } {
        Ok(d) => {
            info!("VIRTIO_NETD: Driver initialized successfully");
            d
        }
        Err(e) => {
            error!("VIRTIO_NETD: Failed to initialize driver: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    let mac = driver.mac();
    info!(
        "VIRTIO_NETD: MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    );

    // Wait for link up before proceeding.
    info!("VIRTIO_NETD: Waiting for link...");
    loop {
        if driver.link_up() {
            info!("VIRTIO_NETD: Link is UP");
            break;
        }
        stem::time::sleep_ms(100);
    }

    let features = driver.device_features();

    // Create the VFS provider port pair.
    //   req_write → kernel sends VFS RPCs here
    //   req_read  → this daemon reads RPCs here
    let (req_write, req_read) = match port_create(VFS_RPC_MAX_REQ * 8) {
        Ok(handles) => {
            info!("VIRTIO_NETD: Created VFS provider port");
            handles
        }
        Err(e) => {
            error!("VIRTIO_NETD: Failed to create provider port: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // Mount at /dev/net/virtio0 via SYS_VFS_MOUNT.
    // Pass the write end so the kernel can send us RPCs.
    match vfs_mount(req_write, "/dev/net/virtio0") {
        Ok(()) => {
            info!("VIRTIO_NETD: Mounted at /dev/net/virtio0");
        }
        Err(e) => {
            error!("VIRTIO_NETD: Failed to mount VFS provider: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    }

    // Initialize shared VFS state.
    let mut state = NetVfsState::new(mac, true, features);

    // Main loop: interleave hardware polling with VFS RPC handling.
    let mut req_buf = vec![0u8; VFS_RPC_MAX_REQ];
    loop {
        // 1. Poll for link-state changes and queue events.
        if let Some(link_up) = driver.poll_link_change() {
            state.link_up = link_up;
            let event = if link_up { "link-up" } else { "link-down" };
            state.push_event(event);
            info!("VIRTIO_NETD: Link state changed: {}", event);
        }

        // 2. Poll hardware for received frames and buffer them.
        if let Some(frame) = driver.poll_rx() {
            let frame_vec = frame.to_vec();
            state.push_rx_frame(frame_vec);
        }

        // 3. Service any pending VFS RPC (non-blocking).
        match port_try_recv(req_read, &mut req_buf) {
            Ok(n) if n > 0 => {
                handle_vfs_rpc(&mut state, &mut driver, &req_buf[..n]);
            }
            Err(e) if e != abi::errors::Errno::EAGAIN => {
                warn!("VIRTIO_NETD: port_try_recv error: {:?}", e);
            }
            _ => {}
        }

        stem::time::sleep_ms(1);
    }
}
