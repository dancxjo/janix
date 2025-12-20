//! Display/scanout-related Thing models.

extern crate alloc;

use abi::{PropKey, PropType, PropValue, Thing, ThingId};
use crate::graph_kinds;
use alloc::vec::Vec;
use alloc::string::ToString;
use alloc::string::String;

#[derive(Clone, Debug)]
pub struct Display {
    pub id: ThingId,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub active_buffer_index: i64,
}

impl Thing for Display {
    const KIND: &'static str = graph_kinds::KIND_DISPLAY;
    const DESCRIPTION: &'static str = "A display sink capable of scanning out a SharedBuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_NAME.to_string(), PropValue::Str(self.name.clone())));
        out.push((graph_kinds::PROP_WIDTH.to_string(), PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT.to_string(), PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX.to_string(), PropValue::I64(self.active_buffer_index)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut width = 0;
        let mut height = 0;
        let mut active_buffer_index = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_NAME => if let PropValue::Str(v) = &prop.1 { name = v.clone(); },
                graph_kinds::PROP_WIDTH => if let PropValue::U64(v) = prop.1 { width = v; },
                graph_kinds::PROP_HEIGHT => if let PropValue::U64(v) = prop.1 { height = v; },
                graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX => if let PropValue::I64(v) = prop.1 { active_buffer_index = v; },
                _ => {}
            }
        }
        Display { id, name, width, height, active_buffer_index }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_NAME, PropType::Str),
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX, PropType::I64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct SharedBuffer {
    pub id: ThingId,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: String,
}

impl Thing for SharedBuffer {
    const KIND: &'static str = graph_kinds::KIND_SHARED_BUFFER;
    const DESCRIPTION: &'static str = "A kernel-owned shared memory buffer that can be mapped into userland";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_WIDTH.to_string(), PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT.to_string(), PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_STRIDE.to_string(), PropValue::U64(self.stride)));
        out.push((graph_kinds::PROP_PIXEL_FORMAT.to_string(), PropValue::Str(self.pixel_format.clone())));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = String::new();

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_WIDTH => if let PropValue::U64(v) = prop.1 { width = v; },
                graph_kinds::PROP_HEIGHT => if let PropValue::U64(v) = prop.1 { height = v; },
                graph_kinds::PROP_STRIDE => if let PropValue::U64(v) = prop.1 { stride = v; },
                graph_kinds::PROP_PIXEL_FORMAT => if let PropValue::Str(v) = &prop.1 { pixel_format = v.clone(); },
                _ => {}
            }
        }
        SharedBuffer { id, width, height, stride, pixel_format }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_STRIDE, PropType::U64),
            (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct DisplayFramebuffer {
    pub id: ThingId,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: String,
    pub power_state: String,
    pub refresh_interval_ns: u64,
    pub frames_presented: u64,
    pub last_present_ns: u64,
}

impl Thing for DisplayFramebuffer {
    const KIND: &'static str = graph_kinds::KIND_DISPLAY_FRAMEBUFFER;
    const DESCRIPTION: &'static str = "A userland-published framebuffer description backed by a SharedBuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_NAME.to_string(), PropValue::Str(self.name.clone())));
        out.push((graph_kinds::PROP_WIDTH.to_string(), PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT.to_string(), PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_STRIDE.to_string(), PropValue::U64(self.stride)));
        out.push((graph_kinds::PROP_PIXEL_FORMAT.to_string(), PropValue::Str(self.pixel_format.clone())));
        out.push((graph_kinds::PROP_POWER_STATE.to_string(), PropValue::Str(self.power_state.clone())));
        out.push((graph_kinds::PROP_REFRESH_INTERVAL_NS.to_string(), PropValue::U64(self.refresh_interval_ns)));
        out.push((graph_kinds::PROP_FRAMES_PRESENTED.to_string(), PropValue::U64(self.frames_presented)));
        out.push((graph_kinds::PROP_LAST_PRESENT_NS.to_string(), PropValue::U64(self.last_present_ns)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = String::new();
        let mut power_state = String::new();
        let mut refresh_interval_ns = 0;
        let mut frames_presented = 0;
        let mut last_present_ns = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_NAME => if let PropValue::Str(v) = &prop.1 { name = v.clone(); },
                graph_kinds::PROP_WIDTH => if let PropValue::U64(v) = prop.1 { width = v; },
                graph_kinds::PROP_HEIGHT => if let PropValue::U64(v) = prop.1 { height = v; },
                graph_kinds::PROP_STRIDE => if let PropValue::U64(v) = prop.1 { stride = v; },
                graph_kinds::PROP_PIXEL_FORMAT => if let PropValue::Str(v) = &prop.1 { pixel_format = v.clone(); },
                graph_kinds::PROP_POWER_STATE => if let PropValue::Str(v) = &prop.1 { power_state = v.clone(); },
                graph_kinds::PROP_REFRESH_INTERVAL_NS => if let PropValue::U64(v) = prop.1 { refresh_interval_ns = v; },
                graph_kinds::PROP_FRAMES_PRESENTED => if let PropValue::U64(v) = prop.1 { frames_presented = v; },
                graph_kinds::PROP_LAST_PRESENT_NS => if let PropValue::U64(v) = prop.1 { last_present_ns = v; },
                _ => {}
            }
        }
        DisplayFramebuffer { id, name, width, height, stride, pixel_format, power_state, refresh_interval_ns, frames_presented, last_present_ns }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_NAME, PropType::Str),
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_STRIDE, PropType::U64),
            (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
            (graph_kinds::PROP_POWER_STATE, PropType::Str),
            (graph_kinds::PROP_REFRESH_INTERVAL_NS, PropType::U64),
            (graph_kinds::PROP_FRAMES_PRESENTED, PropType::U64),
            (graph_kinds::PROP_LAST_PRESENT_NS, PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct DisplayFrame {
    pub id: ThingId,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: String,
}

impl Thing for DisplayFrame {
    const KIND: &'static str = graph_kinds::KIND_DISPLAY_FRAME;
    const DESCRIPTION: &'static str = "A single frame produced by a compositor targeting a framebuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_WIDTH.to_string(), PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT.to_string(), PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_STRIDE.to_string(), PropValue::U64(self.stride)));
        out.push((graph_kinds::PROP_PIXEL_FORMAT.to_string(), PropValue::Str(self.pixel_format.clone())));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = String::new();

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_WIDTH => if let PropValue::U64(v) = prop.1 { width = v; },
                graph_kinds::PROP_HEIGHT => if let PropValue::U64(v) = prop.1 { height = v; },
                graph_kinds::PROP_STRIDE => if let PropValue::U64(v) = prop.1 { stride = v; },
                graph_kinds::PROP_PIXEL_FORMAT => if let PropValue::Str(v) = &prop.1 { pixel_format = v.clone(); },
                _ => {}
            }
        }
        DisplayFrame { id, width, height, stride, pixel_format }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_STRIDE, PropType::U64),
            (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct DisplayPresentRequest {
    pub id: ThingId,
    pub framebuffer_id: ThingId,
    pub frame_index: u64,
    pub requested_at_ns: u64,
    pub presented_at_ns: Option<u64>,
    pub completed: bool,
}

impl Thing for DisplayPresentRequest {
    const KIND: &'static str = graph_kinds::KIND_DISPLAY_PRESENT_REQUEST;
    const DESCRIPTION: &'static str =
        "A compositor request for a framebuffer driver to present a frame";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_FRAMEBUFFER_ID.to_string(),
            PropValue::U64(self.framebuffer_id.0),
        ));
        out.push((graph_kinds::PROP_FRAME_INDEX.to_string(),
            PropValue::U64(self.frame_index),
        ));
        out.push((graph_kinds::PROP_REQUESTED_AT_NS.to_string(),
            PropValue::U64(self.requested_at_ns),
        ));
        if let Some(presented) = self.presented_at_ns {
            out.push((graph_kinds::PROP_PRESENTED_AT_NS.to_string(), PropValue::U64(presented)));
        }
        out.push((graph_kinds::PROP_COMPLETED.to_string(), PropValue::Bool(self.completed)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut framebuffer_id = ThingId(0);
        let mut frame_index = 0;
        let mut requested_at_ns = 0;
        let mut presented_at_ns = None;
        let mut completed = false;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_FRAMEBUFFER_ID => {
                    if let PropValue::U64(v) = prop.1 {
                        framebuffer_id = ThingId(v);
                    }
                }
                graph_kinds::PROP_FRAME_INDEX => {
                    if let PropValue::U64(v) = prop.1 {
                        frame_index = v;
                    }
                }
                graph_kinds::PROP_REQUESTED_AT_NS => {
                    if let PropValue::U64(v) = prop.1 {
                        requested_at_ns = v;
                    }
                }
                graph_kinds::PROP_PRESENTED_AT_NS => {
                    if let PropValue::U64(v) = prop.1 {
                        presented_at_ns = Some(v);
                    }
                }
                graph_kinds::PROP_COMPLETED => {
                    if let PropValue::Bool(v) = prop.1 {
                        completed = v;
                    }
                }
                _ => {}
            }
        }

        DisplayPresentRequest {
            id,
            framebuffer_id,
            frame_index,
            requested_at_ns,
            presented_at_ns,
            completed,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_FRAMEBUFFER_ID, PropType::U64),
            (graph_kinds::PROP_FRAME_INDEX, PropType::U64),
            (graph_kinds::PROP_REQUESTED_AT_NS, PropType::U64),
            (graph_kinds::PROP_PRESENTED_AT_NS, PropType::U64),
            (graph_kinds::PROP_COMPLETED, PropType::Bool),
        ]
    }
}
