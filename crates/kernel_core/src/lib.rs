#![no_std]

extern crate alloc;

pub mod graph;
pub mod sched;

use hw::HardwareBridge;
use graph::{GraphStore, seed_builtins};

pub struct Kernel<B: HardwareBridge> {
    bridge: B,
    graph: GraphStore,
}

impl<B: HardwareBridge> Kernel<B> {
    pub fn new(bridge: B) -> Self {
        Self { 
            bridge,
            graph: GraphStore::new(),
        }
    }

    pub fn boot(&mut self) -> ! {
        self.bridge.log(thing_models::milestones::KERNEL_ENTRY);
        self.bridge.log("\n");
        self.bridge.log(thing_models::milestones::BRIDGE_ONLINE);
        self.bridge.log("\n");

        self.bridge.log("THINGOS: graph init\n");
        seed_builtins(&mut self.graph);
        self.bridge.log("THINGOS: graph seeded\n");

        loop {
            self.bridge.log(thing_models::milestones::IDLE_LOOP);
            self.bridge.log("\n");
            self.bridge.idle();
        }
    }
}
