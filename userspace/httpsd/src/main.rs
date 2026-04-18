#![no_std]
#![no_main]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use core::str;
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use stem::abi::syscall::vfs_flags::{O_RDONLY, O_RDWR, O_WRONLY};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_write};
use stem::syscall::{channel_create, vfs_mount};
use stem::{info, warn};

const MOUNT_POINT: &str = "/https";
const ROOT_HANDLE: u64 = 1;
const MODE_DIR: u32 = 0o040_555;

/// Dynamic HTTPS-backed VFS provider.
///
/// Path model:
/// - `/https/<host>` is a synthetic directory if `<host>` resolves in DNS.
/// - Any deeper path remains synthetic and directory-like for navigation.
/// - Reading a deeper node performs an HTTPS GET of
///   `https://<host>/<path-after-host>` and streams the response bytes.
///
/// Example:
/// - `cat /https/en.wikipedia.org/en/wiki/Dormouse`
///   fetches `https://en.wikipedia.org/en/wiki/Dormouse`.
struct HttpsProvider {
    next_handle: u64,
    path_to_handle: BTreeMap<String, u64>,
    handle_to_path: BTreeMap<u64, String>,
    cached_bodies: BTreeMap<u64, alloc::vec::Vec<u8>>,
    host_resolve_cache: BTreeMap<String, bool>,
}

impl HttpsProvider {
    fn new() -> Self {
        Self {
            next_handle: ROOT_HANDLE + 1,
            path_to_handle: BTreeMap::new(),
            handle_to_path: BTreeMap::new(),
            cached_bodies: BTreeMap::new(),
            host_resolve_cache: BTreeMap::new(),
        }
    }

    fn dispatch(&mut self, req: &ProviderRequest) -> ProviderResponse {
        match req.op {
            VfsRpcOp::Lookup => self.handle_lookup(&req.payload),
            VfsRpcOp::Read => self.handle_read(&req.payload),
            VfsRpcOp::Stat => self.handle_stat(&req.payload),
            VfsRpcOp::Readdir => self.handle_readdir(&req.payload),
            VfsRpcOp::Close => ProviderResponse::ok_empty(),
            VfsRpcOp::Poll => ProviderResponse::ok_bytes(&1u32.to_le_bytes()),
            VfsRpcOp::Write => ProviderResponse::err(Errno::EROFS),
            _ => ProviderResponse::err(Errno::ENOSYS),
        }
    }

    fn handle_lookup(&mut self, payload: &[u8]) -> ProviderResponse {
        if payload.len() < 4 {
            return ProviderResponse::err(Errno::EINVAL);
        }

        let Some(path) = parse_lookup_path(payload) else {
            return ProviderResponse::err(Errno::EINVAL);
        };
        let canonical = canonical_path(path);

        if canonical.is_empty() {
            return ProviderResponse::ok_u64(ROOT_HANDLE);
        }

        let Some((host, _rest)) = split_host_path(&canonical) else {
            return ProviderResponse::err(Errno::ENOENT);
        };

        if !self.host_resolves(host) {
            return ProviderResponse::err(Errno::ENOENT);
        }

        let handle = self.ensure_handle(&canonical);
        ProviderResponse::ok_u64(handle)
    }

    fn handle_read(&mut self, payload: &[u8]) -> ProviderResponse {
        if payload.len() < 20 {
            return ProviderResponse::err(Errno::EINVAL);
        }

        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
        let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
        let len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

        let Some(path) = self.handle_to_path.get(&handle).cloned() else {
            return ProviderResponse::err(Errno::EBADF);
        };

        let Some((host, path_after_host)) = split_host_path(&path) else {
            // Root is not a readable byte stream.
            return ProviderResponse::ok_read(&[]);
        };

        if path_after_host.is_empty() {
            // Host nodes are directories; reading them returns EOF.
            return ProviderResponse::ok_read(&[]);
        }

        if !self.cached_bodies.contains_key(&handle) {
            let url = alloc::format!("https://{host}/{path_after_host}");
            match fetch_https_body(&url) {
                Ok(body) => {
                    self.cached_bodies.insert(handle, body);
                }
                Err(err) => {
                    warn!("httpsd: GET {} failed: {:?}", url, err);
                    return ProviderResponse::err(err);
                }
            }
        }

        let Some(body) = self.cached_bodies.get(&handle) else {
            return ProviderResponse::err(Errno::EIO);
        };

        if offset >= body.len() || len == 0 {
            return ProviderResponse::ok_read(&[]);
        }

        let end = core::cmp::min(offset.saturating_add(len), body.len());
        ProviderResponse::ok_read(&body[offset..end])
    }

    fn handle_stat(&self, payload: &[u8]) -> ProviderResponse {
        if payload.len() < 8 {
            return ProviderResponse::err(Errno::EINVAL);
        }

        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
        if handle == ROOT_HANDLE {
            return ProviderResponse::ok_stat(MODE_DIR, 0, ROOT_HANDLE);
        }

        let Some(path) = self.handle_to_path.get(&handle) else {
            return ProviderResponse::err(Errno::EBADF);
        };

        let size = self
            .cached_bodies
            .get(&handle)
            .map(|body| body.len() as u64)
            .unwrap_or(0);

        // Directory-like nodes are easier to navigate with `cd` while still
        // allowing reads for deeper URL segments.
        let _ = path;
        ProviderResponse::ok_stat(MODE_DIR, size, handle)
    }

    fn handle_readdir(&self, payload: &[u8]) -> ProviderResponse {
        if payload.len() < 8 {
            return ProviderResponse::err(Errno::EINVAL);
        }
        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));

        // Root can list hosts we've resolved so far; all other nodes are dynamic
        // and discovery is URL-driven, so we expose an empty listing.
        if handle != ROOT_HANDLE {
            return ProviderResponse::ok_read(&[]);
        }

        let mut out = alloc::vec::Vec::new();
        for host in self.host_resolve_cache.keys() {
            if !self.host_resolve_cache.get(host).copied().unwrap_or(false) {
                continue;
            }
            let mut entry = alloc::vec![0u8; 8 + 1 + host.len()];
            entry[..8].copy_from_slice(&ROOT_HANDLE.to_le_bytes());
            entry[8] = host.len() as u8;
            entry[9..].copy_from_slice(host.as_bytes());
            out.extend_from_slice(&entry);
        }

        ProviderResponse::ok_read(&out)
    }

    fn ensure_handle(&mut self, path: &str) -> u64 {
        if let Some(handle) = self.path_to_handle.get(path).copied() {
            return handle;
        }

        let handle = self.next_handle;
        self.next_handle = self.next_handle.saturating_add(1);
        self.path_to_handle.insert(path.to_string(), handle);
        self.handle_to_path.insert(handle, path.to_string());
        handle
    }

    fn host_resolves(&mut self, host: &str) -> bool {
        if let Some(cached) = self.host_resolve_cache.get(host).copied() {
            return cached;
        }

        let resolved = resolve_host_via_netd(host);
        self.host_resolve_cache.insert(host.to_string(), resolved);
        resolved
    }
}

fn parse_lookup_path(payload: &[u8]) -> Option<&str> {
    if payload.len() <= 4 {
        return Some("");
    }
    str::from_utf8(&payload[4..]).ok()
}

fn canonical_path(path: &str) -> String {
    path.trim_matches('/').to_string()
}

fn split_host_path(path: &str) -> Option<(&str, &str)> {
    let path = path.trim_matches('/');
    if path.is_empty() {
        return None;
    }
    match path.find('/') {
        Some(idx) => Some((&path[..idx], &path[idx + 1..])),
        None => Some((path, "")),
    }
}

/// Resolve a hostname through `/net/dns/lookup`.
///
/// The protocol is text-based:
/// 1. Write host bytes to `/net/dns/lookup`.
/// 2. Re-read until a line with the IPv4 result arrives.
fn resolve_host_via_netd(host: &str) -> bool {
    let Ok(fd) = vfs_open("/net/dns/lookup", O_WRONLY) else {
        return false;
    };

    let write_ok = vfs_write(fd, host.as_bytes()).is_ok();
    let _ = vfs_close(fd);
    if !write_ok {
        return false;
    }

    let Ok(read_fd) = vfs_open("/net/dns/lookup", O_RDONLY) else {
        return false;
    };

    let mut ok = false;
    let mut buf = [0u8; 64];
    for _ in 0..50 {
        match vfs_read(read_fd, &mut buf) {
            Ok(n) if n > 0 => {
                if let Ok(s) = str::from_utf8(&buf[..n]) {
                    ok = is_ipv4_text(s.trim());
                    if ok {
                        break;
                    }
                }
            }
            Ok(_) => {}
            Err(Errno::EAGAIN) => {}
            Err(_) => break,
        }
        stem::time::sleep_ms(10);
    }
    let _ = vfs_close(read_fd);

    ok
}

fn is_ipv4_text(s: &str) -> bool {
    let mut count = 0usize;
    for part in s.split('.') {
        if part.is_empty() || part.parse::<u8>().is_err() {
            return false;
        }
        count += 1;
    }
    count == 4
}

/// Fetch an HTTPS URL via netd's host-side proxy (10.0.2.2:8081).
///
/// Wire path:
/// 1. Allocate `/net/tcp/new`.
/// 2. Open `/net/tcp/<id>/{ctl,data}`.
/// 3. `write(ctl, "connect 10.0.2.2 8081")`.
/// 4. Send `GET /?url=<encoded>` request on `data`.
/// 5. Read full response and return the bytes after `\\r\\n\\r\\n`.
fn fetch_https_body(url: &str) -> Result<alloc::vec::Vec<u8>, Errno> {
    let socket_id = allocate_tcp_socket_id()?;
    let ctl_path = alloc::format!("/net/tcp/{}/ctl", socket_id);
    let data_path = alloc::format!("/net/tcp/{}/data", socket_id);

    let ctl_fd = vfs_open(&ctl_path, O_RDWR)?;
    let data_fd = match vfs_open(&data_path, O_RDWR) {
        Ok(fd) => fd,
        Err(e) => {
            let _ = vfs_close(ctl_fd);
            return Err(e);
        }
    };

    let connect_cmd = b"connect 10.0.2.2 8081";
    if vfs_write(ctl_fd, connect_cmd).is_err() {
        let _ = vfs_close(data_fd);
        let _ = vfs_close(ctl_fd);
        return Err(Errno::EIO);
    }
    stem::time::sleep_ms(30);

    let request_path = alloc::format!("/?url={}", url_encode(url));
    let request = alloc::format!(
        "GET {} HTTP/1.1\r\nHost: 10.0.2.2:8081\r\nConnection: close\r\n\r\n",
        request_path
    );
    if vfs_write(data_fd, request.as_bytes()).is_err() {
        let _ = vfs_close(data_fd);
        let _ = vfs_close(ctl_fd);
        return Err(Errno::EIO);
    }

    let mut response = alloc::vec::Vec::new();
    let mut buf = [0u8; 2048];
    let mut empty_reads = 0u8;
    for _ in 0..300 {
        match vfs_read(data_fd, &mut buf) {
            Ok(0) => {
                empty_reads = empty_reads.saturating_add(1);
                if empty_reads > 4 {
                    break;
                }
            }
            Ok(n) => {
                empty_reads = 0;
                response.extend_from_slice(&buf[..n]);
            }
            Err(Errno::EAGAIN) => {
                stem::time::sleep_ms(10);
                continue;
            }
            Err(_) => {
                let _ = vfs_close(data_fd);
                let _ = vfs_close(ctl_fd);
                return Err(Errno::EIO);
            }
        }
        stem::time::sleep_ms(5);
    }

    let _ = vfs_close(data_fd);
    let _ = vfs_close(ctl_fd);

    let Some(header_end) = find_subsequence(&response, b"\r\n\r\n") else {
        return Err(Errno::EIO);
    };
    Ok(response[header_end + 4..].to_vec())
}

fn allocate_tcp_socket_id() -> Result<String, Errno> {
    let fd = vfs_open("/net/tcp/new", O_RDONLY)?;
    let mut buf = [0u8; 32];
    let n = vfs_read(fd, &mut buf)?;
    let _ = vfs_close(fd);
    if n == 0 {
        return Err(Errno::EIO);
    }
    let id = str::from_utf8(&buf[..n]).map_err(|_| Errno::EIO)?.trim();
    if id.is_empty() {
        return Err(Errno::EIO);
    }
    Ok(id.to_string())
}

fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push('%');
            out.push(char::from_digit((b >> 4) as u32, 16).unwrap_or('0'));
            out.push(char::from_digit((b & 0xF) as u32, 16).unwrap_or('0'));
        }
    }
    out
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("httpsd: starting");

    let (req_read, req_write) = match channel_create(4096) {
        Ok(ch) => ch,
        Err(e) => {
            warn!("httpsd: channel_create failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    match vfs_mount(req_write, MOUNT_POINT) {
        Ok(()) => info!("httpsd: mounted at {} (read={})", MOUNT_POINT, req_read),
        Err(e) => {
            warn!("httpsd: mount failed at {}: {:?}", MOUNT_POINT, e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    }

    let mut lp = ProviderLoop::new(req_read);
    let mut provider = HttpsProvider::new();

    loop {
        let req = match lp.next_request() {
            Ok(req) => req,
            Err(e) => {
                warn!("httpsd: provider channel closed: {:?}", e);
                break;
            }
        };

        let resp = provider.dispatch(&req);
        if let Err(e) = lp.send_response(req.resp_port, resp) {
            warn!("httpsd: send_response failed: {:?}", e);
        }
    }

    stem::syscall::exit(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_path_trims_slashes() {
        assert_eq!(canonical_path("/a/b/"), "a/b");
    }

    #[test]
    fn split_host_path_works() {
        assert_eq!(split_host_path("en.wikipedia.org").unwrap(), ("en.wikipedia.org", ""));
        assert_eq!(
            split_host_path("en.wikipedia.org/en/wiki/Dormouse").unwrap(),
            ("en.wikipedia.org", "en/wiki/Dormouse")
        );
    }

    #[test]
    fn ipv4_text_validation() {
        assert!(is_ipv4_text("1.2.3.4"));
        assert!(!is_ipv4_text("1.2.3"));
        assert!(!is_ipv4_text("abc"));
    }

    #[test]
    fn parse_lookup_path_handles_empty() {
        let payload = [0u8; 4];
        assert_eq!(parse_lookup_path(&payload), Some(""));
    }

    #[test]
    fn url_encode_encodes_reserved_chars() {
        assert_eq!(
            url_encode("https://en.wikipedia.org/en/wiki/Dormouse"),
            "https%3a%2f%2fen.wikipedia.org%2fen%2fwiki%2fDormouse"
        );
    }
}
