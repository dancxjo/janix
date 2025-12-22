extern crate alloc;

use abi::ThingId;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use compositor_api;
use thing_models::graph_kinds;
use thing_models::{PropKey, PropType, PropValue};
use thing_os::prelude::*;
use thing_os::{list_things_by_kind, load_thing, update_props};

use crate::flex::{AlignItems, FlexDirection, FlexWrap, JustifyContent, prop_as_f32};
use crate::widget_layout::{self, LayoutItem, LayoutSpec, Rect};

#[derive(Debug, Clone)]
pub struct WidgetNode {
    pub id: ThingId,

    // absolute overlay escape hatch
    pub x: Option<i32>,
    pub y: Option<i32>,

    // sizing hints
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub min_width: Option<i32>,
    pub min_height: Option<i32>,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,

    // flex behavior when this widget is a container
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub justify: Option<JustifyContent>,
    pub align: Option<AlignItems>,
    pub gap: Option<i32>,

    // flex behavior when this widget is a child
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,

    // visual properties
    pub text: Option<String>,
    pub font_size: Option<u32>,
    pub fg_color: Option<u32>,
    pub bg_color: Option<u32>,
}

impl WidgetNode {
    pub fn is_overlay(&self) -> bool {
        self.x.is_some() && self.y.is_some()
    }
}

impl thing_os::Thing for WidgetNode {
    const KIND: &'static str = compositor_api::KIND_WIDGET;
    const DESCRIPTION: &'static str = "UI Widget";
    fn schema() -> &'static [(&'static str, PropType)] {
        &[]
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let get_i32 = |k: &str| -> Option<i32> {
            props
                .iter()
                .flatten()
                .find(|(key, _)| *key == k)
                .and_then(|(_, v)| match v {
                    PropValue::I64(n) => Some(*n as i32),
                    _ => None,
                })
        };

        let get_prop = |k: &str| -> Option<&PropValue> {
            props
                .iter()
                .flatten()
                .find(|(key, _)| *key == k)
                .map(|(_, v)| v)
        };

        let text = get_prop(graph_kinds::PROP_TEXT).and_then(|v| match v {
            PropValue::Str(s) => Some(s.clone()),
            _ => None,
        });

        // Helper for color/size which are I64 or U64 depending on encoding, usually I64 in properties
        let get_u32 = |k: &str| -> Option<u32> {
            match get_prop(k) {
                Some(PropValue::I64(v)) => Some(*v as u32),
                Some(PropValue::U64(v)) => Some(*v as u32),
                _ => None,
            }
        };

        let font_size = get_u32(graph_kinds::PROP_FONT_SIZE);
        let fg_color = get_u32(graph_kinds::PROP_FG_COLOR);
        let bg_color = get_u32(graph_kinds::PROP_BG_COLOR);

        let flex_direction =
            get_prop(compositor_api::PROP_FLEX_DIRECTION).and_then(FlexDirection::from_prop);

        let flex_wrap = get_prop(compositor_api::PROP_FLEX_WRAP).and_then(FlexWrap::from_prop);

        let justify =
            get_prop(compositor_api::PROP_JUSTIFY_CONTENT).and_then(JustifyContent::from_prop);

        let align = get_prop(compositor_api::PROP_ALIGN_ITEMS).and_then(AlignItems::from_prop);

        let flex_grow = get_prop(compositor_api::PROP_FLEX_GROW).and_then(prop_as_f32);

        let flex_shrink = get_prop(compositor_api::PROP_FLEX_SHRINK).and_then(prop_as_f32);

        Self {
            id,
            x: get_i32(compositor_api::PROP_X),
            y: get_i32(compositor_api::PROP_Y),

            width: get_i32(graph_kinds::PROP_WIDTH),
            height: get_i32(graph_kinds::PROP_HEIGHT),
            min_width: get_i32(compositor_api::PROP_MIN_WIDTH),
            min_height: get_i32(compositor_api::PROP_MIN_HEIGHT),
            max_width: get_i32(compositor_api::PROP_MAX_WIDTH),
            max_height: get_i32(compositor_api::PROP_MAX_HEIGHT),

            flex_direction,
            flex_wrap,
            justify,
            align,
            gap: get_i32(compositor_api::PROP_GAP),

            flex_grow,
            flex_shrink,

            text,
            font_size,
            fg_color,
            bg_color,
        }
    }

    fn to_props(&self, _out: &mut Vec<(PropKey, PropValue)>) {}
}

/// Collect widget ids that are direct children of `parent` via LINK_WIDGET_CHILD.
pub fn widget_children(parent: ThingId) -> Vec<ThingId> {
    thing_os::link_targets(parent, compositor_api::LINK_WIDGET_CHILD)
}

/// Lay out and return computed rects for children inside `container`.
/// Also writes computed width/height back to each child if changed.
pub fn layout_children(
    widgets: &BTreeMap<ThingId, WidgetNode>,
    container_id: ThingId,
    container_rect: Rect,
    children: &[ThingId],
) -> Vec<(ThingId, Rect)> {
    // Container’s layout spec (defaults match Basic-OS behavior)
    let (gap, spec) = if let Some(container) = widgets.get(&container_id) {
        let gap = container.gap.unwrap_or(0);
        let dir = container.flex_direction.unwrap_or(FlexDirection::Column);
        let wrap = container.flex_wrap.unwrap_or(FlexWrap::NoWrap);
        let justify = container.justify.unwrap_or_default();
        let align = container.align.unwrap_or(AlignItems::Stretch);
        (
            gap,
            LayoutSpec::Flex {
                direction: dir,
                wrap,
                justify,
                align,
            },
        )
    } else {
        (
            0,
            LayoutSpec::Flex {
                direction: FlexDirection::Column,
                wrap: FlexWrap::NoWrap,
                justify: JustifyContent::Start,
                align: AlignItems::Stretch,
            },
        )
    };

    let mut relative = Vec::new();
    let mut overlay = Vec::new();

    for id in children {
        if let Some(w) = widgets.get(id) {
            if w.is_overlay() {
                overlay.push(*id);
            } else {
                relative.push(*id);
            }
        }
    }

    // --- relative children (flex flow) ---
    let items: Vec<LayoutItem> = relative
        .iter()
        .filter_map(|id| {
            widgets.get(id).map(|w| {
                // This is the “30px fallback height” trick from Basic-OS
                let min_w = w.min_width.or(w.width).unwrap_or(0).max(0) as u32;
                let min_h = w.min_height.or(w.height).unwrap_or(30).max(0) as u32;

                LayoutItem {
                    id: *id,
                    min_width: min_w,
                    min_height: min_h,
                    max_width: w.max_width.map(|v| v.max(0) as u32),
                    max_height: w.max_height.map(|v| v.max(0) as u32),
                    flex_grow: w.flex_grow.unwrap_or(0.0),
                    flex_shrink: w.flex_shrink.unwrap_or(1.0),
                }
            })
        })
        .collect();

    let mut rects = widget_layout::layout(container_rect, spec, &items, gap);

    // --- overlay children (absolute) ---
    for id in overlay {
        if let Some(w) = widgets.get(&id) {
            let x = container_rect.x + w.x.unwrap_or(0);
            let y = container_rect.y + w.y.unwrap_or(0);
            let ww = w.width.unwrap_or(0).max(0) as u32;
            let hh = w.height.unwrap_or(0).max(0) as u32;
            rects.push((id, Rect::new(x, y, ww, hh)));
        }
    }

    // Write back computed width/height for graph-backed widgets when changed
    for (id, r) in &rects {
        if let Some(w) = widgets.get(id) {
            let width_changed = w.width.map(|v| v.max(0) as u32 != r.w).unwrap_or(true);
            let height_changed = w.height.map(|v| v.max(0) as u32 != r.h).unwrap_or(true);

            if width_changed || height_changed {
                let mut props = Vec::new();
                if width_changed {
                    props.push((
                        graph_kinds::PROP_WIDTH.to_string(),
                        PropValue::I64(r.w as i64),
                    ));
                }
                if height_changed {
                    props.push((
                        graph_kinds::PROP_HEIGHT.to_string(),
                        PropValue::I64(r.h as i64),
                    ));
                }
                let _ = update_props(*id, &props);
            }
        }
    }

    rects
}
