//! Font client for IPC communication with fontd service.
//!
//! This module provides:
//! - GlyphCache: Caches glyph placements keyed by (face, size, glyph)
//! - AtlasMapping: Tracks mapped atlas bytespaces per (face, size)
//! - FontClient: IPC interface to fontd using ports

extern crate alloc;

use abi::font_protocol::{
    decode_response_tag, EnsureGlyphs, EnsureGlyphsResp, FaceMetrics, FontResponseTag,
    GetFaceMetrics, GlyphPlacement,
};
use abi::ids::HandleId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use spin::Mutex;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::sys::{bytespace_map, bytespace_unmap, find, prop_get};
use stem::thing::ThingId;

/// Cached glyph entry with atlas location
#[derive(Debug, Clone, Copy)]
pub struct GlyphEntry {
    pub atlas_bytespace: ThingId,
    pub atlas_version: u64,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance: i16,
}

impl From<&GlyphPlacement> for GlyphEntry {
    fn from(p: &GlyphPlacement) -> Self {
        Self {
            atlas_bytespace: ThingId::default(), // Filled in by caller
            atlas_version: 0,
            x: p.x,
            y: p.y,
            w: p.w,
            h: p.h,
            bearing_x: p.bearing_x,
            bearing_y: p.bearing_y,
            advance: p.advance,
        }
    }
}

/// Atlas mapping state for a (face, size) combination
pub struct AtlasMapping {
    pub bytespace_id: ThingId,
    pub version: u64,
    pub width: u32,
    pub height: u32,
    /// Stored as usize for Send safety, convert to pointer when accessing
    ptr_val: usize,
}

impl AtlasMapping {
    pub fn new(bytespace_id: ThingId, version: u64, width: u32, height: u32) -> Self {
        Self {
            bytespace_id,
            version,
            width,
            height,
            ptr_val: 0,
        }
    }

    /// Map the atlas bytespace if not already mapped
    pub fn ensure_mapped(&mut self) -> Option<*const u8> {
        if self.ptr_val == 0 {
            if let Ok(p) = bytespace_map(self.bytespace_id) {
                self.ptr_val = p as usize;
            }
        }
        if self.ptr_val != 0 {
            Some(self.ptr_val as *const u8)
        } else {
            None
        }
    }

    /// Get pixel at (x, y) assuming A8 format
    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        if x >= self.width || y >= self.height || self.ptr_val == 0 {
            return 0;
        }
        let offset = (y * self.width + x) as usize;
        unsafe { *(self.ptr_val as *const u8).add(offset) }
    }

    /// Unmap and clear pointer
    pub fn unmap(&mut self) {
        if self.ptr_val != 0 {
            let _ = bytespace_unmap(self.bytespace_id, self.ptr_val as *mut u8);
            self.ptr_val = 0;
        }
    }
}

/// Glyph key for cache lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlyphKey {
    pub face_id: u64,
    pub px_size: u16,
    pub glyph_id: u32,
}

impl GlyphKey {
    pub fn new(face_id: ThingId, px_size: u16, glyph_id: u32) -> Self {
        Self {
            face_id: face_id.to_u64_lossy(),
            px_size,
            glyph_id,
        }
    }
}

/// Atlas key for mapping lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtlasKey {
    pub face_id: u64,
    pub px_size: u16,
}

impl AtlasKey {
    pub fn new(face_id: ThingId, px_size: u16) -> Self {
        Self {
            face_id: face_id.to_u64_lossy(),
            px_size,
        }
    }
}

/// Font client state - owns caches and port handles
pub struct FontClient {
    /// Port to send requests TO fontd
    req_port: PortHandle,
    /// Port to receive responses FROM fontd
    resp_port: PortHandle,
    /// Glyph placement cache
    glyph_cache: BTreeMap<GlyphKey, GlyphEntry>,
    /// Atlas mappings per (face, size)
    atlas_mappings: BTreeMap<AtlasKey, AtlasMapping>,
    /// Metrics cache per (face, size)
    metrics_cache: BTreeMap<AtlasKey, FaceMetrics>,
    /// Pending glyph requests (batched)
    pending_requests: Vec<(ThingId, u16, Vec<u32>)>,
}

impl FontClient {
    pub fn new(req_port: PortHandle, resp_port: PortHandle) -> Self {
        Self {
            req_port,
            resp_port,
            glyph_cache: BTreeMap::new(),
            atlas_mappings: BTreeMap::new(),
            metrics_cache: BTreeMap::new(),
            pending_requests: Vec::new(),
        }
    }

    /// Discover fontd service from graph
    pub fn discover() -> Option<Self> {
        let mut nodes = [ThingId::default(); 4];
        let count = find("svc.FontD", &mut nodes).ok()?;
        if count == 0 {
            return None;
        }
        let svc_node = nodes[0];
        let req = prop_get(svc_node, "fontd.req").ok()? as PortHandle;
        let resp = prop_get(svc_node, "fontd.resp").ok()? as PortHandle;
        if req == 0 || resp == 0 {
            return None;
        }
        Some(Self::new(req, resp))
    }

    /// Check if client is connected
    pub fn is_connected(&self) -> bool {
        self.req_port != 0 && self.resp_port != 0
    }

    /// Get cached glyph entry if it exists and version matches
    pub fn get_glyph(&self, face_id: ThingId, px_size: u16, glyph_id: u32) -> Option<&GlyphEntry> {
        let key = GlyphKey::new(face_id, px_size, glyph_id);
        let entry = self.glyph_cache.get(&key)?;

        // Check if atlas version is still current
        let atlas_key = AtlasKey::new(face_id, px_size);
        if let Some(mapping) = self.atlas_mappings.get(&atlas_key) {
            if mapping.version == entry.atlas_version {
                return Some(entry);
            }
        }
        None
    }

    /// Get atlas mapping for a (face, size)
    pub fn get_atlas_mapping(
        &mut self,
        face_id: ThingId,
        px_size: u16,
    ) -> Option<&mut AtlasMapping> {
        let key = AtlasKey::new(face_id, px_size);
        self.atlas_mappings.get_mut(&key)
    }

    /// Ensure glyphs are available (batch request to fontd)
    /// Returns placements for all requested glyphs that are now available
    pub fn ensure_glyphs(
        &mut self,
        face_id: ThingId,
        px_size: u16,
        glyph_ids: &[u32],
    ) -> Vec<GlyphEntry> {
        if !self.is_connected() || glyph_ids.is_empty() {
            return Vec::new();
        }

        // Partition into cached and missing
        let mut results = Vec::new();
        let mut missing = Vec::new();

        for &gid in glyph_ids {
            if let Some(entry) = self.get_glyph(face_id, px_size, gid) {
                results.push(*entry);
            } else {
                missing.push(gid);
            }
        }

        if missing.is_empty() {
            return results;
        }

        // Build and send EnsureGlyphs request
        let req = EnsureGlyphs {
            face_id,
            px_size,
            glyph_ids: missing.clone(),
        };

        let mut req_buf = [0u8; 2048];
        let Some(req_len) = req.encode(&mut req_buf) else {
            return results;
        };

        if port_send(self.req_port, &req_buf[..req_len]).is_err() {
            return results;
        }

        // Wait for response (blocking for now - TODO: make async)
        let mut resp_buf = [0u8; 16384];
        let resp_len = match port_recv(self.resp_port, &mut resp_buf) {
            Ok(len) if len > 0 => len,
            _ => return results,
        };

        // Decode response
        let Some(tag) = decode_response_tag(&resp_buf[..resp_len]) else {
            return results;
        };

        if tag != FontResponseTag::EnsureGlyphsResp {
            return results;
        }

        let Some(resp) = EnsureGlyphsResp::decode(&resp_buf[1..resp_len]) else {
            return results;
        };

        // Update atlas mapping
        let atlas_key = AtlasKey::new(face_id, px_size);
        let mapping = self.atlas_mappings.entry(atlas_key).or_insert_with(|| {
            AtlasMapping::new(
                resp.atlas_bytespace,
                resp.atlas_version,
                resp.atlas_width,
                resp.atlas_height,
            )
        });

        // Check for version change
        if mapping.version != resp.atlas_version {
            // Unmap old atlas
            mapping.unmap();
            mapping.bytespace_id = resp.atlas_bytespace;
            mapping.version = resp.atlas_version;
            mapping.width = resp.atlas_width;
            mapping.height = resp.atlas_height;

            // Invalidate cached glyphs for this face/size
            let prefix = GlyphKey::new(face_id, px_size, 0);
            self.glyph_cache
                .retain(|k, _| k.face_id != prefix.face_id || k.px_size != prefix.px_size);
        }

        // Cache new placements
        for p in &resp.placements {
            let key = GlyphKey::new(face_id, px_size, p.glyph_id);
            let mut entry = GlyphEntry::from(p);
            entry.atlas_bytespace = resp.atlas_bytespace;
            entry.atlas_version = resp.atlas_version;
            self.glyph_cache.insert(key, entry);
            results.push(entry);
        }

        results
    }

    /// Get face metrics (cached)
    pub fn get_metrics(&mut self, face_id: ThingId, px_size: u16) -> Option<FaceMetrics> {
        let key = AtlasKey::new(face_id, px_size);
        if let Some(m) = self.metrics_cache.get(&key) {
            return Some(*m);
        }

        if !self.is_connected() {
            return None;
        }

        // Request metrics
        let req = GetFaceMetrics { face_id, px_size };
        let mut req_buf = [0u8; 32];
        let req_len = req.encode(&mut req_buf)?;

        if port_send(self.req_port, &req_buf[..req_len]).is_err() {
            return None;
        }

        let mut resp_buf = [0u8; 64];
        let resp_len = port_recv(self.resp_port, &mut resp_buf).ok()?;
        if resp_len == 0 {
            return None;
        }

        if decode_response_tag(&resp_buf)? != FontResponseTag::FaceMetrics {
            return None;
        }

        let metrics = FaceMetrics::decode(&resp_buf[1..resp_len])?;
        self.metrics_cache.insert(key, metrics);
        Some(metrics)
    }
}

// ============================================================================
// Global font client instance
// ============================================================================

static FONT_CLIENT: Mutex<Option<FontClient>> = Mutex::new(None);

/// Initialize the global font client (called from main)
pub fn init() {
    let mut guard = FONT_CLIENT.lock();
    if guard.is_none() {
        *guard = FontClient::discover();
    }
}

/// Check if font client is available
pub fn is_available() -> bool {
    FONT_CLIENT
        .lock()
        .as_ref()
        .map(|c| c.is_connected())
        .unwrap_or(false)
}

/// Ensure glyphs are available (batch request)
pub fn ensure_glyphs(face_id: ThingId, px_size: u16, glyph_ids: &[u32]) -> Vec<GlyphEntry> {
    let mut guard = FONT_CLIENT.lock();
    match guard.as_mut() {
        Some(client) => client.ensure_glyphs(face_id, px_size, glyph_ids),
        None => Vec::new(),
    }
}

/// Get cached glyph entry
pub fn get_glyph(face_id: ThingId, px_size: u16, glyph_id: u32) -> Option<GlyphEntry> {
    let guard = FONT_CLIENT.lock();
    guard
        .as_ref()?
        .get_glyph(face_id, px_size, glyph_id)
        .copied()
}

/// Get face metrics
pub fn get_metrics(face_id: ThingId, px_size: u16) -> Option<FaceMetrics> {
    let mut guard = FONT_CLIENT.lock();
    guard.as_mut()?.get_metrics(face_id, px_size)
}

/// Get atlas mapping for blitting
pub fn with_atlas<F, R>(face_id: ThingId, px_size: u16, f: F) -> Option<R>
where
    F: FnOnce(&mut AtlasMapping) -> R,
{
    let mut guard = FONT_CLIENT.lock();
    let client = guard.as_mut()?;
    let mapping = client.get_atlas_mapping(face_id, px_size)?;
    mapping.ensure_mapped()?;
    Some(f(mapping))
}
