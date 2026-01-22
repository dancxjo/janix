//! Model graph helpers for Blossom.
//!
//! Blossom scans the UI graph for view nodes and extracts the minimal model
//! needed for snapshot composition.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use abi::schema::{kinds, keys, rels};
use stem::thing::sys::{bytespace_info, bytespace_read, get_edges, get_kind, intern, prop_get};
use stem::thing::ThingId;

#[derive(Clone, Debug)]
pub struct KindIds {
    pub window: u32,
    pub viewport: u32,
    pub tile: u32,
    pub text_run: u32,
    pub chrome: u32,
    pub has_child: u32,
}

impl KindIds {
    pub fn load() -> Self {
        Self {
            window: intern(kinds::UI_WINDOW).unwrap_or(0),
            viewport: intern(kinds::UI_VIEWPORT).unwrap_or(0),
            tile: intern(kinds::UI_TILE).unwrap_or(0),
            text_run: intern(kinds::UI_TEXT_RUN).unwrap_or(0),
            chrome: intern(kinds::UI_CHROME).unwrap_or(0),
            has_child: intern(rels::HAS_CHILD).unwrap_or(0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WindowModel {
    pub id: ThingId,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub inset_right: u32,
    pub inset_bottom: u32,
    pub z_index: i32,
    pub bg_color: u32,
    pub viewport: Option<ViewportModel>,
    pub text_runs: Vec<TextRunModel>,
}

#[derive(Clone, Debug)]
pub struct ViewportModel {
    pub id: ThingId,
    pub width: u32,
    pub height: u32,
    pub scroll_x: i32,
    pub scroll_y: i32,
    pub clip: bool,
    pub tiles: Vec<TileModel>,
    pub text_runs: Vec<TextRunModel>,
}

#[derive(Clone, Debug)]
pub struct TileModel {
    pub id: ThingId,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub asset: ThingId,
}

#[derive(Clone, Debug)]
pub struct TextRunModel {
    pub id: ThingId,
    pub x: i32,
    pub y: i32,
    pub text: String,
    pub color: u32,
    pub size_px: u32,
    pub center_x: bool,
    pub center_y: bool,
}

pub fn read_windows(kind_ids: &KindIds, windows: &[ThingId]) -> Vec<WindowModel> {
    let mut out = Vec::new();
    for &id in windows {
        if let Some(model) = read_window(kind_ids, id) {
            out.push(model);
        }
    }
    out
}

fn read_window(kind_ids: &KindIds, id: ThingId) -> Option<WindowModel> {
    let kind = get_kind(id).ok()?;
    if kind.0 as u32 != kind_ids.window {
        return None;
    }
    let width = prop_get(id, keys::UI_WIDTH).unwrap_or(0) as u32;
    let height = prop_get(id, keys::UI_HEIGHT).unwrap_or(0) as u32;
    let x = prop_get(id, keys::UI_X).unwrap_or(0) as i32;
    let y = prop_get(id, keys::UI_Y).unwrap_or(0) as i32;
    let inset_right = prop_get(id, keys::UI_INSET_RIGHT).unwrap_or(0) as u32;
    let inset_bottom = prop_get(id, keys::UI_INSET_BOTTOM).unwrap_or(0) as u32;
    let z_index = prop_get(id, keys::UI_Z_INDEX).unwrap_or(0) as i32;
    let bg_color = prop_get(id, keys::UI_BG_COLOR).unwrap_or(0xFF1F1F1F) as u32;

    let children = fetch_children(id, kind_ids.has_child);
    let mut viewport = None;
    let mut text_runs = Vec::new();
    for child in children {
        if let Some(kind) = get_kind(child).ok() {
            let sym = kind.0 as u32;
            if sym == kind_ids.viewport {
                viewport = read_viewport(kind_ids, child);
            } else if sym == kind_ids.text_run {
                if let Some(run) = read_text_run(child) {
                    text_runs.push(run);
                }
            }
        }
    }

    Some(WindowModel {
        id,
        x,
        y,
        width,
        height,
        inset_right,
        inset_bottom,
        z_index,
        bg_color,
        viewport,
        text_runs,
    })
}

fn read_viewport(kind_ids: &KindIds, id: ThingId) -> Option<ViewportModel> {
    let width = prop_get(id, keys::UI_WIDTH).unwrap_or(0) as u32;
    let height = prop_get(id, keys::UI_HEIGHT).unwrap_or(0) as u32;
    let scroll_x = prop_get(id, keys::UI_SCROLL_X).unwrap_or(0) as i32;
    let scroll_y = prop_get(id, keys::UI_SCROLL_Y).unwrap_or(0) as i32;
    let clip = prop_get(id, keys::UI_CLIP).unwrap_or(0) != 0;
    let children = fetch_children(id, kind_ids.has_child);
    let mut tiles = Vec::new();
    let mut text_runs = Vec::new();
    for child in children {
        if let Some(kind) = get_kind(child).ok() {
            let sym = kind.0 as u32;
            if sym == kind_ids.tile {
                if let Some(tile) = read_tile(child) {
                    tiles.push(tile);
                }
            } else if sym == kind_ids.text_run {
                if let Some(run) = read_text_run(child) {
                    text_runs.push(run);
                }
            }
        }
    }
    Some(ViewportModel {
        id,
        width,
        height,
        scroll_x,
        scroll_y,
        clip,
        tiles,
        text_runs,
    })
}

fn read_tile(id: ThingId) -> Option<TileModel> {
    let width = prop_get(id, keys::UI_WIDTH).unwrap_or(0) as u32;
    let height = prop_get(id, keys::UI_HEIGHT).unwrap_or(0) as u32;
    let x = prop_get(id, keys::UI_X).unwrap_or(0) as i32;
    let y = prop_get(id, keys::UI_Y).unwrap_or(0) as i32;
    let asset = ThingId::from_u64(prop_get(id, keys::UI_TILE_ASSET).unwrap_or(0));
    Some(TileModel {
        id,
        x,
        y,
        width,
        height,
        asset,
    })
}

fn read_text_run(id: ThingId) -> Option<TextRunModel> {
    let x = prop_get(id, keys::UI_X).unwrap_or(0) as i32;
    let y = prop_get(id, keys::UI_Y).unwrap_or(0) as i32;
    let color = prop_get(id, keys::UI_FG_COLOR).unwrap_or(0xFFFFFFFF) as u32;
    let size_px = prop_get(id, keys::UI_FONT_SIZE).unwrap_or(16) as u32;
    let center_x = prop_get(id, keys::UI_CENTER_X).unwrap_or(0) != 0;
    let center_y = prop_get(id, keys::UI_CENTER_Y).unwrap_or(0) != 0;
    let text = read_string(prop_get(id, keys::UI_TEXT).unwrap_or(0))
        .unwrap_or_else(|| String::from(""));

    Some(TextRunModel {
        id,
        x,
        y,
        text,
        color,
        size_px,
        center_x,
        center_y,
    })
}

fn fetch_children(id: ThingId, has_child: u32) -> Vec<ThingId> {
    let mut edges_buf = [abi::types::Edge::default(); 64];
    let mut children = Vec::new();
    if let Ok(count) = get_edges(id, &mut edges_buf) {
        for edge in &edges_buf[..count] {
            if edge.predicate.to_u64_lossy() == has_child as u64 {
                children.push(edge.to);
            }
        }
    }
    children
}

fn read_string(val: u64) -> Option<String> {
    let bs_id = ThingId::from_u64(val);
    if bs_id.to_u64_lossy() == 0 {
        return None;
    }
    let size = bytespace_info(bs_id).ok()?;
    let mut buf = alloc::vec![0u8; size];
    let len = bytespace_read(bs_id, 0, &mut buf).ok()?;
    String::from_utf8(buf[..len].to_vec()).ok()
}
