#![feature(restricted_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
extern crate stem; // Force linkage

mod asset;
mod blit;
mod blossom_client;
mod bmp;
mod bristle;
mod compositor;
mod cursor;
mod cursor_rasterizer;
mod damage;
mod damage_accumulator;
mod drawlist;
mod font_client;
mod font_graph;
mod frame;
mod frame_loop;
pub mod geometry;
mod gpu_compositor;
mod isa;
mod log_ratelimit;
mod logging;
mod lowered;
mod paint_vm;
pub mod painter_resources;
pub mod perf;
mod present;
mod raster;
mod reclaimer;
mod render_graph;
mod render_state;
pub mod snapshot;
mod state;
mod surface;
mod scene_graph;
mod window;
mod svg;
mod tessellate;
mod text_cache;
mod text_render;
mod ui;
mod ui_events;
mod vir;
mod window_manager;
mod wayland;

pub use painter_resources::ASSETS;

use abi::hid::Key;
use abi::ids::HandleId;
use stem::thing::sys::{find, prop_get};
use stem::thing::ThingId;

use abi::display_driver_protocol::BindPayload;
use abi::schema::input::{
    FILTER_BUTTON, FILTER_POINTER, SUBSCRIBER_FILTER, SUBSCRIBER_PORT, SVC_INPUT_SUBSCRIBER,
};
use abi::schema::{hid, keys, kinds};
use stem::syscall::{port_create, topic_subscribe, PortHandle};

use crate::asset::AssetBank;
use crate::bristle::{poll_bristle, MouseAccelConfig, MouseAccelState};
use crate::compositor::CompositorTarget;
use crate::cursor::CursorState;
use crate::cursor_rasterizer::CursorRasterizer;
use crate::frame::FrameBuilder;
use crate::frame_loop::FrameLoop;
use crate::paint_vm::{PaintPipeline, WindowHit};
use crate::present::{evaluate_present_strategy, DriverPresenter, Presenter, PresenterImpl};
use crate::snapshot::SnapshotInvalidation;
use crate::state::{CompositionMode, DamageOverlayState, DebugFlags, OverlayMode};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::sync::Arc;

const BLOSSOM_BORDER: i32 = 2;
const BLOSSOM_TITLE_BAR_HEIGHT: i32 = 24;

#[derive(Clone, Copy)]
struct DragState {
    window_id: ThingId,
    start_mouse: (i32, i32),
    start_rect: crate::geometry::Rect,
}

/// Tracks cursor performance metrics to ensure "butter smooth" behavior.
#[derive(Default)]
struct CursorMetrics {
    /// Total number of cursor movements
    cursor_moves: u64,
    /// Total number of cursor rasterizations (should be ~0 during movement)
    cursor_rasterizations: u64,
    /// Number of frames where only cursor moved (fast path eligible)
    frames_cursor_only: u64,
    /// Total damage rects from cursor (should be ~2 per move)
    damage_rects_from_cursor: u64,
    /// Last frame when metrics were logged
    last_log_frame: u64,
}

impl CursorMetrics {
    /// Log metrics periodically (every ~120 frames)
    fn maybe_log(&mut self, frame: u64) {
        if frame > 0 && frame % 120 == 0 && frame != self.last_log_frame {
            stem::info!(
                "[cursor metrics] frame={} moves={} rasterizations={} cursor_only_frames={} damage_rects={}",
                frame,
                self.cursor_moves,
                self.cursor_rasterizations,
                self.frames_cursor_only,
                self.damage_rects_from_cursor
            );
            self.last_log_frame = frame;
        }
    }

    /// Record a cursor movement
    fn record_move(&mut self) {
        self.cursor_moves += 1;
    }

    /// Record a cursor rasterization
    fn record_rasterization(&mut self) {
        self.cursor_rasterizations += 1;
    }

    /// Record a cursor-only frame (fast path)
    fn record_cursor_only_frame(&mut self) {
        self.frames_cursor_only += 1;
    }

    /// Record damage rects attributed to cursor
    fn record_cursor_damage_rects(&mut self, count: u64) {
        self.damage_rects_from_cursor += count;
    }
}

#[derive(Default)]
struct CursorUnderlay {
    bs_id: ThingId,
    rect: crate::geometry::Rect,
    pixels: alloc::vec::Vec<u32>,
    valid: bool,
}

#[derive(Default)]
struct GraphInputState {
    node_id: Option<ThingId>,
    last_keyboard_gen: u64,
}

fn find_bristle_node() -> Option<ThingId> {
    let mut input_nodes = [ThingId::default(); 16];
    match find(hid::SVC_INPUT, &mut input_nodes) {
        Ok(count) if count > 0 => {
            let count = count.min(input_nodes.len());
            let mut best = input_nodes[0];
            for node in input_nodes.iter().take(count).skip(1) {
                if node.to_u64_lossy() > best.to_u64_lossy() {
                    best = *node;
                }
            }
            Some(best)
        }
        _ => None,
    }
}

fn subscribe_bristle_topic() -> Option<PortHandle> {
    let input_node = find_bristle_node()?;
    if let Ok(topic_id) = prop_get(input_node, abi::schema::input::INPUT_TOPIC_ID) {
        if let Ok((write, read)) = port_create(4096) {
            if topic_subscribe(topic_id as u32, write).is_ok() {
                stem::info!(
                    "[bloom] dynamically subscribed to input topic {} on svc.Input {} via port {}",
                    topic_id,
                    input_node.to_u64_lossy(),
                    read
                );
                return Some(read);
            }
        }
    }
    None
}

fn sync_input_from_graph(
    pressed_keys: &mut BTreeSet<Key>,
    bristle_node: Option<ThingId>,
    graph_input: &mut GraphInputState,
    had_key_event: bool,
    frame: u64,
) {
    use abi::schema::keyboard as kb;

    let Some(node) = bristle_node else {
        if frame % 120 == 0 {
            stem::warn!("[bloom] no svc.Input node found for graph input fallback");
        }
        return;
    };

    let keyboard_gen = prop_get(node, kb::KEYBOARD_GEN).unwrap_or(0);
    if graph_input.node_id != Some(node) {
        graph_input.node_id = Some(node);
        graph_input.last_keyboard_gen = keyboard_gen;
    } else if keyboard_gen < graph_input.last_keyboard_gen {
        graph_input.last_keyboard_gen = 0;
        pressed_keys.clear();
    }

    if !had_key_event && keyboard_gen > graph_input.last_keyboard_gen {
        graph_input.last_keyboard_gen = keyboard_gen;
        let key = Key::from_raw(prop_get(node, kb::KEYBOARD_LAST_KEY).unwrap_or(0) as u16);
        if key != Key::Unknown {
            let edge = prop_get(node, kb::KEYBOARD_KEY_EDGE).unwrap_or(0);
            if edge == 0 {
                pressed_keys.remove(&key);
            } else {
                pressed_keys.insert(key);
            }
        }
    }
}

#[inline]
fn capture_cursor_underlay(
    surface: &surface::PixelBuffer,
    bs_id: ThingId,
    rect: crate::geometry::Rect,
    underlay: &mut CursorUnderlay,
) {
    let bounds = crate::geometry::Rect::full(surface.width(), surface.height());
    let clipped = rect.clip(bounds);
    if clipped.is_empty() {
        underlay.valid = false;
        return;
    }

    let w = clipped.width() as usize;
    let h = clipped.height() as usize;
    let row_bytes = w * 4;
    let total_pixels = w * h;

    if underlay.pixels.len() != total_pixels {
        underlay.pixels.resize(total_pixels, 0);
    }

    unsafe {
        let dst = underlay.pixels.as_mut_ptr() as *mut u8;
        for row in 0..h {
            let src_off =
                (clipped.y() as usize + row) * surface.stride_bytes + (clipped.x() as usize * 4);
            core::ptr::copy_nonoverlapping(
                surface.ptr.add(src_off),
                dst.add(row * row_bytes),
                row_bytes,
            );
        }
    }

    underlay.bs_id = bs_id;
    underlay.rect = clipped;
    underlay.valid = true;
}

#[inline]
fn restore_cursor_underlay(surface: &mut surface::PixelBuffer, underlay: &CursorUnderlay) {
    if !underlay.valid {
        return;
    }

    let rect = underlay.rect;
    if rect.is_empty() {
        return;
    }

    let bounds = crate::geometry::Rect::full(surface.width(), surface.height());
    if rect.clip(bounds) != rect {
        return;
    }

    let w = rect.width() as usize;
    let h = rect.height() as usize;
    let row_bytes = w * 4;
    if underlay.pixels.len() != w * h {
        return;
    }

    unsafe {
        let src = underlay.pixels.as_ptr() as *const u8;
        for row in 0..h {
            let dst_off =
                (rect.y() as usize + row) * surface.stride_bytes + (rect.x() as usize * 4);
            core::ptr::copy_nonoverlapping(
                src.add(row * row_bytes),
                surface.ptr.add(dst_off),
                row_bytes,
            );
        }
    }
}

fn log_simd_backend() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        #[cfg(target_feature = "sse2")]
        {
            crate::log!("[bloom] SIMD backend: SSE2 (x86_64)");
            crate::trace_event!("bloom.simd.backend", "SSE2");
        }
        #[cfg(not(target_feature = "sse2"))]
        {
            crate::log!("[bloom] SIMD backend: Scalar (x86_64, no SSE2)");
            crate::trace_event!("bloom.simd.backend", "Scalar");
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(target_feature = "neon")]
        {
            crate::log!("[bloom] SIMD backend: NEON (aarch64)");
            crate::trace_event!("bloom.simd.backend", "NEON");
        }
        #[cfg(not(target_feature = "neon"))]
        {
            crate::log!("[bloom] SIMD backend: Scalar (aarch64, no NEON)");
            crate::trace_event!("bloom.simd.backend", "Scalar");
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        crate::log!("[bloom] SIMD backend: Scalar (other arch)");
        crate::trace_event!("bloom.simd.backend", "Scalar");
    }
}

fn clear_surface(surface: &mut surface::PixelBuffer, color: u32) {
    raster::fill_rect_copy(surface, 0, 0, surface.width(), surface.height(), color);
}

fn clear_damage(surface: &mut surface::PixelBuffer, damage: &crate::damage::Damage, color: u32) {
    for rect in damage.iter() {
        raster::fill_rect_copy(
            surface,
            rect.x(),
            rect.y(),
            rect.width(),
            rect.height(),
            color,
        );
    }
}

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn window_rect_from_props(
    window_id: ThingId,
    screen_w: i32,
    screen_h: i32,
) -> crate::geometry::Rect {
    let w = stem::thing::sys::prop_get(window_id, keys::UI_WIDTH).unwrap_or(0) as i32;
    let h = stem::thing::sys::prop_get(window_id, keys::UI_HEIGHT).unwrap_or(0) as i32;
    if w <= 0 || h <= 0 {
        return crate::geometry::Rect::new(0, 0, 0, 0);
    }
    let mut x = stem::thing::sys::prop_get(window_id, keys::UI_X).unwrap_or(0) as i32;
    let mut y = stem::thing::sys::prop_get(window_id, keys::UI_Y).unwrap_or(0) as i32;
    let inset_right =
        stem::thing::sys::prop_get(window_id, keys::UI_INSET_RIGHT).unwrap_or(0) as i32;
    let inset_bottom =
        stem::thing::sys::prop_get(window_id, keys::UI_INSET_BOTTOM).unwrap_or(0) as i32;
    if inset_right > 0 {
        x = screen_w - inset_right - w;
    }
    if inset_bottom > 0 {
        y = screen_h - inset_bottom - h;
    }
    crate::geometry::Rect::new(x, y, w, h)
}

fn tile_windows(screen_w: i32, screen_h: i32) {
    let mut windows = [ThingId::default(); 128];
    let count = stem::thing::sys::find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);

    let mut photosynthesis = None;
    let mut font_explorer = None;

    for win in windows.iter().take(count) {
        let mut title_buf = [0u8; 128];
        if let Ok(val) = stem::thing::sys::prop_get(*win, keys::UI_TITLE) {
            let bs_id = ThingId::from_u64(val);
            if let Ok(len) = stem::thing::sys::bytespace_read(bs_id, 0, &mut title_buf) {
                let title = core::str::from_utf8(&title_buf[..len]).unwrap_or("");
                if title.contains("Photosynthesis") {
                    photosynthesis = Some(*win);
                } else if title.contains("Font Explorer") {
                    font_explorer = Some(*win);
                }
            }
        }
    }

    // Tiling logic:
    // Photosynthesis: Left 1/2
    // Font Explorer: Upper Right 1/4

    // UI_MANUAL_POSITION must be checked before tiling!

    // Tiling logic:
    // Photosynthesis: Left 1/2
    // Font Explorer: Upper Right 1/4

    // UI_MANUAL_POSITION must be checked before tiling!

    if let Some(win) = photosynthesis {
        let manual = stem::thing::sys::prop_get(win, keys::UI_MANUAL_POSITION).unwrap_or(0);
        stem::info!(
            "[bloom] tile_windows: Photosynthesis id={:?} manual={}",
            win,
            manual
        );
        if manual == 0 {
            let _ = stem::thing::sys::prop_set(win, keys::UI_X, 0);
            let _ = stem::thing::sys::prop_set(win, keys::UI_Y, 0);
            let _ = stem::thing::sys::prop_set(win, keys::UI_WIDTH, (screen_w / 2) as u64);
            let _ = stem::thing::sys::prop_set(win, keys::UI_HEIGHT, screen_h as u64);
            let _ = stem::thing::sys::prop_set(win, keys::UI_INSET_RIGHT, 0);
            let _ = stem::thing::sys::prop_set(win, keys::UI_INSET_BOTTOM, 0);
            let _ = stem::thing::sys::prop_set(win, keys::UI_MANUAL_POSITION, 1);
        } else {
            stem::info!("[bloom] tile_windows: SKIPPING Photosynthesis (manual override)");
        }
    }

    if let Some(win) = font_explorer {
        let manual = stem::thing::sys::prop_get(win, keys::UI_MANUAL_POSITION).unwrap_or(0);
        stem::info!(
            "[bloom] tile_windows: FontExplorer id={:?} manual={}",
            win,
            manual
        );
        if manual == 0 {
            let _ = stem::thing::sys::prop_set(win, keys::UI_X, (screen_w / 2) as u64);
            let _ = stem::thing::sys::prop_set(win, keys::UI_Y, 0);
            let _ = stem::thing::sys::prop_set(win, keys::UI_WIDTH, (screen_w / 2) as u64);
            let _ = stem::thing::sys::prop_set(win, keys::UI_HEIGHT, (screen_h / 2) as u64);
            let _ = stem::thing::sys::prop_set(win, keys::UI_INSET_RIGHT, 0);
            let _ = stem::thing::sys::prop_set(win, keys::UI_INSET_BOTTOM, 0);
            let _ = stem::thing::sys::prop_set(win, keys::UI_MANUAL_POSITION, 1);
        } else {
            stem::info!("[bloom] tile_windows: SKIPPING FontExplorer (manual override)");
        }
    }
}

fn in_title_bar(rect: crate::geometry::Rect, _x: i32, y: i32) -> bool {
    let local_y = y - rect.y();
    let title_top = BLOSSOM_BORDER;
    let title_bottom = BLOSSOM_BORDER + BLOSSOM_TITLE_BAR_HEIGHT;
    local_y >= title_top && local_y < title_bottom
}

fn in_client_area(rect: crate::geometry::Rect, x: i32, y: i32) -> bool {
    let left = rect.x() + BLOSSOM_BORDER;
    let right = rect.x() + rect.width() - BLOSSOM_BORDER;
    let top = rect.y() + BLOSSOM_BORDER + BLOSSOM_TITLE_BAR_HEIGHT;
    let bottom = rect.y() + rect.height() - BLOSSOM_BORDER;
    x >= left && x < right && y >= top && y < bottom
}

fn clamp_window_rect(
    rect: crate::geometry::Rect,
    screen_w: i32,
    screen_h: i32,
) -> crate::geometry::Rect {
    let mut r = rect;
    let min_visible = BLOSSOM_TITLE_BAR_HEIGHT;
    if r.y() + min_visible < 0 {
        r.origin.y = -min_visible + 1;
    }
    if r.y() > screen_h - min_visible {
        r.origin.y = screen_h - min_visible;
    }
    if r.x() + r.width() < min_visible {
        r.origin.x = min_visible - r.width();
    }
    if r.x() > screen_w - min_visible {
        r.origin.x = screen_w - min_visible;
    }
    r
}

fn set_focus(focused_window: &mut Option<ThingId>, target: Option<ThingId>) {
    if *focused_window == target {
        return;
    }
    if let Some(prev) = focused_window.take() {
        let _ = stem::thing::sys::prop_set(prev, keys::UI_FOCUSED, 0);
    }
    if let Some(next) = target {
        let _ = stem::thing::sys::prop_set(next, keys::UI_FOCUSED, 1);
        *focused_window = Some(next);
    }
}

fn should_force_full_damage(causes: &[SnapshotInvalidation]) -> Option<SnapshotInvalidation> {
    causes.iter().copied().find(|cause| {
        matches!(
            cause,
            SnapshotInvalidation::ThemeChanged
                | SnapshotInvalidation::WallpaperChanged
                | SnapshotInvalidation::Forced
        )
    })
}

fn requires_window_rescan(causes: &[SnapshotInvalidation]) -> bool {
    causes.iter().any(|cause| {
        matches!(
            cause,
            SnapshotInvalidation::GeometryChanged | SnapshotInvalidation::Forced
        )
    })
}

fn requires_paint_refresh(causes: &[SnapshotInvalidation]) -> bool {
    causes.iter().any(|cause| {
        matches!(
            cause,
            SnapshotInvalidation::GeometryChanged
                | SnapshotInvalidation::ContentChanged
                | SnapshotInvalidation::Forced
        )
    })
}

fn cycle_windows_in_order(
    order: &[ThingId],
    current: Option<ThingId>,
    reverse: bool,
    max_z: &mut i32,
) -> Option<ThingId> {
    if order.is_empty() {
        return None;
    }

    let target_idx = match current.and_then(|id| order.iter().position(|wid| *wid == id)) {
        Some(idx) => {
            if reverse {
                if idx == 0 {
                    order.len() - 1
                } else {
                    idx - 1
                }
            } else {
                (idx + 1) % order.len()
            }
        }
        None => {
            if reverse {
                order.len() - 1
            } else {
                0
            }
        }
    };
    let target = order[target_idx];

    // Publish focus status to graph
    if let Some(prev) = current {
        let _ = stem::thing::sys::prop_set(prev, keys::UI_FOCUSED, 0);
    }
    let _ = stem::thing::sys::prop_set(target, keys::UI_FOCUSED, 1);

    *max_z = max_z.saturating_add(1);
    let _ = stem::thing::sys::prop_set(target, keys::UI_Z_INDEX, *max_z as u64);
    Some(target)
}

#[cfg_attr(not(test), stem::main)]
fn main(arg: usize) -> ! {
    logging::init();
    perf::init();

    // Log SIMD backend selection for masked compositing
    log_simd_backend();

    use stem::thing::sys::{bytespace_map, bytespace_unmap};
    let bs_id = ThingId::from_u64(arg as u64);
    let (mut arg_req, mut arg_resp, mut bristle_evt) = (0, 0, 0);
    let mut display_bs_id = stem::thing::ThingId::default();
    if let Ok(ptr) = bytespace_map(bs_id) {
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u32, 16) };
        if slice[0] == 0xB100AA01 {
            arg_req = slice[1];
            arg_resp = slice[2];
            bristle_evt = slice[3];
            let mut id_bytes = [0u8; 16];
            id_bytes[0..4].copy_from_slice(&slice[4].to_le_bytes());
            id_bytes[4..8].copy_from_slice(&slice[5].to_le_bytes());
            id_bytes[8..12].copy_from_slice(&slice[6].to_le_bytes());
            id_bytes[12..16].copy_from_slice(&slice[7].to_le_bytes());
            display_bs_id = stem::thing::ThingId(id_bytes);
        }
        let _ = bytespace_unmap(bs_id, ptr);
    } else {
        arg_req = unpack_handle(arg, 0) as u32;
        arg_resp = unpack_handle(arg, 1) as u32;
        bristle_evt = unpack_handle(arg, 2) as u32;
    }

    stem::info!(
        "[bloom] EARLY boot args: bristle_evt={} arg_req={} arg_resp={}",
        bristle_evt,
        arg_req,
        arg_resp
    );
    let target = if display_bs_id.to_u64_lossy() != 0 {
        CompositorTarget::map_from_bytespace(display_bs_id, (arg_req, arg_resp))
            .or_else(|_| CompositorTarget::discover_and_map((arg_req, arg_resp), 2000))
    } else {
        CompositorTarget::discover_and_map((arg_req, arg_resp), 2000)
    }
    .expect("compositor discover");
    let _ = target.backend;

    // Buffer mapping cache for swapchain - maps bytespace IDs to their virtual addresses
    let mut buffer_cache: BTreeMap<ThingId, *mut u8> = BTreeMap::new();

    let (
        mut final_ptr,
        mut final_size,
        mut final_width,
        mut final_height,
        mut final_stride,
        mut final_bs_id,
        mut using_zero_copy,
        mut final_age,
    ) = (
        target.ptr,
        target.size_bytes,
        target.width,
        target.height,
        target.stride_bytes,
        target.bs_id,
        false,
        0u32,
    );
    let mut screen_format = target.format;

    let mut presenter = if target.driver_req != 0 {
        let mut d = DriverPresenter::new(target.driver_req, target.driver_resp);
        d.start_handshake();

        // Pump a few times to receive MSG_WELCOME (use yield_now, not sleep which can hang)
        for _ in 0..10 {
            d.pump();
            stem::yield_now();
        }

        // Immediate acquire to satisfy driver's need for a bound context before first present
        let mut d_presenter = PresenterImpl::Driver(d);
        stem::info!("[bloom] Requesting initial driver buffer...");
        let (acq_id, acq_w, acq_h, acq_s, acq_f, acq_age) = d_presenter.acquire_buffer();
        stem::info!("[bloom] ACQUIRED RETURNED!");

        // Map the initial buffer
        if let Ok(ptr) = stem::thing::sys::bytespace_map(acq_id) {
            buffer_cache.insert(acq_id, ptr);
            final_ptr = ptr;
            final_size = (acq_h * acq_s) as usize;
            final_width = acq_w;
            final_height = acq_h;
            final_stride = acq_s;
            final_bs_id = acq_id;
            screen_format = acq_f;
            final_age = acq_age;
            stem::info!(
                "[bloom] ACQUIRED initial driver buffer: {:p} (bs_id={:?})",
                ptr,
                acq_id
            );
        } else {
            stem::error!("[bloom] FAILED to map initial driver buffer");
        }

        d_presenter
    } else {
        stem::warn!("[bloom] WARNING: No display driver found! Using NullPresenter (headless mode). Screen will be black.");
        PresenterImpl::Null(present::NullPresenter)
    };

    let _ = using_zero_copy; // Suppress unused warning

    stem::info!("bloom: creating surface...");
    let mut surface = unsafe {
        surface::PixelBuffer::new(
            final_ptr,
            final_size,
            final_width,
            final_height,
            final_stride,
        )
    };
    stem::info!("bloom: surface created!");

    // UI Root
    let mut roots = [ThingId::default(); 1];
    stem::info!("bloom: finding UI CROWN...");
    let ui_crown = match stem::thing::sys::find(abi::schema::kinds::UI_CROWN, &mut roots) {
        Ok(count) if count > 0 => {
            stem::info!("bloom: found existing UI CROWN");
            roots[0]
        }
        _ => {
            stem::info!("bloom: creating new UI CROWN");
            stem::ui::UiBuilder::create_root()
        }
    };
    stem::info!("bloom: UI CROWN initialized!");

    let _ = ui_crown;
    let mut paint_pipeline = PaintPipeline::new();
    let mut scene = crate::scene_graph::SceneGraph::new();

    // Spawn asset workers with diagnostic logging
    use stem::stack::{Stack, StackSpec};
    let s_spec = StackSpec {
        reserve_bytes: 256 * 1024,
        ..StackSpec::default()
    };

    match Stack::alloc_growing_stack(s_spec) {
        Ok(stack) => match stem::thread::spawn_on(stack, painter_resources::asset_watcher_entry) {
            Ok(tid) => stem::info!("bloom: spawned asset watcher (tid={})", tid),
            Err(e) => stem::error!("bloom: FAILED to spawn asset watcher: {:?}", e),
        },
        Err(e) => stem::error!("bloom: FAILED to alloc asset watcher stack: {:?}", e),
    }

    let mut loop_ctrl = FrameLoop::new(60);
    let mut screen_w = final_width as i32;
    let mut screen_h = final_height as i32;

    // Cursor state
    // Prefer Bristle's broker topic over the legacy boot-wired event port. The
    // topic path is the authoritative distribution channel and has proven more
    // responsive than the older direct handoff.
    let mut bristle_evt_handle = subscribe_bristle_topic().unwrap_or(bristle_evt as PortHandle);

    stem::info!(
        "[bloom] bristle_evt_handle = {} (legacy was {})",
        bristle_evt_handle,
        bristle_evt
    );
    let mut cursor = CursorState::new(screen_w / 2, screen_h / 2);
    let mut bristle_node = find_bristle_node();
    let mut graph_input = GraphInputState::default();
    let mut paint_pending_rebuilds = false;
    let mut cursor_rasterizer = CursorRasterizer::new();
    let mut pressed_keys: BTreeSet<Key> = BTreeSet::new();
    let mut prev_keys: BTreeSet<Key> = BTreeSet::new();
    let mut prev_cursor_buttons = cursor.buttons();
    let mut ui_dispatch = ui_events::UiEventDispatcher::new();
    let mut focused_window: Option<ThingId> = None;
    let mut alt_cycle_order: alloc::vec::Vec<ThingId> = alloc::vec::Vec::new();
    let mut maximized_windows: alloc::collections::BTreeMap<ThingId, crate::geometry::Rect> =
        alloc::collections::BTreeMap::new();
    let mut alt_cycle_max_z: i32 = 0;
    let mut alt_prev_down = false;
    let accel_cfg = MouseAccelConfig::default();
    let mut accel_state = MouseAccelState::default();
    // Track previous cursor position for damage computation
    let mut prev_cursor_x = cursor.x;
    let mut prev_cursor_y = cursor.y;
    let mut prev_cursor_gen = crate::frame::AssetGeneration::ZERO;
    let mut cursor_underlays: BTreeMap<ThingId, CursorUnderlay> = BTreeMap::new();
    let mut drag_state: Option<DragState> = None;

    let mut debug_flags = DebugFlags::default();
    let mut overlay_state = DamageOverlayState::default();
    let mut cursor_metrics = CursorMetrics::default();

    let mut wayland_server = crate::wayland::server::WaylandServer::new().expect("Failed to start WaylandServer");
    stem::info!("bloom: WaylandServer started at /run/wayland-0");

    // Composition mode: CPU (default) or GPU (virgl-accelerated)
    #[cfg(feature = "gpu")]
    let composition_mode = if target.backend == crate::compositor::DisplayBackend::VirtioGpu
        && presenter.has_3d_cap()
    {
        stem::info!("bloom: VirtioGpu + Virgl 3D detected - enabling GPU composition mode");
        CompositionMode::Gpu
    } else {
        stem::info!(
            "bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode"
        );
        CompositionMode::Cpu
    };
    #[cfg(not(feature = "gpu"))]
    let composition_mode = CompositionMode::Cpu;

    // GPU compositor instance for virgl 3D composition
    #[cfg(feature = "gpu")]
    let mut gpu_compositor = if composition_mode == CompositionMode::Gpu {
        let mut gc = gpu_compositor::GpuCompositor::new();
        gc.set_scanout_resource(
            /* will be set later from driver */ 0,
            final_width,
            final_height,
        );
        gc.mark_initialized();
        Some(gc)
    } else {
        None
    };

    // Window Manager disabled in paint pipeline (no legacy chrome/hit testing)

    // Glyph Arrival Watch
    stem::info!("bloom: calling intern for FONT_GLYPH");
    let glyph_watch_pred = stem::thing::sys::intern(kinds::FONT_GLYPH).unwrap_or(0);
    stem::info!("bloom: intern returned FONT_GLYPH={}", glyph_watch_pred);
    let glyph_watch = if glyph_watch_pred != 0 {
        use abi::root::RootWatchFilter;
        use abi::types::{WatchMode, WatchSpec};
        let filter = RootWatchFilter::predicate(glyph_watch_pred);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        stem::info!("bloom: calling root_watch_open for FONT_GLYPH");
        let res = stem::syscall::root_watch_open(&spec).ok();
        stem::info!("bloom: root_watch_open ret={:?}", res);
        res
    } else {
        None
    };

    // UI Window Watch - triggers dirty when windows are created/modified
    stem::info!("bloom: calling intern for UI_WINDOW");
    let ui_window_kind = stem::thing::sys::intern(kinds::UI_WINDOW).unwrap_or(0);
    stem::info!("bloom: intern returned UI_WINDOW={}", ui_window_kind);
    let ui_window_watch = if ui_window_kind != 0 {
        use abi::root::RootWatchFilter;
        use abi::types::{WatchMode, WatchSpec};
        // Use kind() filter to watch for node creation, not predicate() which watches edges
        let filter = RootWatchFilter::subject(ui_window_kind.into());
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        stem::info!("bloom: calling root_watch_open for UI_WINDOW");
        let res = stem::syscall::root_watch_open(&spec).ok();
        stem::info!("bloom: root_watch_open ret={:?}", res);
        res
    } else {
        None
    };

    // UI Paint Watch - triggers dirty when paint generation changes
    stem::info!("bloom: calling intern for UI_PAINT_GEN");
    let ui_paint_gen_key = stem::thing::sys::intern(keys::UI_PAINT_GEN).unwrap_or(0);
    stem::info!("bloom: intern returned UI_PAINT_GEN={}", ui_paint_gen_key);
    let ui_paint_watch = if ui_paint_gen_key != 0 {
        use abi::root::RootWatchFilter;
        use abi::types::{WatchMode, WatchSpec};
        let filter = RootWatchFilter::predicate(ui_paint_gen_key);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        stem::info!("bloom: calling root_watch_open for UI_PAINT_GEN");
        let res = stem::syscall::root_watch_open(&spec).ok();
        stem::info!("bloom: root_watch_open ret={:?}", res);
        res
    } else {
        None
    };

    // Track watch event counts for diagnostics
    let mut ui_watch_events_total: u64 = 0;
    // Removed force_full_damage bool, using invalidation_causes vector
    let mut invalidation_causes: alloc::vec::Vec<SnapshotInvalidation> =
        alloc::vec::Vec::with_capacity(16);

    // We purposefully do NOT wait for fonts here (e.g. NotoSans-Regular).
    // The UI should just start drawing immediately, even if it means some text is missing
    // or using fallback fonts for the first few frames.
    stem::info!("[bloom] Starting UI loop immediately (not waiting for fonts)");

    // Signal that the compositor is taking over the framebuffer
    stem::syscall::console_disable();

    let mut first_frame_rendered = false;
    let mut last_loop_start_ns = stem::monotonic_ns();

    let mut current_bs_id = final_bs_id;
    let mut latest_wayland_commit: Option<crate::wayland::server::WaylandSurfaceCommit> = None;
    let mut current_age = final_age;

    loop {
        let loop_start_ns = stem::monotonic_ns();
        let loop_gap_ns = loop_start_ns.saturating_sub(last_loop_start_ns);
        if loop_gap_ns > 100_000_000 {
            stem::warn!(
                "[bloom] compositor loop gap {:.3}ms",
                loop_gap_ns as f64 / 1_000_000.0
            );
        }
        last_loop_start_ns = loop_start_ns;
        loop_ctrl.next();
        
        wayland_server.pump();
        for commit in wayland_server.committed_surfaces.drain(..) {
            stem::info!("Wayland frame committed! bs_id={}, w={}, h={}", commit.bs_id, commit.width, commit.height);
            latest_wayland_commit = Some(commit);
            paint_pending_rebuilds = true; // force repaint to show the latest buffer
        }

        invalidation_causes.clear();
        let updates = ASSETS.publish_pending();
        if updates.wallpaper_changed {
            invalidation_causes.push(SnapshotInvalidation::WallpaperChanged);
        }

        // 0. Update surface if buffer changed
        if let PresenterImpl::Driver(ref mut d) = presenter {
            if current_bs_id != final_bs_id {
                // Remap surface for new buffer
                let (bs_id, w, h, s, _f, age) = (
                    final_bs_id,
                    final_width,
                    final_height,
                    final_stride,
                    0,
                    current_age,
                );

                // Use cached pointer or map if new
                let ptr = if let Some(&ptr) = buffer_cache.get(&bs_id) {
                    ptr
                } else {
                    match stem::thing::sys::bytespace_map(bs_id) {
                        Ok(p) => {
                            buffer_cache.insert(bs_id, p);
                            p
                        }
                        Err(e) => {
                            stem::error!("[bloom] FAILED to map buffer {:?}: {:?}", bs_id, e);
                            // Fallback to current pointer if possible, though this may lead to corruption
                            surface.ptr
                        }
                    }
                };

                let size = (h * s) as usize;
                surface = unsafe { surface::PixelBuffer::new(ptr, size, w, h, s) };
                current_bs_id = bs_id;
            }
        }

        // Handle raw input before any expensive graph/watch/paint work so cursor motion
        // is not delayed behind compositor bookkeeping.
        if bristle_evt_handle == 0 {
            if let Some(handle) = subscribe_bristle_topic() {
                bristle_evt_handle = handle;
            }
        }

        let mut poll_stats = crate::bristle::PollStats::default();
        if bristle_evt_handle != 0 {
            prev_keys = pressed_keys.clone();
            poll_stats = {
                crate::trace_span!("bloom.loop.poll_bristle");
                poll_bristle(
                    bristle_evt_handle,
                    &mut cursor,
                    &mut pressed_keys,
                    &accel_cfg,
                    &mut accel_state,
                    screen_w,
                    screen_h,
                )
            };
            bristle_node = find_bristle_node().or(bristle_node);
            sync_input_from_graph(
                &mut pressed_keys,
                bristle_node,
                &mut graph_input,
                poll_stats.had_key_event,
                loop_ctrl.frame_number(),
            );
        }

        // Poll font client for IPC responses. Font warmup should not trigger
        // immediate compositor redraws; those redraws can monopolize the main
        // loop under emulation and make cursor motion feel stuck.
        let _font_cache_updated = crate::font_client::poll();

        // 0. Check for new glyphs in graph
        if let Some(gw) = glyph_watch {
            let mut g_seq = 0u64;
            let mut g_buf = [0u8; 1024];
            if let Ok(len) = stem::syscall::root_watch_try_next(gw, &mut g_seq, &mut g_buf) {
                if len > 0 {
                    crate::font_graph::mark_dirty();
                }
            }
        }

        // 1. Check for UI window changes
        if let Some(uw) = ui_window_watch {
            let mut w_seq = 0u64;
            let mut w_buf = [0u8; 256];
            let mut drained = 0u32;
            // Drain all pending events this frame
            while let Ok(len) = stem::syscall::root_watch_try_next(uw, &mut w_seq, &mut w_buf) {
                if len > 0 {
                    drained += 1;
                } else {
                    break;
                }
            }
            if drained > 0 {
                ui_watch_events_total += drained as u64;
                stem::info!(
                    "[bloom] UI watch: drained {} events (total={})",
                    drained,
                    ui_watch_events_total
                );
                invalidation_causes.push(SnapshotInvalidation::GeometryChanged);
            }
        }

        // 2. Check for UI_PAINT updates
        if let Some(pw) = ui_paint_watch {
            let mut p_seq = 0u64;
            let mut p_buf = [0u8; 256];
            let mut drained = 0u32;
            while let Ok(len) = stem::syscall::root_watch_try_next(pw, &mut p_seq, &mut p_buf) {
                if len > 0 {
                    drained += 1;
                } else {
                    break;
                }
            }
            if drained > 0 {
                invalidation_causes.push(SnapshotInvalidation::ContentChanged);
            }
        }

        let cursor_moved_since_last_frame = cursor.x != prev_cursor_x || cursor.y != prev_cursor_y;
        let pointer_motion_only = first_frame_rendered
            && poll_stats.had_pointer_event
            && !poll_stats.had_key_event
            && cursor.buttons() == prev_cursor_buttons
            && cursor_moved_since_last_frame;
        let cursor_frame_priority =
            pointer_motion_only && (paint_pending_rebuilds || !invalidation_causes.is_empty());
        let should_process_updates = !cursor_frame_priority
            && (!first_frame_rendered || paint_pending_rebuilds || !invalidation_causes.is_empty());

        // Refresh window state after input so hit-testing sees current geometry, but let
        // pure pointer-motion frames bypass background window rebuild work.
        let paint_res = if should_process_updates {
            let rescan_windows =
                !first_frame_rendered || requires_window_rescan(&invalidation_causes);
            let refresh_paint =
                !first_frame_rendered || requires_paint_refresh(&invalidation_causes);
            let res = paint_pipeline.process_updates(
                &mut scene,
                screen_w,
                screen_h,
                rescan_windows,
                refresh_paint,
                || {
                    if bristle_evt_handle == 0 {
                        return;
                    }
                    let progress_poll = poll_bristle(
                        bristle_evt_handle,
                        &mut cursor,
                        &mut pressed_keys,
                        &accel_cfg,
                        &mut accel_state,
                        screen_w,
                        screen_h,
                    );
                    if progress_poll.had_pointer_event || progress_poll.had_key_event {
                        bristle_node = find_bristle_node().or(bristle_node);
                        sync_input_from_graph(
                            &mut pressed_keys,
                            bristle_node,
                            &mut graph_input,
                            progress_poll.had_key_event,
                            loop_ctrl.frame_number(),
                        );
                    }
                },
            );
            paint_pending_rebuilds = res.pending_rebuilds;
            res
        } else {
            crate::paint_vm::PaintResult {
                damage: alloc::vec::Vec::new(),
                pending_rebuilds: paint_pending_rebuilds,
            }
        };

        // Late-latch input that arrived while process_updates() was running so cursor motion
        // does not wait an extra compositor iteration.
        if bristle_evt_handle != 0 {
            let late_poll_stats = {
                crate::trace_span!("bloom.loop.poll_bristle_late");
                poll_bristle(
                    bristle_evt_handle,
                    &mut cursor,
                    &mut pressed_keys,
                    &accel_cfg,
                    &mut accel_state,
                    screen_w,
                    screen_h,
                )
            };
            if late_poll_stats.had_pointer_event || late_poll_stats.had_key_event {
                bristle_node = find_bristle_node().or(bristle_node);
                sync_input_from_graph(
                    &mut pressed_keys,
                    bristle_node,
                    &mut graph_input,
                    late_poll_stats.had_key_event,
                    loop_ctrl.frame_number(),
                );
            }
        }

        // Input-driven window management
        if bristle_evt_handle != 0 {
            let shift_down =
                pressed_keys.contains(&Key::LeftShift) || pressed_keys.contains(&Key::RightShift);

            // F11 Toggle: Maximize/Restore focused window
            if pressed_keys.contains(&Key::F11) && !prev_keys.contains(&Key::F11) {
                if let Some(focused) = focused_window {
                    if let Some(restore_rect) = maximized_windows.remove(&focused) {
                        // Restore
                        stem::info!(
                            "[bloom] F11: Restoring window {:?} to {:?}",
                            focused,
                            restore_rect
                        );
                        let _ = stem::thing::sys::prop_set(
                            focused,
                            keys::UI_X,
                            restore_rect.x() as u64,
                        );
                        let _ = stem::thing::sys::prop_set(
                            focused,
                            keys::UI_Y,
                            restore_rect.y() as u64,
                        );
                        let _ = stem::thing::sys::prop_set(
                            focused,
                            keys::UI_WIDTH,
                            restore_rect.width() as u64,
                        );
                        let _ = stem::thing::sys::prop_set(
                            focused,
                            keys::UI_HEIGHT,
                            restore_rect.height() as u64,
                        );
                        // Ensure manual position is set so tiling doesn't clobber it immediately
                        let _ = stem::thing::sys::prop_set(focused, keys::UI_MANUAL_POSITION, 1);
                    } else {
                        // Maximize
                        let x = stem::thing::sys::prop_get(focused, keys::UI_X).unwrap_or(0) as i32;
                        let y = stem::thing::sys::prop_get(focused, keys::UI_Y).unwrap_or(0) as i32;
                        let w =
                            stem::thing::sys::prop_get(focused, keys::UI_WIDTH).unwrap_or(0) as i32;
                        let h = stem::thing::sys::prop_get(focused, keys::UI_HEIGHT).unwrap_or(0)
                            as i32;
                        let current_rect = crate::geometry::Rect::new(x, y, w, h);

                        maximized_windows.insert(focused, current_rect);
                        stem::info!(
                            "[bloom] F11: Maximizing window {:?} (saved {:?})",
                            focused,
                            current_rect
                        );

                        let _ = stem::thing::sys::prop_set(focused, keys::UI_X, 0);
                        let _ = stem::thing::sys::prop_set(focused, keys::UI_Y, 0);
                        let _ =
                            stem::thing::sys::prop_set(focused, keys::UI_WIDTH, screen_w as u64);
                        let _ =
                            stem::thing::sys::prop_set(focused, keys::UI_HEIGHT, screen_h as u64);
                        let _ = stem::thing::sys::prop_set(focused, keys::UI_MANUAL_POSITION, 1);
                    }
                    invalidation_causes.push(SnapshotInvalidation::Forced);
                }
            }

            // ----------------------------------------------------------------
            // Debug Damage Tracking Keyboard Shortcuts
            // ----------------------------------------------------------------
            // F9: Toggle damage rect overlay (Shift+F9 for raw rects)
            // F8: Toggle damage cause color-coded overlay
            // F7: Auto-tile windows (existing feature)
            // F6: Toggle damage statistics display
            // F5: Toggle force full damage (always redraw entire frame)
            // F4: Toggle disable damage tracking (always full redraw)
            // F3: Toggle replay last frame damage (diagnostic tool)
            // ----------------------------------------------------------------

            if pressed_keys.contains(&Key::F9) && !prev_keys.contains(&Key::F9) {
                if shift_down {
                    debug_flags.show_raw_damage_rects = !debug_flags.show_raw_damage_rects;
                    stem::info!(
                        "[bloom] debug: raw damage overlay {}",
                        if debug_flags.show_raw_damage_rects {
                            "ON"
                        } else {
                            "OFF"
                        }
                    );
                } else {
                    debug_flags.show_damage_rects = !debug_flags.show_damage_rects;
                    stem::info!(
                        "[bloom] debug: damage overlay {}",
                        if debug_flags.show_damage_rects {
                            "ON"
                        } else {
                            "OFF"
                        }
                    );
                }
            }

            // F8: Toggle damage cause visualization
            if pressed_keys.contains(&Key::F8) && !prev_keys.contains(&Key::F8) {
                debug_flags.show_damage_causes = !debug_flags.show_damage_causes;
                stem::info!(
                    "[bloom] debug: damage cause overlay {}",
                    if debug_flags.show_damage_causes {
                        "ON"
                    } else {
                        "OFF"
                    }
                );
            }

            // F6: Toggle damage stats
            if pressed_keys.contains(&Key::F6) && !prev_keys.contains(&Key::F6) {
                debug_flags.show_damage_stats = !debug_flags.show_damage_stats;
                stem::info!(
                    "[bloom] debug: damage stats {}",
                    if debug_flags.show_damage_stats {
                        "ON"
                    } else {
                        "OFF"
                    }
                );
            }

            // F5: Toggle force full damage
            if pressed_keys.contains(&Key::F5) && !prev_keys.contains(&Key::F5) {
                debug_flags.force_full_damage = !debug_flags.force_full_damage;
                stem::info!(
                    "[bloom] debug: force full damage {}",
                    if debug_flags.force_full_damage {
                        "ON"
                    } else {
                        "OFF"
                    }
                );
            }

            // F4: Toggle disable damage tracking
            if pressed_keys.contains(&Key::F4) && !prev_keys.contains(&Key::F4) {
                debug_flags.disable_damage_tracking = !debug_flags.disable_damage_tracking;
                stem::info!(
                    "[bloom] debug: disable damage tracking {}",
                    if debug_flags.disable_damage_tracking {
                        "ON (always full redraw)"
                    } else {
                        "OFF"
                    }
                );
            }

            // F3: Toggle replay last frame damage
            if pressed_keys.contains(&Key::F3) && !prev_keys.contains(&Key::F3) {
                debug_flags.replay_last_frame_damage = !debug_flags.replay_last_frame_damage;
                stem::info!(
                    "[bloom] debug: replay last frame damage {}",
                    if debug_flags.replay_last_frame_damage {
                        "ON"
                    } else {
                        "OFF"
                    }
                );
            }

            if pressed_keys.contains(&Key::F7) && !prev_keys.contains(&Key::F7) {
                tile_windows(screen_w, screen_h);
                invalidation_causes.push(SnapshotInvalidation::Forced);
                stem::info!("[bloom] F7 pressed, auto-tiling windows");
            }

            // Alt-Tab Loading (use cached state)
            let alt_down =
                pressed_keys.contains(&Key::LeftAlt) || pressed_keys.contains(&Key::RightAlt);
            if alt_down && !alt_prev_down {
                crate::trace_counter!("bloom.win_cache.cycle.start", 1);
                // rebuild order using cached state
                let (order, max_z) = paint_pipeline.build_window_cycle_order();
                alt_cycle_order = order;
                alt_cycle_max_z = max_z;
                crate::trace_counter!("bloom.win_cache.avoided_find", 1);
                crate::trace_counter!(
                    "bloom.win_cache.avoided_prop_get",
                    alt_cycle_order.len() as u64
                ); // approximated
            }
            if !alt_down && alt_prev_down {
                alt_cycle_order.clear();
            }
            alt_prev_down = alt_down;
            if alt_down
                && !alt_cycle_order.is_empty()
                && pressed_keys.contains(&Key::Tab)
                && !prev_keys.contains(&Key::Tab)
            {
                if let Some(next) = cycle_windows_in_order(
                    &alt_cycle_order,
                    focused_window,
                    shift_down,
                    &mut alt_cycle_max_z,
                ) {
                    set_focus(&mut focused_window, Some(next));
                    crate::trace_counter!("bloom.win_cache.cycle.next", 1);
                }
            }

            let current_buttons = cursor.buttons();
            let left_down = (current_buttons & 1) != 0;
            let left_prev = (prev_cursor_buttons & 1) != 0;
            prev_cursor_buttons = current_buttons;
            let cursor_moved = cursor.x != prev_cursor_x || cursor.y != prev_cursor_y;

            if left_down && !left_prev {
                crate::trace_counter!("bloom.win_cache.hittest.count", 1);
                if let Some(hit_id) = scene.hit_test(cursor.x, cursor.y) {
                    let hit_rect = scene.get_surface(hit_id).map(|s| s.rect()).unwrap_or_default();
                    crate::trace_counter!("bloom.win_cache.avoided_find", 1);
                    set_focus(&mut focused_window, Some(hit_id));
                    
                    use crate::window_manager::{hit_test, Hit};
                    let hit_result = hit_test(cursor.x, cursor.y, hit_rect, false);
                    
                    if hit_result == Hit::TitleBar {
                        let inset_right =
                            stem::thing::sys::prop_get(hit_id, keys::UI_INSET_RIGHT).unwrap_or(0);
                        let inset_bottom =
                            stem::thing::sys::prop_get(hit_id, keys::UI_INSET_BOTTOM).unwrap_or(0);
                        if inset_right == 0 && inset_bottom == 0 {
                            drag_state = Some(DragState {
                                window_id: hit_id,
                                start_mouse: (cursor.x, cursor.y),
                                start_rect: hit_rect,
                            });
                            // Optimized raise: use cached max_z
                            let max_z = paint_pipeline.max_z_excluding(hit_id);
                            crate::trace_counter!("bloom.win_cache.raise.count", 1);
                            crate::trace_counter!("bloom.win_cache.avoided_find", 1); // raise_window used to find
                            let _ = stem::thing::sys::prop_set(
                                hit_id,
                                keys::UI_Z_INDEX,
                                (max_z as u64).saturating_add(1),
                            );
                        }
                    } else if hit_result == Hit::ClientArea {
                        // Optimized raise
                        let max_z = paint_pipeline.max_z_excluding(hit_id);
                        crate::trace_counter!("bloom.win_cache.raise.count", 1);
                        crate::trace_counter!("bloom.win_cache.avoided_find", 1);
                        let _ = stem::thing::sys::prop_set(
                            hit_id,
                            keys::UI_Z_INDEX,
                            (max_z as u64).saturating_add(1),
                        );
                        ui_dispatch.dispatch_click(cursor.x, cursor.y, screen_w, screen_h);
                    }
                } else {
                    set_focus(&mut focused_window, None);
                }
            }

            if !left_down && left_prev {
                drag_state = None;
            }

            if let Some(drag) = drag_state {
                set_focus(&mut focused_window, Some(drag.window_id));
                if left_down && cursor_moved {
                    let delta_x = cursor.x - drag.start_mouse.0;
                    let delta_y = cursor.y - drag.start_mouse.1;
                    let mut next_rect = crate::geometry::Rect::new(
                        drag.start_rect.x() + delta_x,
                        drag.start_rect.y() + delta_y,
                        drag.start_rect.width(),
                        drag.start_rect.height(),
                    );
                    next_rect = clamp_window_rect(next_rect, screen_w, screen_h);
                    let _ = stem::thing::sys::prop_set(
                        drag.window_id,
                        keys::UI_X,
                        next_rect.x() as u64,
                    );
                    let _ = stem::thing::sys::prop_set(
                        drag.window_id,
                        keys::UI_Y,
                        next_rect.y() as u64,
                    );
                    let _ = stem::thing::sys::prop_set(drag.window_id, keys::UI_INSET_RIGHT, 0);
                    let _ = stem::thing::sys::prop_set(drag.window_id, keys::UI_INSET_BOTTOM, 0);

                    // User moved the window, prevent auto-tiling
                    let _ = stem::thing::sys::prop_set(drag.window_id, keys::UI_MANUAL_POSITION, 1);
                    stem::info!(
                        "[bloom] drag: set UI_MANUAL_POSITION=1 for id={:?}",
                        drag.window_id
                    );
                }
            }

            ui_dispatch.dispatch_keyboard(&pressed_keys, &prev_keys);

            // Alt-Tab logic moved handled earlier (lines 656+)

            alt_prev_down = alt_down;
        }

        // Run UI Pipeline
        let mut list = drawlist::DrawList::new();
        // Damage Tracking (cursor fallback handling)
        let bounds = crate::geometry::Rect::full(screen_w, screen_h);
        let mut damage = damage::Damage::empty(bounds);

        // Force full damage on first frame to ensure UI appears immediately
        if !first_frame_rendered {
            damage = damage::Damage::full_with_cause(bounds, damage::DamageCause::ForceFull, None);
        }

        // Add damage from paint pipeline with appropriate causes. On pure pointer-motion
        // frames we intentionally defer scene work so cursor updates stay responsive.
        if !pointer_motion_only {
            for rect in &paint_res.damage {
                damage.add_rect_with_cause(*rect, damage::DamageCause::ContentChanged, None);
            }
        }

        if !pointer_motion_only {
            if let PresenterImpl::Driver(ref mut d) = presenter {
                d.expand_damage(&mut damage, current_age);
            }
        }

        let cursor_moved = cursor.x != prev_cursor_x || cursor.y != prev_cursor_y;
        let mut cursor_asset = ASSETS.get_cursor();

        // Track cursor movement
        if cursor_moved {
            cursor_metrics.record_move();
        }

        // If no cursor asset but we have a reactive cursor assigned, try to get it
        if cursor_asset.is_none() {
            if let Ok(root_id) = find(kinds::UI_CROWN, &mut [ThingId::default(); 1]) {
                if let Ok(cursor_id) = prop_get(ThingId::from_u64(root_id as u64), "ui.cursor") {
                    // The watcher should have enqueued it, but we check here too
                }
            }
        }

        // Count cursor damage rects before processing
        let cursor_damage_start = damage.rect_count();

        if let Some(asset) = cursor_asset {
            let (snapshot_opt, rasterized) = cursor_rasterizer.get_snapshot(&asset);

            // Track rasterization
            if rasterized {
                cursor_metrics.record_rasterization();
            }

            if let Some(snapshot) = snapshot_opt {
                let cursor_changed = snapshot.gen != prev_cursor_gen;

                if cursor_moved || cursor_changed {
                    let (cw, ch) = (snapshot.image.width as i32, snapshot.image.height as i32);
                    let old_rect = crate::geometry::Rect::new(
                        prev_cursor_x - snapshot.hotspot_x,
                        prev_cursor_y - snapshot.hotspot_y,
                        cw,
                        ch,
                    )
                    .expand(2)
                    .clip(bounds);
                    let new_rect = crate::geometry::Rect::new(
                        cursor.x - snapshot.hotspot_x,
                        cursor.y - snapshot.hotspot_y,
                        cw,
                        ch,
                    )
                    .expand(2)
                    .clip(bounds);

                    let cause = if cursor_changed {
                        damage::DamageCause::CursorShapeChanged
                    } else {
                        damage::DamageCause::CursorMoved
                    };

                    if !old_rect.is_empty() {
                        damage.add_rect_with_cause(old_rect, cause, None);
                    }
                    if !new_rect.is_empty() {
                        damage.add_rect_with_cause(new_rect, cause, None);
                    }
                    prev_cursor_x = cursor.x;
                    prev_cursor_y = cursor.y;
                    prev_cursor_gen = snapshot.gen;
                }
            } else {
                // Asset known but snapshot not ready: use crosshair damage
                if cursor_moved {
                    let old_rect =
                        crate::geometry::Rect::new(prev_cursor_x - 8, prev_cursor_y - 8, 17, 17)
                            .expand(2)
                            .clip(bounds);
                    let new_rect = crate::geometry::Rect::new(cursor.x - 8, cursor.y - 8, 17, 17)
                        .expand(2)
                        .clip(bounds);
                    if !old_rect.is_empty() {
                        damage.add_rect_with_cause(
                            old_rect,
                            damage::DamageCause::CursorMoved,
                            None,
                        );
                    }
                    if !new_rect.is_empty() {
                        damage.add_rect_with_cause(
                            new_rect,
                            damage::DamageCause::CursorMoved,
                            None,
                        );
                    }
                    prev_cursor_x = cursor.x;
                    prev_cursor_y = cursor.y;
                }
            }
        } else {
            // No asset: use crosshair damage
            if cursor_moved {
                let old_rect =
                    crate::geometry::Rect::new(prev_cursor_x - 8, prev_cursor_y - 8, 17, 17)
                        .expand(2)
                        .clip(bounds);
                let new_rect = crate::geometry::Rect::new(cursor.x - 8, cursor.y - 8, 17, 17)
                    .expand(2)
                    .clip(bounds);
                if !old_rect.is_empty() {
                    damage.add_rect_with_cause(old_rect, damage::DamageCause::CursorMoved, None);
                }
                if !new_rect.is_empty() {
                    damage.add_rect_with_cause(new_rect, damage::DamageCause::CursorMoved, None);
                }
                prev_cursor_x = cursor.x;
                prev_cursor_y = cursor.y;
            }
        }

        // Track cursor damage rects added
        let cursor_damage_end = damage.rect_count();
        let cursor_damage_added = cursor_damage_end.saturating_sub(cursor_damage_start);
        if cursor_damage_added > 0 {
            cursor_metrics.record_cursor_damage_rects(cursor_damage_added as u64);
        }

        if let Some(full_cause) = should_force_full_damage(&invalidation_causes) {
            if debug_flags.show_damage_stats {
                stem::info!("[bloom] Full damage forced by: {:?}", invalidation_causes);
            }
            // Use the first force-full invalidation cause for damage tracking.
            let cause = damage::DamageCause::from_invalidation(full_cause);
            damage = damage::Damage::full_with_cause(bounds, cause, None);
        }

        // Apply debug flags for deterministic damage modes
        if debug_flags.force_full_damage {
            damage = damage::Damage::full_with_cause(bounds, damage::DamageCause::ForceFull, None);
        }

        if debug_flags.disable_damage_tracking {
            // Disable damage tracking means always render full frame
            damage = damage::Damage::full_with_cause(bounds, damage::DamageCause::ForceFull, None);
        }

        // Replay last frame damage if enabled
        if debug_flags.replay_last_frame_damage {
            if let Some(last_damage) = overlay_state.last_frame_damage {
                damage = damage::Damage::empty(bounds);
                damage.add_rect_with_cause(last_damage, damage::DamageCause::ForceFull, None);
            }
        } else {
            // Store current damage for potential replay next frame
            if !damage.is_empty() {
                overlay_state.last_frame_damage = Some(damage.bounding_box());
            } else {
                // Clear stale damage when frame is empty
                overlay_state.last_frame_damage = None;
            }
        }

        {
            crate::trace_span!("bloom.loop.damage");
            // damage calculation trace (already mostly done but wrapping ensures consistency)
        }

        // Detect cursor-only frames: damage is only from cursor movement or shape changes
        let is_cursor_only_frame = if !damage.is_empty() && !damage.is_full {
            let mut all_cursor_damage = true;
            for record in damage.iter_records() {
                // Accept both cursor movement and cursor shape changes
                if record.cause != damage::DamageCause::CursorMoved
                    && record.cause != damage::DamageCause::CursorShapeChanged
                {
                    all_cursor_damage = false;
                    break;
                }
            }
            all_cursor_damage
        } else {
            false
        };

        if is_cursor_only_frame {
            cursor_metrics.record_cursor_only_frame();
        }

        // Periodic cursor metrics logging
        let frame_num = loop_ctrl.frame_number();
        cursor_metrics.maybe_log(frame_num);

        if damage.is_empty() {
            presenter.pump();
            loop_ctrl.sleep_until_input((bristle_evt_handle != 0).then_some(bristle_evt_handle));
            continue;
        }

        // Render
        let token = presenter.acquire_frame(
            crate::frame::FrameSpec::new(final_width, final_height, screen_format),
            ASSETS.current_generation(),
        );
        let mut builder = FrameBuilder::new(token);

        if damage.is_full {
            builder.mark_full_damage();
        } else {
            for rect in damage.iter() {
                builder.add_damage(rect);
            }
        }

        builder.prepare_present_damage();
        let snapshot = builder.present_damage();
        let strategy = evaluate_present_strategy(presenter.negotiation_info(), snapshot);

        // Use consolidated rects for BOTH rasterization and GPU present
        // This ensures we draw exactly what we tell the GPU to transfer
        // IMPORTANT: Don't use add_rect() as it would re-consolidate the already-consolidated rects
        let damage = if snapshot.is_full() {
            damage::Damage::full(bounds)
        } else {
            damage::Damage::from_rects(bounds, snapshot.rects())
        };

        overlay_state.update(
            snapshot.rects(),
            snapshot.raw_rects(),
            strategy.mode,
            &invalidation_causes,
            snapshot.overflowed(),
        );

        // Record damage in journal (debug builds only)
        #[cfg(debug_assertions)]
        {
            overlay_state.journal.record_frame(&damage);
        }

        append_damage_overlay(
            &mut list,
            &overlay_state,
            &debug_flags,
            Some(&damage),
            screen_w,
            screen_h,
        );

        // Cursor-only fast path: restore old cursor underlay and skip scene composition.
        // Only valid when the current buffer is stable (age=1) and we captured underlay for
        // this exact bytespace on the previous frame.
        let cursor_underlay = cursor_underlays.entry(current_bs_id).or_default();
        let cursor_only_fast_path =
            is_cursor_only_frame && cursor_underlay.valid && list.commands_ref().is_empty();

        if !cursor_only_fast_path {
            // Execute drawlist (wallpaper + UI) - cursor is NOT in the DrawList
            let rects: alloc::vec::Vec<_> = damage.iter().collect();

            // Reactive wallpaper selection
            let mut wallpaper = ASSETS.get_wallpaper();
            if let Some(focused) = focused_window {
                let mut wp_asset_id = prop_get(focused, "ui.wallpaper").unwrap_or(0);

                // If window doesn't have it, check its task parent
                if wp_asset_id == 0 {
                    if let Ok(task_id) = prop_get(focused, abi::schema::rels::RUNS_ON) {
                        wp_asset_id =
                            prop_get(ThingId::from_u64(task_id), "ui.wallpaper").unwrap_or(0);
                    }
                }

                if wp_asset_id != 0 {
                    if let Some(wp) = ASSETS.get_image_by_id(ThingId::from_u64(wp_asset_id)) {
                        wallpaper = Some(wp);
                    }
                }
            }

            paint_pipeline.compose(
                &scene,
                &mut surface,
                &rects,
                wallpaper.as_ref(),
                crate::geometry::Color::from_u32(0xFF101018),
            );

            crate::trace_span!("bloom.loop.raster");
            raster::execute_with_damage(&mut surface, &list, &damage, false);

            if let Some(ref commit) = latest_wayland_commit {
                if let Ok(ptr) = stem::thing::sys::bytespace_map(stem::thing::ThingId::from_u64(commit.bs_id)) {
                    let w = commit.width.min(screen_w as u32) as i32;
                    let h = commit.height.min(screen_h as u32) as i32;
                    let wb = commit.stride as usize;
                    unsafe {
                        let src = ptr as *const u8;
                        let dst = surface.ptr;
                        for y in 0..h {
                            core::ptr::copy_nonoverlapping(
                                src.add((y as usize) * wb),
                                dst.add((y as usize) * surface.stride_bytes),
                                (w as usize) * 4,
                            );
                        }
                    }
                    let _ = stem::thing::sys::bytespace_unmap(stem::thing::ThingId::from_u64(commit.bs_id), ptr);
                }
            }

            // GPU composition path (when enabled)
            // Uploads window textures and submits virgl BLIT commands
            #[cfg(feature = "gpu")]
            if composition_mode == CompositionMode::Gpu {
                if let Some(ref mut gc) = gpu_compositor {
                    // === TEXTURE UPLOAD PHASE ===
                    // Upload textures for windows that need it
                    if let PresenterImpl::Driver(ref mut driver_presenter) = presenter {
                        // Collect windows that need texture upload
                        let windows: alloc::vec::Vec<_> = paint_pipeline
                            .windows_for_gpu_upload()
                            .map(|(id, rect, paint_gen, geom_gen)| {
                                (
                                    id.to_u64_lossy(),
                                    rect.width().max(0) as u32,
                                    rect.height().max(0) as u32,
                                    paint_gen + geom_gen,
                                )
                            })
                            .collect();

                        for (window_id, width, height, generation) in windows {
                            // Check if texture needs creation or update
                            if gc.texture_needs_update(window_id, generation) {
                                // Look up cached raster from render_state
                                // For now, create/update the texture registration
                                // (full pixel upload requires render_state access)

                                // Check if texture exists
                                let resource_id = if gc.get_texture(window_id).is_none() {
                                    // Create new texture via presenter
                                    // client_id = window_id, format = 2 (BGRA)
                                    match driver_presenter
                                        .send_create_texture_3d(window_id, width, height, 2)
                                    {
                                        Some(res_id) => {
                                            gc.register_texture(
                                                window_id, res_id, width, height, generation,
                                            );
                                            Some(res_id)
                                        }
                                        None => None,
                                    }
                                } else {
                                    // Already have texture, just update generation
                                    gc.get_texture(window_id).map(|t| t.resource_id)
                                };

                                // If we have a valid resource, we could upload pixels here
                                // This requires access to the rasterized window content from render_state
                                // For now, the texture is registered for BLIT commands
                                if let Some(res_id) = resource_id {
                                    gc.register_texture(
                                        window_id, res_id, width, height, generation,
                                    );
                                    crate::trace_counter!("bloom.gpu.texture_upload", 1);
                                }
                            }
                        }
                    }

                    // === QUAD SUBMISSION PHASE ===
                    let quads = paint_pipeline.build_gpu_quads();
                    if !quads.is_empty() {
                        // Extract ctx_id before mutable borrow
                        let ctx_id = gc.context_id();

                        // Build BLIT commands for each quad
                        let cmd_bytes = gc.render_quads(&quads);

                        // Submit 3D commands via driver presenter
                        if !cmd_bytes.is_empty() {
                            // Copy bytes to avoid lifetime issue with gc borrow
                            let cmd_vec: alloc::vec::Vec<u8> = cmd_bytes.to_vec();
                            if let PresenterImpl::Driver(ref mut driver_presenter) = presenter {
                                driver_presenter.send_submit_3d(ctx_id, &cmd_vec);
                            }
                        }

                        crate::trace_counter!("bloom.gpu.quads", quads.len() as u64);
                    }
                }
            }
        } else {
            restore_cursor_underlay(&mut surface, cursor_underlay);
        }

        // ============================================================================
        // CURSOR OVERLAY - "Butter Smooth" Late-Latched Composition
        // ============================================================================
        //
        // The cursor is rendered AFTER all window composition and rasterization.
        // This ensures cursor updates are decoupled from heavy scene redraws:
        //
        // 1. Cursor rasterization is cached (only happens on shape/asset changes)
        // 2. Cursor movement only costs 2 tiny damage rects (old + new position)
        // 3. Cursor is blitted from cache, no SVG/text rasterization involved
        // 4. Cursor damage never triggers full-frame invalidation
        //
        // This architecture ensures the cursor feels "butter smooth" even when
        // the UI is busy with expensive repaints.
        // ============================================================================

        let cursor_bounds = crate::geometry::Rect::full(surface.width(), surface.height());
        let mut cursor_rect =
            crate::geometry::Rect::new(cursor.x - 8, cursor.y - 8, 17, 17).clip(cursor_bounds);

        // Cursor overlay: blend cached snapshot or draw fallback
        let cursor_drawn = if let Some(asset) = ASSETS.get_cursor() {
            let (snapshot_opt, _rasterized) = cursor_rasterizer.get_snapshot(&asset);
            // Note: rasterization tracking already done in damage section above

            if let Some(snapshot) = snapshot_opt {
                let cx = cursor.x - snapshot.hotspot_x;
                let cy = cursor.y - snapshot.hotspot_y;
                cursor_rect = crate::geometry::Rect::new(
                    cx,
                    cy,
                    snapshot.image.width as i32,
                    snapshot.image.height as i32,
                )
                .clip(cursor_bounds);
                capture_cursor_underlay(&surface, current_bs_id, cursor_rect, cursor_underlay);
                raster::blit_cursor_overlay(&mut surface, &snapshot.image, cx, cy);
                true
            } else {
                false
            }
        } else {
            false
        };

        if !cursor_drawn {
            capture_cursor_underlay(&surface, current_bs_id, cursor_rect, cursor_underlay);
            raster::draw_crosshair(&mut surface, cursor.x, cursor.y, 0xFFFFFFFF);
        }

        {
            crate::trace_span!("bloom.loop.present");
            let token = builder.finish();
            presenter.present_frame(token);
            presenter.pump();

            // Acquire NEXT buffer for the next frame
            if let PresenterImpl::Driver(_) = presenter {
                let acquire_start_ns = stem::monotonic_ns();
                let (next_bs_id, next_w, next_h, next_s, next_f, next_age) =
                    presenter.acquire_buffer();
                let acquire_ns = stem::monotonic_ns().saturating_sub(acquire_start_ns);
                if acquire_ns > 50_000_000 {
                    stem::warn!(
                        "[bloom] acquire_buffer took {:.3}ms",
                        acquire_ns as f64 / 1_000_000.0
                    );
                }

                // Use cached pointer or map if new
                let next_ptr = if let Some(&ptr) = buffer_cache.get(&next_bs_id) {
                    ptr
                } else {
                    let ptr = stem::thing::sys::bytespace_map(next_bs_id).unwrap();
                    buffer_cache.insert(next_bs_id, ptr);
                    ptr
                };
                let next_size = (next_h * next_s) as usize;

                // Update surface to point to the new buffer
                unsafe {
                    surface.update_buffer(next_ptr, next_size, next_w, next_h, next_s);
                }

                final_bs_id = next_bs_id;
                final_width = next_w;
                final_height = next_h;
                final_stride = next_s;
                screen_w = next_w as i32;
                screen_h = next_h as i32;
                screen_format = next_f;
                current_age = next_age;
            }

            if !first_frame_rendered {
                stem::info!("[CONTRACT] [bloom] First frame rendered");
                first_frame_rendered = true;
            }
        }
        loop_ctrl.sleep_until_input((bristle_evt_handle != 0).then_some(bristle_evt_handle));
    }
}

fn append_damage_overlay(
    list: &mut drawlist::DrawList,
    overlay_state: &DamageOverlayState,
    flags: &DebugFlags,
    damage_opt: Option<&damage::Damage>,
    screen_w: i32,
    _screen_h: i32,
) {
    if !flags.show_damage_rects
        && !flags.show_raw_damage_rects
        && !flags.show_damage_stats
        && !flags.show_damage_causes
    {
        return;
    }

    const MERGED_COLOR: u32 = 0xFFFF00FF;
    const RAW_COLOR: u32 = 0xFF00FFFF;
    const TEXT_BOX_COLOR: u32 = 0x88000000;
    const TEXT_COLOR: u32 = 0xFFFFFFFF;

    // Show damage rects with cause-based coloring if enabled
    if flags.show_damage_causes {
        if let Some(damage) = damage_opt {
            for record in damage.iter_records() {
                let color = record.cause.debug_color();
                draw_rect_outline(list, record.rect, color);
            }
        }
    } else if flags.show_damage_rects {
        for rect in overlay_state.present() {
            draw_rect_outline(list, *rect, MERGED_COLOR);
        }
    }

    if flags.show_raw_damage_rects {
        for rect in overlay_state.raw() {
            draw_rect_outline(list, *rect, RAW_COLOR);
        }
    }

    if flags.show_damage_stats {
        let suffix = if overlay_state.overflowed {
            " (overflow)"
        } else {
            ""
        };
        let text = alloc::format!(
            "DAMAGE: merged={} raw={} reason={:?}{}",
            overlay_state.present().len(),
            overlay_state.raw().len(),
            overlay_state.reasons,
            suffix
        );
        let x = 8;
        let y = 8;
        let width = 360;
        let height = 30;
        list.rect(
            x - 4,
            y - 4,
            width,
            height,
            crate::geometry::Color::from_u32(TEXT_BOX_COLOR),
        );
        list.text(
            &text,
            None,
            x,
            y,
            14.0,
            crate::geometry::Color::from_u32(TEXT_COLOR),
        );
    }
}

fn draw_rect_outline(list: &mut drawlist::DrawList, rect: crate::geometry::Rect, color: u32) {
    if rect.width() <= 0 || rect.height() <= 0 {
        return;
    }
    let c = crate::geometry::Color::from_u32(color);
    list.rect(rect.x(), rect.y(), rect.width(), 1, c);
    list.rect(rect.x(), rect.y() + rect.height() - 1, rect.width(), 1, c);
    list.rect(rect.x(), rect.y(), 1, rect.height(), c);
    list.rect(rect.x() + rect.width() - 1, rect.y(), 1, rect.height(), c);
}
