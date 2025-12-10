use abi::PropValue;
use abi::ThingId;
use kernel_core::graph::{self, GraphEvent};
use kernel_core::graph_kinds;
use kernel_core::sched_types::CpuId;

use crate::context_switch::{arch_current_thread, arch_switch_to_thread};

fn cpu_index_from_node(node: ThingId) -> Option<usize> {
    match graph::get_prop(node, "index") {
        Some(PropValue::U64(idx)) => Some(idx as usize),
        _ => None,
    }
}

fn on_runs_on_edge(event: &GraphEvent) {
    if let GraphEvent::EdgeAdded {
        from: thread_id,
        edge_kind,
        to: cpu_node,
    } = event
    {
        if *edge_kind != graph_kinds::EDGE_RUNS_ON {
            return;
        }

        if let Some(cpu_index) = cpu_index_from_node(*cpu_node) {
            let cpu = cpu_index as CpuId;
            let current = arch_current_thread(cpu);
            if current != Some(*thread_id) {
                arch_switch_to_thread(cpu, *thread_id);
            }
        }
    }
}

pub fn init_graph_subscriptions() {
    graph::subscribe_edge_added(graph_kinds::EDGE_RUNS_ON, on_runs_on_edge);
}
