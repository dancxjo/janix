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
use stem::syscall::{
    channel_send, channel_try_recv, monotonic_ns, vm_map, vm_unmap, ChannelHandle,
};
use stem::thing::sys::{find, prop_get, stat};
use stem::thing::ThingId;

/// Cached glyph entry with atlas location
#[derive(Debug, Clone, Copy)]
pub struct GlyphEntry {
    pub atlas_fd: u32,
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
            atlas_fd: 0, // Filled in by caller
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
    pub atlas_fd: u32,
    pub version: u64,
    pub width: u32,
    pub height: u32,
    /// Stored as usize for Send safety, convert to pointer when accessing
    ptr_val: usize,
}

impl AtlasMapping {
    pub fn new(atlas_fd: u32, version: u64, width: u32, height: u32) -> Self {
        Self {
            atlas_fd,
            version,
            width,
            height,
            ptr_val: 0,
        }
    }

    /// Map the atlas bytespace if not already mapped
    pub fn ensure_mapped(&mut self) -> Option<*const u8> {
        if self.ptr_val == 0 {
            use abi::vm::{VmBacking, VmMapReq, VmProt};
            let (_, size, _) = stat(self.atlas_fd).ok()?;
            let req = VmMapReq {
                addr_hint: 0,
                len: size as usize,
                prot: VmProt::READ | VmProt::USER,
                flags: abi::vm::VmMapFlags::empty(),
                backing: VmBacking::File {
                    fd: self.atlas_fd,
                    offset: 0,
                },
            };
            if let Ok(resp) = vm_map(&req) {
                self.ptr_val = resp.addr as usize;
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
            let (_, size, _) = stat(self.atlas_fd).unwrap();
            let _ = vm_unmap(self.ptr_val, size as usize);
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
    req_port: ChannelHandle,
    /// Port to receive responses FROM fontd
    resp_port: ChannelHandle,
    /// Glyph placement cache
    glyph_cache: BTreeMap<GlyphKey, GlyphEntry>,
    /// Atlas mappings per (face, size)
    atlas_mappings: BTreeMap<AtlasKey, AtlasMapping>,
    /// Metrics cache per (face, size)
    metrics_cache: BTreeMap<AtlasKey, FaceMetrics>,
    /// Pending glyph requests (timestamp of request)
    pending_requests: BTreeMap<GlyphKey, u64>,
    /// At most one in-flight metrics request. Responses are not tagged.
    pending_metrics_request: Option<(AtlasKey, u64)>,
}

impl FontClient {
    pub fn new(req_port: ChannelHandle, resp_port: ChannelHandle) -> Self {
        Self {
            req_port,
            resp_port,
            glyph_cache: BTreeMap::new(),
            atlas_mappings: BTreeMap::new(),
            metrics_cache: BTreeMap::new(),
            pending_requests: BTreeMap::new(),
            pending_metrics_request: None,
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
        let req = prop_get(svc_node, "fontd.req").ok()? as ChannelHandle;
        let resp = prop_get(svc_node, "fontd.resp").ok()? as ChannelHandle;
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

    /// Ensure glyphs are available (send request if needed, don't wait)
    /// Returns currently matched cache entries. DOES NOT BLOCK.
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
        let now = monotonic_ns();

        for &gid in glyph_ids {
            if let Some(entry) = self.get_glyph(face_id, px_size, gid) {
                results.push(*entry);
            } else {
                // Check if already pending (and not timed out - e.g. 500ms)
                let key = GlyphKey::new(face_id, px_size, gid);
                let is_pending = self
                    .pending_requests
                    .get(&key)
                    .map(|ts| now - ts < 500_000_000)
                    .unwrap_or(false);

                if !is_pending {
                    missing.push(gid);
                    self.pending_requests.insert(key, now);
                }
            }
        }

        if missing.is_empty() {
            return results;
        }

        // Build and send EnsureGlyphs request
        let req = EnsureGlyphs {
            face_id,
            px_size,
            glyph_ids: missing,
        };

        let mut req_buf = [0u8; 2048];
        if let Some(req_len) = req.encode(&mut req_buf) {
            let _ = channel_send(self.req_port, &req_buf[..req_len]);
        }

        results
    }

    /// Poll for responses from fontd
    /// Returns true if any state was updated (caller should damage/redraw)
    pub fn poll(&mut self) -> bool {
        if !self.is_connected() {
            return false;
        }

        let mut updated = false;
        let mut resp_buf = [0u8; 16384];

        // Drain up to 10 messages per poll to avoid starving the loop
        for _ in 0..10 {
            let resp_len = match channel_try_recv(self.resp_port, &mut resp_buf) {
                Ok(len) if len > 0 => len,
                _ => break,
            };

            // Decode response
            let Some(tag) = decode_response_tag(&resp_buf[..resp_len]) else {
                continue;
            };

            match tag {
                FontResponseTag::EnsureGlyphsResp => {
                    let Some(resp) = EnsureGlyphsResp::decode(&resp_buf[1..resp_len]) else {
                        continue;
                    };

                    // Update atlas mapping
                    let atlas_key = AtlasKey::new(resp.req_face_id, resp.req_px_size);
                    let mapping = self.atlas_mappings.entry(atlas_key).or_insert_with(|| {
                        AtlasMapping::new(
                            resp.atlas_fd,
                            resp.atlas_version,
                            resp.atlas_width,
                            resp.atlas_height,
                        )
                    });

                    // Check for version change
                    if mapping.version != resp.atlas_version {
                        // Unmap old atlas
                        mapping.unmap();
                        mapping.atlas_fd = resp.atlas_fd;
                        mapping.version = resp.atlas_version;
                        mapping.width = resp.atlas_width;
                        mapping.height = resp.atlas_height;

                        // Invalidate cached glyphs for this face/size
                        let prefix = GlyphKey::new(resp.req_face_id, resp.req_px_size, 0);
                        self.glyph_cache.retain(|k, _| {
                            k.face_id != prefix.face_id || k.px_size != prefix.px_size
                        });
                    }

                    // Cache new placements and clear pending
                    for p in &resp.placements {
                        let key = GlyphKey::new(resp.req_face_id, resp.req_px_size, p.glyph_id);
                        let mut entry = GlyphEntry::from(p);
                        entry.atlas_fd = resp.atlas_fd;
                        entry.atlas_version = resp.atlas_version;
                        self.glyph_cache.insert(key, entry);
                        self.pending_requests.remove(&key);
                        updated = true;
                    }
                }
                FontResponseTag::FaceMetrics => {
                    let Some(metrics) = FaceMetrics::decode(&resp_buf[1..resp_len]) else {
                        continue;
                    };
                    if let Some((atlas_key, _)) = self.pending_metrics_request.take() {
                        self.metrics_cache.insert(atlas_key, metrics);
                        updated = true;
                    }
                }
                _ => {}
            }
        }
        updated
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

        let now = monotonic_ns();
        if let Some((pending_key, ts)) = self.pending_metrics_request {
            if now.saturating_sub(ts) < 500_000_000 {
                if pending_key == key {
                    return None;
                }
                return None;
            }
            self.pending_metrics_request = None;
        }

        let req = GetFaceMetrics { face_id, px_size };
        let mut req_buf = [0u8; 32];
        let req_len = req.encode(&mut req_buf)?;

        if channel_send(self.req_port, &req_buf[..req_len]).is_err() {
            return None;
        }

        self.pending_metrics_request = Some((key, now));
        None
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

/// Poll global font client for updates
pub fn poll() -> bool {
    let mut guard = FONT_CLIENT.lock();
    if guard.is_none() {
        *guard = FontClient::discover();
    }
    guard.as_mut().map(|c| c.poll()).unwrap_or(false)
}
