use crate::graph::{self, GraphEvent};
use crate::graph_kinds;
use crate::hw;

pub fn init() {
    graph::subscribe_node_created(graph_kinds::KIND_IO_PORT_OP, on_op_created);
    graph::subscribe_prop_changed(graph_kinds::KIND_IO_PORT_OP, "status", on_status_changed);
}

fn on_op_created(event: &GraphEvent) {
    if let GraphEvent::NodeCreated { id, .. } = event {
        hw::io::process_io_op(*id);
    }
}

fn on_status_changed(event: &GraphEvent) {
    if let GraphEvent::PropChanged { id, key, .. } = event {
        if *key == "status" {
            hw::io::process_io_op(*id);
        }
    }
}
