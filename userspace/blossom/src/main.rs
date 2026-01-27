#![no_std]
#![no_main]

extern crate alloc;
extern crate stem;

mod emit_paint;
mod graph_ui;
mod layout;
mod scene;

use abi::root::RootWatchFilter;
use abi::schema::{keys, kinds};
use abi::svg_protocol::{
    decode_request_tag, encode_error, RasterizeSvgRequest, RasterizeSvgResponse, SvgRequestTag,
    SvgSource, SvgStatus,
};
use abi::types::HandleId;
use abi::types::{WatchMode, WatchSpec};
use abi::watch;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashMap;
use log::debug;
use stem::info;
use stem::syscall;
use stem::thing::sys::{
    bytespace_create, bytespace_info, bytespace_read, bytespace_write, create_node, find, prop_get,
    prop_set,
};
use stem::thing::ThingId;

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
    info!("BLOSSOM: Init UI pipeline...");
    let mut ui = UiPipeline::new();
    info!("BLOSSOM: UI pipeline ready");

    // Create IPC ports for blossom service
    let (blossom_req, blossom_resp) = match (syscall::port_create(8192), syscall::port_create(8192))
    {
        (Ok(req), Ok(resp)) => {
            // Create service node and advertise port handles
            if let Ok(svc_node) = create_node("svc.Blossom") {
                let _ = prop_set(svc_node, "blossom.req", req.0 as u64);
                let _ = prop_set(svc_node, "blossom.resp", resp.1 as u64);
                info!(
                    "BLOSSOM: Service node created, req={}, resp={}",
                    req.0, resp.1
                );
            }
            (req.1, resp.0) // blossom uses (read, write) sides
        }
        _ => {
            info!("BLOSSOM: Failed to create IPC ports, exiting");
            loop {
                syscall::sleep_ms(1000);
            }
        }
    };

    info!("BLOSSOM: Service ready");

    let mut ipc_buf = [0u8; 16384]; // Reduced from 64KB to fit in initial stack commit
    let mut resp_buf = [0u8; 256];

    loop {
        // Process IPC requests
        match syscall::port_recv(blossom_req, &mut ipc_buf) {
            Ok(len) if len > 0 => {
                if let Some(resp_len) =
                    handle_ipc_request(&ipc_buf[..len], &mut resp_buf, &mut state)
                {
                    let _ = syscall::port_send(blossom_resp, &resp_buf[..resp_len]);
                }
            }
            _ => {}
        }

        ui.poll();
        syscall::sleep_ms(5);
    }
}

#[derive(Clone, Copy)]
struct WindowState {
    last_gen: u64,
    last_w: i32,
    last_h: i32,
    last_bg: u32,
    last_title_bs: u64,
}

struct UiWatcher {
    id: usize,
    key: u32,
    seq: u64,
    buf: [u8; 4096],
}

struct UiPipeline {
    watchers: Vec<UiWatcher>,
    windows: BTreeMap<ThingId, WindowState>,
    ui_symbols: graph_ui::UiSymbols,
}

impl UiPipeline {
    fn new() -> Self {
        let mut watchers = Vec::new();
        let keys_to_watch = [
            keys::UI_SCENE_GEN,
            keys::UI_WIDTH,
            keys::UI_HEIGHT,
            keys::UI_BG_COLOR,
            keys::UI_TITLE,
        ];
        for key in keys_to_watch {
            if let Ok(pred) = stem::thing::sys::intern(key) {
                let filter = RootWatchFilter::predicate(pred);
                let spec = WatchSpec {
                    mode: WatchMode::StreamOnly as u32,
                    filter_ptr: &filter as *const _ as u64,
                    filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
                    ..Default::default()
                };
                if let Ok(id) = syscall::root_watch_open(&spec) {
                    watchers.push(UiWatcher {
                        id,
                        key: pred,
                        seq: 0,
                        buf: [0u8; 4096],
                    });
                }
            }
        }

        Self {
            watchers,
            windows: BTreeMap::new(),
            ui_symbols: graph_ui::UiSymbols::intern_sys(),
        }
    }

    fn poll(&mut self) {
        self.refresh_windows();
        let mut dirty: BTreeMap<ThingId, bool> = BTreeMap::new();

        for watcher in &mut self.watchers {
            while let Ok(len) =
                syscall::root_watch_next(watcher.id, &mut watcher.seq, &mut watcher.buf)
            {
                let mut cursor = 0usize;
                while cursor < len {
                    if let Ok((header, value)) = watch::decode_event(&watcher.buf[cursor..len]) {
                        let event_len = watch::WATCH_EVENT_HEADER_LEN + value.len();
                        cursor = cursor.saturating_add(event_len);
                        let pred = header.predicate.to_u32_lossy();
                        if pred == watcher.key {
                            dirty.insert(header.subject, true);
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        let mut window_ids: Vec<ThingId> = self.windows.keys().copied().collect();
        for window_id in window_ids.iter_mut() {
            let gen = prop_get(*window_id, keys::UI_SCENE_GEN).unwrap_or(0);
            let w = prop_get(*window_id, keys::UI_WIDTH).unwrap_or(0) as i32;
            let h = prop_get(*window_id, keys::UI_HEIGHT).unwrap_or(0) as i32;
            let entry = self.windows.get(window_id).cloned().unwrap_or(WindowState {
                last_gen: 0,
                last_w: 0,
                last_h: 0,
                last_bg: 0,
                last_title_bs: 0,
            });
            if gen != entry.last_gen || w != entry.last_w || h != entry.last_h {
                dirty.insert(*window_id, true);
            }
            let bg = prop_get(*window_id, keys::UI_BG_COLOR).unwrap_or(0) as u32;
            if bg != entry.last_bg {
                dirty.insert(*window_id, true);
            }
            let title_bs = prop_get(*window_id, keys::UI_TITLE).unwrap_or(0);
            if title_bs != entry.last_title_bs {
                dirty.insert(*window_id, true);
            }
        }

        for (window_id, _) in dirty {
            if let Err(e) = self.process_window(window_id) {
                info!(
                    "BLOSSOM: window {} update failed: {:?}",
                    window_id.to_u64_lossy(),
                    e
                );
            }
        }
    }

    fn refresh_windows(&mut self) {
        let mut windows = [ThingId::default(); 128];
        let count = find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);
        for win in windows.iter().take(count) {
            self.windows.entry(*win).or_insert(WindowState {
                last_gen: 0,
                last_w: 0,
                last_h: 0,
                last_bg: 0,
                last_title_bs: 0,
            });
        }
    }

    fn process_window(&mut self, window_id: ThingId) -> Result<(), abi::errors::Errno> {
        let mut sys_graph = graph_ui::SysGraph;
        if let Some(root_id) = graph_ui::find_root_ui(&sys_graph, &self.ui_symbols, window_id) {
            return self.process_graph_ui(window_id, root_id, &mut sys_graph);
        }
        let bs_id = prop_get(window_id, keys::UI_SCENE_BYTESPACE).unwrap_or(0);
        if bs_id == 0 {
            return Ok(());
        }
        let window_bg = prop_get(window_id, keys::UI_BG_COLOR).unwrap_or(0) as u32;
        let title_override = read_string_prop(window_id, keys::UI_TITLE);
        let bytes = read_bytespace(ThingId::from_u64(bs_id))?;
        let scene = match scene::SceneGraph::from_bytes(&bytes) {
            Ok(scene) => scene,
            Err(_) => return Ok(()),
        };

        let mut w = prop_get(window_id, keys::UI_WIDTH).unwrap_or(0) as i32;
        let mut h = prop_get(window_id, keys::UI_HEIGHT).unwrap_or(0) as i32;
        if w <= 0 || h <= 0 {
            if let Some(meta) = scene.nodes.get(scene.root).and_then(|n| n.window_meta) {
                if w <= 0 && meta.init_w > 0 {
                    w = meta.init_w;
                }
                if h <= 0 && meta.init_h > 0 {
                    h = meta.init_h;
                }
                if w <= 0 && meta.min_w > 0 {
                    w = meta.min_w;
                }
                if h <= 0 && meta.min_h > 0 {
                    h = meta.min_h;
                }
            }
        }
        if w <= 0 || h <= 0 {
            return Ok(());
        }

        let root_rect = layout::LayoutRect { x: 0, y: 0, w, h };
        let rects = layout::layout_scene(&scene, root_rect);
        let paint_bytes =
            emit_paint::emit_paint(&scene, &rects, window_bg, title_override.as_deref());
        let paint_bs = bytespace_create(paint_bytes.len(), 0, 0)?;
        let _ = bytespace_write(paint_bs, 0, &paint_bytes);
        let _ = prop_set(window_id, keys::UI_PAINT_BYTESPACE, paint_bs.to_u64_lossy());
        let current = prop_get(window_id, keys::UI_PAINT_GEN).unwrap_or(0);
        let _ = prop_set(window_id, keys::UI_PAINT_GEN, current.saturating_add(1));

        self.windows.insert(
            window_id,
            WindowState {
                last_gen: prop_get(window_id, keys::UI_SCENE_GEN).unwrap_or(0),
                last_w: w,
                last_h: h,
                last_bg: window_bg,
                last_title_bs: prop_get(window_id, keys::UI_TITLE).unwrap_or(0),
            },
        );
        Ok(())
    }

    fn process_graph_ui(
        &mut self,
        window_id: ThingId,
        root_id: ThingId,
        graph: &mut graph_ui::SysGraph,
    ) -> Result<(), abi::errors::Errno> {
        let window_bg = prop_get(window_id, keys::UI_BG_COLOR).unwrap_or(0) as u32;
        let mut w = prop_get(window_id, keys::UI_WIDTH).unwrap_or(0) as i32;
        let mut h = prop_get(window_id, keys::UI_HEIGHT).unwrap_or(0) as i32;
        if w <= 0 || h <= 0 {
            return Ok(());
        }

        let tree = match graph_ui::build_tree(graph, &self.ui_symbols, root_id) {
            Some(tree) => tree,
            None => return Ok(()),
        };
        let root_rect = layout::LayoutRect { x: 0, y: 0, w, h };
        let rects = graph_ui::layout_tree(&tree, root_rect);
        graph_ui::write_bounds(graph, &tree, &rects);
        let paint_bytes = graph_ui::emit_paint(&tree, &rects, window_bg);
        let paint_bs = bytespace_create(paint_bytes.len(), 0, 0)?;
        let _ = bytespace_write(paint_bs, 0, &paint_bytes);
        let _ = prop_set(window_id, keys::UI_PAINT_BYTESPACE, paint_bs.to_u64_lossy());
        let current = prop_get(window_id, keys::UI_PAINT_GEN).unwrap_or(0);
        let _ = prop_set(window_id, keys::UI_PAINT_GEN, current.saturating_add(1));

        self.windows.insert(
            window_id,
            WindowState {
                last_gen: prop_get(window_id, keys::UI_SCENE_GEN).unwrap_or(0),
                last_w: w,
                last_h: h,
                last_bg: window_bg,
                last_title_bs: prop_get(window_id, keys::UI_TITLE).unwrap_or(0),
            },
        );
        Ok(())
    }
}

fn read_string_prop(node: ThingId, key: &str) -> Option<String> {
    let bs = prop_get(node, key).ok()?;
    if bs == 0 {
        return None;
    }
    let bytes = read_bytespace(ThingId::from_u64(bs)).ok()?;
    core::str::from_utf8(&bytes)
        .ok()
        .map(|s| s.trim_end_matches('\0').to_string())
}

fn read_bytespace(bs_id: ThingId) -> Result<Vec<u8>, abi::errors::Errno> {
    let size = bytespace_info(bs_id)?;
    if size == 0 {
        return Ok(Vec::new());
    }
    let mut out = Vec::with_capacity(size);
    out.resize(size, 0);
    let mut offset = 0usize;
    while offset < size {
        let end = core::cmp::min(offset + 4096, size);
        let read = bytespace_read(bs_id, offset, &mut out[offset..end])?;
        if read == 0 {
            break;
        }
        offset = offset.saturating_add(read);
    }
    Ok(out)
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

fn handle_rasterize(
    req: RasterizeSvgRequest,
    resp: &mut [u8],
    state: &mut Blossom,
) -> Option<usize> {
    // Get SVG bytes from source
    let svg_bytes: Vec<u8> = match req.source {
        SvgSource::Bytespace(bs_id) => {
            let size = bytespace_info(bs_id).ok()?;
            if size == 0 {
                return encode_error(SvgStatus::ErrInvalidSvg, resp);
            }
            let mut bytes = vec![0u8; size];
            let mut offset = 0usize;
            while offset < size {
                let end = core::cmp::min(offset + 4096, size);
                let read = bytespace_read(bs_id, offset, &mut bytes[offset..end]).ok()?;
                if read == 0 {
                    break;
                }
                offset = offset.saturating_add(read);
            }
            bytes
        }
        SvgSource::InlineBytes(bytes) => bytes,
    };

    if svg_bytes.is_empty() {
        return encode_error(SvgStatus::ErrInvalidSvg, resp);
    }

    // Compute hashes
    let svg_hash = Blossom::compute_svg_hash(&svg_bytes);
    let variant_hash =
        Blossom::compute_variant_hash(svg_hash, req.width, req.height, req.pixel_format, req.flags);

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
    debug!(
        "BLOSSOM: Cache miss for variant {:016x}, rasterizing {}x{}",
        variant_hash, req.width, req.height
    );
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
        pixels[offset..offset + 4].copy_from_slice(&pixel.to_le_bytes());
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
        let _ = prop_set(
            variant_node,
            abi::schema::keys::SVG_VARIANT_HASH,
            variant_hash,
        );
        let _ = prop_set(
            variant_node,
            abi::schema::keys::SVG_RASTER_BYTESPACE,
            bs_id.to_u64_lossy(),
        );
        let _ = prop_set(
            variant_node,
            abi::schema::keys::SVG_RASTER_WIDTH,
            req.width as u64,
        );
        let _ = prop_set(
            variant_node,
            abi::schema::keys::SVG_RASTER_HEIGHT,
            req.height as u64,
        );
    }

    info!(
        "BLOSSOM: Rasterized variant {:016x} ({}x{})",
        variant_hash, req.width, req.height
    );

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
