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
mod socket_api;

use alloc::format;
use abi::schema::keys;
use ipc_device::IpcNicDevice;
use socket_api::SocketApi;
use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
use smoltcp::wire::EthernetAddress;
use stem::syscall::port::{port_create, port_recv, port_send, PortHandle};
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

    // Create socket API port
    let (api_write_port, api_read_port) = match port_create(8192) {
        Ok((w, r)) => {
            info!("NETD: Created socket API port (write={}, read={})", w, r);
            (w, r)
        }
        Err(e) => {
            error!("NETD: Failed to create socket API port: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // Publish socket API port to graph
    let mut net_buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find("svc.net.Stack", &mut net_buf) {
        if count > 0 {
            let net_id = net_buf[0];
            thingsys::prop_set(net_id, "net.socket_api", api_write_port as u64).ok();
            info!("NETD: Published socket API port {} to graph", api_write_port);
        }
    }

    // Initialize socket API
    let mut socket_api = SocketApi::new();

    // Socket buffers for incoming connections (statically allocated)
    // TX buffers sized to handle ~30KB HTTP responses (e.g., graph.js)
    // We need enough buffers for: multiple listener respawns + concurrent connections
    
    // Dedicated buffers for listener sockets (used by TCP_LISTEN and respawned listeners)
    // Listeners need minimal buffers but we need one per simultaneous listener
    static mut LISTENER_RX_0: [u8; 4096] = [0; 4096];
    static mut LISTENER_TX_0: [u8; 4096] = [0; 4096];
    static mut LISTENER_RX_1: [u8; 4096] = [0; 4096];
    static mut LISTENER_TX_1: [u8; 4096] = [0; 4096];
    static mut LISTENER_RX_2: [u8; 4096] = [0; 4096];
    static mut LISTENER_TX_2: [u8; 4096] = [0; 4096];
    static mut LISTENER_RX_3: [u8; 4096] = [0; 4096];
    static mut LISTENER_TX_3: [u8; 4096] = [0; 4096];
    
    // Buffers for active connections (larger for HTTP responses)
    static mut CONN_RX_0: [u8; 8192] = [0; 8192];
    static mut CONN_TX_0: [u8; 32768] = [0; 32768];
    static mut CONN_RX_1: [u8; 8192] = [0; 8192];
    static mut CONN_TX_1: [u8; 32768] = [0; 32768];
    static mut CONN_RX_2: [u8; 8192] = [0; 8192];
    static mut CONN_TX_2: [u8; 32768] = [0; 32768];
    static mut CONN_RX_3: [u8; 8192] = [0; 8192];
    static mut CONN_TX_3: [u8; 32768] = [0; 32768];
    static mut CONN_RX_4: [u8; 8192] = [0; 8192];
    static mut CONN_TX_4: [u8; 32768] = [0; 32768];
    static mut CONN_RX_5: [u8; 8192] = [0; 8192];
    static mut CONN_TX_5: [u8; 32768] = [0; 32768];
    static mut CONN_RX_6: [u8; 8192] = [0; 8192];
    static mut CONN_TX_6: [u8; 32768] = [0; 32768];
    static mut CONN_RX_7: [u8; 8192] = [0; 8192];
    static mut CONN_TX_7: [u8; 32768] = [0; 32768];

    let mut next_listener_buf = 0usize;
    let mut next_conn_buf = 0usize;
    let mut api_msg_buf = [0u8; 16384];

    // Socket storage for smoltcp - support up to 16 sockets
    let mut sockets_storage: [SocketStorage; 16] = Default::default();
    let mut socket_set = SocketSet::new(&mut sockets_storage[..]);

    // Main service loop
    loop {
        let now = IpcNicDevice::now();

        // Poll the interface to process any pending packets
        iface.poll(now, &mut device, &mut socket_set);

        // Process socket API messages
        // Message format: [4: response_port][2: msg_type][payload...]
        match port_recv(api_read_port, &mut api_msg_buf) {
            Ok(len) if len > 6 => {
                // Extract the client's response port from the message header
                let client_response_port = u32::from_le_bytes([
                    api_msg_buf[0],
                    api_msg_buf[1],
                    api_msg_buf[2],
                    api_msg_buf[3],
                ]) as PortHandle;
                
                // The rest is the actual socket API message
                let msg_body = &api_msg_buf[4..len];
                
                // Determine message type to select appropriate buffer pool
                let msg_type = if msg_body.len() >= 2 {
                    u16::from_le_bytes([msg_body[0], msg_body[1]])
                } else {
                    0
                };
                
                // Buffer assignment:
                // - TCP_LISTEN uses LARGE buffers because the listener socket becomes the connection
                // - TCP_ACCEPT uses SMALL buffers for the respawned listener
                // - All other operations don't create sockets, so buffer choice doesn't matter
                let uses_large_buf = msg_type == socket_api::MSG_TCP_LISTEN;
                
                let response = unsafe {
                    if uses_large_buf {
                        // TCP_LISTEN: Use large connection buffers (listener becomes connection)
                        let (rx, tx) = match next_conn_buf % 8 {
                            0 => (&mut CONN_RX_0[..], &mut CONN_TX_0[..]),
                            1 => (&mut CONN_RX_1[..], &mut CONN_TX_1[..]),
                            2 => (&mut CONN_RX_2[..], &mut CONN_TX_2[..]),
                            3 => (&mut CONN_RX_3[..], &mut CONN_TX_3[..]),
                            4 => (&mut CONN_RX_4[..], &mut CONN_TX_4[..]),
                            5 => (&mut CONN_RX_5[..], &mut CONN_TX_5[..]),
                            6 => (&mut CONN_RX_6[..], &mut CONN_TX_6[..]),
                            _ => (&mut CONN_RX_7[..], &mut CONN_TX_7[..]),
                        };
                        next_conn_buf = next_conn_buf.wrapping_add(1);
                        socket_api.process_message(&mut socket_set, msg_body, rx, tx)
                    } else if msg_type == socket_api::MSG_TCP_ACCEPT {
                        // TCP_ACCEPT: Use small buffers for respawned listener
                        let (rx, tx) = match next_listener_buf % 4 {
                            0 => (&mut LISTENER_RX_0[..], &mut LISTENER_TX_0[..]),
                            1 => (&mut LISTENER_RX_1[..], &mut LISTENER_TX_1[..]),
                            2 => (&mut LISTENER_RX_2[..], &mut LISTENER_TX_2[..]),
                            _ => (&mut LISTENER_RX_3[..], &mut LISTENER_TX_3[..]),
                        };
                        next_listener_buf = next_listener_buf.wrapping_add(1);
                        socket_api.process_message(&mut socket_set, msg_body, rx, tx)
                    } else {
                        // Other operations (SEND, RECV, CLOSE) - buffers not used for socket creation
                        // Just pass any buffer (won't be used)
                        socket_api.process_message(&mut socket_set, msg_body, &mut CONN_RX_0[..], &mut CONN_TX_0[..])
                    }
                };

                // Send response to the client's response port
                if let Err(e) = port_send(client_response_port, &response) {
                    warn!("NETD: Failed to send API response to port {}: {:?}", client_response_port, e);
                }
            }
            _ => {}
        }

        stem::time::sleep_ms(5);
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
