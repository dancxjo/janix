#![no_std]
#![no_main]

extern crate alloc;
extern crate stem;

mod atlas;

use abi::font_protocol::{
    decode_request_tag, encode_error, encode_pong, AtlasFormat, EnsureGlyphs, EnsureGlyphsResp,
    FaceMetrics, FontError, FontRequestTag, GetFaceMetrics, GlyphPlacement,
};
use abi::ids::HandleId as AbiHandleId;
use abi::root::RootWatchFilter;
use abi::schema::{keys, kinds, rels};
use abi::types::{HandleId, WatchMode, WatchSpec};
use abi::watch::{self, WatchOp};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use fontdue::{Font, FontSettings};
use log::{debug, error, warn};
use stem::info;
use stem::syscall;
use stem::thing::sys::{
    bytespace_create, bytespace_info, bytespace_map, bytespace_unmap, bytespace_write, create_node,
    find, get_edges, intern, link, prop_get, prop_set,
};
use stem::thing::ThingId;
use ttf_parser::{name_id, Face};

use alloc::collections::BTreeMap;
use atlas::{AtlasCache, AtlasKey};
use hashbrown::HashMap;

/// Font service port name
const FONTD_PORT_NAME: &str = "fontd";

struct FontD {
    fonts: HashMap<ThingId, Font>,
    glyph_cache: BTreeMap<u64, ThingId>,
    atlas_cache: AtlasCache,
    metrics_cache: BTreeMap<(u64, u16), FaceMetrics>,
}

impl FontD {
    fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            glyph_cache: BTreeMap::new(),
            atlas_cache: AtlasCache::new(),
            metrics_cache: BTreeMap::new(),
        }
    }

    fn ensure_font(&mut self, face_id: ThingId) -> Option<&Font> {
        if self.fonts.contains_key(&face_id) {
            return self.fonts.get(&face_id);
        }

        let asset_id = resolve_face_asset(face_id)?;
        let size = bytespace_info(asset_id).ok()?;
        let ptr = bytespace_map(asset_id).ok()?;
        let data = unsafe { core::slice::from_raw_parts(ptr as *const u8, size) };

        // We MUST own the bytes for Fontdue (it doesn't copy)
        let mut owned = Vec::with_capacity(size);
        owned.extend_from_slice(data);
        let _ = bytespace_unmap(asset_id, ptr);

        // Leak the Vec to get a 'static slice for Fontdue
        // In a real system we'd manage this better, but here we just keep it in memory
        let static_slice: &'static [u8] = Vec::leak(owned);

        let font = match Font::from_bytes(static_slice, FontSettings::default()) {
            Ok(font) => font,
            Err(e) => {
                error!("FONTD: Failed to parse font {:?}: {}", asset_id, e);
                return None;
            }
        };

        self.fonts.insert(face_id, font);
        self.fonts.get(&face_id)
    }
}

#[stem::main]
fn main() -> ! {
    info!("FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)");
    let mut state = FontD::new();

    // Open IPC ports for font service
    // Clients send to fontd_req (write), fontd reads from fontd_req (read)
    // Fontd sends to fontd_resp (write), clients read from fontd_resp (read)
    let (fontd_req, fontd_resp) = match (syscall::port_create(8192), syscall::port_create(8192)) {
        (Ok(req), Ok(resp)) => {
            // Create service node and advertise port handles
            if let Ok(svc_node) = create_node("svc.FontD") {
                let _ = prop_set(svc_node, "fontd.req", req.0 as u64); // Client writes here
                let _ = prop_set(svc_node, "fontd.resp", resp.1 as u64); // Client reads here
                info!(
                    "FONTD: Service node created, req={}, resp={}",
                    req.0, resp.1
                );
            }
            (req.1, resp.0) // fontd uses (read, write) sides
        }
        _ => {
            warn!("FONTD: Failed to create IPC ports, running in watch-only mode");
            (0, 0)
        }
    };

    // Open watches for font import/glyph requests (legacy compatibility)
    let glyph_watch = open_watch(kinds::FONT_GLYPH_REQUEST);
    let import_watch = open_watch(kinds::FONT_IMPORT_REQUEST);
    // Watch for new font assets (auto-import path)
    let asset_watch = open_watch(kinds::ASSET);
    info!(
        "FONTD: Opened ASSET watch (handle={}) for kind '{}'",
        asset_watch,
        kinds::ASSET
    );

    info!("FONTD: Service ready");

    let mut seq_out = 0u64;
    let mut watch_buf = [0u8; 4096];
    let mut ipc_buf = [0u8; 8192];
    let mut resp_buf = [0u8; 16384];

    loop {
        // Process watch events (legacy path)
        if let Ok(len) = syscall::root_watch_next(glyph_watch, &mut seq_out, &mut watch_buf) {
            if len > 0 {
                process_glyph_events(&watch_buf[..len], &mut state);
            }
        }
        if let Ok(len) = syscall::root_watch_next(import_watch, &mut seq_out, &mut watch_buf) {
            if len > 0 {
                process_import_events(&watch_buf[..len], &mut state);
            }
        }
        // Process new font assets (auto-import path)
        if let Ok(len) = syscall::root_watch_next(asset_watch, &mut seq_out, &mut watch_buf) {
            if len > 0 {
                process_asset_events(&watch_buf[..len], &mut state);
            }
        }

        // Process IPC requests (new atlas-based path)
        if fontd_req != 0 {
            match syscall::port_recv(fontd_req, &mut ipc_buf) {
                Ok(len) if len > 0 => {
                    if let Some(resp_len) =
                        handle_ipc_request(&ipc_buf[..len], &mut resp_buf, &mut state)
                    {
                        let _ = syscall::port_send(fontd_resp, &resp_buf[..resp_len]);
                    }
                }
                _ => {}
            }
        }

        syscall::sleep_ms(10);
    }
}

fn open_watch(kind: &str) -> usize {
    let pred = intern(kind).unwrap_or(0);
    let filter = RootWatchFilter::kind(pred as u32);
    let spec = WatchSpec {
        mode: WatchMode::QueryThenStream as u32,
        filter_ptr: &filter as *const _ as u64,
        filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
        ..Default::default()
    };
    syscall::root_watch_open(&spec).expect("FONTD: Failed to open watch")
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
    let cache_key = (req.face_id.to_u64_lossy(), req.px_size);

    // Check cache first
    if let Some(metrics) = state.metrics_cache.get(&cache_key) {
        return metrics.encode(resp);
    }

    // Load font and compute metrics
    let font = state.ensure_font(req.face_id)?;
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

    // Ensure font is loaded first, this mutably borrows state briefly
    if state.ensure_font(req.face_id).is_none() {
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
        if let Some(font) = state.fonts.get(&req.face_id) {
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
        atlas_bytespace: atlas.bytespace_id,
        atlas_width: atlas.width,
        atlas_height: atlas.height,
        atlas_format: AtlasFormat::A8,
        atlas_version: atlas.version,
        placements,
        missing,
    };

    response.encode(resp)
}

fn process_glyph_events(payload: &[u8], state: &mut FontD) {
    let mut cursor = 0usize;
    while cursor < payload.len() {
        if let Ok((header, value)) = watch::decode_event(&payload[cursor..]) {
            cursor += watch::WATCH_EVENT_HEADER_LEN + value.len();
            if WatchOp::from_u8(header.op) == Some(WatchOp::Upsert) {
                handle_glyph_request(header.subject, state);
            }
        } else {
            break;
        }
    }
}

fn handle_glyph_request(req_id: ThingId, state: &mut FontD) {
    let face_handle = prop_get(req_id, keys::FONT_REQUEST_FACE).unwrap_or(0);
    if face_handle == 0 {
        return;
    }
    let face_id = ThingId::from_u64(face_handle);

    let codepoint = prop_get(req_id, keys::FONT_REQUEST_CODEPOINT).unwrap_or(0) as u32;
    let px_size = prop_get(req_id, keys::FONT_REQUEST_PX_SIZE).unwrap_or(16) as u16;

    let cache_key = hash64(face_handle, codepoint, px_size as u32);
    if let Some(&glyph_node) = state.glyph_cache.get(&cache_key) {
        let _ = link(req_id, rels::FONT_HAS_RESULT, glyph_node);
        return;
    }

    let font = match state.ensure_font(face_id) {
        Some(f) => f,
        None => return,
    };

    let (metrics, bitmap) = font.rasterize(
        core::char::from_u32(codepoint).unwrap_or(' '),
        px_size as f32,
    );
    let glyph_node = create_node(kinds::FONT_GLYPH).unwrap_or_else(|_| ThingId::default());
    if glyph_node == ThingId::default() {
        return;
    }

    let bs_id = bytespace_create(bitmap.len(), 0, 0).unwrap_or_else(|_| ThingId::default());
    let _ = bytespace_write(bs_id, 0, &bitmap);

    let _ = prop_set(glyph_node, keys::FONT_GLYPH_CODEPOINT, codepoint as u64);
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_PX_SIZE, px_size as u64);
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_BITMAP, bs_id.to_u64_lossy());
    let _ = prop_set(
        glyph_node,
        keys::FONT_GLYPH_ADVANCE,
        metrics.advance_width as u64,
    );
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_WIDTH, metrics.width as u64);
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_HEIGHT, metrics.height as u64);
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_OFFSET_X, metrics.xmin as u64);
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_OFFSET_Y, metrics.ymin as u64);
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_CACHE_KEY, cache_key);

    let _ = link(req_id, rels::FONT_HAS_RESULT, glyph_node);
    let _ = link(face_id, rels::FONT_HAS_GLYPH, glyph_node);
    state.glyph_cache.insert(cache_key, glyph_node);
}

fn process_import_events(payload: &[u8], state: &mut FontD) {
    let mut cursor = 0usize;
    while cursor < payload.len() {
        if let Ok((header, value)) = watch::decode_event(&payload[cursor..]) {
            cursor += watch::WATCH_EVENT_HEADER_LEN + value.len();
            if WatchOp::from_u8(header.op) == Some(WatchOp::Upsert) {
                handle_import_request(header.subject, state);
            }
        } else {
            break;
        }
    }
}

/// Process asset watch events and auto-import fonts.
fn process_asset_events(payload: &[u8], state: &mut FontD) {
    let font_kind_sym = intern("font").unwrap_or(0) as u64;
    if font_kind_sym == 0 {
        return;
    }

    let mut cursor = 0usize;
    let mut event_count = 0usize;
    let mut font_count = 0usize;

    while cursor < payload.len() {
        if let Ok((header, value)) = watch::decode_event(&payload[cursor..]) {
            cursor += watch::WATCH_EVENT_HEADER_LEN + value.len();
            event_count += 1;

            if WatchOp::from_u8(header.op) == Some(WatchOp::Upsert) {
                // Check if this asset is a font
                let asset_kind = prop_get(header.subject, keys::ASSET_KIND).unwrap_or(0);
                if asset_kind == font_kind_sym {
                    font_count += 1;
                    // Get the bytespace for this font asset
                    let bs_val = prop_get(header.subject, keys::ASSET_BYTESPACE).unwrap_or(0);
                    if bs_val != 0 {
                        handle_font_asset_import(header.subject, ThingId::from_u64(bs_val), state);
                    }
                }
            }
        } else {
            break;
        }
    }

    if event_count > 0 {
        info!(
            "FONTD: Processed {} ASSET events ({} fonts)",
            event_count, font_count
        );
    }
}

/// Directly import a font from an asset's bytespace (auto-import path).
fn handle_font_asset_import(asset_id: ThingId, bs_id: ThingId, _state: &mut FontD) {
    info!("FONTD: Auto-importing font asset {:?}", asset_id);

    let size = match bytespace_info(bs_id) {
        Ok(s) => s,
        Err(_) => return,
    };
    let ptr = match bytespace_map(bs_id) {
        Ok(p) => p,
        Err(_) => return,
    };
    let data = unsafe { core::slice::from_raw_parts(ptr as *const u8, size) };

    let face = match Face::parse(data, 0) {
        Ok(f) => f,
        Err(_) => {
            let _ = bytespace_unmap(bs_id, ptr);
            return;
        }
    };

    let family_name = extract_name(&face, name_id::TYPOGRAPHIC_FAMILY, name_id::FAMILY)
        .unwrap_or_else(|| "Unknown".into());
    let style_name = extract_name(&face, name_id::TYPOGRAPHIC_SUBFAMILY, name_id::SUBFAMILY)
        .unwrap_or_else(|| "Regular".into());

    let family_key = intern(&family_name).unwrap_or(0) as u64;
    let family_id =
        get_or_create_node_by_prop(kinds::FONT_FAMILY, keys::FONT_FAMILY_KEY, family_key);
    let _ = prop_set(family_id, keys::FONT_FAMILY_KEY, family_key);
    set_prop_bytespace_str(family_id, keys::FONT_NAME, &family_name);

    let face_id = create_node(kinds::FONT_FACE).unwrap_or_else(|_| ThingId::default());
    let _ = prop_set(face_id, keys::FONT_WEIGHT, face.weight().to_number() as u64);
    let _ = prop_set(face_id, keys::FONT_WIDTH, face.width().to_number() as u64);
    set_prop_bytespace_str(face_id, keys::FONT_STYLE, &style_name);

    let _ = link(family_id, rels::FONT_HAS_FACE, face_id);
    let _ = link(face_id, rels::FONT_HAS_ASSET, bs_id);

    let _ = bytespace_unmap(bs_id, ptr);
    info!(
        "FONTD: Auto-imported font '{}' style '{}' from asset {:?}",
        family_name, style_name, asset_id
    );
}

fn handle_import_request(req_id: ThingId, _state: &mut FontD) {
    let bs_val = prop_get(req_id, keys::FONT_IMPORT_ASSET).unwrap_or(0);
    if bs_val == 0 {
        return;
    }
    let asset_id = ThingId::from_u64(bs_val);

    info!("FONTD: Importing font asset {:?}", asset_id);

    let size = match bytespace_info(asset_id) {
        Ok(s) => s,
        Err(_) => return,
    };
    let ptr = match bytespace_map(asset_id) {
        Ok(p) => p,
        Err(_) => return,
    };
    let data = unsafe { core::slice::from_raw_parts(ptr as *const u8, size) };

    let face = match Face::parse(data, 0) {
        Ok(f) => f,
        Err(_) => {
            let _ = bytespace_unmap(asset_id, ptr);
            return;
        }
    };

    let family_name = extract_name(&face, name_id::TYPOGRAPHIC_FAMILY, name_id::FAMILY)
        .unwrap_or_else(|| "Unknown".into());
    let style_name = extract_name(&face, name_id::TYPOGRAPHIC_SUBFAMILY, name_id::SUBFAMILY)
        .unwrap_or_else(|| "Regular".into());

    let family_key = intern(&family_name).unwrap_or(0) as u64;
    let family_id =
        get_or_create_node_by_prop(kinds::FONT_FAMILY, keys::FONT_FAMILY_KEY, family_key);
    let _ = prop_set(family_id, keys::FONT_FAMILY_KEY, family_key);
    set_prop_bytespace_str(family_id, keys::FONT_NAME, &family_name);

    let face_id = create_node(kinds::FONT_FACE).unwrap_or_else(|_| ThingId::default());
    let _ = prop_set(face_id, keys::FONT_WEIGHT, face.weight().to_number() as u64);
    let _ = prop_set(face_id, keys::FONT_WIDTH, face.width().to_number() as u64);
    set_prop_bytespace_str(face_id, keys::FONT_STYLE, &style_name);

    let _ = link(family_id, rels::FONT_HAS_FACE, face_id);
    let _ = link(face_id, rels::FONT_HAS_ASSET, asset_id);

    // Mark as done
    let _ = prop_set(req_id, keys::FONT_IMPORT_STATUS, 1);
    let _ = bytespace_unmap(asset_id, ptr);
    info!(
        "FONTD: Imported family '{}' style '{}'",
        family_name, style_name
    );
}

fn resolve_face_asset(face_id: ThingId) -> Option<ThingId> {
    let mut edges = [abi::types::Edge::default(); 16];
    let count = get_edges(face_id, &mut edges).ok()?;
    let asset_rel = intern(rels::FONT_HAS_ASSET).unwrap_or(0) as u64;
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() == asset_rel {
            return Some(edge.to);
        }
    }
    None
}

fn extract_name(face: &Face<'_>, primary: u16, fallback: u16) -> Option<String> {
    face.names()
        .into_iter()
        .find(|n| n.name_id == primary && n.is_unicode())
        .or_else(|| {
            face.names()
                .into_iter()
                .find(|n| n.name_id == fallback && n.is_unicode())
        })
        .and_then(|n| {
            let mut buf = Vec::with_capacity(n.name.len() / 2);
            for chunk in n.name.chunks_exact(2) {
                buf.push(u16::from_be_bytes([chunk[0], chunk[1]]));
            }
            String::from_utf16(&buf).ok()
        })
}

fn get_or_create_node_by_prop(kind: &str, key: &str, val: u64) -> ThingId {
    let mut nodes = [ThingId::default(); 128];
    if let Ok(count) = find(kind, &mut nodes) {
        for id in nodes.iter().take(count) {
            if let Ok(v) = prop_get(*id, key) {
                if v == val {
                    return *id;
                }
            }
        }
    }
    create_node(kind).unwrap_or_else(|_| ThingId::default())
}

fn set_prop_bytespace_str(id: ThingId, key: &str, val: &str) {
    if let Ok(bs) = bytespace_create(val.len(), 0, 0) {
        let _ = bytespace_write(bs, 0, val.as_bytes());
        let _ = prop_set(id, key, bs.to_u64_lossy());
    }
}

fn hash64(a: u64, b: u32, c: u32) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for &byte in a
        .to_le_bytes()
        .iter()
        .chain(b.to_le_bytes().iter())
        .chain(c.to_le_bytes().iter())
    {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
