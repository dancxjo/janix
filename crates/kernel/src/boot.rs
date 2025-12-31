//! Boot sequence
//!
//! Orchestrates kernel initialization in strict order.

use crate::log::{self, Level};
use crate::machine::Machine;
use crate::{graph, sched, symbols, syscall};

/// Main kernel entry point
///
/// Called by Bran after constructing a Machine implementation.
/// This function never returns.
pub fn boot(machine: &'static dyn Machine) -> ! {
    // Phase 1: Initialize logging (enables debug output)
    log::init(machine);
    
    // Phase 2: Initialize symbol table
    symbols::init();
    log::klog(Level::Info, "KERNEL", "symbols init");
    
    // Phase 3: Initialize graph store
    graph::init();
    log::klog(Level::Info, "KERNEL", "graph init");
    
    // Phase 4: Initialize syscall dispatch
    syscall::init();
    
    // Phase 5: Initialize scheduler
    sched::init(machine);
    
    // Phase 6: Spawn Sprout (stub for now)
    spawn_sprout(machine);
    
    // Phase 7: Enter scheduler loop
    sched::run(machine)
}

/// Spawn the Sprout init process
fn spawn_sprout(machine: &'static dyn Machine) {
    log::klog(Level::Info, "KERNEL", "spawning sprout");
    
    // Look for a module named "sprout"
    let modules = machine.modules();
    let mut found_sprout = false;
    
    modules.list(&mut |info| {
        if info.path.contains("sprout") {
            found_sprout = true;
            // TODO: Actually spawn the process
            // For now, just note that we found it
        }
    });
    
    if !found_sprout {
        log::klog(Level::Warn, "KERNEL", "sprout module not found");
    }
}
