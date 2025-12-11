use abi::PropValue;
use abi::ThingId;
use kernel::graph::{self, GraphEvent};
use kernel::graph_kinds;
use kernel::sched_types::CpuId;

use crate::context_switch::{arch_current_thread, arch_switch_to_thread};

fn cpu_index_from_node(node: ThingId) -> Option<usize> {
    match graph::get_prop(node, "index") {
        Some(PropValue::U64(idx)) => Some(idx as usize),
        _ => None,
    }
}

fn on_runs_on_link(event: &GraphEvent) {
    if let GraphEvent::LinkAdded(link) = event {
        if link.pred != graph_kinds::LINK_RUNS_ON {
            return;
        }

        if let Some(cpu_index) = cpu_index_from_node(link.dst) {
            let cpu = cpu_index as CpuId;
            let current = arch_current_thread(cpu);
            if current != Some(link.src) {
                arch_switch_to_thread(cpu, link.src);
            }
        }
    }
}

pub fn init_graph_subscriptions() {
    graph::subscribe_link_added(graph_kinds::LINK_RUNS_ON, on_runs_on_link);
}
