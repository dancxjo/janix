#![allow(dead_code)]

use alloc::string::ToString;
use alloc::vec::Vec;
use core::cmp::max;

use abi::ThingId;
use thing_models::PropValue;
use thing_models::graph_kinds;
use thing_os::Window;
use thing_os::prelude::*;
use thing_os::update_props;

use crate::config::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};

#[derive(Debug, Clone, PartialEq)]
pub struct StackedWindow {
    pub id: ThingId,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub z_index: i64,
    pub active: bool,
}

impl StackedWindow {
    pub fn from_window(window: &Window) -> Self {
        Self {
            id: window.id,
            x: window.x,
            y: window.y,
            width: window.width.max(MIN_WINDOW_WIDTH),
            height: window.height.max(MIN_WINDOW_HEIGHT),
            z_index: window.z_index as i64,
            active: window.active,
        }
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        let w = self.width.max(MIN_WINDOW_WIDTH);
        let h = self.height.max(MIN_WINDOW_HEIGHT);
        x >= self.x && x < self.x + w && y >= self.y && y < self.y + h
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPolicy {
    Free,
    Tiled,
}

impl LayoutPolicy {
    pub fn from_i64(value: i64) -> Self {
        match value {
            1 => LayoutPolicy::Tiled,
            _ => LayoutPolicy::Free,
        }
    }

    pub fn to_i64(self) -> i64 {
        match self {
            LayoutPolicy::Free => 0,
            LayoutPolicy::Tiled => 1,
        }
    }
}

impl Default for LayoutPolicy {
    fn default() -> Self {
        LayoutPolicy::Free
    }
}

pub fn stack_and_clamp(windows: &[Window], fb_width: i32, fb_height: i32) -> Vec<StackedWindow> {
    let mut stacked: Vec<StackedWindow> = windows.iter().map(StackedWindow::from_window).collect();
    stacked.sort_by_key(|w| w.z_index);
    for win in stacked.iter_mut() {
        clamp_window(win, fb_width, fb_height);
    }
    stacked
}

pub fn auto_tile(windows: &[Window], fb_width: i32, fb_height: i32) -> Vec<StackedWindow> {
    let count = windows.len();
    if count == 0 {
        return Vec::new();
    }

    let cols = ceil_sqrt(count).max(1);
    let rows = ((count as i32) + cols - 1) / cols;

    let tile_w = max(fb_width / cols, MIN_WINDOW_WIDTH);
    let tile_h = max(fb_height / rows, MIN_WINDOW_HEIGHT);

    let mut stacked: Vec<StackedWindow> = windows.iter().map(StackedWindow::from_window).collect();
    stacked.sort_by_key(|w| w.z_index);

    for (idx, win) in stacked.iter_mut().enumerate() {
        let i = idx as i32;
        let col = i % cols;
        let row = i / cols;

        win.x = col * tile_w;
        win.y = row * tile_h;
        win.width = tile_w;
        win.height = tile_h;

        clamp_window(win, fb_width, fb_height);
    }

    stacked
}

pub fn persist_stack(stack: &[StackedWindow]) {
    for win in stack {
        let props = [
            (
                graph_kinds::PROP_WINDOW_X.to_string(),
                PropValue::I64(win.x as i64),
            ),
            (
                graph_kinds::PROP_WINDOW_Y.to_string(),
                PropValue::I64(win.y as i64),
            ),
            (
                graph_kinds::PROP_WINDOW_WIDTH.to_string(),
                PropValue::I64(win.width as i64),
            ),
            (
                graph_kinds::PROP_WINDOW_HEIGHT.to_string(),
                PropValue::I64(win.height as i64),
            ),
        ];
        let _ = update_props(win.id, &props);
    }
}

pub fn hit_test<'a>(stacked: &'a [StackedWindow], x: i32, y: i32) -> Option<&'a StackedWindow> {
    let mut best: Option<&StackedWindow> = None;
    for window in stacked {
        if window.contains_point(x, y) {
            match best {
                Some(current) if current.z_index > window.z_index => {}
                _ => best = Some(window),
            }
        }
    }
    best
}

fn clamp_window(w: &mut StackedWindow, fb_width: i32, fb_height: i32) {
    let max_w = max(fb_width, MIN_WINDOW_WIDTH);
    let max_h = max(fb_height, MIN_WINDOW_HEIGHT);

    if w.width > max_w {
        w.width = max_w;
    }
    if w.height > max_h {
        w.height = max_h;
    }

    let max_x = max(fb_width - w.width, 0);
    let max_y = max(fb_height - w.height, 0);

    w.x = w.x.clamp(0, max_x);
    w.y = w.y.clamp(0, max_y);
}

fn ceil_sqrt(n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut cols = 1i32;
    let target = n as i32;
    while cols * cols < target {
        cols += 1;
    }
    cols
}
pub fn apply_layout(
    policy: LayoutPolicy,
    windows: &[Window],
    fb_width: i32,
    fb_height: i32,
) -> Vec<StackedWindow> {
    match policy {
        LayoutPolicy::Free => stack_and_clamp(windows, fb_width, fb_height),
        LayoutPolicy::Tiled => auto_tile(windows, fb_width, fb_height),
    }
}
