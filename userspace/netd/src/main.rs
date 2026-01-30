#![no_std]
#![no_main]

//! # Network Service (netd)
//!
//! Provides networking capabilities using smoltcp TCP/IP stack.
//! - Connects to virtio_netd for frame I/O via IPC
//! - Runs DHCP to acquire IP address
//! - Provides DNS resolver
//! - Exposes socket API for applications

extern crate alloc;

mod dhcp;
mod dns;
mod driver_protocol;
mod ipc_device;

use alloc::format;
use abi::schema::keys;
use ipc_device::IpcNicDevice;
use smoltcp::iface::{Config, Interface};
use smoltcp::wire::EthernetAddress;
use stem::syscall::port::{port_recv, port_send, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{info, warn, error};

/// Graph kind for the network driver service (published by virtio_netd)
const KIND_NET_DRIVER: &str = "svc.net.Driver";

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NETD: Starting network stack service...");

    // Wait for virtio_netd to be ready
    info!("NETD: Looking for virtio_netd driver service...");
    let (tx_port, rx_port, mac) = loop {
        match find_driver_service() {
            Some(result) => break result,
            None => {
                stem::time::sleep_ms(100);
            }
        }
    };
    
    info!(
        "NETD: Connected to driver - MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    );

    // Create IPC-backed smoltcp device
    let mut device = IpcNicDevice::new(tx_port, rx_port, mac);
    
    // Create smoltcp interface
    let mac_addr = EthernetAddress(mac);
    let config = Config::new(mac_addr.into());
    let mut iface = Interface::new(config, &mut device, IpcNicDevice::now());
    
    // Start DHCP
    info!("NETD: Starting DHCP...");
    let dhcp_config = match dhcp::run_dhcp(&mut iface, &mut device) {
        Ok(cfg) => {
            info!(
                "NETD: DHCP complete - IP: {}, Gateway: {}, DNS: {}",
                cfg.ip, cfg.gateway, cfg.dns
            );
            cfg
        }
        Err(e) => {
            warn!("NETD: DHCP failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // Publish network configuration to graph
    if let Ok(net_id) = thingsys::create_node("svc.net.Stack") {
        // Pack IP address into u64
        let ip_packed = {
            let octets = dhcp_config.ip.as_bytes();
            (octets[0] as u64) |
            ((octets[1] as u64) << 8) |
            ((octets[2] as u64) << 16) |
            ((octets[3] as u64) << 24)
        };
        thingsys::prop_set(net_id, "net.ip", ip_packed).ok();
        
        let gw_packed = {
            let octets = dhcp_config.gateway.as_bytes();
            (octets[0] as u64) |
            ((octets[1] as u64) << 8) |
            ((octets[2] as u64) << 16) |
            ((octets[3] as u64) << 24)
        };
        thingsys::prop_set(net_id, "net.gateway", gw_packed).ok();
        
        let dns_packed = {
            let octets = dhcp_config.dns.as_bytes();
            (octets[0] as u64) |
            ((octets[1] as u64) << 8) |
            ((octets[2] as u64) << 16) |
            ((octets[3] as u64) << 24)
        };
        thingsys::prop_set(net_id, "net.dns", dns_packed).ok();
        
        info!("NETD: Published network configuration to graph");
    }

    info!("NETD: Network stack ready, entering service loop");
    
    // Main service loop - for now just keep interface alive
    // TODO: Implement socket API for applications
    loop {
        let now = IpcNicDevice::now();
        
        // Poll the interface to process any pending packets
        // Note: We need to create an empty socket set for poll
        let mut sockets_storage: [smoltcp::iface::SocketStorage; 0] = [];
        let mut socket_set = smoltcp::iface::SocketSet::new(&mut sockets_storage[..]);
        iface.poll(now, &mut device, &mut socket_set);
        
        stem::time::sleep_ms(10);
    }
}

/// Find the virtio_netd driver service and get port handles + MAC address
fn find_driver_service() -> Option<(PortHandle, PortHandle, [u8; 6])> {
    // Look for svc.net.Driver node
    let mut buf = [ThingId::default(); 1];
    let count = thingsys::find(KIND_NET_DRIVER, &mut buf).ok()?;
    
    if count == 0 {
        return None;
    }
    
    let driver_id = buf[0];
    info!("NETD: Found driver service node {:?}", driver_id);
    
    // Get TX port handle (we write to this)
    let tx_port = thingsys::prop_get(driver_id, keys::WRITE_PORT_HANDLE).ok()? as PortHandle;
    
    // Get RX port handle (we read from this)
    let rx_port = thingsys::prop_get(driver_id, "net.rx_port").ok()? as PortHandle;
    
    // Get MAC address (packed in u64)
    let mac_packed = thingsys::prop_get(driver_id, "net.mac").ok()?;
    let mac = [
        (mac_packed & 0xFF) as u8,
        ((mac_packed >> 8) & 0xFF) as u8,
        ((mac_packed >> 16) & 0xFF) as u8,
        ((mac_packed >> 24) & 0xFF) as u8,
        ((mac_packed >> 32) & 0xFF) as u8,
        ((mac_packed >> 40) & 0xFF) as u8,
    ];
    
    info!("NETD: Driver TX port={}, RX port={}", tx_port, rx_port);
    
    Some((tx_port, rx_port, mac))
}
