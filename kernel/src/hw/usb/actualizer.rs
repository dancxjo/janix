use crate::graph::{self, GraphEvent};
use abi::graph_kinds;

pub fn init() {
    graph::subscribe_node_created(graph_kinds::KIND_USB_TRANSFER_REQUEST, on_request_created);
}

fn on_request_created(event: &GraphEvent) {
    if let GraphEvent::NodeCreated { id, .. } = event {
        crate::println!("New USB Transfer Request: {:?}", id);
        // TODO: Process request
    }
}
