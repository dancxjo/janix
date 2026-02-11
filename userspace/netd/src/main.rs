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
extern crate stem;

mod dhcp;
mod dns;
mod driver_protocol;
mod ipc_device;
mod net_mirror;
mod socket_api;

use abi::schema::keys;
use alloc::format;
use alloc::vec;
use ipc_device::IpcNicDevice;
use net_mirror::{
    default_routes_from_gateway, stable_iface_key_from_mac, AddressSnapshot, IfaceSnapshot,
    NetGraphMirror, NetSnapshot,
};
use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
use smoltcp::wire::{EthernetAddress, IpCidr};
use socket_api::SocketApi;
use stem::syscall::port::{port_create, port_recv, port_send_all, port_wait, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{error, info, warn};

/// Graph kind for the network driver service (published by virtio_netd)
const KIND_NET_DRIVER: &str = "svc.net.Driver";

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NETD: Starting network stack service...");

    // Wait for virtio_netd to be ready
    info!("NETD: Looking for virtio_netd driver service...");
    let (tx_port, rx_port, mac, initial_link_up, iface_mtu) = loop {
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
    let mut device = IpcNicDevice::new(tx_port, rx_port, mac, initial_link_up);

    // Create smoltcp interface
    let mac_addr = EthernetAddress(mac);
    let config = Config::new(mac_addr.into());
    let mut iface = Interface::new(config, &mut device, IpcNicDevice::now());

    // Create socket API port early so it's available in the graph immediately
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

    // Publish network stack node early
    let net_id =
        thingsys::create_node("svc.net.Stack").expect("NETD: Failed to create svc.net.Stack node");

    let mac_packed = {
        (mac[0] as u64)
            | ((mac[1] as u64) << 8)
            | ((mac[2] as u64) << 16)
            | ((mac[3] as u64) << 24)
            | ((mac[4] as u64) << 32)
            | ((mac[5] as u64) << 40)
    };
    thingsys::prop_set(net_id, "net.mac", mac_packed).ok();
    thingsys::prop_set(net_id, keys::WRITE_PORT_HANDLE, api_write_port as u64).ok();
    thingsys::prop_set(net_id, "net.ip", 0).ok(); // Offline initially
    thingsys::prop_set(net_id, "net.link_up", if initial_link_up { 1 } else { 0 }).ok();
    thingsys::prop_set(net_id, "net.mtu", iface_mtu as u64).ok();

    info!("NETD: Published initial stack node {:?} to graph", net_id);

    let mut net_mirror = match NetGraphMirror::new() {
        Ok(m) => m,
        Err(e) => {
            warn!("NETD: Failed to initialize net graph mirror: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };
    let mut next_mirror_refresh_ms = 0u64;
    let bootstrap_snapshot = NetSnapshot {
        iface: IfaceSnapshot {
            stable_key: stable_iface_key_from_mac(mac),
            name: "eth0".into(),
            mac_packed,
            mtu: device.mtu(),
            link_up: device.link_up(),
            driver: KIND_NET_DRIVER.into(),
            speed_mbps: None,
        },
        addrs: vec![],
        routes: vec![],
    };
    if let Err(e) = net_mirror.apply(&bootstrap_snapshot, stem::time::now().as_millis() as u64) {
        warn!("NETD: Bootstrap net graph mirror apply failed: {:?}", e);
    }
    let mut last_link_state = device.link_up();

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

    // Update network configuration in graph
    let ip_packed = {
        let octets = dhcp_config.ip.as_bytes();
        (octets[0] as u64)
            | ((octets[1] as u64) << 8)
            | ((octets[2] as u64) << 16)
            | ((octets[3] as u64) << 24)
    };
    thingsys::prop_set(net_id, "net.ip", ip_packed).ok();

    let gw_packed = {
        let octets = dhcp_config.gateway.as_bytes();
        (octets[0] as u64)
            | ((octets[1] as u64) << 8)
            | ((octets[2] as u64) << 16)
            | ((octets[3] as u64) << 24)
    };
    thingsys::prop_set(net_id, "net.gateway", gw_packed).ok();

    let dns_packed = {
        let octets = dhcp_config.dns.as_bytes();
        (octets[0] as u64)
            | ((octets[1] as u64) << 8)
            | ((octets[2] as u64) << 16)
            | ((octets[3] as u64) << 24)
    };
    thingsys::prop_set(net_id, "net.dns", dns_packed).ok();

    info!(
        "NETD: Updated network configuration in graph (IP: {})",
        dhcp_config.ip
    );

    let gateway = dhcp_config.gateway.as_bytes();
    let mut routes = default_routes_from_gateway([gateway[0], gateway[1], gateway[2], gateway[3]]);
    let initial_snapshot = NetSnapshot {
        iface: IfaceSnapshot {
            stable_key: stable_iface_key_from_mac(mac),
            name: "eth0".into(),
            mac_packed,
            mtu: device.mtu(),
            link_up: device.link_up(),
            driver: KIND_NET_DRIVER.into(),
            speed_mbps: None,
        },
        addrs: vec![AddressSnapshot {
            family: "ipv4".into(),
            ip: format!(
                "{}.{}.{}.{}",
                dhcp_config.ip.as_bytes()[0],
                dhcp_config.ip.as_bytes()[1],
                dhcp_config.ip.as_bytes()[2],
                dhcp_config.ip.as_bytes()[3]
            ),
            prefix: dhcp_config.prefix_len,
        }],
        routes: routes.clone(),
    };
    if let Err(e) = net_mirror.apply(&initial_snapshot, stem::time::now().as_millis() as u64) {
        warn!("NETD: Initial net graph mirror apply failed: {:?}", e);
    }

    info!("NETD: Network stack ready, entering service loop");

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

    let mut next_listener_buf = 0usize;
    let mut next_conn_buf = 0usize;
    let mut api_msg_buf = [0u8; 16384];
    let mut api_buffered = 0usize;

    // Socket storage for smoltcp - support up to 64 sockets
    // This needs to be large enough to handle:
    // - Multiple listener sockets (respawned on each accept)
    // - Concurrent active connections
    // - Sockets in TIME_WAIT or FIN_WAIT states waiting for cleanup
    let mut sockets_storage: [SocketStorage; 64] = [SocketStorage::EMPTY; 64];
    let mut socket_set = SocketSet::new(&mut sockets_storage[..]);

    // Main service loop
    loop {
        let now = IpcNicDevice::now();

        // Poll the interface to process any pending packets
        iface.poll(now, &mut device, &mut socket_set);

        let now_ms = stem::time::now().as_millis() as u64;
        let link_changed = device.link_up() != last_link_state;
        if link_changed || now_ms >= next_mirror_refresh_ms {
            let mut addrs = alloc::vec::Vec::new();
            for cidr in iface.ip_addrs() {
                let v4 = match *cidr {
                    IpCidr::Ipv4(v4) => v4,
                };
                let addr = v4.address();
                let ip = addr.as_bytes();
                addrs.push(AddressSnapshot {
                    family: "ipv4".into(),
                    ip: format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]),
                    prefix: v4.prefix_len(),
                });
            }
            let gateway = dhcp_config.gateway.as_bytes();
            routes = default_routes_from_gateway([gateway[0], gateway[1], gateway[2], gateway[3]]);
            let snapshot = NetSnapshot {
                iface: IfaceSnapshot {
                    stable_key: stable_iface_key_from_mac(device.mac()),
                    name: "eth0".into(),
                    mac_packed,
                    mtu: device.mtu(),
                    link_up: device.link_up(),
                    driver: KIND_NET_DRIVER.into(),
                    speed_mbps: None,
                },
                addrs,
                routes: routes.clone(),
            };
            if let Err(e) = net_mirror.apply(&snapshot, now_ms) {
                warn!("NETD: net graph mirror apply failed: {:?}", e);
            }
            let link_now = device.link_up();
            thingsys::prop_set(net_id, "net.link_up", if link_now { 1 } else { 0 }).ok();
            last_link_state = link_now;
            next_mirror_refresh_ms = now_ms.saturating_add(1000);
        }

        // Garbage collect closed sockets to prevent SocketSet exhaustion
        socket_api.gc_closed_sockets(&mut socket_set);
        socket_api.flush_graph(&mut socket_set, now_ms);

        // Process incoming Socket API requests and driver traffic
        match stem::syscall::port::port_wait(&[api_read_port, rx_port], abi::syscall::port_wait::READABLE) {
            Ok(p) if p == api_read_port => {
                while stem::syscall::port::port_len(api_read_port).unwrap_or(0) > 0 {
                    let space = api_msg_buf.len().saturating_sub(api_buffered);
                    if space == 0 {
                        warn!("NETD: Socket API buffer full (16KB), dropping messages to avoid stall");
                        api_buffered = 0; // Emergency clear
                        break;
                    }

                    match stem::syscall::port::port_recv(api_read_port, &mut api_msg_buf[api_buffered..]) {
                        Ok(n) if n > 0 => {
                            api_buffered += n;

                            let mut offset = 0;
                            while api_buffered.saturating_sub(offset) >= 16 {
                                match decode_socket_api_envelope(&api_msg_buf[offset..api_buffered]) {
                                    EnvelopeDecode::Complete {
                                        client_response_port,
                                        caller_tid,
                                        msg_body,
                                        msg_type,
                                        consumed,
                                    } => {
                                        let uses_large_buf = msg_type == socket_api::MSG_TCP_LISTEN
                                            || msg_type == socket_api::MSG_TCP_CONNECT;

                                        let response = if uses_large_buf {
                                            let (rx, tx) = match next_conn_buf % 4 {
                                                0 => (
                                                    unsafe { &mut CONN_RX_0[..] },
                                                    unsafe { &mut CONN_TX_0[..] },
                                                ),
                                                1 => (
                                                    unsafe { &mut CONN_RX_1[..] },
                                                    unsafe { &mut CONN_TX_1[..] },
                                                ),
                                                2 => (
                                                    unsafe { &mut CONN_RX_2[..] },
                                                    unsafe { &mut CONN_TX_2[..] },
                                                ),
                                                _ => (
                                                    unsafe { &mut CONN_RX_3[..] },
                                                    unsafe { &mut CONN_TX_3[..] },
                                                ),
                                            };
                                            next_conn_buf = next_conn_buf.wrapping_add(1);
                                            socket_api.process_message(
                                                &mut iface,
                                                &mut device,
                                                &mut socket_set,
                                                msg_body,
                                                caller_tid,
                                                rx,
                                                tx,
                                                Some(dhcp_config.dns),
                                            )
                                        } else if msg_type == socket_api::MSG_TCP_ACCEPT {
                                            let (rx, tx) = match next_conn_buf % 4 {
                                                0 => (
                                                    unsafe { &mut CONN_RX_0[..] },
                                                    unsafe { &mut CONN_TX_0[..] },
                                                ),
                                                1 => (
                                                    unsafe { &mut CONN_RX_1[..] },
                                                    unsafe { &mut CONN_TX_1[..] },
                                                ),
                                                2 => (
                                                    unsafe { &mut CONN_RX_2[..] },
                                                    unsafe { &mut CONN_TX_2[..] },
                                                ),
                                                _ => (
                                                    unsafe { &mut CONN_RX_3[..] },
                                                    unsafe { &mut CONN_TX_3[..] },
                                                ),
                                            };
                                            next_conn_buf = next_conn_buf.wrapping_add(1);
                                            socket_api.process_message(
                                                &mut iface,
                                                &mut device,
                                                &mut socket_set,
                                                msg_body,
                                                caller_tid,
                                                rx,
                                                tx,
                                                Some(dhcp_config.dns),
                                            )
                                        } else {
                                            socket_api.process_message(
                                                &mut iface,
                                                &mut device,
                                                &mut socket_set,
                                                msg_body,
                                                caller_tid,
                                                unsafe { &mut CONN_RX_0[..] },
                                                unsafe { &mut CONN_TX_0[..] },
                                                Some(dhcp_config.dns),
                                            )
                                        };

                                        // Non-blocking response send: never stall the main loop
                                        // waiting for a client's response port to drain.
                                        match port_send_all(client_response_port, &response) {
                                            Ok(n) if n == response.len() => {} // success
                                            Ok(n) => {
                                                warn!(
                                                    "NETD: short Socket API response to port {} ({}/{})",
                                                    client_response_port, n, response.len()
                                                );
                                            }
                                            Err(e) => {
                                                warn!(
                                                    "NETD: dropped Socket API response to port {}: {:?}",
                                                    client_response_port, e
                                                );
                                            }
                                        }

                                        offset += consumed;
                                    }
                                    EnvelopeDecode::NeedMore => {
                                        // Valid header but incomplete payload.
                                        break;
                                    }
                                    EnvelopeDecode::Malformed => {
                                        // Without a sync marker in the protocol, byte-by-byte
                                        // re-alignment can produce false-positive decodes from junk.
                                        // Drop the currently buffered chunk and wait for fresh bytes.
                                        let dropped = api_buffered - offset;
                                        warn!(
                                            "NETD: Dropping {} buffered Socket API bytes after malformed frame",
                                            dropped
                                        );
                                        offset = api_buffered;
                                        break;
                                    }
                                }
                            }

                            if offset > 0 {
                                api_msg_buf.copy_within(offset..api_buffered, 0);
                                api_buffered -= offset;
                            }
                        }
                        _ => break,
                    }
                }
            }
            Ok(p) if p == rx_port => {
                // Driver has new frames or link updates.
                // Just wake up; the next loop iteration will call `iface.poll` which
                // calls `device.receive()`, draining the driver port.
            }
            _ => {}
        }

        if api_buffered > 8192 {
            warn!("NETD: API buffer overflow ({} bytes), clearing", api_buffered);
            api_buffered = 0;
        }

        // Minimal sleep for responsive I/O - TX buffers drain faster
        stem::time::sleep_ms(1);
    }
}

const SOCKET_API_ENVELOPE_HEADER_LEN: usize = 16;
const SOCKET_API_MAX_PAYLOAD_LEN: usize = 4096;

enum EnvelopeDecode<'a> {
    Complete {
        client_response_port: PortHandle,
        caller_tid: Option<u64>,
        msg_body: &'a [u8],
        msg_type: u16,
        consumed: usize,
    },
    NeedMore,
    Malformed,
}

fn decode_socket_api_envelope(packet: &[u8]) -> EnvelopeDecode<'_> {
    // Robust V2 protocol: [4: response_port][8: caller_tid][2: msg_type][2: payload_len][payload...]
    if packet.len() < SOCKET_API_ENVELOPE_HEADER_LEN {
        return EnvelopeDecode::NeedMore;
    }

    let response_port =
        u32::from_le_bytes([packet[0], packet[1], packet[2], packet[3]]) as PortHandle;
    let caller_tid = u64::from_le_bytes([
        packet[4], packet[5], packet[6], packet[7], packet[8], packet[9], packet[10], packet[11],
    ]);
    let msg_type = u16::from_le_bytes([packet[12], packet[13]]);
    let payload_len = u16::from_le_bytes([packet[14], packet[15]]) as usize;

    if payload_len > SOCKET_API_MAX_PAYLOAD_LEN {
        warn!(
            "NETD: Malformed Socket API message: payload too large (len={}, port={}, tid=0x{:016x}, type=0x{:04x}, payload_len={}, hex={:02x?})",
            packet.len(),
            response_port,
            caller_tid,
            msg_type,
            payload_len,
            &packet[..core::cmp::min(packet.len(), SOCKET_API_ENVELOPE_HEADER_LEN)]
        );
        return EnvelopeDecode::Malformed;
    }

    if packet.len() < SOCKET_API_ENVELOPE_HEADER_LEN + payload_len {
        // Valid header, partial message.
        return EnvelopeDecode::NeedMore;
    }

    if socket_api::is_known_msg_type(msg_type) {
        return EnvelopeDecode::Complete {
            client_response_port: response_port,
            caller_tid: Some(caller_tid),
            msg_body: &packet[12..SOCKET_API_ENVELOPE_HEADER_LEN + payload_len],
            msg_type,
            consumed: SOCKET_API_ENVELOPE_HEADER_LEN + payload_len,
        };
    }

    if msg_type == 0 {
        warn!(
            "NETD: Protocol desync! Received RESP_OK (0) as request type. Full header: {:02x?}",
            &packet[..SOCKET_API_ENVELOPE_HEADER_LEN]
        );
    }

    warn!(
        "NETD: Malformed Socket API message: len={}, port={}, tid=0x{:016x}, type=0x{:04x}, payload_len={}, hex={:02x?}",
        packet.len(),
        response_port,
        caller_tid,
        msg_type,
        payload_len,
        &packet[..core::cmp::min(packet.len(), SOCKET_API_ENVELOPE_HEADER_LEN)]
    );
    EnvelopeDecode::Malformed
}

/// Find the virtio_netd driver service and get port handles + MAC address
fn find_driver_service() -> Option<(PortHandle, PortHandle, [u8; 6], bool, u32)> {
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

    let link_up = thingsys::prop_get(driver_id, "net.link_up")
        .ok()
        .unwrap_or(1)
        != 0;
    let mtu = thingsys::prop_get(driver_id, "net.mtu")
        .ok()
        .unwrap_or(1500) as u32;

    info!(
        "NETD: Driver TX port={}, RX port={}, link_up={}, mtu={}",
        tx_port, rx_port, link_up, mtu
    );

    Some((tx_port, rx_port, mac, link_up, mtu))
}
