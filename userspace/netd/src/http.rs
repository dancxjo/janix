//! HTTP/1.1 client

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use smoltcp::iface::Interface;
use smoltcp::socket::tcp::{self, Socket as TcpSocket, SocketBuffer};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{IpAddress, IpEndpoint, Ipv4Address};

use crate::smol_device::ThingNicDevice;

#[derive(Debug)]
pub enum HttpError {
    Timeout,
    ConnectionFailed,
    InvalidResponse,
    Redirect,
}

pub struct HttpResponse {
    pub status_code: u16,
    pub body: Vec<u8>,
}

pub fn http_get(
    iface: &mut Interface,
    device: &mut ThingNicDevice,
    ip: Ipv4Address,
    host: &str,
    path: &str,
) -> Result<HttpResponse, HttpError> {
    let mut rx_data = [0u8; 8192];
    let mut tx_data = [0u8; 2048];
    
    let tcp_rx_buffer = SocketBuffer::new(&mut rx_data[..]);
    let tcp_tx_buffer = SocketBuffer::new(&mut tx_data[..]);
    let mut tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

    let local_port = 49152 + (stem::time::monotonic_ns() % 16384) as u16;
    tcp_socket.set_timeout(Some(Duration::from_secs(10)));

    let mut sockets_storage: [smoltcp::iface::SocketStorage; 1] = Default::default();
    let mut socket_set = smoltcp::iface::SocketSet::new(&mut sockets_storage[..]);
    let tcp_handle = socket_set.add(tcp_socket);
    
    let endpoint = IpEndpoint::new(IpAddress::Ipv4(ip), 80);

    stem::info!("HTTP: Connecting to {}:80 (from port {})", ip, local_port);

    let start = ThingNicDevice::now();
    let timeout = start + Duration::from_secs(30);

    // Connect
    let socket = socket_set.get_mut::<TcpSocket>(tcp_handle);
    socket
        .connect(iface.context(), endpoint, local_port)
        .map_err(|_| HttpError::ConnectionFailed)?;

    let mut connected = false;
    let mut request_sent = false;
    let mut response_data = Vec::new();
    let mut headers_complete = false;
    let mut status_code = 0u16;
    #[allow(unused_assignments)]
    let mut content_length: Option<usize> = None;
    let mut chunked = false;

    loop {
        let now = ThingNicDevice::now();
        if now > timeout {
            return Err(HttpError::Timeout);
        }

        iface.poll(now, device, &mut socket_set);

        let socket = socket_set.get_mut::<TcpSocket>(tcp_handle);

        if !connected && socket.may_send() {
            connected = true;
            stem::info!("HTTP: Connected");
        }

        if connected && !request_sent && socket.can_send() {
            let request = format!(
                "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                path, host
            );
            socket.send_slice(request.as_bytes()).ok();
            request_sent = true;
            stem::info!("HTTP: Request sent");
        }

        if request_sent && socket.can_recv() {
            let data = socket.recv(|buffer| {
                let len = buffer.len();
                response_data.extend_from_slice(buffer);
                (len, ())
            }).ok();

            if data.is_some() {
                // Parse headers if not yet complete
                if !headers_complete {
                    if let Some(end_of_headers) = find_pattern(&response_data, b"\r\n\r\n") {
                        headers_complete = true;
                        let headers = &response_data[..end_of_headers];

                        // Parse status code
                        if let Some(status) = parse_status_code(headers) {
                            status_code = status;
                            stem::info!("HTTP: Status {}", status_code);

                            // Check for redirect
                            if status_code >= 300 && status_code < 400 {
                                return Err(HttpError::Redirect);
                            }
                        }

                        // Parse Content-Length
                        content_length = parse_header(headers, b"Content-Length")
                            .and_then(|v| core::str::from_utf8(v).ok())
                            .and_then(|v| v.parse().ok());

                        // Check for chunked encoding
                        if let Some(encoding) = parse_header(headers, b"Transfer-Encoding") {
                            if encoding == b"chunked" {
                                chunked = true;
                            }
                        }

                        stem::info!(
                            "HTTP: Headers complete (chunked={}, content_length={:?})",
                            chunked,
                            content_length
                        );
                    }
                }
            }
        }

        if !socket.is_open() {
            stem::info!("HTTP: Connection closed ({} bytes)", response_data.len());
            break;
        }

        stem::time::sleep_ms(10);
    }

    if !headers_complete {
        return Err(HttpError::InvalidResponse);
    }

    // Extract body
    let body = if let Some(end_of_headers) = find_pattern(&response_data, b"\r\n\r\n") {
        let body_start = end_of_headers + 4;
        let mut body_data = response_data[body_start..].to_vec();

        // Handle chunked encoding
        if chunked {
            body_data = decode_chunked(&body_data);
        }

        body_data
    } else {
        Vec::new()
    };

    stem::info!("HTTP: Body extracted ({} bytes)", body.len());

    Ok(HttpResponse { status_code, body })
}

fn find_pattern(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn parse_status_code(headers: &[u8]) -> Option<u16> {
    let first_line = headers.split(|&b| b == b'\n').next()?;
    let parts: Vec<&[u8]> = first_line.splitn(3, |&b| b == b' ').collect();
    if parts.len() >= 2 {
        core::str::from_utf8(parts[1])
            .ok()
            .and_then(|s| s.parse().ok())
    } else {
        None
    }
}

fn parse_header<'a>(headers: &'a [u8], name: &[u8]) -> Option<&'a [u8]> {
    for line in headers.split(|&b| b == b'\n') {
        if line.len() > name.len() + 2 {
            let (key, rest) = line.split_at(name.len());
            if key.eq_ignore_ascii_case(name) && rest.starts_with(b":") {
                let value_start = rest.iter().position(|&b| b != b':' && b != b' ')?;
                let value = &rest[value_start..];
                let value_end = value.iter().position(|&b| b == b'\r').unwrap_or(value.len());
                return Some(&value[..value_end]);
            }
        }
    }
    None
}

fn decode_chunked(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut pos = 0;

    while pos < data.len() {
        // Find chunk size line
        let chunk_line_end = data[pos..]
            .windows(2)
            .position(|w| w == b"\r\n")
            .map(|p| pos + p);

        if let Some(end) = chunk_line_end {
            let size_str = core::str::from_utf8(&data[pos..end]).ok();
            if let Some(size_hex) = size_str {
                if let Ok(size) = usize::from_str_radix(size_hex.trim(), 16) {
                    if size == 0 {
                        break;
                    }
                    let chunk_start = end + 2;
                    let chunk_end = chunk_start + size;
                    if chunk_end <= data.len() {
                        result.extend_from_slice(&data[chunk_start..chunk_end]);
                        pos = chunk_end + 2; // Skip trailing \r\n
                        continue;
                    }
                }
            }
        }
        break;
    }

    result
}
