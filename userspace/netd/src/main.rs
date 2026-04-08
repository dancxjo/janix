#![feature(restricted_std)]
#![no_main]

//! # Network Service (netd)
//!
//! Provides networking capabilities using smoltcp TCP/IP stack.
//! - Connects to virtio_netd for frame I/O via VFS (/dev/net/virtio0/)
//! - Runs DHCP to acquire IP address
//! - Provides DNS resolver
//! - Exposes socket API for applications

extern crate alloc;
extern crate stem;

mod dhcp;
mod dns;
mod socket_api;
mod vfs_device;
use abi::schema::keys;
use abi::syscall::vfs_flags::{O_NONBLOCK, O_RDONLY, O_WRONLY};
use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
use smoltcp::wire::{EthernetAddress, IpCidr};
use socket_api::SocketApi;
use stem::syscall::port::{port_create, port_recv, port_send_all, PortHandle};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::thing::sys as thingsys;
use stem::{error, info, warn};
use vfs_device::VfsNicDevice;

/// Path prefix for the virtio NIC VFS provider (published by virtio_netd).
const VIRTIO0_PATH: &str = "/dev/net/virtio0";

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("NETD: Starting network stack service...");

    // Open /dev/net/virtio0/{rx,tx,events} — retry until virtio_netd is ready.
    info!("NETD: Waiting for virtio_netd VFS provider at {}...", VIRTIO0_PATH);
    let (rx_fd, tx_fd, events_fd, mac, iface_mtu, initial_link_up) = open_nic_device();

    info!(
        "NETD: Connected to driver - MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    );

    // Create VFS-backed smoltcp device
    let mut device = VfsNicDevice::new(rx_fd, tx_fd, events_fd, mac, iface_mtu as usize, initial_link_up);

    // Create smoltcp interface
    let mac_addr = EthernetAddress(mac);
    let config = Config::new(mac_addr.into());
    let mut iface = Interface::new(config, &mut device, VfsNicDevice::now());

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
    let ip_packed = {
        let octets = dhcp_config.ip.as_bytes();
        (octets[0] as u64)
            | ((octets[1] as u64) << 8)
            | ((octets[2] as u64) << 16)
            | ((octets[3] as u64) << 24)
    };
    let gw_packed = {
        let octets = dhcp_config.gateway.as_bytes();
        (octets[0] as u64)
            | ((octets[1] as u64) << 8)
            | ((octets[2] as u64) << 16)
            | ((octets[3] as u64) << 24)
    };
    let dns_packed = {
        let octets = dhcp_config.dns.as_bytes();
        (octets[0] as u64)
            | ((octets[1] as u64) << 8)
            | ((octets[2] as u64) << 16)
            | ((octets[3] as u64) << 24)
    };
    // Publish IP/gateway/DNS to graph immediately after DHCP.
    // This is a one-time cost; DHCP itself already blocked.
    thingsys::prop_set(net_id, "net.ip", ip_packed).ok();
    thingsys::prop_set(net_id, "net.gateway", gw_packed).ok();
    thingsys::prop_set(net_id, "net.dns", dns_packed).ok();

    info!(
        "NETD: DHCP configured — IP: {}, GW: {}, DNS: {}",
        dhcp_config.ip, dhcp_config.gateway, dhcp_config.dns
    );


    info!("NETD: Network stack ready, entering service loop");

    // Initialize socket API
    let mut socket_api = SocketApi::new();

    // Sockets storage handles 64 dynamic buffers managed entirely by `SocketApi`
    let mut api_msg_buf = [0u8; 16384];
    let mut api_buffered = 0usize;

    // Socket storage for smoltcp - support up to 256 sockets
    // This needs to be large enough to handle:
    // - Multiple listener sockets (respawned on each accept)
    // - Concurrent active connections
    // - Sockets in TIME_WAIT or FIN_WAIT states waiting for cleanup
    let mut sockets_storage: [SocketStorage; 256] = [SocketStorage::EMPTY; 256];
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
        let now = VfsNicDevice::now();
        iface.poll(now, &mut device, &mut socket_set);

        // --- Phase 1: Non-blockingly poll for additional network frames ---
        // iface.poll calls device.receive() which reads from rx_fd non-blockingly;
        // one additional poll catches frames that arrived during the first pass.
        {
            let now = VfsNicDevice::now();
            if iface.poll(now, &mut device, &mut socket_set) {
                did_work = true;
            }
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
                                msg_type: _,
                                consumed,
                            } => {
                                api_msgs_this_pass += 1;

                                let response = socket_api.process_message(
                                    &mut iface,
                                    &mut device,
                                    &mut socket_set,
                                    msg_body,
                                    caller_tid,
                                    Some(dhcp_config.dns),
                                );

                                // Non-blocking response send: never stall the main loop
                                // waiting for a client's response port to drain.
                                match port_send_all(client_response_port, &response) {
                                    Ok(n) if n == response.len() => {} // success
                                    Ok(n) => {
                                        warn!(
                                            "NETD: short Socket API response to port {} ({}/{})",
                                            client_response_port,
                                            n,
                                            response.len()
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
        let now = VfsNicDevice::now();
        iface.poll(now, &mut device, &mut socket_set);

        if api_buffered > 8192 {
            warn!(
                "NETD: API buffer overflow ({} bytes), clearing",
                api_buffered
            );
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

        let current_link_state = device.link_up();
        if current_link_state != _last_link_state {
            _last_link_state = current_link_state;
            let _ = thingsys::prop_set_async(
                net_id,
                "net.link_up",
                if current_link_state { 1 } else { 0 },
            );
            info!(
                "NETD: Link state changed {} -> async updated graph",
                if current_link_state { "UP" } else { "DOWN" }
            );
        }

        // Garbage collect closed sockets (local operation, fast, no IPC)
        socket_api.gc_closed_sockets(&mut socket_set);

        // Only wait if no work was done, otherwise spin back immediately
        // to process remaining network frames or API messages.
        // Wait on the socket API port; network frame arrival is caught by
        // non-blocking device.receive() in the next iface.poll() call.
        if !did_work {
            let _ = stem::syscall::port::port_wait(&[api_read_port], abi::syscall::port_wait::READABLE);
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

/// Open the virtio NIC device files, retrying until the VFS provider is ready.
///
/// Returns `(rx_fd, tx_fd, events_fd, mac, mtu, link_up)`.
///
/// NOTE: If `/dev/net/virtio0/rx` disappears after this returns (driver exit),
/// the caller should close all fds and call this function again with backoff.
fn open_nic_device() -> (u32, u32, u32, [u8; 6], u32, bool) {
    let rx_path = alloc::format!("{}/rx", VIRTIO0_PATH);
    let tx_path = alloc::format!("{}/tx", VIRTIO0_PATH);
    let events_path = alloc::format!("{}/events", VIRTIO0_PATH);
    let mac_path = alloc::format!("{}/mac", VIRTIO0_PATH);
    let mtu_path = alloc::format!("{}/mtu", VIRTIO0_PATH);

    loop {
        // rx is opened non-blocking so device.receive() never stalls the loop
        let rx_fd = match vfs_open(&rx_path, O_RDONLY | O_NONBLOCK) {
            Ok(fd) => fd,
            Err(_) => {
                stem::time::sleep_ms(100);
                continue;
            }
        };

        let tx_fd = match vfs_open(&tx_path, O_WRONLY) {
            Ok(fd) => fd,
            Err(e) => {
                warn!("NETD: Failed to open {}: {:?}", tx_path, e);
                vfs_close(rx_fd).ok();
                stem::time::sleep_ms(100);
                continue;
            }
        };

        // events is opened non-blocking; errors are silently ignored
        let events_fd = match vfs_open(&events_path, O_RDONLY | O_NONBLOCK) {
            Ok(fd) => fd,
            Err(e) => {
                warn!("NETD: Failed to open {}: {:?}", events_path, e);
                vfs_close(rx_fd).ok();
                vfs_close(tx_fd).ok();
                stem::time::sleep_ms(100);
                continue;
            }
        };

        // Fallback MAC uses the QEMU/KVM default prefix (52:54:00) to avoid
        // conflicts with real hardware addresses.
        let mac = read_mac_file(&mac_path).unwrap_or([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
        let mtu = read_u32_file(&mtu_path).unwrap_or(1500);

        // Assume link up initially; poll_events() will update the state
        info!(
            "NETD: Opened VFS NIC device (rx={}, tx={}, events={}, mtu={})",
            rx_fd, tx_fd, events_fd, mtu
        );
        return (rx_fd, tx_fd, events_fd, mac, mtu, true);
    }
}

/// Read and parse a MAC address from a text file (`xx:xx:xx:xx:xx:xx\n`).
fn read_mac_file(path: &str) -> Option<[u8; 6]> {
    let mut buf = [0u8; 24];
    let n = read_file_bytes(path, &mut buf)?;
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    parse_mac(s)
}

/// Parse a MAC address string in `xx:xx:xx:xx:xx:xx` format.
fn parse_mac(s: &str) -> Option<[u8; 6]> {
    let mut mac = [0u8; 6];
    let mut count = 0usize;
    for (i, hex) in s.split(':').enumerate() {
        if i >= 6 {
            return None; // Too many octets
        }
        mac[i] = u8::from_str_radix(hex.trim(), 16).ok()?;
        count += 1;
    }
    if count != 6 {
        return None; // Too few octets
    }
    Some(mac)
}

/// Read a decimal `u32` from a text file.
fn read_u32_file(path: &str) -> Option<u32> {
    let mut buf = [0u8; 16];
    let n = read_file_bytes(path, &mut buf)?;
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    s.parse().ok()
}

/// Read raw bytes from a file, returning the number of bytes read.
fn read_file_bytes(path: &str, buf: &mut [u8]) -> Option<usize> {
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let result = vfs_read(fd, buf).ok();
    vfs_close(fd).ok();
    result
}
