#![allow(dead_code)]

use alloc::vec::Vec;
use core::cmp::max;

use thing_os::prelude::*;
use thing_os::{Window, graph_kinds};

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

pub fn persist_stack<S: Sys>(sys: &mut S, stack: &[StackedWindow]) {
    for win in stack {
        let props = [
            (graph_kinds::PROP_WINDOW_X, PropValue::I64(win.x as i64)),
            (graph_kinds::PROP_WINDOW_Y, PropValue::I64(win.y as i64)),
            (
                graph_kinds::PROP_WINDOW_WIDTH,
                PropValue::I64(win.width as i64),
            ),
            (
                graph_kinds::PROP_WINDOW_HEIGHT,
                PropValue::I64(win.height as i64),
            ),
        ];
        let _ = update_props(sys, win.id, &props);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH, TITLE_BAR_HEIGHT};
    use crate::test_support::{MockSys, success};
    use abi::{KernelRequest, PropValue, ThingId, graph_kinds};

    fn window(id: u64, z_index: i32) -> Window {
        Window {
            id: ThingId(id),
            place_id: ThingId(0),
            x: 10,
            y: 10,
            width: 120,
            height: 90,
            z_index: z_index as i32,
            active: false,
            title: "".into(),
            draggable: true,
            resizable: true,
            closable: true,
            minimizable: true,
        }
    }

    #[test]
    fn stack_and_clamp_orders_and_clamps_windows() {
        let mut w1 = window(1, 2);
        w1.x = -5;
        w1.y = -10;
        w1.width = 300;
        w1.height = 20;

        let mut w2 = window(2, 1);
        w2.x = 150;
        w2.width = 100;

        let stacked = stack_and_clamp(&[w1.clone(), w2.clone()], 200, 120);
        assert_eq!(stacked.len(), 2);
        assert_eq!(stacked[0].id, w2.id, "sorted by z-index");
        assert_eq!(stacked[0].x, 100, "clamped inside framebuffer");
        assert_eq!(stacked[1].x, 0);
        assert_eq!(stacked[1].y, 0);
        assert_eq!(stacked[1].width, 200, "clamped to fb width");
        assert_eq!(
            stacked[1].height, MIN_WINDOW_HEIGHT,
            "minimum height enforced via StackedWindow::from_window"
        );
    }

    #[test]
    fn contains_point_respects_minimum_size() {
        let mut win = window(1, 0);
        win.width = 10;
        win.height = 10;
        let stacked = StackedWindow::from_window(&win);
        assert!(stacked.contains_point(stacked.x + MIN_WINDOW_WIDTH - 1, stacked.y));
        assert!(!stacked.contains_point(
            stacked.x + MIN_WINDOW_WIDTH,
            stacked.y + MIN_WINDOW_HEIGHT + TITLE_BAR_HEIGHT
        ));
    }

    #[test]
    fn hit_test_prefers_highest_z_index() {
        let mut a = StackedWindow::from_window(&window(1, 1));
        a.width = 200;
        a.height = 200;
        let mut b = StackedWindow::from_window(&window(2, 5));
        b.x = 50;
        b.y = 50;
        b.width = 50;
        b.height = 50;
        let stacked = vec![a, b.clone()];
        let hit = hit_test(&stacked, 60, 60).expect("expected hit");
        assert_eq!(hit.id, b.id, "top-most window should win");
    }

    #[test]
    fn auto_tile_assigns_grid_positions() {
        let mut a = window(1, 2);
        let mut b = window(2, 1);
        let mut c = window(3, 3);
        a.width = 200;
        b.width = 200;
        c.width = 200;

        let tiles = auto_tile(&[a, b, c], 200, 120);
        assert_eq!(tiles.len(), 3);
        assert_eq!((tiles[0].x, tiles[0].y), (0, 0));
        assert_eq!((tiles[1].x, tiles[1].y), (100, 0));
        assert_eq!((tiles[2].x, tiles[2].y), (0, 60));
        assert_eq!(tiles[0].width, 100);
        assert_eq!(tiles[0].height, MIN_WINDOW_HEIGHT);
    }

    #[test]
    fn persist_stack_writes_back_geometry() {
        let mut a = StackedWindow::from_window(&window(1, 1));
        a.x = 10;
        a.y = 20;
        a.width = 111;
        a.height = 222;
        let mut b = StackedWindow::from_window(&window(2, 2));
        b.x = 30;
        b.y = 40;
        b.width = 333;
        b.height = 444;

        let responses = vec![success(), success()];
        let mut sys = MockSys::with_responses(responses);
        persist_stack(&mut sys, &[a, b]);
        let requests = sys.drain_requests();
        assert_eq!(requests.len(), 2);

        for request in requests {
            match request {
                KernelRequest::ThingUpdate { id, props } => {
                    let x = props
                        .iter()
                        .find(|p| p.0 == graph_kinds::PROP_WINDOW_X)
                        .map(|(_, v)| v.clone());
                    let y = props
                        .iter()
                        .find(|p| p.0 == graph_kinds::PROP_WINDOW_Y)
                        .map(|(_, v)| v.clone());
                    assert!(x.is_some() && y.is_some());
                    if id == ThingId(1) {
                        assert_eq!(x, Some(PropValue::I64(10)));
                        assert_eq!(y, Some(PropValue::I64(20)));
                    } else if id == ThingId(2) {
                        assert_eq!(x, Some(PropValue::I64(30)));
                        assert_eq!(y, Some(PropValue::I64(40)));
                    } else {
                        panic!("unexpected ThingUpdate id {:?}", id);
                    }
                }
                other => panic!("unexpected request {other:?}"),
            }
        }
    }
}
