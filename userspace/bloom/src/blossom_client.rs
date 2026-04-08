//! Blossom Client - IPC client for SVG rasterization service.
//!
//! This module provides a client for communicating with the Blossom SVG cache service.
//! It handles connection establishment, request encoding, and caching of responses.

use abi::svg_protocol::{
    encode_ping, RasterizeSvgRequest, RasterizeSvgResponse, SvgSource, SvgStatus,
};
use alloc::collections::BTreeMap;
use stem::syscall;

/// Cached raster variant metadata
#[derive(Debug)]
pub struct CachedRaster {
    pub fd: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub variant_hash: u64,
}

/// Blossom IPC client
pub struct BlossomClient {
    /// Port handle for sending requests (u32 as expected by syscall)
    req_port: u32,
    /// Port handle for receiving responses
    resp_port: u32,
    /// Local cache: variant_hash -> raster metadata
    cache: BTreeMap<u64, CachedRaster>,
    /// Connected flag
    connected: bool,
}

impl BlossomClient {
    /// Create a new disconnected client
    pub fn new() -> Self {
        Self {
            req_port: 0,
            resp_port: 0,
            cache: BTreeMap::new(),
            connected: false,
        }
    }

    /// Attempt to connect to the Blossom service
    pub fn connect(&mut self) -> bool {
        if self.connected {
            return true;
        }

        // Use VFS service discovery
        use abi::syscall::vfs_flags::O_RDONLY;
        use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};

        let read_port = |path: &str| -> Option<u32> {
            let fd = vfs_open(path, O_RDONLY).ok()?;
            let mut buf = [0u8; 16];
            let n = vfs_read(fd, &mut buf).ok()?;
            let _ = vfs_close(fd);
            core::str::from_utf8(&buf[..n]).ok()?.trim().parse::<u32>().ok()
        };

        let req = match read_port("/services/blossom/req") {
            Some(v) => v,
            None => return false,
        };
        let resp = match read_port("/services/blossom/resp") {
            Some(v) => v,
            None => return false,
        };

        self.req_port = req;
        self.resp_port = resp;

        // Test connection with ping
        let mut ping_buf = [0u8; 8];
        if let Some(len) = encode_ping(&mut ping_buf) {
            if syscall::channel_send(req, &ping_buf[..len]).is_ok() {
                let mut resp_buf = [0u8; 8];
                if let Ok(resp_len) = syscall::channel_recv(resp, &mut resp_buf) {
                    if resp_len > 0 && resp_buf[0] == 0 {
                        self.connected = true;
                        stem::info!("BLOOM: Connected to Blossom service");
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if connected to Blossom
    #[allow(dead_code)]
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Request rasterization of an SVG from a memfd
    #[allow(dead_code)]
    pub fn rasterize(&mut self, svg_fd: u32, width: u32, height: u32) -> Option<CachedRaster> {
        if !self.connected && !self.connect() {
            return None;
        }

        // Check local memoization cache (use FD as key for now)
        let variant_key = compute_local_cache_key(svg_fd as u64, width, height);

        if let Some(c) = self.cache.get(&variant_key) {
            return Some(CachedRaster {
                fd: c.fd,
                width: c.width,
                height: c.height,
                stride: c.stride,
                variant_hash: c.variant_hash,
            });
        }

        // Build request
        let request = RasterizeSvgRequest {
            source: SvgSource::MemFd(svg_fd),
            width,
            height,
            pixel_format: 1, // BGRA8888
            flags: 0,
        };

        // Encode and send
        let mut req_buf = [0u8; 256];
        let req_len = request.encode(&mut req_buf)?;

        if syscall::channel_send(self.req_port, &req_buf[..req_len]).is_err() {
            self.connected = false;
            return None;
        }

        // Receive response
        let mut resp_buf = [0u8; 256];
        let resp_len = match syscall::channel_recv(self.resp_port, &mut resp_buf) {
            Ok(len) if len > 0 => len,
            _ => {
                self.connected = false;
                return None;
            }
        };

        // Decode response
        let response = RasterizeSvgResponse::decode(&resp_buf[..resp_len])?;

        if response.status != SvgStatus::Ok as u8 {
            return None;
        }

        // Cache locally
        let cached = CachedRaster {
            fd: response.raster_fd,
            width: response.width,
            height: response.height,
            stride: response.stride_bytes,
            variant_hash: response.variant_hash,
        };

        self.cache.insert(variant_key, cached);

        Some(CachedRaster {
            fd: response.raster_fd,
            width: response.width,
            height: response.height,
            stride: response.stride_bytes,
            variant_hash: response.variant_hash,
        })
    }

    /// Invalidate local cache (e.g., when SVG content changes)
    #[allow(dead_code)]
    pub fn invalidate(&mut self, svg_fd: u32) {
        let svg_hash = svg_fd as u64;
        self.cache.retain(|k, _| (k >> 32) != svg_hash);
    }
}

/// Compute a local cache key from SVG id and dimensions
fn compute_local_cache_key(svg_hash: u64, width: u32, height: u32) -> u64 {
    let mut key = svg_hash;
    key = key.wrapping_mul(0x100000001b3) ^ (width as u64);
    key = key.wrapping_mul(0x100000001b3) ^ (height as u64);
    key
}
