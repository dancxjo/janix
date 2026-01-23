#![no_std]
#![no_main]

extern crate alloc;

mod compose;
mod model;
mod raster_svg;
mod raster_text;
mod sched;
mod surface;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use abi::schema::{keys, kinds, rels, ui_snapshot};
use abi::types::{WatchMode, WatchSpec};
use stem::info;
use stem::thing::sys::{
    bytespace_create, bytespace_map, bytespace_unmap, find, prop_set,
};
use stem::thing::ThingId;

use crate::compose::blit;
use crate::model::{KindIds, TextRunModel, TileModel, ViewportModel, WindowModel};
use crate::raster_svg::raster_placeholder;
use crate::raster_text::{draw_text, TextStyle};
use crate::sched::{budget_exhausted, should_tick};
use crate::surface::{MappedSurface, SurfaceSpec};

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
    stem::logging::init();
    info!("blossom: starting painter");

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

    loop {
        let tick_start = stem::monotonic_ns();
        drain_watches(&watch_handles, &mut watch_seq, &mut watch_bufs, &mut dirty_all);

        let mut windows = [ThingId::default(); 64];
        let count = find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);
        let models = model::read_windows(&kind_ids, &windows[..count]);

        for window in models {
            paint_window(&mut render_state, &window, dirty_all, tick_start);
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
            render_state,
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
        if paint_tile(render_state, tile, dirty_all) {
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
            render_state,
            viewport,
            viewport_state,
            &viewport.tiles,
            &viewport.text_runs,
        );
    }

    Some(viewport.id)
}

fn paint_tile(render_state: &mut RenderState, tile: &TileModel, dirty_all: bool) -> bool {
    let tile_state = render_state
        .tiles
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
        raster_placeholder(&mut surface, tile.asset.to_u64_lossy());
        unmap_surface(bs_id, surface);
        present_snapshot(tile.id, tile_state, tile.width, tile.height);
        return true;
    }
    false
}

fn render_viewport_surface(
    render_state: &RenderState,
    viewport: &ViewportModel,
    viewport_state: &mut NodeState,
    tiles: &[TileModel],
    text_runs: &[TextRunModel],
) {
    let bs_id = viewport_state.buffers.back_id();
    if let Some(mut surface) = map_surface(bs_id, &viewport_state.buffers) {
        surface.clear(0xFF202020);
        for tile in tiles {
            if let Some(tile_state) = render_state.tiles.get(&tile.id) {
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
        render_text_runs(&mut surface, text_runs, viewport.width, viewport.height);
        unmap_surface(bs_id, surface);
        present_snapshot(viewport.id, viewport_state, viewport.width, viewport.height);
    }
}

fn render_window_surface(
    render_state: &RenderState,
    window: &WindowModel,
    window_state: &mut NodeState,
    viewport_id: Option<ThingId>,
    text_runs: &[TextRunModel],
) -> bool {
    let bs_id = window_state.buffers.back_id();
    if let Some(mut surface) = map_surface(bs_id, &window_state.buffers) {
        surface.clear(window.bg_color);
        if let Some(viewport_id) = viewport_id {
            if let Some(viewport_state) = render_state.viewports.get(&viewport_id) {
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
        render_text_runs(&mut surface, text_runs, window.width, window.height);
        unmap_surface(bs_id, surface);
        present_snapshot(window.id, window_state, window.width, window.height);
        return true;
    }
    false
}

fn render_text_runs(surface: &mut MappedSurface, runs: &[TextRunModel], width: u32, height: u32) {
    for run in runs {
        let style = TextStyle {
            color: run.color,
            size_px: run.size_px,
        };
        let scale = (style.size_px / 8).max(1) as i32;
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
        draw_text(surface, &run.text, x, y, &style);
    }
}

fn map_surface(bs_id: ThingId, buffers: &SurfaceBuffers) -> Option<MappedSurface> {
    let ptr = bytespace_map(bs_id).ok()?;
    let len = (buffers.stride * buffers.height) as usize;
    let spec = SurfaceSpec {
        width: buffers.width,
        height: buffers.height,
        stride_bytes: buffers.stride,
    };
    Some(unsafe { MappedSurface::from_parts(ptr, len, spec) })
}

fn unmap_surface(bs_id: ThingId, surface: MappedSurface) {
    let _ = bytespace_unmap(bs_id, surface.ptr());
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
