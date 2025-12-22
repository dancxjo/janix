use crate::bridge;
use crate::graph::{self, GraphEvent};
use crate::graph_kinds;

pub fn init() {
    crate::graph::events::subscribe(on_event);
}

fn on_event(event: &GraphEvent) {
    match event {
        GraphEvent::ThingCreated(id) => {
            if let Some(kind) = crate::graph::get_thing_kind(*id) {
                if kind == crate::symbols::intern(graph_kinds::KIND_IO_PORT_OP) {
                    bridge::io::process_io_op(*id);
                } else if kind == crate::symbols::intern(graph_kinds::KIND_INTERRUPT_REQUEST) {
                    bridge::io::process_interrupt_request(*id);
                }
            }
        }
        GraphEvent::ThingUpdated(id) => {
            if let Some(kind) = crate::graph::get_thing_kind(*id) {
                if kind == crate::symbols::intern(graph_kinds::KIND_IO_PORT_OP) {
                    // Check status prop? Or just re-process.
                    bridge::io::process_io_op(*id);
                } else if kind == crate::symbols::intern(graph_kinds::KIND_INTERRUPT_REQUEST) {
                    bridge::io::process_interrupt_request(*id);
                }
            }
        }
        _ => {}
    }
}
