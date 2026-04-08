#![feature(restricted_std)]
#![no_main]

extern crate alloc;
extern crate std;
extern crate stem;

mod atlas;

use abi::font_protocol::{
    AtlasFormat, EnsureGlyphs, EnsureGlyphsResp, FaceMetrics, FontError, FontRequestTag,
    GetFaceMetrics, GlyphPlacement, decode_request_tag, encode_error, encode_pong,
};
use abi::ids::HandleId;
use abi::wait::{WaitKind, WaitResult, WaitSpec, interest, ready};
use alloc::vec::Vec;
use fontdue::{Font, FontSettings};
use stem::syscall;
use stem::thing::ThingId;
use stem::{error, info, warn};

use alloc::collections::BTreeMap;
use atlas::{AtlasCache, AtlasKey};
use hashbrown::HashMap;

struct FontD {
    fonts: HashMap<u64, Font>,
    atlas_cache: AtlasCache,
    metrics_cache: BTreeMap<(u64, u16), FaceMetrics>,
}

impl FontD {
    fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            atlas_cache: AtlasCache::new(),
            metrics_cache: BTreeMap::new(),
        }
    }

    fn ensure_font(&mut self, face_id: u64) -> Option<&Font> {
        if self.fonts.contains_key(&face_id) {
            return self.fonts.get(&face_id);
        }

        // For now, load default font from /boot for face_id == 1
        // Other IDs can be mapped later if we add a "load from path" IPC
        if face_id != 1 {
            return None;
        }

        let path = "/boot/NotoSans-Regular.ttf";
        info!("FONTD: Loading default font from {}", path);

        let data = std::fs::read(path)
            .map_err(|e| {
                error!("FONTD: Failed to read font file {}: {}", path, e);
            })
            .ok()?;

        let static_slice: &'static [u8] = Vec::leak(data);

        let font = match Font::from_bytes(static_slice, FontSettings::default()) {
            Ok(font) => font,
            Err(e) => {
                error!("FONTD: Failed to parse font: {}", e);
                return None;
            }
        };

        self.fonts.insert(face_id, font);
        self.fonts.get(&face_id)
    }
}

#[stem::main]
fn main() -> ! {
    info!("FONTD: Starting Font Service");

    let mut state = FontD::new();

    // Open IPC ports for font service
    // Clients send to fontd_req (write), fontd reads from fontd_req (read)
    // Fontd sends to fontd_resp (write), clients read from fontd_resp (read)
    let (fontd_req, fontd_resp) =
        match (syscall::channel_create(8192), syscall::channel_create(8192)) {
            (Ok(req), Ok(resp)) => {
                // VFS-native service publication
                let _ = syscall::vfs_mkdir("/services");
                let _ = syscall::vfs_mkdir("/services/font");
 
                if let Err(e) = syscall::vfs_mount(req.0, "/services/font/req") {
                    warn!("FONTD: Failed to mount /services/font/req: {:?}", e);
                }
                if let Err(e) = syscall::vfs_mount(resp.1, "/services/font/resp") {
                    warn!("FONTD: Failed to mount /services/font/resp: {:?}", e);
                }
 
                info!(
                    "FONTD: Service ports mounted to /services/font/{{req,resp}}"
                );
                (req.1, resp.0)
            }
            _ => {
                warn!("FONTD: Failed to create IPC ports");
                (0, 0)
            }
        };

    info!("FONTD: Service ready");

    let mut ipc_buf = [0u8; 8192];
    let mut resp_buf = [0u8; 16384];
    let mut ready_buf = [WaitResult::default(); 1];
    let mut wait_specs = [WaitSpec::default(); 1];

    loop {
        let mut wait_count = 0usize;

        if fontd_req != 0 {
            wait_specs[wait_count] = WaitSpec {
                kind: WaitKind::Port as u32,
                flags: interest::READABLE,
                object: fontd_req as u64,
                token: 1,
            };
            wait_count += 1;
        }

        if wait_count == 0 {
            syscall::sleep_ms(100);
            continue;
        }

        let ready_count = match syscall::wait_many(&wait_specs[..wait_count], &mut ready_buf, None)
        {
            Ok(n) => n,
            Err(err) => {
                warn!("FONTD: wait_many failed: {:?}", err);
                syscall::sleep_ms(10);
                continue;
            }
        };

        for ready_result in &ready_buf[..ready_count] {
            match ready_result.token {
                1 if (ready_result.flags & ready::READABLE) != 0 => loop {
                    match syscall::channel_try_recv(fontd_req, &mut ipc_buf) {
                        Ok(len) if len > 0 => {
                            if let Some(resp_len) =
                                handle_ipc_request(&ipc_buf[..len], &mut resp_buf, &mut state)
                            {
                                let _ = syscall::channel_send(fontd_resp, &resp_buf[..resp_len]);
                            }
                        }
                        Ok(_) | Err(abi::errors::Errno::EAGAIN) => break,
                        Err(err) => {
                            warn!("FONTD: fontd_req recv failed: {:?}", err);
                            break;
                        }
                    }
                },
                _ => {}
            }
        }
    }
}

// ============================================================================
// IPC Request Handlers (Atlas-based path)
// ============================================================================

fn handle_ipc_request(req: &[u8], resp: &mut [u8], state: &mut FontD) -> Option<usize> {
    let tag = decode_request_tag(req)?;
    match tag {
        FontRequestTag::Ping => encode_pong(resp),
        FontRequestTag::GetFaceMetrics => {
            let metrics_req = GetFaceMetrics::decode(&req[1..])?;
            handle_get_metrics(metrics_req, resp, state)
        }
        FontRequestTag::EnsureGlyphs => {
            let ensure_req = EnsureGlyphs::decode(&req[1..])?;
            handle_ensure_glyphs(ensure_req, resp, state)
        }
    }
}

fn handle_get_metrics(req: GetFaceMetrics, resp: &mut [u8], state: &mut FontD) -> Option<usize> {
    let face_id_u64 = req.face_id.to_u64_lossy();
    let cache_key = (face_id_u64, req.px_size);

    // Check cache first
    if let Some(metrics) = state.metrics_cache.get(&cache_key) {
        return metrics.encode(resp);
    }

    // Load font and compute metrics
    let font = state.ensure_font(face_id_u64)?;
    let line_metrics = font.horizontal_line_metrics(req.px_size as f32)?;

    let metrics = FaceMetrics {
        ascent: line_metrics.ascent as i16,
        descent: line_metrics.descent as i16,
        line_gap: line_metrics.line_gap as i16,
        units_per_em: font.units_per_em() as u16,
    };

    state.metrics_cache.insert(cache_key, metrics);
    metrics.encode(resp)
}

fn handle_ensure_glyphs(req: EnsureGlyphs, resp: &mut [u8], state: &mut FontD) -> Option<usize> {
    let atlas_key = AtlasKey::new(req.face_id, req.px_size);
    let face_id_u64 = req.face_id.to_u64_lossy();

    // Ensure font is loaded first, this mutably borrows state briefly
    if state.ensure_font(face_id_u64).is_none() {
        return encode_error(FontError::UnknownFace, resp);
    }

    // Now process glyphs - separate borrows for font and atlas
    let mut placements = Vec::new();
    let mut missing = Vec::new();

    // Pre-rasterize all needed glyphs (borrowing font only)
    let mut rasterized: Vec<(u32, fontdue::Metrics, Vec<u8>)> = Vec::new();
    for &glyph_id in &req.glyph_ids {
        // Check atlas first (immutable borrow of atlas_cache)
        if state
            .atlas_cache
            .get(&atlas_key)
            .map(|a| a.get_placement(glyph_id).is_some())
            .unwrap_or(false)
        {
            continue; // Already in atlas
        }

        // Need to rasterize
        if let Some(font) = state.fonts.get(&face_id_u64) {
            let ch = core::char::from_u32(glyph_id).unwrap_or(' ');
            let (metrics, bitmap) = font.rasterize(ch, req.px_size as f32);
            rasterized.push((glyph_id, metrics, bitmap));
        }
    }

    // Now pack everything into atlas (mutable borrow of atlas_cache only)
    let atlas = state.atlas_cache.get_or_create(atlas_key);

    // First collect existing placements
    for &glyph_id in &req.glyph_ids {
        if let Some(p) = atlas.get_placement(glyph_id) {
            placements.push(*p);
        }
    }

    // Now pack rasterized glyphs
    for (glyph_id, metrics, bitmap) in rasterized {
        if metrics.width == 0 || metrics.height == 0 {
            // Empty glyph (space, etc) - still valid
            let placement = GlyphPlacement {
                glyph_id,
                x: 0,
                y: 0,
                w: 0,
                h: 0,
                bearing_x: 0,
                bearing_y: 0,
                advance: metrics.advance_width as i16,
            };
            placements.push(placement);
            continue;
        }

        // Pack into atlas
        match atlas.pack_glyph(
            glyph_id,
            &bitmap,
            metrics.width as u32,
            metrics.height as u32,
            metrics.xmin as i16,
            metrics.ymin as i16,
            metrics.advance_width as i16,
        ) {
            Some(p) => placements.push(p),
            None => missing.push(glyph_id),
        }
    }

    // Commit atlas to bytespace
    if !atlas.commit() {
        return encode_error(FontError::AtlasAllocationFailed, resp);
    }

    // Build response
    let response = EnsureGlyphsResp {
        req_face_id: req.face_id,
        req_px_size: req.px_size,
        atlas_fd: atlas.atlas_fd,
        atlas_width: atlas.width,
        atlas_height: atlas.height,
        atlas_format: AtlasFormat::A8,
        atlas_version: atlas.version,
        placements,
        missing,
    };

    response.encode(resp)
}
