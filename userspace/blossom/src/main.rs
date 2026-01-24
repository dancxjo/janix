#![no_std]
#![no_main]

extern crate alloc;

mod compose;
mod model;
mod painter_resources;
mod sched;
mod surface;
#[macro_use]
mod perf;
mod logging;
mod log_ratelimit;

// Copied modules
mod geometry;
mod isa;
mod drawlist;
mod lowered;
mod raster;
mod font_graph;
mod asset;
mod bmp;
mod reclaimer;
mod frame;
mod damage;
mod svg;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use abi::schema::{keys, kinds, rels, ui_snapshot};
use abi::types::{WatchMode, WatchSpec};
use stem::info;
use stem::thing::sys::{
    bytespace_create, bytespace_map, bytespace_unmap, find, prop_set, prop_get
};
use stem::thing::ThingId;
use abi::ids::HandleId;

use crate::compose::blit;
use crate::model::{KindIds, TextRunModel, TileModel, ViewportModel, WindowModel};
use crate::sched::{budget_exhausted, should_tick};
use crate::surface::Surface;
use crate::painter_resources::ASSETS;
use crate::drawlist::DrawList;
use crate::geometry::{Color, Rect};
use crate::damage::{Damage, Rect as DamageRect};
use crate::raster::execute_with_damage;

const TILE_RASTER_BUDGET: usize = 8;
const BLOSSOM_TICK_BUDGET_NS: u64 = 8_000_000;
const VIEWPORT_HZ: u64 = 60;
const WINDOW_HZ: u64 = 60;

struct SurfaceBuffers {
    front: ThingId,
    back: ThingId,
    width: u32,
    height: u32,
    stride: u32,
    front_is_a: bool,
}

impl Default for SurfaceBuffers {
    fn default() -> Self {
        Self {
            front: ThingId::from_u64(0),
            back: ThingId::from_u64(0),
            width: 0,
            height: 0,
            stride: 0,
            front_is_a: true,
        }
    }
}

impl SurfaceBuffers {
    fn ensure(&mut self, width: u32, height: u32) {
        if self.width == width && self.height == height && self.front.to_u64_lossy() != 0 {
            return;
        }
        let stride = width.saturating_mul(4);
        let size = stride.saturating_mul(height) as usize;
        let a = bytespace_create(size, 0, 0).expect("create bytespace");
        let b = bytespace_create(size, 0, 0).expect("create bytespace");
        self.front = a;
        self.back = b;
        self.front_is_a = true;
        self.width = width;
        self.height = height;
        self.stride = stride;
    }

    fn back_id(&self) -> ThingId {
        if self.front_is_a {
            self.back
        } else {
            self.front
        }
    }

    fn swap(&mut self) -> ThingId {
        self.front_is_a = !self.front_is_a;
        if self.front_is_a {
            self.front
        } else {
            self.back
        }
    }
}

struct NodeState {
    buffers: SurfaceBuffers,
    epoch: u64,
    last_present_ns: u64,
    last_size: (u32, u32),
    last_scroll: (i32, i32),
    last_asset: ThingId,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            buffers: SurfaceBuffers::default(),
            epoch: 0,
            last_present_ns: 0,
            last_size: (0, 0),
            last_scroll: (0, 0),
            last_asset: ThingId::from_u64(0),
        }
    }
}

struct RenderState {
    windows: BTreeMap<ThingId, NodeState>,
    viewports: BTreeMap<ThingId, NodeState>,
    tiles: BTreeMap<ThingId, NodeState>,
}

impl RenderState {
    fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
            viewports: BTreeMap::new(),
            tiles: BTreeMap::new(),
        }
    }
}

#[stem::main]
fn main() -> ! {
    crate::logging::init();
    info!("blossom: starting painter");

    // Initialize asset loaders
    use stem::stack::{Stack, StackSpec};
    let s_spec = StackSpec {
        reserve_bytes: 256 * 1024,
        initial_commit_bytes: 64 * 1024,
        ..StackSpec::default()
    };
    stem::thread::spawn_on(
        Stack::alloc_growing_stack(s_spec).unwrap(),
        painter_resources::wallpaper_loader_entry,
    ).ok();
    stem::thread::spawn_on(
        Stack::alloc_growing_stack(s_spec).unwrap(),
        painter_resources::cursor_loader_entry,
    ).ok();
    stem::thread::spawn_on(
        Stack::alloc_growing_stack(s_spec).unwrap(),
        painter_resources::font_loader_entry,
    ).ok();

    let kind_ids = KindIds::load();
    let mut render_state = RenderState::new();

    let mut watch_handles = Vec::new();
    let mut watch_bufs = Vec::new();
    let mut watch_seq = Vec::new();
    let predicates = blossom_predicates();
    for predicate in predicates {
        let filter = abi::root::RootWatchFilter::predicate(predicate);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            start_seq: 0,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<abi::root::RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(id) = stem::syscall::root_watch_open(&spec) {
            watch_handles.push(id);
            watch_bufs.push([0u8; 4096]);
            watch_seq.push(0);
        }
    }

    let mut dirty_all = true;
    let mut cursor_asset_gen = crate::frame::AssetGeneration::ZERO;

    loop {
        let tick_start = stem::monotonic_ns();
        ASSETS.publish_pending();

        drain_watches(&watch_handles, &mut watch_seq, &mut watch_bufs, &mut dirty_all);

        let mut windows = [ThingId::default(); 64];
        let count = find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);
        let models = model::read_windows(&kind_ids, &windows[..count]);

        for window in models {
            paint_window(&mut render_state, &window, dirty_all, tick_start);
        }

        // Paint cursor if changed
        if let Some(cursor) = ASSETS.get_cursor() {
            if cursor.generation() > cursor_asset_gen {
                 paint_cursor_snapshot(&cursor);
                 cursor_asset_gen = cursor.generation();
            }
        }

        dirty_all = false;

        let elapsed = stem::monotonic_ns().saturating_sub(tick_start);
        if elapsed < 4_000_000 {
            stem::sleep_ms(4);
        }
    }
}

fn blossom_predicates() -> Vec<u32> {
    let mut preds = Vec::new();
    for key in [
        keys::UI_X,
        keys::UI_Y,
        keys::UI_WIDTH,
        keys::UI_HEIGHT,
        keys::UI_INSET_RIGHT,
        keys::UI_INSET_BOTTOM,
        keys::UI_BG_COLOR,
        keys::UI_FG_COLOR,
        keys::UI_FONT_SIZE,
        keys::UI_TEXT,
        keys::UI_CENTER_X,
        keys::UI_CENTER_Y,
        keys::UI_SCROLL_X,
        keys::UI_SCROLL_Y,
        keys::UI_CLIP,
        keys::UI_TILE_ASSET,
        keys::UI_TILE_STATE,
        rels::HAS_CHILD,
        // Added Paint Epoch
        keys::UI_PAINT_EPOCH,
    ] {
        if let Ok(id) = stem::thing::sys::intern(key) {
            preds.push(id);
        }
    }
    preds
}

fn drain_watches(
    handles: &[usize],
    seq: &mut [u64],
    bufs: &mut [[u8; 4096]],
    dirty_all: &mut bool,
) {
    for (idx, handle) in handles.iter().enumerate() {
        loop {
            match stem::syscall::root_watch_next(*handle, &mut seq[idx], &mut bufs[idx]) {
                Ok(len) if len > 0 => {
                    *dirty_all = true;
                    let _ = len;
                }
                Ok(_) | Err(abi::errors::Errno::EAGAIN) => break,
                Err(abi::errors::Errno::EOVERFLOW) => {
                    *dirty_all = true;
                    break;
                }
                Err(_) => break,
            }
        }
    }
}

fn paint_window(
    render_state: &mut RenderState,
    window: &WindowModel,
    dirty_all: bool,
    tick_start: u64,
) {

    let viewport_model = window.viewport.clone();
    let viewport_dirty = dirty_all || viewport_model.is_some();
    let viewport_snapshot = viewport_model
        .as_ref()
        .and_then(|viewport| paint_viewport(render_state, viewport, viewport_dirty, tick_start));

    let window_state = render_state
        .windows
        .entry(window.id)
        .or_insert_with(NodeState::default);

    let size_changed = window_state.last_size != (window.width, window.height);
    if size_changed {
        window_state.last_size = (window.width, window.height);
        window_state.buffers.ensure(window.width, window.height);
    }

    let now = stem::monotonic_ns();
    if (dirty_all || viewport_snapshot.is_some() || size_changed)
        && should_tick(now, window_state.last_present_ns, WINDOW_HZ)
    {
        window_state.last_present_ns = now;
        if render_window_surface(
            &render_state.viewports,
            window,
            window_state,
            viewport_snapshot,
            &window.text_runs,
        ) {
        }
    }
}

fn paint_viewport(
    render_state: &mut RenderState,
    viewport: &ViewportModel,
    dirty_all: bool,
    tick_start: u64,
) -> Option<ThingId> {
    let viewport_state = render_state
        .viewports
        .entry(viewport.id)
        .or_insert_with(NodeState::default);

    let size_changed = viewport_state.last_size != (viewport.width, viewport.height);
    let scroll_changed = viewport_state.last_scroll != (viewport.scroll_x, viewport.scroll_y);
    if size_changed {
        viewport_state.last_size = (viewport.width, viewport.height);
        viewport_state.buffers.ensure(viewport.width, viewport.height);
    }
    if scroll_changed {
        viewport_state.last_scroll = (viewport.scroll_x, viewport.scroll_y);
    }

    let mut tiles_presented = false;
    let mut tile_budget = TILE_RASTER_BUDGET;

    for tile in &viewport.tiles {
        if tile_budget == 0 {
            break;
        }
        if budget_exhausted(tick_start, stem::monotonic_ns(), BLOSSOM_TICK_BUDGET_NS) {
            break;
        }
        if paint_tile(&mut render_state.tiles, tile, dirty_all) {
            tiles_presented = true;
            tile_budget -= 1;
        }
    }

    let now = stem::monotonic_ns();
    if (dirty_all || tiles_presented || size_changed || scroll_changed)
        && should_tick(now, viewport_state.last_present_ns, VIEWPORT_HZ)
    {
        viewport_state.last_present_ns = now;
        render_viewport_surface(
            &render_state.tiles,
            viewport,
            viewport_state,
            &viewport.tiles,
            &viewport.text_runs,
        );
    }

    Some(viewport.id)
}

fn paint_tile(tiles: &mut BTreeMap<ThingId, NodeState>, tile: &TileModel, dirty_all: bool) -> bool {
    let tile_state = tiles
        .entry(tile.id)
        .or_insert_with(NodeState::default);

    let size_changed = tile_state.last_size != (tile.width, tile.height);
    let asset_changed = tile_state.last_asset != tile.asset;

    if size_changed {
        tile_state.last_size = (tile.width, tile.height);
        tile_state.buffers.ensure(tile.width, tile.height);
    }
    if asset_changed {
        tile_state.last_asset = tile.asset;
    }

    if !dirty_all && !size_changed && !asset_changed {
        return false;
    }

    let bs_id = tile_state.buffers.back_id();
    if let Some(mut surface) = map_surface(bs_id, &tile_state.buffers) {
        render_svg_tile(&mut surface, tile.asset);
        unmap_surface(bs_id, surface);
        present_snapshot(tile.id, tile_state, tile.width, tile.height);
        return true;
    }
    false
}

fn render_svg_tile(surface: &mut Surface, asset_id: ThingId) {
    if asset_id.to_u64_lossy() == 0 {
         // Clear transparent
         surface.clear(0);
         return;
    }
    // Map the SVG bytes
    if let Ok(ptr) = bytespace_map(asset_id) {
         if let Ok(size) = stem::thing::sys::bytespace_info(asset_id) {
              let slice = unsafe { core::slice::from_raw_parts(ptr, size) };
              if let Ok(svg_str) = core::str::from_utf8(slice) {
                   // Rasterize SVG
                   let width = surface.width() as i32;
                   let height = surface.height() as i32;
                   // Assuming 1:1 scale for tile for now, or use viewbox?
                   // svg::render_to_buffer logic:
                   let pixels = crate::svg::render_to_buffer(svg_str, width, height, 1.0);

                   // Blit pixels
                   for y in 0..height {
                        for x in 0..width {
                             let idx = (y * width + x) as usize;
                             if idx < pixels.len() {
                                  surface.put_px(x, y, pixels[idx]);
                             }
                        }
                   }
              }
         }
         let _ = bytespace_unmap(asset_id, ptr);
    }
}

fn render_viewport_surface(
    tile_states: &BTreeMap<ThingId, NodeState>,
    viewport: &ViewportModel,
    viewport_state: &mut NodeState,
    tiles: &[TileModel],
    text_runs: &[TextRunModel],
) {
    let bs_id = viewport_state.buffers.back_id();
    if let Some(mut surface) = map_surface(bs_id, &viewport_state.buffers) {
        // Clear background
        surface.clear(0xFF202020);

        // Composite tiles
        for tile in tiles {
            if let Some(tile_state) = tile_states.get(&tile.id) {
                if let Some(tile_surface) = map_surface(tile_state.buffers.front, &tile_state.buffers)
                {
                    let dx = tile.x - viewport.scroll_x;
                    let dy = tile.y - viewport.scroll_y;
                    blit(
                        &tile_surface,
                        &mut surface,
                        0,
                        0,
                        tile.width as i32,
                        tile.height as i32,
                        dx,
                        dy,
                    );
                    unmap_surface(tile_state.buffers.front, tile_surface);
                }
            }
        }

        // Render text overlays using DrawList and Raster
        if !text_runs.is_empty() {
             let mut list = DrawList::new();
             render_text_runs_to_list(&mut list, text_runs, viewport.width, viewport.height);

             let damage = Damage::full(DamageRect::new(0, 0, surface.width() as i32, surface.height() as i32));
             execute_with_damage(&mut surface, &list, &damage, false);
        }

        unmap_surface(bs_id, surface);
        present_snapshot(viewport.id, viewport_state, viewport.width, viewport.height);
    }
}

fn render_window_surface(
    viewport_states: &BTreeMap<ThingId, NodeState>,
    window: &WindowModel,
    window_state: &mut NodeState,
    viewport_id: Option<ThingId>,
    text_runs: &[TextRunModel],
) -> bool {
    let bs_id = window_state.buffers.back_id();
    if let Some(mut surface) = map_surface(bs_id, &window_state.buffers) {
        // Clear background
        surface.clear(window.bg_color);

        // Composite viewport
        if let Some(viewport_id) = viewport_id {
            if let Some(viewport_state) = viewport_states.get(&viewport_id) {
                if let Some(viewport_surface) =
                    map_surface(viewport_state.buffers.front, &viewport_state.buffers)
                {
                    blit(
                        &viewport_surface,
                        &mut surface,
                        0,
                        0,
                        viewport_state.buffers.width as i32,
                        viewport_state.buffers.height as i32,
                        0,
                        0,
                    );
                    unmap_surface(viewport_state.buffers.front, viewport_surface);
                }
            }
        }

        // Render Text
         if !text_runs.is_empty() {
             let mut list = DrawList::new();
             render_text_runs_to_list(&mut list, text_runs, window.width, window.height);

             let damage = Damage::full(DamageRect::new(0, 0, surface.width() as i32, surface.height() as i32));
             execute_with_damage(&mut surface, &list, &damage, false);
        }

        unmap_surface(bs_id, surface);
        present_snapshot(window.id, window_state, window.width, window.height);
        return true;
    }
    false
}

fn render_text_runs_to_list(list: &mut DrawList, runs: &[TextRunModel], width: u32, height: u32) {
    for run in runs {
        let size_px = run.size_px as f32;
        let scale = (run.size_px / 8).max(1) as i32;
        let text_width = (run.text.len() as i32) * (6 * scale);
        let text_height = 7 * scale;
        let mut x = run.x;
        let mut y = run.y;
        if run.center_x {
            x = (width as i32 - text_width) / 2;
        }
        if run.center_y {
            y = (height as i32 - text_height) / 2;
        }
        list.text(&run.text, x, y, size_px, Color::from_u32(run.color));
    }
}

fn map_surface(bs_id: ThingId, buffers: &SurfaceBuffers) -> Option<Surface> {
    let ptr = bytespace_map(bs_id).ok()?;
    let len = (buffers.stride * buffers.height) as usize;
    Some(unsafe {
        Surface::new(ptr as *mut u8, len, buffers.width, buffers.height, buffers.stride)
    })
}

fn unmap_surface(bs_id: ThingId, surface: Surface) {
    let _ = bytespace_unmap(bs_id, surface.ptr);
}

fn present_snapshot(node_id: ThingId, state: &mut NodeState, width: u32, height: u32) {
    let stride = width.saturating_mul(4);
    let bs_id = state.buffers.swap();
    prop_set(node_id, keys::UI_SNAPSHOT_BYTESPACE, bs_id.to_u64_lossy()).ok();
    prop_set(node_id, keys::UI_SNAPSHOT_WIDTH, width as u64).ok();
    prop_set(node_id, keys::UI_SNAPSHOT_HEIGHT, height as u64).ok();
    prop_set(node_id, keys::UI_SNAPSHOT_STRIDE, stride as u64).ok();
    prop_set(node_id, keys::UI_SNAPSHOT_FORMAT, ui_snapshot::PIXEL_FORMAT_RGBA8888).ok();
    state.epoch = state.epoch.saturating_add(1);
    prop_set(node_id, keys::UI_PRESENT_EPOCH, state.epoch).ok();
}

#[allow(static_mut_refs)]
fn paint_cursor_snapshot(asset: &crate::asset::CursorAsset) {
    // 1. Get/Create bytespace for cursor
    // We need a persistent handle for the cursor bytespace or create a new one every time?
    // Creating new one every time is safer for atomic updates (swap).
    // Let's store it in static or just create/map/unmap and rely on single buffer for now or double buffer.
    // Bloom uses "compositor only", so it reads a bytespace.
    // If we update it, we should probably double buffer.
    // For simplicity, let's just create one and reuse it, hoping Bloom reads it atomically enough (it copies).
    // Or better: Use UI_ROOT properties to store the "Current Cursor Bytespace ID".
    // We can swap IDs.

    // We need state for cursor buffers.
    // Let's use a static for now since main loop owns it.
    static mut CURSOR_BUFFERS: Option<SurfaceBuffers> = None;

    // We need UI_ROOT to set properties.
    let mut roots = [ThingId::default(); 1];
    let root_id = match stem::thing::sys::find(abi::schema::kinds::UI_ROOT, &mut roots) {
        Ok(count) if count > 0 => roots[0],
        _ => return, // No root
    };

    let frame = match asset {
        crate::asset::CursorAsset::Static(f) => f,
        crate::asset::CursorAsset::Animated { frames } => &frames[0],
    };

    unsafe {
        if CURSOR_BUFFERS.is_none() {
            CURSOR_BUFFERS = Some(SurfaceBuffers::default());
        }
        if let Some(buffers) = CURSOR_BUFFERS.as_mut() {
            buffers.ensure(frame.image.width, frame.image.height);
            let bs_id = buffers.back_id();
            if let Some(mut surface) = map_surface(bs_id, buffers) {
                surface.clear(0); // Clear transparent

                // Copy pixels
                let pixels = &frame.image.pixels;
                for y in 0..frame.image.height as i32 {
                    for x in 0..frame.image.width as i32 {
                        let idx = (y * frame.image.width as i32 + x) as usize;
                        if idx < pixels.len() {
                            surface.put_px(x, y, pixels[idx]);
                        }
                    }
                }

                unmap_surface(bs_id, surface);

                let present_id = buffers.swap();
                prop_set(root_id, keys::UI_CURSOR_SNAPSHOT_BYTESPACE, present_id.to_u64_lossy()).ok();
                prop_set(root_id, keys::UI_CURSOR_SNAPSHOT_WIDTH, frame.image.width as u64).ok();
                prop_set(root_id, keys::UI_CURSOR_SNAPSHOT_HEIGHT, frame.image.height as u64).ok();
                prop_set(root_id, keys::UI_CURSOR_SNAPSHOT_STRIDE, (frame.image.width * 4) as u64).ok();
                prop_set(root_id, keys::UI_CURSOR_SNAPSHOT_FORMAT, ui_snapshot::PIXEL_FORMAT_RGBA8888).ok();
            }
        }
    }
}
