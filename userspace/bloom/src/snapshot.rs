//! Snapshot-only window compositor.
//!
//! Bloom reads presented window snapshots and composites them onto the
//! framebuffer. It never interprets model keys beyond window geometry and
//! snapshot metadata.

use abi::schema::{keys, kinds, snapshot_mode};
use alloc::vec::Vec;
use stem::thing::sys::{bytespace_map, bytespace_unmap, find, prop_get};
use stem::thing::{HandleId, ThingId};

use crate::surface::Surface;
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotInvalidation {
    GeometryChanged,
    ContentChanged,
    FontChanged,
    ThemeChanged,
    Forced,
}

impl fmt::Display for SnapshotInvalidation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GeometryChanged => write!(f, "GeometryChanged"),
            Self::ContentChanged => write!(f, "ContentChanged"),
            Self::FontChanged => write!(f, "FontChanged"),
            Self::ThemeChanged => write!(f, "ThemeChanged"),
            Self::Forced => write!(f, "Forced"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WindowSnapshot {
    pub id: ThingId,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub z_index: i32,
    pub snapshot: Option<SnapshotMeta>,
}

#[derive(Clone, Debug)]
pub struct SnapshotMeta {
    pub bytespace: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub epoch: u64,
    /// Snapshot mode (WRITE_ONCE or MUTABLE_DIRTY)
    pub mode: u64,
    /// Whether the snapshot bytespace is frozen (immutable)
    pub frozen: bool,
}

pub fn collect_windows(screen_w: i32, screen_h: i32) -> Vec<WindowSnapshot> {
    let mut windows = [ThingId::default(); 64];
    let count = find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);
    let mut out = Vec::new();
    for &id in &windows[..count] {
        if let Some(win) = read_window(id, screen_w, screen_h) {
            out.push(win);
        }
    }
    out.sort_by_key(|win| win.z_index);
    out
}

fn read_window(id: ThingId, screen_w: i32, screen_h: i32) -> Option<WindowSnapshot> {
    let width = prop_get(id, keys::UI_WIDTH).unwrap_or(0) as u32;
    let height = prop_get(id, keys::UI_HEIGHT).unwrap_or(0) as u32;
    let mut x = prop_get(id, keys::UI_X).unwrap_or(0) as i32;
    let mut y = prop_get(id, keys::UI_Y).unwrap_or(0) as i32;
    let inset_right = prop_get(id, keys::UI_INSET_RIGHT).unwrap_or(0) as i32;
    let inset_bottom = prop_get(id, keys::UI_INSET_BOTTOM).unwrap_or(0) as i32;
    let z_index = prop_get(id, keys::UI_Z_INDEX).unwrap_or(0) as i32;

    if inset_right > 0 {
        x = screen_w - inset_right - width as i32;
    }
    if inset_bottom > 0 {
        y = screen_h - inset_bottom - height as i32;
    }

    let bytespace = ThingId::from_u64(prop_get(id, keys::UI_SNAPSHOT_BYTESPACE).unwrap_or(0));
    let epoch = prop_get(id, keys::UI_PRESENT_EPOCH).unwrap_or(0);
    let mode = prop_get(id, keys::UI_SNAPSHOT_MODE).unwrap_or(snapshot_mode::WRITE_ONCE);
    let frozen = prop_get(id, keys::UI_SNAPSHOT_FROZEN).unwrap_or(0) != 0;
    let dirty = prop_get(id, keys::UI_SNAPSHOT_DIRTY).unwrap_or(0);

    let snapshot = if bytespace.to_u64_lossy() != 0 && epoch > 0 {
        // Validate mode-specific invariants
        if mode == snapshot_mode::MUTABLE_DIRTY {
            // MUTABLE_DIRTY: skip if dirty flag is set
            if dirty != 0 {
                // Snapshot is being updated, skip this frame
                return Some(WindowSnapshot {
                    id,
                    x,
                    y,
                    width,
                    height,
                    z_index,
                    snapshot: None, // Skip dirty snapshot
                });
            }
        }
        // Note: For WRITE_ONCE mode, we trust the contract.
        // In debug builds, the kernel will assert on mutation attempts.

        Some(SnapshotMeta {
            bytespace,
            width: prop_get(id, keys::UI_SNAPSHOT_WIDTH).unwrap_or(width as u64) as u32,
            height: prop_get(id, keys::UI_SNAPSHOT_HEIGHT).unwrap_or(height as u64) as u32,
            stride: prop_get(id, keys::UI_SNAPSHOT_STRIDE).unwrap_or((width * 4) as u64) as u32,
            format: prop_get(id, keys::UI_SNAPSHOT_FORMAT).unwrap_or(0) as u32,
            epoch,
            mode,
            frozen,
        })
    } else {
        None
    };

    Some(WindowSnapshot {
        id,
        x,
        y,
        width,
        height,
        z_index,
        snapshot,
    })
}

pub fn composite_windows(surface: &mut Surface, windows: &[WindowSnapshot]) {
    for win in windows {
        match &win.snapshot {
            Some(snapshot) => composite_snapshot(surface, win, snapshot),
            None => draw_missing_snapshot(surface, win),
        }
    }
}

fn composite_snapshot(surface: &mut Surface, win: &WindowSnapshot, snapshot: &SnapshotMeta) {
    let Ok(ptr) = bytespace_map(snapshot.bytespace) else {
        return;
    };
    let src = unsafe {
        core::slice::from_raw_parts(
            ptr as *const u8,
            (snapshot.stride * snapshot.height) as usize,
        )
    };
    blit_rgba(
        surface,
        src,
        snapshot.stride as usize,
        snapshot.width as i32,
        snapshot.height as i32,
        win.x,
        win.y,
    );
    let _ = bytespace_unmap(snapshot.bytespace, ptr);
}

fn draw_missing_snapshot(surface: &mut Surface, win: &WindowSnapshot) {
    fill_rect(
        surface,
        win.x,
        win.y,
        win.width as i32,
        win.height as i32,
        0xFF550000,
    );
    // Outline to make it obvious.
    fill_rect(surface, win.x, win.y, win.width as i32, 2, 0xFFFF0000);
    fill_rect(
        surface,
        win.x,
        win.y + win.height as i32 - 2,
        win.width as i32,
        2,
        0xFFFF0000,
    );
    fill_rect(surface, win.x, win.y, 2, win.height as i32, 0xFFFF0000);
    fill_rect(
        surface,
        win.x + win.width as i32 - 2,
        win.y,
        2,
        win.height as i32,
        0xFFFF0000,
    );
}

fn blit_rgba(
    surface: &mut Surface,
    src: &[u8],
    src_stride: usize,
    width: i32,
    height: i32,
    dst_x: i32,
    dst_y: i32,
) {
    for y in 0..height {
        for x in 0..width {
            let sx = x as usize * 4;
            let sy = y as usize * src_stride;
            let idx = sy + sx;
            if idx + 4 > src.len() {
                continue;
            }
            let r = src[idx + 0];
            let g = src[idx + 1];
            let b = src[idx + 2];
            let a = src[idx + 3];
            let px = (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32);
            blend_pixel(surface, dst_x + x, dst_y + y, px);
        }
    }
}

fn blend_pixel(surface: &mut Surface, x: i32, y: i32, rgba: u32) {
    let a = (rgba >> 24) & 0xFF;
    if a == 0 {
        return;
    }
    if a == 0xFF {
        surface.put_px(x, y, rgba);
        return;
    }
    let dst = surface.get_px(x, y);
    let sr = (rgba >> 16) & 0xFF;
    let sg = (rgba >> 8) & 0xFF;
    let sb = rgba & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;
    let inv = 255 - a;
    let r = (sr * a + dr * inv) / 255;
    let g = (sg * a + dg * inv) / 255;
    let b = (sb * a + db * inv) / 255;
    surface.put_px(x, y, 0xFF00_0000 | (r << 16) | (g << 8) | b);
}

fn fill_rect(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, rgba: u32) {
    for dy in 0..h {
        for dx in 0..w {
            surface.put_px(x + dx, y + dy, rgba);
        }
    }
}
