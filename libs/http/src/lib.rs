#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use core::str::FromStr;

use stem::syscall::port::{port_create, port_recv, port_send, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;

// Constants from netd
const KIND_NET_STACK: &str = "svc.net.Stack";
const MSG_TCP_CONNECT: u16 = 0x0200;
const MSG_TCP_SEND: u16 = 0x0201;
const MSG_TCP_RECV: u16 = 0x0202;
const MSG_TCP_CLOSE: u16 = 0x0203;
const MSG_DNS_QUERY: u16 = 0x0500;

const RESP_OK: u16 = 0x0000;
const RESP_ERROR: u16 = 0x0001;
const RESP_HANDLE: u16 = 0x0002;
const RESP_DATA: u16 = 0x0003;
const RESP_ACCEPT: u16 = 0x0004;

pub struct TcpStream {
    socket_handle: u32,
    netd_port: PortHandle,
    my_port: PortHandle,
}

impl TcpStream {
    pub fn connect(host: &str, port: u16) -> Result<Self, String> {
        // Find netd
        let mut buf = [ThingId::default(); 1];
        if thingsys::find(KIND_NET_STACK, &mut buf).map_err(|_| "Net stack not found")? == 0 {
            return Err("Net stack not found".to_string());
        }
        let net_id = buf[0];
        let netd_port = thingsys::prop_get(net_id, "net.socket_api")
            .map_err(|_| "Socket API port not found")? as PortHandle;

        // Create my response port
        let (my_port, _) = port_create(8192).map_err(|_| "Failed to create response port")?;

        // Resolve DNS if host is not an IP
        let ip = if let Ok(ip) = parse_ipv4(host) {
            ip
        } else {
            Self::resolve_dns(netd_port, my_port, host)?
        };

        // Connect
        let mut msg = Vec::with_capacity(8);
        msg.extend_from_slice(&MSG_TCP_CONNECT.to_le_bytes());
        msg.extend_from_slice(&ip);
        msg.extend_from_slice(&port.to_le_bytes());

        let response = send_recv(netd_port, my_port, &msg)?;
        let resp_type = u16::from_le_bytes([response[0], response[1]]);

        if resp_type == RESP_HANDLE {
            let handle = u32::from_le_bytes([response[2], response[3], response[4], response[5]]);
            Ok(Self {
                socket_handle: handle,
                netd_port,
                my_port,
            })
        } else {
            Err("Connection failed".to_string())
        }
    }

    fn resolve_dns(
        netd_port: PortHandle,
        my_port: PortHandle,
        host: &str,
    ) -> Result<[u8; 4], String> {
        let mut msg = Vec::new();
        msg.extend_from_slice(&MSG_DNS_QUERY.to_le_bytes());
        msg.extend_from_slice(host.as_bytes());

        let response = send_recv(netd_port, my_port, &msg)?;
        let resp_type = u16::from_le_bytes([response[0], response[1]]);

        if resp_type == RESP_DATA {
            if response.len() >= 6 {
                Ok([response[2], response[3], response[4], response[5]])
            } else {
                Err("Invalid DNS response".to_string())
            }
        } else {
            Err("DNS resolution failed".to_string())
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        let mut total_sent = 0;
        // Chunk the data to avoid the 4KB kernel IPC cap.
        // V2 header [12 bytes] + msg_type [2 bytes] + handle [4 bytes] = 18 bytes overhead.
        for chunk in data.chunks(4000) {
            let mut msg = Vec::with_capacity(6 + chunk.len());
            msg.extend_from_slice(&MSG_TCP_SEND.to_le_bytes());
            msg.extend_from_slice(&self.socket_handle.to_le_bytes());
            msg.extend_from_slice(chunk);

            let response = send_recv(self.netd_port, self.my_port, &msg)?;
            if response.len() < 4 {
                return Err("Invalid response length from netd".to_string());
            }
            let resp_type = u16::from_le_bytes([response[0], response[1]]);

            if resp_type == RESP_OK {
                let sent = u16::from_le_bytes([response[2], response[3]]);
                total_sent += sent as usize;
                if sent as usize < chunk.len() {
                    // Partial send at TCP level, stop chunking
                    break;
                }
            } else {
                return Err("Write failed at netd".to_string());
            }
        }
        Ok(total_sent)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        let mut msg = Vec::with_capacity(10);
        msg.extend_from_slice(&MSG_TCP_RECV.to_le_bytes());
        msg.extend_from_slice(&self.socket_handle.to_le_bytes());
        // Use a safe max read size to avoid RESP_DATA ghosting hazard.
        // Kernel cap is 4096. RESP_DATA header is 2 bytes.
        let len = (buf.len() as u16).min(4000);
        msg.extend_from_slice(&len.to_le_bytes());

        let response = send_recv(self.netd_port, self.my_port, &msg)?;
        if response.len() < 2 {
            return Ok(0);
        }
        let resp_type = u16::from_le_bytes([response[0], response[1]]);

        if resp_type == RESP_DATA {
            let data = &response[2..];
            let copy_len = data.len().min(buf.len());
            buf[..copy_len].copy_from_slice(&data[..copy_len]);
            Ok(copy_len)
        } else {
            Ok(0)
        }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        let mut msg = Vec::with_capacity(6);
        msg.extend_from_slice(&MSG_TCP_CLOSE.to_le_bytes());
        msg.extend_from_slice(&self.socket_handle.to_le_bytes());
        let _ = send_recv(self.netd_port, self.my_port, &msg);
    }
}

fn send_recv(netd_port: PortHandle, my_port: PortHandle, msg: &[u8]) -> Result<Vec<u8>, String> {
    if msg.len() < 2 {
        return Err("Invalid message".to_string());
    }
    let msg_type = u16::from_le_bytes([msg[0], msg[1]]);
    let payload = &msg[2..];

    // V2 Robust format: [4: response_port][8: caller_tid][2: msg_type][2: payload_len][payload...]
    let mut packet = Vec::with_capacity(16 + payload.len());
    packet.extend_from_slice(&(my_port as u32).to_le_bytes());
    let tid = stem::syscall::get_tid().unwrap_or(0);
    packet.extend_from_slice(&tid.to_le_bytes());
    packet.extend_from_slice(&msg_type.to_le_bytes());
    packet.extend_from_slice(&(payload.len() as u16).to_le_bytes());
    packet.extend_from_slice(payload);

    // Kernel IPC is capped at 4096 bytes.
    if packet.len() > 4096 {
        stem::warn!(
            "http: IPC message too large (len={}), will likely be truncated by kernel",
            packet.len()
        );
    }

    port_send(netd_port, &packet).map_err(|_| "Send failed")?;

    let mut buf = [0u8; 8192]; // Large enough for response
    let start = stem::time::monotonic_ns();
    loop {
        match port_recv(my_port, &mut buf) {
            Ok(len) if len > 0 => return Ok(buf[..len].to_vec()),
            _ => {
                if stem::time::monotonic_ns() - start > 5_000_000_000 {
                    // 5s timeout
                    return Err("Timeout waiting for netd response".to_string());
                }
                stem::thread::yield_now();
            }
        }
    }
}

fn parse_ipv4(s: &str) -> Result<[u8; 4], ()> {
    let mut parts = s.split('.');
    let a = parts.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
    let b = parts.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
    let c = parts.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
    let d = parts.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
    if parts.next().is_some() {
        return Err(());
    }
    Ok([a, b, c, d])
}

// Minimal HTTP client
pub struct HttpClient;

impl HttpClient {
    pub fn post(url: &str, body: &str) -> Result<Response, String> {
        Self::request("POST", url, Some(body))
    }

    pub fn get(url: &str) -> Result<Response, String> {
        Self::request("GET", url, None)
    }

    fn request(method: &str, url: &str, body: Option<&str>) -> Result<Response, String> {
        let (host, port, path, final_url) = if url.starts_with("http://") {
            let rest = &url[7..];
            let (host_port, path) = if let Some(idx) = rest.find('/') {
                (&rest[..idx], &rest[idx..])
            } else {
                (rest, "/")
            };

            let (host, port) = if let Some(idx) = host_port.find(':') {
                (
                    &host_port[..idx],
                    host_port[idx + 1..]
                        .parse::<u16>()
                        .map_err(|_| "Invalid port")?,
                )
            } else {
                (host_port, 80)
            };
            (host.to_string(), port, path.to_string(), url.to_string())
        } else {
            // Use proxy for non-http (likely https)
            // Proxy format: http://10.0.2.2:8081/?url=<encoded_url>
            // Note: 10.0.2.2 is QEMU host loopback
            let encoded_url = url_encode(url);
            let proxy_path = format!("/?url={}", encoded_url);
            ("10.0.2.2".to_string(), 8081, proxy_path, url.to_string())
        };

        let mut stream = TcpStream::connect(&host, port)?;

        let mut req = String::new();
        write!(req, "{} {} HTTP/1.1\r\n", method, path).ok();
        write!(req, "Host: {}\r\n", host).ok();
        write!(req, "Connection: close\r\n").ok();
        if host == "10.0.2.2" {
            write!(req, "X-Original-URL: {}\r\n", final_url).ok();
        }
        if let Some(b) = body {
            write!(req, "Content-Length: {}\r\n", b.len()).ok();
            write!(req, "Content-Type: application/json\r\n").ok();
        }
        write!(req, "\r\n").ok();
        if let Some(b) = body {
            req.push_str(b);
        }

        stream.write(req.as_bytes())?;

        let mut buffer = Vec::new();
        let mut temp_buf = [0u8; 1024];
        let mut body_start = 0;
        let mut headers_done = false;

        // Initial read loop to find headers
        for _ in 0..20 {
            // Limit tries
            let n = stream.read(&mut temp_buf)?;
            if n == 0 {
                break;
            }
            buffer.extend_from_slice(&temp_buf[..n]);

            if let Some(idx) = find_subsequence(&buffer, b"\r\n\r\n") {
                body_start = idx + 4;
                headers_done = true;
                break;
            }
        }

        if !headers_done {
            // Maybe no body or something weird, but let's assume we have what we have
        }

        Ok(Response {
            stream,
            buffer,
            cursor: body_start,
        })
    }
}

pub struct Response {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Response {
    pub fn read_chunk(&mut self) -> Result<Vec<u8>, String> {
        if self.cursor < self.buffer.len() {
            let chunk = self.buffer[self.cursor..].to_vec();
            self.cursor = self.buffer.len();
            return Ok(chunk);
        }

        let mut buf = [0u8; 1024];
        let n = self.stream.read(&mut buf)?;
        if n == 0 {
            return Ok(Vec::new());
        }
        Ok(buf[..n].to_vec())
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.as_bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(b) {
            out.push(*b as char);
        } else {
            write!(out, "%{:02X}", b).ok();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn test_url_encoding() {
        // Alphanumeric - should not be encoded
        assert_eq!(url_encode("abc123XYZ"), "abc123XYZ");

        // Allowed characters - should not be encoded
        assert_eq!(url_encode("a-b_c.d~e"), "a-b_c.d~e");

        // Space - should be encoded as %20
        assert_eq!(url_encode("hello world"), "hello%20world");

        // Special characters - should be encoded
        // / -> %2F, : -> %3A
        assert_eq!(url_encode("http://example.com"), "http%3A%2F%2Fexample.com");

        // Empty string
        assert_eq!(url_encode(""), "");
    }

    #[test]
    fn test_parse_ipv4() {
        // Valid IPs
        assert_eq!(parse_ipv4("127.0.0.1"), Ok([127, 0, 0, 1]));
        assert_eq!(parse_ipv4("192.168.1.100"), Ok([192, 168, 1, 100]));
        assert_eq!(parse_ipv4("0.0.0.0"), Ok([0, 0, 0, 0]));
        assert_eq!(parse_ipv4("255.255.255.255"), Ok([255, 255, 255, 255]));

        // Invalid format
        assert_eq!(parse_ipv4(""), Err(()));
        assert_eq!(parse_ipv4("1.2.3"), Err(()));
        assert_eq!(parse_ipv4("1.2.3.4.5"), Err(()));

        // Invalid numbers
        assert_eq!(parse_ipv4("256.0.0.1"), Err(()));
        assert_eq!(parse_ipv4("-1.0.0.0"), Err(()));

        // Non-numeric
        assert_eq!(parse_ipv4("a.b.c.d"), Err(()));
    }

    #[test]
    fn test_find_subsequence() {
        let sep = b"\r\n\r\n";

        // Found at end (typical header case)
        let data = b"Hello world\r\n\r\n";
        assert_eq!(find_subsequence(data, sep), Some(11));

        // Found in middle
        let data2 = b"Hello\r\n\r\nBody";
        assert_eq!(find_subsequence(data2, sep), Some(5));

        // Not found
        let data3 = b"Hello world";
        assert_eq!(find_subsequence(data3, sep), None);

        // Found at start
        let data4 = b"\r\n\r\nStart";
        assert_eq!(find_subsequence(data4, sep), Some(0));

        // Partial match
        let data5 = b"Partial\r\n\rEnd";
        assert_eq!(find_subsequence(data5, sep), None);

        // Overlapping needle
        assert_eq!(find_subsequence(b"aaaaa", b"aa"), Some(0));
    }
}
