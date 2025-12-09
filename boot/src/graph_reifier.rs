use abi::PropValue;
use abi::ThingId;
use kernel_core::graph::{self, GraphEvent};
use kernel_core::graph_kinds;
use kernel_core::log;

const MAX_CPUS: usize = 4;
static mut CURRENT_THREADS: [Option<ThingId>; MAX_CPUS] = [None; MAX_CPUS];

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
            unsafe {
                if cpu_index < MAX_CPUS {
                    let previous = CURRENT_THREADS[cpu_index];
                    CURRENT_THREADS[cpu_index] = Some(*thread_id);

                    if previous != Some(*thread_id) {
                        // Placeholder hook for a real context switch once arch code is ready.
                        log("Graph reifier observed sched.runs_on change; context switch pending.");
                    }
                }
            }
        }
    }
}

pub fn init_graph_subscriptions() {
    graph::subscribe_edge_added(graph_kinds::EDGE_RUNS_ON, on_runs_on_edge);
}
