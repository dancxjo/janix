#![no_std]
#![no_main]

//! # Network Service (netd)
//!
//! Provides networking capabilities using smoltcp TCP/IP stack.
//! - Connects to virtio-net NIC via syscalls
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

use alloc::format;
use smol_device::ThingNicDevice;
use smoltcp::iface::{Config, Interface};
use smoltcp::wire::EthernetAddress;
use stem::{info, warn};

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NETD: Starting network service...");

    // Get MAC address from NIC
    let mut mac_buf = [0u8; 6];
    if let Err(_) = stem::pal::net::nic_mac(&mut mac_buf) {
        warn!("NETD: Failed to get MAC address, no NIC available");
        loop {
            stem::time::sleep_ms(1000);
        }
    }

    info!(
        "NETD: MAC address: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac_buf[0], mac_buf[1], mac_buf[2], mac_buf[3], mac_buf[4], mac_buf[5]
    );

    // Wait for link up
    info!("NETD: Waiting for link...");
    loop {
        if let Ok(link) = stem::pal::net::nic_link_up() {
            if link {
                info!("NETD: Link is up");
                break;
            }
        }
        stem::time::sleep_ms(100);
    }

    // Create smoltcp interface
    let mac_addr = EthernetAddress(mac_buf);
    let mut device = ThingNicDevice::new();
    
    let config = Config::new(mac_addr.into());
    let mut iface = Interface::new(config, &mut device, ThingNicDevice::now());
    
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
