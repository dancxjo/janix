use abi::ThingId;
use alloc::boxed::Box;
use kernel::graph::{self, GraphEvent};
use kernel::graph_kinds;
use kernel::sched_types::CpuId;
use kernel::symbols;
use thing_models::PropValue;

use crate::context_switch::{arch_current_thread, arch_switch_to_thread};

fn cpu_index_from_node(node: ThingId) -> Option<usize> {
    match graph::get_prop(node, "index") {
        Some(PropValue::U64(idx)) => Some(idx as usize),
        _ => None,
    }
}

fn on_graph_event(event: &GraphEvent) {
    let link_runs_on = graph_kinds::LINK_RUNS_ON;

    if let GraphEvent::LinkAdded { src, dst, pred } = event {
        // Predicate comparison
        if *pred == link_runs_on {
            if let Some(cpu_index) = cpu_index_from_node(*dst) {
                let cpu = cpu_index as CpuId;
                let current = arch_current_thread(cpu);
                if current != Some(*src) {
                    arch_switch_to_thread(cpu, *src);
                }
            }
        }
    } else if let GraphEvent::ThingCreated(id) = event {
        let kind_io_op = symbols::intern(graph_kinds::KIND_IO_PORT_OP);
        let is_io_op = graph::with_thing(*id, |thing| thing.kind == kind_io_op).unwrap_or(false);
        if is_io_op {
            kernel::bridge::io::process_io_op(*id);
        }
    }
}

pub fn init_graph_subscriptions() {
    graph::events::subscribe(on_graph_event);
}
