#![no_std]
#![no_main]

extern crate alloc;
extern crate stem;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use stem::thing::ThingId;
use stem::thing::sys::{
    bytespace_info, bytespace_map, bytespace_unmap, bytespace_create, bytespace_write,
    create_node, link, prop_get, prop_set, get_edges, find, intern,
};
use abi::schema::{keys, kinds, rels};
use abi::types::{WatchSpec, WatchMode, HandleId};
use abi::watch::{self, WatchOp};
use abi::root::RootWatchFilter;
use stem::syscall;
use log::{info, error, warn};
use fontdue::{Font, FontSettings};
use ttf_parser::{Face, name_id};

use alloc::collections::BTreeMap;
use hashbrown::HashMap;

struct FontD {
    fonts: HashMap<ThingId, Font>,
    glyph_cache: BTreeMap<u64, ThingId>,
}

impl FontD {
    fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            glyph_cache: BTreeMap::new(),
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
    info!("FONTD: Starting Graph-Native Font Service");
    let mut state = FontD::new();

    let glyph_watch = open_watch(kinds::FONT_GLYPH_REQUEST);
    let import_watch = open_watch(kinds::FONT_IMPORT_REQUEST);

    info!("FONTD: Watches active.");

    let mut seq_out = 0u64;
    let mut watch_buf = [0u8; 4096];

    loop {
        if let Ok(len) = syscall::root_watch_next(glyph_watch, &mut seq_out, &mut watch_buf) {
            if len > 0 { process_glyph_events(&watch_buf[..len], &mut state); }
        }
        if let Ok(len) = syscall::root_watch_next(import_watch, &mut seq_out, &mut watch_buf) {
            if len > 0 { process_import_events(&watch_buf[..len], &mut state); }
        }
        syscall::sleep_ms(50);
    }
}

fn open_watch(kind: &str) -> usize {
    let pred = intern(kind).unwrap_or(0);
    let filter = RootWatchFilter::predicate(pred);
    let spec = WatchSpec {
        mode: WatchMode::QueryThenStream as u32,
        filter_ptr: &filter as *const _ as u64,
        filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
        ..Default::default()
    };
    syscall::root_watch_open(&spec).expect("FONTD: Failed to open watch")
}

fn process_glyph_events(payload: &[u8], state: &mut FontD) {
    let mut cursor = 0usize;
    while cursor < payload.len() {
        if let Ok((header, value)) = watch::decode_event(&payload[cursor..]) {
            cursor += watch::WATCH_EVENT_HEADER_LEN + value.len();
            if WatchOp::from_u8(header.op) == Some(WatchOp::Upsert) {
                handle_glyph_request(header.subject, state);
            }
        } else { break; }
    }
}

fn handle_glyph_request(req_id: ThingId, state: &mut FontD) {
    let face_handle = prop_get(req_id, keys::FONT_REQUEST_FACE).unwrap_or(0);
    if face_handle == 0 { return; }
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

    let (metrics, bitmap) = font.rasterize(core::char::from_u32(codepoint).unwrap_or(' '), px_size as f32);
    let glyph_node = create_node(kinds::FONT_GLYPH).unwrap_or_else(|_| ThingId::default());
    if glyph_node == ThingId::default() { return; }

    let bs_id = bytespace_create(bitmap.len(), 0, 0).unwrap_or_else(|_| ThingId::default());
    let _ = bytespace_write(bs_id, 0, &bitmap);

    let _ = prop_set(glyph_node, keys::FONT_GLYPH_CODEPOINT, codepoint as u64);
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_PX_SIZE, px_size as u64);
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_BITMAP, bs_id.to_u64_lossy());
    let _ = prop_set(glyph_node, keys::FONT_GLYPH_ADVANCE, metrics.advance_width as u64);
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
        } else { break; }
    }
}

fn handle_import_request(req_id: ThingId, _state: &mut FontD) {
    let bs_val = prop_get(req_id, keys::FONT_IMPORT_ASSET).unwrap_or(0);
    if bs_val == 0 { return; }
    let asset_id = ThingId::from_u64(bs_val);

    info!("FONTD: Importing font asset {:?}", asset_id);

    let size = match bytespace_info(asset_id) { Ok(s) => s, Err(_) => return };
    let ptr = match bytespace_map(asset_id) { Ok(p) => p, Err(_) => return };
    let data = unsafe { core::slice::from_raw_parts(ptr as *const u8, size) };

    let face = match Face::parse(data, 0) {
        Ok(f) => f,
        Err(_) => {
            let _ = bytespace_unmap(asset_id, ptr);
            return;
        }
    };

    let family_name = extract_name(&face, name_id::TYPOGRAPHIC_FAMILY, name_id::FAMILY).unwrap_or_else(|| "Unknown".into());
    let style_name = extract_name(&face, name_id::TYPOGRAPHIC_SUBFAMILY, name_id::SUBFAMILY).unwrap_or_else(|| "Regular".into());

    let family_key = intern(&family_name).unwrap_or(0) as u64;
    let family_id = get_or_create_node_by_prop(kinds::FONT_FAMILY, keys::FONT_FAMILY_KEY, family_key);
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
    info!("FONTD: Imported family '{}' style '{}'", family_name, style_name);
}

fn resolve_face_asset(face_id: ThingId) -> Option<ThingId> {
    let mut edges = [abi::types::Edge::default(); 16];
    let count = get_edges(face_id, &mut edges).ok()?;
    let asset_rel = intern(rels::FONT_HAS_ASSET).unwrap_or(0) as u64;
    for edge in edges.iter().take(count) {
        if edge.predicate.to_u64_lossy() == asset_rel { return Some(edge.to); }
    }
    None
}

fn extract_name(face: &Face<'_>, primary: u16, fallback: u16) -> Option<String> {
    face.names().into_iter().find(|n| n.name_id == primary && n.is_unicode())
        .or_else(|| face.names().into_iter().find(|n| n.name_id == fallback && n.is_unicode()))
        .and_then(|n| {
            let mut buf = Vec::with_capacity(n.name.len() / 2);
            for chunk in n.name.chunks_exact(2) { buf.push(u16::from_be_bytes([chunk[0], chunk[1]])); }
            String::from_utf16(&buf).ok()
        })
}

fn get_or_create_node_by_prop(kind: &str, key: &str, val: u64) -> ThingId {
    let mut nodes = [ThingId::default(); 128];
    if let Ok(count) = find(kind, &mut nodes) {
        for id in nodes.iter().take(count) {
            if let Ok(v) = prop_get(*id, key) { if v == val { return *id; } }
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
    for &byte in a.to_le_bytes().iter().chain(b.to_le_bytes().iter()).chain(c.to_le_bytes().iter()) {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
