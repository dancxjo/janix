use crate::graph::{self, GraphEvent};
use crate::graph_kinds;
use crate::bridge;

pub fn init() {
    graph::subscribe_node_created(graph_kinds::KIND_IO_PORT_OP, on_op_created);
    graph::subscribe_prop_changed(graph_kinds::KIND_IO_PORT_OP, "status", on_status_changed);
    graph::subscribe_node_created(graph_kinds::KIND_INTERRUPT_REQUEST, on_interrupt_request_created);
    graph::subscribe_prop_changed(graph_kinds::KIND_INTERRUPT_REQUEST, abi::graph_kinds::PROP_ENABLED, on_interrupt_enabled_changed);
}

fn on_op_created(event: &GraphEvent) {
    if let GraphEvent::ThingCreated { id, .. } = event {
        bridge::io::process_io_op(*id);
    }
}

fn on_status_changed(event: &GraphEvent) {
    if let GraphEvent::PropUpdated { id, key, .. } = event {
        if *key == "status" {
            bridge::io::process_io_op(*id);
        }
    }
}

fn on_interrupt_request_created(event: &GraphEvent) {
    if let GraphEvent::ThingCreated { id, .. } = event {
        bridge::io::process_interrupt_request(*id);
    }
}

fn on_interrupt_enabled_changed(event: &GraphEvent) {
    if let GraphEvent::PropUpdated { id, key, .. } = event {
        if *key == abi::graph_kinds::PROP_ENABLED {
            bridge::io::process_interrupt_request(*id);
        }
    }
}
