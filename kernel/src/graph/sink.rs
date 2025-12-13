use abi::graph_ops::{GraphOp, GraphSink, GraphDriver, GraphEvent, ThingProps};
use crate::graph;
use abi::ThingId;
use alloc::vec::Vec;

pub struct KernelGraphSink;

impl GraphSink for KernelGraphSink {
    fn submit(&mut self, op: GraphOp) -> Result<(), &'static str> {
        match op {
            GraphOp::CreateThing { kind, props } => {
                graph::create_thing(kind, &props);
                Ok(())
            }
            GraphOp::UpdateProps { id, props } => {
                if graph::update_thing(id, &props) {
                    Ok(())
                } else {
                    Err("Failed to update thing")
                }
            }
        }
    }
}

impl GraphDriver for KernelGraphSink {
    fn subscribe(&mut self, kind: &'static str, handler: fn(&GraphEvent)) {
        // Now GraphEvent is the same type.
        // graph::events::subscribe_node_created expects fn(&GraphEvent).
        // Since we unified them, it should work.
        graph::events::subscribe_node_created(kind, handler);
    }

    fn get_thing(&self, id: ThingId) -> Option<ThingProps> {
        if let Some((_kind, props)) = graph::get_thing(id) {
            let mut vec = Vec::new();
            for entry in props.iter() {
                if let Some((k, v)) = entry {
                     vec.push((*k, v.clone()));
                }
            }
            Some(ThingProps { props: vec })
        } else {
            None
        }
    }
}
