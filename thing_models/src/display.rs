//! Display/scanout-related Thing models.

extern crate alloc;

use abi::{PropKey, PropType, PropValue, Thing, ThingId, graph_kinds};
use alloc::vec::Vec;

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
        out.push((
            graph_kinds::PROP_FRAMEBUFFER_ID,
            PropValue::U64(self.framebuffer_id.0),
        ));
        out.push((
            graph_kinds::PROP_FRAME_INDEX,
            PropValue::U64(self.frame_index),
        ));
        out.push((
            graph_kinds::PROP_REQUESTED_AT_NS,
            PropValue::U64(self.requested_at_ns),
        ));
        if let Some(presented) = self.presented_at_ns {
            out.push((graph_kinds::PROP_PRESENTED_AT_NS, PropValue::U64(presented)));
        }
        out.push((graph_kinds::PROP_COMPLETED, PropValue::Bool(self.completed)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut framebuffer_id = ThingId(0);
        let mut frame_index = 0;
        let mut requested_at_ns = 0;
        let mut presented_at_ns = None;
        let mut completed = false;

        for prop in props.iter().flatten() {
            match prop.0 {
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
