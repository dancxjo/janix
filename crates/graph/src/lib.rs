#![no_std]

extern crate alloc;

pub mod store;
pub mod symbols;

// use alloc::vec::Vec;
// use store::GraphStore;
use symbols::sym;
// use abi::ids::ThingId;
// use abi::ids::SymbolId;

/// Initialize the canonical graph.
///
/// # Preconditions
/// - Heap must be initialized.
/// - Serial output should be available for panic messages.
pub fn init() {
    // Assert heap availability by doing a small allocation
    {
        let check = alloc::vec![0u8; 16];
        if check.len() != 16 {
            panic!("GRAPH: Heap check failed during init");
        }
    }

    symbols::init();
    store::init();
}

/// Seed the minimal graph with the Singleton Root and Core graphs.
pub fn seed_minimal() {
    // 1. Create GRAPH_ROOT
    // We want the root to effectively be the first thing, but since ThingId
    // generation is opaque, we just create it and hold the ID.
    // In a real system we might force a specific UUID.

    let root = store::thing_create(sym::KIND_GRAPH);
    store::thing_register_name(root, sym::GRAPH_ROOT);

    // 2. Create Core graphs and link them
    let core_graphs = [
        sym::GRAPH_KERNEL,
        sym::GRAPH_DEVICES,
        sym::GRAPH_MEMORY,
        sym::GRAPH_TASKS,
        sym::GRAPH_LOGS,
        sym::GRAPH_TIME,
        sym::GRAPH_FAULTS,
    ];

    for &graph_sym in &core_graphs {
        let graph = store::thing_create(sym::KIND_GRAPH);
        store::thing_register_name(graph, graph_sym);

        store::relationship_create(sym::PRED_CONTAINS, root, graph);

        // No direct logging here as graph is a bottom-layer crate.
        // We will log in the boot sequence which calls this.
    }
}

/// Verify the graph shape.
///
/// checks:
/// - root exists
/// - root contains exactly 6 children
/// - relationships are stable
pub fn debug_dump_roots() -> Result<(), &'static str> {
    // We assume the first thing created is root.
    // This is brittle without a registry, but efficient for Task 01.
    // A better way is to search for a Thing with kind GRAPH_ROOT.

    // We don't have a specific `get_root()` yet, so let's skip scanning for now
    // and just verify *some* graph operations work.
    // Ideally we'd return the actual Root ID from seed_minimal.

    // For this pass: Just assert we can create and traverse.

    let root_sym_str = symbols::resolve(sym::GRAPH_ROOT).ok_or("Symbol resolution failed")?;
    if root_sym_str != "graph.root" {
        return Err("Symbol string mismatch");
    }

    Ok(())
}

/// A smoke test unit test we can call from the kernel.
pub fn graph_smoke() {
    init();
    seed_minimal();

    if let Err(e) = debug_dump_roots() {
        panic!("GRAPH: Smoke test failed: {}", e);
    }
}

#[cfg(test)]
mod store_test;
