//! Layout engine for Bloom UI.
//!
//! This module separates layout computation from pixel drawing.
//! All layout functions take pre-resolved scene data (no graph syscalls).

use crate::scene::Rect;
use crate::ui::WidgetKind;
use alloc::vec::Vec;

/// Title bar height in pixels (classic window manager style).
pub const TITLE_BAR_HEIGHT: i32 = 22;

/// A widget with its computed screen position.
#[derive(Clone)]
pub struct PlacedWidget {
    pub rect: Rect,
    pub widget: WidgetKind,
}

/// Flex layout item descriptor.
#[derive(Clone, Copy)]
pub struct FlexItem {
    pub min_h: u32,
    pub pref_h: u32,
    pub grow: u16, // 0..65535
}

impl FlexItem {
    /// Convert a WidgetKind into a FlexItem with reasonable defaults.
    pub fn from_widget(widget: &WidgetKind) -> Self {
        match widget {
            WidgetKind::Label(label, _) => {
                let h = (label.style.size as u32 + 6).max(14);
                FlexItem { min_h: h, pref_h: h, grow: 0 }
            }
            WidgetKind::Button(_, _) => {
                FlexItem { min_h: 32, pref_h: 32, grow: 0 }
            }
            WidgetKind::Canvas(ref c) => {
                let h = c.height;
                FlexItem { min_h: h, pref_h: h, grow: 0 }
            }
            WidgetKind::DrawList(ref dl) => {
                let h = dl.height;
                FlexItem { min_h: h, pref_h: h, grow: 0 }
            }
        }
    }
}

/// Compute flex column layout.
///
/// Distributes space among items based on flex grow factors.
/// Items with grow=0 get their preferred height; extra space is
/// distributed proportionally among items with grow > 0.
pub fn flex_column(
    container: Rect,
    padding: i32,
    gap: i32,
    items: &[FlexItem],
) -> Vec<Rect> {
    let inner_h = container.h.saturating_sub((padding as u32) * 2) as i32;
    let inner_w = container.w.saturating_sub((padding as u32) * 2);
    let n = items.len() as i32;
    if n <= 0 {
        return Vec::new();
    }

    let gaps = gap * (n - 1);
    let mut base_sum = 0i32;
    let mut grow_sum = 0u32;

    for it in items {
        base_sum += it.pref_h.max(it.min_h) as i32;
        grow_sum += it.grow as u32;
    }

    let extra = (inner_h - gaps - base_sum).max(0);
    let mut y = container.y + padding;

    let mut out = Vec::new();
    for it in items {
        let base = it.pref_h.max(it.min_h) as i32;
        let add = if grow_sum > 0 && it.grow > 0 {
            (extra as i64 * it.grow as i64 / grow_sum as i64) as i32
        } else {
            0
        };

        let h = (base + add).max(it.min_h as i32);
        out.push(Rect {
            x: container.x + padding,
            y,
            w: inner_w,
            h: h as u32,
        });
        y += h + gap;
    }
    out
}

/// Compute flex row layout.
///
/// Distributes horizontal space among items based on flex grow factors.
pub fn flex_row(
    container: Rect,
    padding: i32,
    gap: i32,
    items: &[FlexItem],
) -> Vec<Rect> {
    let inner_w = container.w.saturating_sub((padding as u32) * 2) as i32;
    let inner_h = container.h.saturating_sub((padding as u32) * 2);
    let n = items.len() as i32;
    if n <= 0 {
        return Vec::new();
    }

    let gaps = gap * (n - 1);
    let mut base_sum = 0i32;
    let mut grow_sum = 0u32;

    // For row layout, we use pref_h as width preference
    for it in items {
        base_sum += it.pref_h.max(it.min_h) as i32;
        grow_sum += it.grow as u32;
    }

    let extra = (inner_w - gaps - base_sum).max(0);
    let mut x = container.x + padding;

    let mut out = Vec::new();
    for it in items {
        let base = it.pref_h.max(it.min_h) as i32;
        let add = if grow_sum > 0 && it.grow > 0 {
            (extra as i64 * it.grow as i64 / grow_sum as i64) as i32
        } else {
            0
        };

        let w = (base + add).max(it.min_h as i32);
        out.push(Rect {
            x,
            y: container.y + padding,
            w: w as u32,
            h: inner_h,
        });
        x += w + gap;
    }
    out
}

/// Layout widgets using the Layout descriptor.
///
/// This is the main entry point called by render_window.
/// Uses flex_column/flex_row internally based on LayoutKind.
pub fn layout_widgets(
    layout: &models::Layout,
    children: &[WidgetKind],
    window_rect: Rect,
) -> Vec<PlacedWidget> {
    use models::LayoutKind;

    let padding = layout.padding as i32;
    let gap = layout.gap as i32;

    // Content area starts below the title bar
    let content_rect = Rect {
        x: window_rect.x,
        y: window_rect.y + TITLE_BAR_HEIGHT,
        w: window_rect.w,
        h: window_rect.h.saturating_sub(TITLE_BAR_HEIGHT as u32),
    };

    // Build flex items from widgets
    let flex_items: Vec<FlexItem> = children.iter().map(FlexItem::from_widget).collect();

    // Compute layout
    let rects = match layout.kind {
        LayoutKind::Column => flex_column(content_rect, padding, gap, &flex_items),
        LayoutKind::Row => flex_row(content_rect, padding, gap, &flex_items),
    };

    // Zip rects with widgets
    rects
        .into_iter()
        .zip(children.iter())
        .map(|(rect, widget)| PlacedWidget {
            rect,
            widget: widget.clone(),
        })
        .collect()
}

/// Simple column layout without flex grow (original behavior).
pub fn layout_widgets_simple_column(
    window_rect: Rect,
    padding: i32,
    gap: i32,
    children: &[WidgetKind],
) -> Vec<PlacedWidget> {
    let usable_w = window_rect.w.saturating_sub((padding as u32) * 2);
    let start_x = window_rect.x + padding;
    let mut cursor_y = window_rect.y + TITLE_BAR_HEIGHT + padding;

    let mut out = Vec::new();
    for child in children {
        let h = match child {
            WidgetKind::Label(label, _) => (label.style.size as i32 + 6).max(14),
            WidgetKind::Button(_, _) => 32,
            WidgetKind::Canvas(c) => c.height as i32,
            WidgetKind::DrawList(dl) => dl.height as i32,
        };

        let rect = Rect {
            x: start_x,
            y: cursor_y,
            w: usable_w,
            h: h as u32,
        };
        cursor_y += h + gap;
        out.push(PlacedWidget {
            rect,
            widget: child.clone(),
        });
    }
    out
}
