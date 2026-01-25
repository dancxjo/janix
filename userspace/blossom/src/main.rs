#![no_std]
#![no_main]

extern crate alloc;
extern crate stem;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use stem::thing::ThingId;
use stem::thing::sys::{
    bytespace_create, bytespace_info, bytespace_map, bytespace_unmap, bytespace_write,
    create_node, prop_set,
};
use abi::schema::kinds;
use abi::svg_protocol::{
    RasterizeSvgRequest, RasterizeSvgResponse, SvgSource, SvgStatus,
    SvgRequestTag, decode_request_tag, encode_error,
};
use stem::syscall;
use log::{info, debug};
use hashbrown::HashMap;

/// Cache entry for a rasterized SVG variant
#[derive(Clone)]
struct CacheEntry {
    bytespace_id: ThingId,
    width: u32,
    height: u32,
    stride: u32,
    pixel_format: u8,
}

/// Blossom SVG cache service state
struct Blossom {
    /// Cache: variant_hash -> raster data
    cache: HashMap<u64, CacheEntry>,
    /// Rasterization counter for tests
    #[allow(dead_code)]
    raster_count: u64,
}

impl Blossom {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            raster_count: 0,
        }
    }
    
    /// Compute content hash of SVG bytes using FNV-1a
    fn compute_svg_hash(bytes: &[u8]) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for &byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
    
    /// Compute variant hash from svg_hash + render parameters
    fn compute_variant_hash(svg_hash: u64, width: u32, height: u32, format: u8, flags: u32) -> u64 {
        let mut hash = svg_hash;
        for byte in width.to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        for byte in height.to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= format as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        for byte in flags.to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
    
    /// Get cached variant if exists
    fn get_cached(&self, variant_hash: u64) -> Option<&CacheEntry> {
        self.cache.get(&variant_hash)
    }
    
    /// Store rasterized variant in cache
    fn store(&mut self, variant_hash: u64, entry: CacheEntry) {
        self.cache.insert(variant_hash, entry);
    }
}

#[stem::main]
fn main() -> ! {
    info!("BLOSSOM: Starting SVG Cache Service");
    let mut state = Blossom::new();

    // Create IPC ports for blossom service
    let (blossom_req, blossom_resp) = match (syscall::port_create(8192), syscall::port_create(8192)) {
        (Ok(req), Ok(resp)) => {
            // Create service node and advertise port handles
            if let Ok(svc_node) = create_node("svc.Blossom") {
                let _ = prop_set(svc_node, "blossom.req", req.0 as u64);
                let _ = prop_set(svc_node, "blossom.resp", resp.1 as u64);
                info!("BLOSSOM: Service node created, req={}, resp={}", req.0, resp.1);
            }
            (req.1, resp.0) // blossom uses (read, write) sides
        }
        _ => {
            info!("BLOSSOM: Failed to create IPC ports, exiting");
            loop { syscall::sleep_ms(1000); }
        }
    };

    info!("BLOSSOM: Service ready");

    let mut ipc_buf = [0u8; 65536]; // Large buffer for SVG bytes
    let mut resp_buf = [0u8; 256];

    loop {
        // Process IPC requests
        match syscall::port_recv(blossom_req, &mut ipc_buf) {
            Ok(len) if len > 0 => {
                if let Some(resp_len) = handle_ipc_request(&ipc_buf[..len], &mut resp_buf, &mut state) {
                    let _ = syscall::port_send(blossom_resp, &resp_buf[..resp_len]);
                }
            }
            _ => {}
        }

        syscall::sleep_ms(5);
    }
}

fn handle_ipc_request(req: &[u8], resp: &mut [u8], state: &mut Blossom) -> Option<usize> {
    let tag = decode_request_tag(req)?;
    match tag {
        SvgRequestTag::Ping => {
            // Simple ping/pong for connectivity check
            if resp.len() >= 1 {
                resp[0] = 0; // Pong
                Some(1)
            } else {
                None
            }
        }
        SvgRequestTag::RasterizeSvg => {
            let request = RasterizeSvgRequest::decode(&req[1..])?;
            handle_rasterize(request, resp, state)
        }
    }
}

fn handle_rasterize(req: RasterizeSvgRequest, resp: &mut [u8], state: &mut Blossom) -> Option<usize> {
    // Get SVG bytes from source
    let svg_bytes: Vec<u8> = match req.source {
        SvgSource::Bytespace(bs_id) => {
            let size = bytespace_info(bs_id).ok()?;
            let ptr = bytespace_map(bs_id).ok()?;
            let data = unsafe { core::slice::from_raw_parts(ptr as *const u8, size) };
            let bytes = data.to_vec();
            let _ = bytespace_unmap(bs_id, ptr);
            bytes
        }
        SvgSource::InlineBytes(bytes) => bytes,
    };
    
    if svg_bytes.is_empty() {
        return encode_error(SvgStatus::ErrInvalidSvg, resp);
    }
    
    // Compute hashes
    let svg_hash = Blossom::compute_svg_hash(&svg_bytes);
    let variant_hash = Blossom::compute_variant_hash(svg_hash, req.width, req.height, req.pixel_format, req.flags);
    
    // Check cache
    if let Some(entry) = state.get_cached(variant_hash) {
        debug!("BLOSSOM: Cache hit for variant {:016x}", variant_hash);
        let response = RasterizeSvgResponse {
            status: SvgStatus::Ok as u8,
            raster_bytespace: entry.bytespace_id,
            width: entry.width,
            height: entry.height,
            stride_bytes: entry.stride,
            pixel_format: entry.pixel_format,
            variant_hash,
        };
        return response.encode(resp);
    }
    
    // Cache miss - need to rasterize
    debug!("BLOSSOM: Cache miss for variant {:016x}, rasterizing {}x{}", variant_hash, req.width, req.height);
    state.raster_count += 1;
    
    // Validate SVG is UTF-8
    let _xml = match core::str::from_utf8(&svg_bytes) {
        Ok(s) => s,
        Err(_) => return encode_error(SvgStatus::ErrInvalidSvg, resp),
    };
    
    // Rasterize SVG to pixel buffer
    // For now, create a simple colored placeholder based on the SVG hash
    // TODO: Integrate full SVG rasterization (requires extracting bloom's raster pipeline)
    let width = req.width;
    let height = req.height;
    let stride = width * 4;
    let pixel_count = (width * height) as usize;
    let pixel_bytes = pixel_count * 4;
    
    // Create bytespace for raster data
    let bs_id = match bytespace_create(pixel_bytes, 0, 0) {
        Ok(id) => id,
        Err(_) => return encode_error(SvgStatus::ErrOom, resp),
    };
    
    // Generate placeholder pixels (color derived from hash for variety)
    let r = ((svg_hash >> 0) & 0xFF) as u8;
    let g = ((svg_hash >> 8) & 0xFF) as u8;
    let b = ((svg_hash >> 16) & 0xFF) as u8;
    let pixel = ((255u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    
    let mut pixels = alloc::vec![0u8; pixel_bytes];
    for i in 0..pixel_count {
        let offset = i * 4;
        pixels[offset..offset+4].copy_from_slice(&pixel.to_le_bytes());
    }
    
    if bytespace_write(bs_id, 0, &pixels).is_err() {
        return encode_error(SvgStatus::ErrOom, resp);
    }
    
    // Store in cache
    let entry = CacheEntry {
        bytespace_id: bs_id,
        width: req.width,
        height: req.height,
        stride,
        pixel_format: req.pixel_format,
    };
    state.store(variant_hash, entry);
    
    // Create graph node for cache introspection
    if let Ok(variant_node) = create_node(kinds::SVG_RASTER_VARIANT) {
        let _ = prop_set(variant_node, abi::schema::keys::SVG_VARIANT_HASH, variant_hash);
        let _ = prop_set(variant_node, abi::schema::keys::SVG_RASTER_BYTESPACE, bs_id.to_u64_lossy());
        let _ = prop_set(variant_node, abi::schema::keys::SVG_RASTER_WIDTH, req.width as u64);
        let _ = prop_set(variant_node, abi::schema::keys::SVG_RASTER_HEIGHT, req.height as u64);
    }
    
    info!("BLOSSOM: Rasterized variant {:016x} ({}x{})", variant_hash, req.width, req.height);
    
    let response = RasterizeSvgResponse {
        status: SvgStatus::Ok as u8,
        raster_bytespace: bs_id,
        width: req.width,
        height: req.height,
        stride_bytes: stride,
        pixel_format: req.pixel_format,
        variant_hash,
    };
    response.encode(resp)
}
