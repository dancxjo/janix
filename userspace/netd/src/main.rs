#![no_std]
#![no_main]

//! # Network Service (netd)
//!
//! Provides networking capabilities using smoltcp TCP/IP stack.
//! - Uses userspace VirtIO-NET driver directly
//! - Runs DHCP to acquire IP address
//! - Performs DNS lookups
//! - HTTP client for fetching web content
//! - Stores results in the graph with XML parsing

extern crate alloc;

mod dhcp;
mod dns;
mod graph_sink;
mod http;
mod smol_device;
mod virtio_net;

use alloc::format;
use smol_device::VirtioNicDevice;
use smoltcp::iface::{Config, Interface};
use smoltcp::wire::EthernetAddress;
use stem::{info, warn};
use virtio_net::VirtioNetDriver;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NETD: Starting network service...");

    // Initialize VirtIO-NET driver using find_and_claim
    info!("NETD: Initializing VirtIO-NET driver...");
    
    let mut driver = match VirtioNetDriver::find_and_claim() {
        Ok(d) => {
            info!("NETD: VirtIO-NET driver initialized successfully");
            d
        }
        Err(e) => {
            warn!("NETD: Failed to initialize VirtIO-NET driver: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // Get MAC address from driver
    let mac_buf = driver.mac();
    info!(
        "NETD: MAC address: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac_buf[0], mac_buf[1], mac_buf[2], mac_buf[3], mac_buf[4], mac_buf[5]
    );

    // Wait for link up
    info!("NETD: Waiting for link...");
    loop {
        if driver.link_up() {
            info!("NETD: Link is up");
            break;
        }
        stem::time::sleep_ms(100);
    }

    // Create smoltcp interface
    let mac_addr = EthernetAddress(mac_buf);
    let mut device = VirtioNicDevice::new(&mut driver);
    
    let config = Config::new(mac_addr.into());
    let mut iface = Interface::new(config, &mut device, VirtioNicDevice::now());
    
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

    // DNS lookup for csszengarden.com
    info!("NETD: Resolving csszengarden.com...");
    let target_ip = match dns::lookup_a(
        &mut iface,
        &mut device,
        dhcp_config.dns,
        "csszengarden.com",
    ) {
        Ok(ip) => {
            info!("NETD: Resolved csszengarden.com to {}", ip);
            ip
        }
        Err(e) => {
            warn!("NETD: DNS lookup failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // HTTP GET
    info!("NETD: Fetching http://csszengarden.com/...");
    let response = match http::http_get(&mut iface, &mut device, target_ip, "csszengarden.com", "/") {
        Ok(resp) => {
            info!(
                "NETD: HTTP {} - {} bytes",
                resp.status_code,
                resp.body.len()
            );
            resp
        }
        Err(e) => {
            warn!("NETD: HTTP GET failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // Store in graph
    info!("NETD: Storing result in graph...");
    let url = format!("http://csszengarden.com/");
    match graph_sink::store_fetch_result(&url, response.status_code, &response.body) {
        Ok(node_id) => {
            info!("NETD: Stored result at node {:?}", node_id);
        }
        Err(e) => {
            warn!("NETD: Failed to store in graph: {:?}", e);
        }
    }

    info!("NETD: Network fetch complete, entering idle loop");
    loop {
        stem::time::sleep_ms(1000);
    }
}
