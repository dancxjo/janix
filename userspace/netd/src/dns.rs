//! Simple DNS client for A record lookups

use alloc::vec::Vec;
use smoltcp::iface::Interface;
use smoltcp::socket::udp::{self, PacketMetadata, Socket as UdpSocket};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{IpAddress, IpEndpoint, Ipv4Address};

use crate::vfs_device::VfsNicDevice;

#[derive(Debug)]
pub enum DnsError {
    Timeout,
    InvalidResponse,
    NoAnswer,
}

pub fn lookup_a(
    iface: &mut Interface,
    device: &mut VfsNicDevice,
    dns_server: Ipv4Address,
    name: &str,
) -> Result<Ipv4Address, DnsError> {
    let mut rx_meta = [PacketMetadata::EMPTY; 4];
    let mut rx_data = [0u8; 2048];
    let mut tx_meta = [PacketMetadata::EMPTY; 4];
    let mut tx_data = [0u8; 2048];

    let udp_rx_buffer = udp::PacketBuffer::new(&mut rx_meta[..], &mut rx_data[..]);
    let udp_tx_buffer = udp::PacketBuffer::new(&mut tx_meta[..], &mut tx_data[..]);
    let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);

    // Bind the socket to a local ephemeral port - this is REQUIRED for smoltcp UDP sockets
    // to receive responses. Without binding, the socket has port=0 and won't match incoming packets.
    let local_port = 49152u16; // Ephemeral port in the private range
    if let Err(e) = udp_socket.bind(local_port) {
        stem::warn!("DNS: Failed to bind socket to port {}: {:?}", local_port, e);
        return Err(DnsError::Timeout);
    }
    stem::info!("DNS: Socket bound to local port {}", local_port);

    let mut sockets_storage: [smoltcp::iface::SocketStorage; 1] = Default::default();
    let mut socket_set = smoltcp::iface::SocketSet::new(&mut sockets_storage[..]);
    let udp_handle = socket_set.add(udp_socket);

    // Build DNS query
    let query = build_dns_query(name);
    let endpoint = IpEndpoint::new(IpAddress::Ipv4(dns_server), 53);

    stem::info!("DNS: Querying {} for {}", dns_server, name);

    let start = VfsNicDevice::now();
    let timeout = start + Duration::from_secs(5);

    let mut sent = false;
    let mut poll_count = 0u32;

    loop {
        let now = VfsNicDevice::now();
        if now > timeout {
            stem::info!("DNS: Timeout after {} polls", poll_count);
            return Err(DnsError::Timeout);
        }

        iface.poll(now, device, &mut socket_set);
        poll_count += 1;

        let socket = socket_set.get_mut::<UdpSocket>(udp_handle);

        if !sent && socket.can_send() {
            socket.send_slice(&query, endpoint).ok();
            sent = true;
            stem::info!(
                "DNS: Query sent to {}:53 (txid=0x1234, {} bytes)",
                dns_server,
                query.len()
            );
        }

        if socket.can_recv() {
            let (data, _) = socket.recv().map_err(|_| DnsError::InvalidResponse)?;
            stem::info!("DNS: Response received ({} bytes)", data.len());
            return parse_dns_response(data);
        }

        stem::time::sleep_ms(10);
    }
}

fn build_dns_query(name: &str) -> Vec<u8> {
    let mut query = Vec::new();

    // Transaction ID
    query.push(0x12);
    query.push(0x34);

    // Flags: standard query
    query.push(0x01);
    query.push(0x00);

    // Questions: 1
    query.push(0x00);
    query.push(0x01);

    // Answer RRs: 0
    query.push(0x00);
    query.push(0x00);

    // Authority RRs: 0
    query.push(0x00);
    query.push(0x00);

    // Additional RRs: 0
    query.push(0x00);
    query.push(0x00);

    // QNAME: encode domain name
    for part in name.split('.') {
        query.push(part.len() as u8);
        query.extend_from_slice(part.as_bytes());
    }
    query.push(0); // End of QNAME

    // QTYPE: A (1)
    query.push(0x00);
    query.push(0x01);

    // QCLASS: IN (1)
    query.push(0x00);
    query.push(0x01);

    query
}

fn parse_dns_response(data: &[u8]) -> Result<Ipv4Address, DnsError> {
    if data.len() < 12 {
        return Err(DnsError::InvalidResponse);
    }

    // Check response flags
    let ancount = u16::from_be_bytes([data[6], data[7]]);
    if ancount == 0 {
        return Err(DnsError::NoAnswer);
    }

    // Skip header (12 bytes) and question section
    let mut pos = 12;

    // Skip question QNAME
    while pos < data.len() && data[pos] != 0 {
        let len = data[pos] as usize;
        if len >= 192 {
            // Compression pointer
            pos += 2;
            break;
        }
        pos += 1 + len;
    }
    if pos < data.len() && data[pos] == 0 {
        pos += 1;
    }

    // Skip QTYPE and QCLASS
    pos += 4;

    // Parse answer section
    for _ in 0..ancount {
        if pos >= data.len() {
            return Err(DnsError::InvalidResponse);
        }

        // Skip NAME (handle compression)
        if data[pos] >= 192 {
            pos += 2;
        } else {
            while pos < data.len() && data[pos] != 0 {
                let len = data[pos] as usize;
                pos += 1 + len;
            }
            pos += 1;
        }

        if pos + 10 > data.len() {
            return Err(DnsError::InvalidResponse);
        }

        let rtype = u16::from_be_bytes([data[pos], data[pos + 1]]);
        let rdlength = u16::from_be_bytes([data[pos + 8], data[pos + 9]]);
        pos += 10;

        if rtype == 1 && rdlength == 4 {
            // A record
            if pos + 4 > data.len() {
                return Err(DnsError::InvalidResponse);
            }
            return Ok(Ipv4Address::from_bytes(&data[pos..pos + 4]));
        }

        pos += rdlength as usize;
    }

    Err(DnsError::NoAnswer)
}
