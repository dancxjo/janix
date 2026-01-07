extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;
use models::{Service, Module, Thing};
use abi::ids::ThingId;
use thing_std::graph::{symbol_intern, symbol_resolve, thing_get_body};

// Trait for graph access abstraction
pub trait GraphProvider {
    fn graph_outgoing(&self, from: ThingId, pred: abi::ids::SymbolId) -> Vec<ThingId>;
    fn thing_find(&self, name: &str) -> Option<ThingId>;
    fn symbol_intern(&self, name: &str) -> abi::ids::SymbolId;

    // Helpers
    fn decode_service(&self, id: ThingId) -> Option<Service>;
    fn decode_module(&self, id: ThingId) -> Option<Module>;
    fn resolve_symbol(&self, sym: abi::ids::SymbolId) -> Option<String>;
}

// Real implementation using thing_std
pub struct SystemGraph;
impl GraphProvider for SystemGraph {
    fn graph_outgoing(&self, from: ThingId, pred: abi::ids::SymbolId) -> Vec<ThingId> {
        thing_std::graph::graph_outgoing(from, pred)
    }

    fn thing_find(&self, name: &str) -> Option<ThingId> {
        thing_std::graph::thing_find(name)
    }

    fn symbol_intern(&self, name: &str) -> abi::ids::SymbolId {
        symbol_intern(name)
    }

    fn decode_service(&self, id: ThingId) -> Option<Service> {
        if let Some((body, _)) = thing_get_body(id) {
            Service::decode_full(&body).ok()
        } else {
            None
        }
    }

    fn decode_module(&self, id: ThingId) -> Option<Module> {
        if let Some((body, _)) = thing_get_body(id) {
            Module::decode_full(&body).ok()
        } else {
            None
        }
    }

    fn resolve_symbol(&self, sym: abi::ids::SymbolId) -> Option<String> {
        symbol_resolve(sym)
            .and_then(|b| String::from_utf8(b).ok())
    }
}

pub struct LaunchPlanItem {
    pub service_id: ThingId,
    pub name: String,
    pub module_name: Option<String>,
    pub deps: Vec<ThingId>,
    pub caps: Vec<thing_std::cap::CapOp>,
}

#[derive(Debug)]
pub enum PlanError {
    GraphMissing(String),
    CycleDetected(Vec<ThingId>),
    DependencyMissing(ThingId, ThingId), // dependent, dependency
}

pub fn build_launch_plan<G: GraphProvider>(graph: &G) -> Result<Vec<LaunchPlanItem>, PlanError> {
    let pred_contains = graph.symbol_intern("predicate.contains");
    let pred_refs = graph.symbol_intern("predicate.references");
    let pred_owns = graph.symbol_intern("predicate.owns");

    let plan_graphs = [
        "graph.services",
        "graph.services.time",
        "graph.apps.clock",
        "graph.services.core",
    ];

    let mut services = BTreeMap::new();
    let mut dependencies = BTreeMap::new();
    let mut service_ids = Vec::new();

    // 1. Discovery
    for graph_name in plan_graphs {
        if let Some(graph_id) = graph.thing_find(graph_name) {
            let members = graph.graph_outgoing(graph_id, pred_contains);
            for svc_id in members {
                if services.contains_key(&svc_id) {
                    continue;
                }

                if let Some(svc) = graph.decode_service(svc_id) {
                    let name = graph.resolve_symbol(svc.name).unwrap_or_else(|| "service.?".to_string());

                    let module_info = {
                        if let Some(mod_id) = graph.graph_outgoing(svc_id, pred_owns).first() {
                            if let Some(module) = graph.decode_module(*mod_id) {
                                let m_name = graph.resolve_symbol(module.name);
                                let caps = module.caps.iter().take(module.cap_count as usize).cloned().collect();

                                // Module deps
                                let mut m_deps = Vec::new();
                                for i in 0..module.dep_count.min(module.deps.len() as u8) {
                                    if let Some(dep_name) = graph.resolve_symbol(module.deps[i as usize]) {
                                        if let Some(dep_id) = graph.thing_find(&dep_name) {
                                            m_deps.push(dep_id);
                                        }
                                    }
                                }
                                Some((m_name, m_deps, caps))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };

                    let (mod_name, mod_deps, caps) = module_info.unwrap_or_default();

                    // Service deps (direct references)
                    let svc_refs = graph.graph_outgoing(svc_id, pred_refs);

                    let mut all_deps = svc_refs;
                    all_deps.extend(mod_deps);
                    all_deps.sort();
                    all_deps.dedup();

                    let item = LaunchPlanItem {
                        service_id: svc_id,
                        name,
                        module_name: mod_name,
                        deps: all_deps.clone(),
                        caps,
                    };

                    services.insert(svc_id, item);
                    dependencies.insert(svc_id, all_deps);
                    service_ids.push(svc_id);
                }
            }
        }
    }

    // 2. Topological Sort (Kahn's Algorithm)
    // Calculate in-degree (number of dependencies)
    let mut in_degree = BTreeMap::new();
    let mut graph_edges = BTreeMap::new(); // dependency -> [dependents]

    for &id in &service_ids {
        in_degree.insert(id, 0);
        graph_edges.insert(id, Vec::new());
    }

    for (&dependent, deps) in &dependencies {
        for &dependency in deps {
            // Only consider dependencies that are in our service list
            if services.contains_key(&dependency) {
                *in_degree.entry(dependent).or_insert(0) += 1;
                graph_edges.entry(dependency).or_default().push(dependent);
            }
        }
    }

    // Queue of services with 0 dependencies
    // Use BinaryHeap or sorted Vec for deterministic ordering
    let mut queue = Vec::new();
    for (&id, &deg) in &in_degree {
        if deg == 0 {
            queue.push(id);
        }
    }
    // Sort by ID for stability
    queue.sort();

    let mut result_plan = Vec::new();

    while !queue.is_empty() {
        let node = queue.remove(0); // Pop first (lowest ID)
        result_plan.push(node);

        if let Some(dependents) = graph_edges.get(&node) {
            for &dependent in dependents {
                let degree = in_degree.get_mut(&dependent).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push(dependent);
                }
            }
        }
        // Re-sort queue to maintain deterministic order after adding new nodes
        queue.sort();
    }

    if result_plan.len() != services.len() {
        // Cycle detected
        let mut remaining = Vec::new();
        for (id, &deg) in &in_degree {
            if deg > 0 {
                remaining.push(*id);
            }
        }
        return Err(PlanError::CycleDetected(remaining));
    }

    // Construct final vector
    let final_plan = result_plan.into_iter()
        .map(|id| services.remove(&id).unwrap())
        .collect();

    Ok(final_plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::ids::{SymbolId, sym};
    use core::cell::RefCell;
    use alloc::vec;

    struct MockGraph {
        outgoing: RefCell<BTreeMap<(ThingId, SymbolId), Vec<ThingId>>>,
        names: RefCell<BTreeMap<String, ThingId>>,
        services: RefCell<BTreeMap<ThingId, Service>>,
        modules: RefCell<BTreeMap<ThingId, Module>>,
        symbols: RefCell<BTreeMap<SymbolId, String>>,
    }

    impl MockGraph {
        fn new() -> Self {
            Self {
                outgoing: RefCell::new(BTreeMap::new()),
                names: RefCell::new(BTreeMap::new()),
                services: RefCell::new(BTreeMap::new()),
                modules: RefCell::new(BTreeMap::new()),
                symbols: RefCell::new(BTreeMap::new()),
            }
        }

        fn add_service(&self, id: ThingId, name: &str, graph: ThingId) {
            let s_name = sym(name);
            self.symbols.borrow_mut().insert(s_name, name.to_string());
            self.services.borrow_mut().insert(id, Service {
                name: s_name,
                state: 0,
                pid: 0,
            });
            // Link to graph
            self.add_rel(graph, sym("predicate.contains"), id);
        }

        fn add_dependency(&self, from: ThingId, to: ThingId) {
            self.add_rel(from, sym("predicate.references"), to);
        }

        fn add_rel(&self, from: ThingId, pred: SymbolId, to: ThingId) {
            self.outgoing.borrow_mut().entry((from, pred)).or_default().push(to);
        }
    }

    impl GraphProvider for MockGraph {
        fn graph_outgoing(&self, from: ThingId, pred: SymbolId) -> Vec<ThingId> {
            self.outgoing.borrow().get(&(from, pred)).cloned().unwrap_or_default()
        }

        fn thing_find(&self, name: &str) -> Option<ThingId> {
            self.names.borrow().get(name).cloned()
        }

        fn symbol_intern(&self, name: &str) -> abi::ids::SymbolId {
            sym(name)
        }

        fn decode_service(&self, id: ThingId) -> Option<Service> {
            self.services.borrow().get(&id).cloned()
        }

        fn decode_module(&self, id: ThingId) -> Option<Module> {
            self.modules.borrow().get(&id).cloned()
        }

        fn resolve_symbol(&self, sym: SymbolId) -> Option<String> {
            self.symbols.borrow().get(&sym).cloned()
        }
    }

    // Stub symbols::intern for tests if needed, but we use sym() helper in tests
    // In actual code, symbol_intern is a syscall.
    // The build_launch_plan function calls symbol_intern.
    // We can't easily mock free functions.
    // We should mock symbol_intern in GraphProvider or use a known constant.
    // Ideally we'd modify build_launch_plan to take pred symbols as args or use provider.
    // BUT, symbol_intern returns deterministic SymbolId based on hash.
    // So sym("predicate.contains") in test matches symbol_intern("predicate.contains") in code if they use same hash.

    // abi::ids::sym uses crc64. symbol_intern uses syscall.
    // If the kernel uses the same hash, we are good.
    // The implementation of symbol_intern in thing_std calls syscall, which calls symbols::intern, which uses crc64.
    // So yes, they are compatible.

    #[test]
    fn test_dag_sort() {
        let graph = MockGraph::new();
        let svc_graph = ThingId(100);
        graph.names.borrow_mut().insert("graph.services".to_string(), svc_graph);

        let a = ThingId(1);
        let b = ThingId(2);
        let c = ThingId(3);

        graph.add_service(a, "service.a", svc_graph);
        graph.add_service(b, "service.b", svc_graph);
        graph.add_service(c, "service.c", svc_graph);

        // A depends on B
        graph.add_dependency(a, b);
        // B depends on C
        graph.add_dependency(b, c);

        // Expected: C, B, A
        let plan = build_launch_plan(&graph).expect("Plan failed");
        let ids: Vec<ThingId> = plan.iter().map(|p| p.service_id).collect();
        assert_eq!(ids, vec![c, b, a]);
    }

    #[test]
    fn test_cycle_detection() {
        let graph = MockGraph::new();
        let svc_graph = ThingId(100);
        graph.names.borrow_mut().insert("graph.services".to_string(), svc_graph);

        let a = ThingId(1);
        let b = ThingId(2);

        graph.add_service(a, "service.a", svc_graph);
        graph.add_service(b, "service.b", svc_graph);

        graph.add_dependency(a, b);
        graph.add_dependency(b, a);

        let res = build_launch_plan(&graph);
        assert!(matches!(res, Err(PlanError::CycleDetected(_))));
    }

    #[test]
    fn test_tie_breaking() {
        let graph = MockGraph::new();
        let svc_graph = ThingId(100);
        graph.names.borrow_mut().insert("graph.services".to_string(), svc_graph);

        let a = ThingId(10);
        let b = ThingId(20);

        // No deps. Should be sorted by ID (10 then 20)
        graph.add_service(b, "service.b", svc_graph);
        graph.add_service(a, "service.a", svc_graph);

        let plan = build_launch_plan(&graph).expect("Plan failed");
        let ids: Vec<ThingId> = plan.iter().map(|p| p.service_id).collect();
        assert_eq!(ids, vec![a, b]);
    }
}
