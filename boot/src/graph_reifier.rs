use thing_models::PropValue;
use abi::ThingId;
use kernel::graph::{self, GraphEvent};
use kernel::symbols;
use kernel::sched_types::CpuId;
use kernel::graph_kinds;
use alloc::boxed::Box;

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
    }
}

pub fn init_graph_subscriptions() {
    graph::events::subscribe(on_graph_event);
}
