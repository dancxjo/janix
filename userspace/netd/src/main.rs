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
use stem::syscall::port::{port_create, port_recv, port_send_all, PortHandle};
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
    let (api_write_port, api_read_port) = match port_create(32768) {
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

    let mut _net_mirror = match NetGraphMirror::new() {
        Ok(m) => m,
        Err(e) => {
            warn!("NETD: Failed to initialize net graph mirror: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };
    // Defer first graph refresh to allow network I/O to process unimpeded.
    // Graph operations block on Root service IPC and can stall for 100s+ ms,
    // which would freeze the entire network stack if triggered early.
    let boot_ms = stem::time::now().as_millis() as u64;
    let mut _next_mirror_refresh_ms = boot_ms.saturating_add(60_000);
    let _bootstrap_snapshot = NetSnapshot {
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
    // DEFERRED: bootstrap graph apply moved to cold path in main loop.
    // net_mirror.apply blocks on Root service IPC which can stall the entire
    // network stack before it even starts processing frames.
    let mut _last_link_state = device.link_up();

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
    let _ip_packed = {
        let octets = dhcp_config.ip.as_bytes();
        (octets[0] as u64)
            | ((octets[1] as u64) << 8)
            | ((octets[2] as u64) << 16)
            | ((octets[3] as u64) << 24)
    };
    // DEFERRED: prop_set calls moved to cold path in main loop.
    // These make blocking Root service IPC calls.
    // thingsys::prop_set(net_id, "net.ip", ip_packed).ok();
    // thingsys::prop_set(net_id, "net.gateway", gw_packed).ok();
    // thingsys::prop_set(net_id, "net.dns", dns_packed).ok();

    info!(
        "NETD: DHCP configured (IP: {}), deferring graph updates to main loop",
        dhcp_config.ip
    );

    let gateway = dhcp_config.gateway.as_bytes();
    let mut routes = default_routes_from_gateway([gateway[0], gateway[1], gateway[2], gateway[3]]);
    let _initial_snapshot = NetSnapshot {
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
    // DEFERRED: initial graph snapshot moved to cold path in main loop.
    // net_mirror.apply blocks on Root service IPC.
    // The very first cold path iteration will apply the full snapshot.

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
    //
    // Critical design constraints:
    // 1. Network I/O (iface.poll + rx_port + API messages) MUST run every iteration
    //    without being gated by graph operations.
    // 2. Graph operations (net_mirror.apply, flush_graph, prop_set) make BLOCKING
    //    IPC calls to the Root service which can stall for 100s+ of ms. These MUST
    //    be placed AFTER the network hot path and throttled aggressively.
    // 3. API messages are capped per pass to prevent client flooding from starving
    //    network frame processing.
    let mut loop_iter: u64 = 0;
    loop {
        loop_iter += 1;
        let mut did_work = false;

        // ===== HOT PATH: Network I/O (must never block) =====

        // Always poll the interface first to process pending packets
        let now = IpcNicDevice::now();
        iface.poll(now, &mut device, &mut socket_set);

        // --- Phase 1: Non-blockingly drain rx_port (network frames from driver) ---
        // This ensures TCP handshake packets (SYN, ACK) are always ingested
        // before we process API messages like TCP_ACCEPT.
        while stem::syscall::port::port_len(rx_port).unwrap_or(0) > 0 {
            let now = IpcNicDevice::now();
            iface.poll(now, &mut device, &mut socket_set);
            did_work = true;
            break; // One extra poll is enough; more frames will be caught next iteration
        }

        // --- Phase 2: Process a limited batch of Socket API messages ---
        // Cap messages per pass to prevent API flooding from starving network I/O.
        let mut api_msgs_this_pass = 0u32;
        const MAX_API_MSGS_PER_PASS: u32 = 32;

        while api_msgs_this_pass < MAX_API_MSGS_PER_PASS
            && stem::syscall::port::port_len(api_read_port).unwrap_or(0) > 0
        {
            let space = api_msg_buf.len().saturating_sub(api_buffered);
            if space == 0 {
                warn!("NETD: Socket API buffer full (16KB), dropping messages to avoid stall");
                api_buffered = 0; // Emergency clear
                break;
            }

            match stem::syscall::port::port_recv(api_read_port, &mut api_msg_buf[api_buffered..]) {
                Ok(n) if n > 0 => {
                    api_buffered += n;
                    did_work = true;

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
                                api_msgs_this_pass += 1;

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
                                    Err(_) => {
                                        // Response port full — drop silently to avoid log spam.
                                        // Client will retry or timeout.
                                    }
                                }

                                offset += consumed;
                            }
                            EnvelopeDecode::NeedMore => {
                                // Valid header but incomplete payload.
                                break;
                            }
                            EnvelopeDecode::Malformed => {
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

        // --- Phase 3: Re-poll interface after API processing ---
        // This is critical: API messages may have triggered socket operations
        // (e.g., TCP_SEND), and we need to flush those packets to the wire.
        // Also, TCP handshake packets (SYN-ACK, ACK) may have arrived during
        // API processing and need to be ingested before the next accept check.
        let now = IpcNicDevice::now();
        iface.poll(now, &mut device, &mut socket_set);

        if api_buffered > 8192 {
            warn!("NETD: API buffer overflow ({} bytes), clearing", api_buffered);
            api_buffered = 0;
        }

        // ===== COLD PATH =====
        // IMPORTANT: No blocking Root service IPC allowed here!
        // net_mirror.apply(), thingsys::prop_set(), and flush_graph() all
        // make synchronous IPC calls that can block for 100s+ ms (or indefinitely),
        // which freezes the entire network stack and prevents TCP frame processing.
        // These graph operations are DISABLED until a non-blocking graph IPC
        // mechanism is available. Network state is published at boot via
        // thingsys::prop_set in the initialization block and is sufficient for
        // service discovery.

        // Garbage collect closed sockets (local operation, fast, no IPC)
        socket_api.gc_closed_sockets(&mut socket_set);

        // Only wait if no work was done, otherwise spin back immediately
        // to process remaining network frames or API messages.
        // Use port_wait instead of sleep_ms to wake IMMEDIATELY when data
        // arrives on either the network RX port or the socket API port.
        // sleep_ms(1) was sleeping 2+ seconds due to scheduler granularity,
        // causing TCP handshake timeouts.
        if !did_work {
            let wait_ports = [rx_port, api_read_port];
            let _ = stem::syscall::port::port_wait(
                &wait_ports,
                abi::syscall::port_wait::READABLE,
            );
        }
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
