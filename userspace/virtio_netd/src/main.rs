//! VirtIO-NET userspace driver
//!
//! This service owns the VirtIO-NET device hardware and shuttles Ethernet frames
//! to/from the network stack (netd) via IPC ports.
//!
//! Architecture:
//! - virtio_netd: Hardware driver (RX/TX queues, DMA buffers, interrupts)
//! - netd: Network stack (smoltcp, DHCP, DNS, socket API)
//! - fetchd: Demo app (HTTP client, document parsing)

#![no_std]
#![no_main]

extern crate alloc;

mod driver;
mod protocol;

use alloc::vec::Vec;
use abi::schema::keys;
use driver::VirtioNetDriver;
use protocol::{NetDriverMsg, MSG_FRAME_RX, MSG_FRAME_TX, MSG_LINK_DOWN, MSG_LINK_UP, MSG_MAC_REQ, MSG_MAC_RESP};
use stem::syscall::port::{port_create, port_recv, port_send, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{info, warn, error};

/// Graph kind for the network driver service
const KIND_NET_DRIVER: &str = "svc.net.Driver";

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("VIRTIO_NETD: Starting VirtIO-NET driver service...");

    // Initialize VirtIO-NET driver
    let mut driver = match VirtioNetDriver::find_and_claim() {
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

    // Get MAC address and link status
    let mac = driver.mac();
    info!(
        "VIRTIO_NETD: MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    );

    // Wait for link up before proceeding
    info!("VIRTIO_NETD: Waiting for link...");
    loop {
        if driver.link_up() {
            info!("VIRTIO_NETD: Link is UP");
            break;
        }
        stem::time::sleep_ms(100);
    }

    // Create port for frame communication
    // write_handle is published to graph for netd to send TX frames
    // read_handle is used by us to receive TX requests from netd
    let (write_handle, read_handle) = match port_create(65536) {
        Ok(handles) => {
            info!("VIRTIO_NETD: Created port (write={}, read={})", handles.0, handles.1);
            handles
        }
        Err(e) => {
            error!("VIRTIO_NETD: Failed to create port: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // Create a second port for RX frames (driver -> netd)
    // We publish this read handle so netd can receive RX frames
    let (rx_write_handle, rx_read_handle) = match port_create(65536) {
        Ok(handles) => {
            info!("VIRTIO_NETD: Created RX port (write={}, read={})", handles.0, handles.1);
            handles
        }
        Err(e) => {
            error!("VIRTIO_NETD: Failed to create RX port: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // Create service node in graph and publish port handles
    let svc_id = match thingsys::create_node(KIND_NET_DRIVER) {
        Ok(id) => {
            info!("VIRTIO_NETD: Created service node {:?}", id);
            id
        }
        Err(e) => {
            error!("VIRTIO_NETD: Failed to create service node: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // Publish MAC address as property (packed into u64)
    let mac_packed = 
        (mac[0] as u64) |
        ((mac[1] as u64) << 8) |
        ((mac[2] as u64) << 16) |
        ((mac[3] as u64) << 24) |
        ((mac[4] as u64) << 32) |
        ((mac[5] as u64) << 40);
    thingsys::prop_set(svc_id, "net.mac", mac_packed).ok();

    // Publish port handles
    // TX port: netd writes to this to send frames to hardware
    thingsys::prop_set(svc_id, keys::WRITE_PORT_HANDLE, write_handle as u64).ok();
    // RX port: netd reads from this to receive frames from hardware
    thingsys::prop_set(svc_id, "net.rx_port", rx_read_handle as u64).ok();
    
    info!("VIRTIO_NETD: Published service - TX port={}, RX port={}", write_handle, rx_read_handle);

    // Main loop: shuttle frames between hardware and netd
    let mut rx_buf = [0u8; 2048];
    loop {
        // Poll hardware for received frames
        if let Some(frame) = driver.poll_rx() {
            // Send frame to netd via RX port
            let msg = NetDriverMsg::new(MSG_FRAME_RX, frame);
            if let Err(e) = port_send(rx_write_handle, &msg.encode()) {
                warn!("VIRTIO_NETD: Failed to send RX frame to netd: {:?}", e);
            }
        }

        // Check for TX requests from netd (non-blocking)
        match port_recv(read_handle, &mut rx_buf) {
            Ok(len) if len > 0 => {
                if let Some(msg) = NetDriverMsg::decode(&rx_buf[..len]) {
                    match msg.msg_type {
                        MSG_FRAME_TX => {
                            // Transmit frame to hardware
                            if let Err(e) = driver.tx(&msg.payload) {
                                warn!("VIRTIO_NETD: TX failed: {}", e);
                            }
                        }
                        MSG_MAC_REQ => {
                            // Respond with MAC address via RX port
                            let mac_msg = NetDriverMsg::new(MSG_MAC_RESP, &mac);
                            let _ = port_send(rx_write_handle, &mac_msg.encode());
                        }
                        _ => {
                            warn!("VIRTIO_NETD: Unknown message type: 0x{:04x}", msg.msg_type);
                        }
                    }
                }
            }
            _ => {}
        }

        // Yield to prevent busy-waiting
        stem::yield_now();
    }
}
