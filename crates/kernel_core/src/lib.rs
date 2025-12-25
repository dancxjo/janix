#![no_std]

extern crate alloc;

pub mod graph;
pub mod sched;
pub mod symbols;
pub mod syscalls;

use hw::HardwareBridge;
use graph::{GraphStore, seed_builtins};

use symbols::{SymbolTable, SymbolError};
use symbols::store::SymbolStore;

pub struct Kernel<B: HardwareBridge> {
    bridge: B,
    graph: GraphStore,
    pub symbols: SymbolTable,
}

impl<B: HardwareBridge> Kernel<B> {
    pub fn new(bridge: B) -> Self {
        Self { 
            bridge,
            graph: GraphStore::new(),
            symbols: SymbolTable::new(),
        }
    }

    pub fn boot(&mut self, mut store: Option<&mut dyn SymbolStore>) -> ! {
        self.bridge.log(thing_models::milestones::KERNEL_ENTRY);
        self.bridge.log("\n");
        self.bridge.log(thing_models::milestones::BRIDGE_ONLINE);
        self.bridge.log("\n");

        self.bridge.log("THINGOS: graph init\n");
        seed_builtins(&mut self.graph);
        self.bridge.log("THINGOS: graph seeded\n");

        self.bridge.log("THINGOS: symbols init\n");
        // Load, Seed, Persist
        if let Some(ref mut s) = store {
             let _ = self.symbols.load_from_store(*s); // simple v0: ignore load error
        }
        
        for &builtin in crate::symbols::builtins::BUILTIN_SYMBOLS {
            // we panic on builtin seed failure as it implies hash mismatch or logic bug
            self.symbols.seed_builtin(builtin).expect("Builtin seed failed");
        }

        if let Some(ref mut s) = store {
             let _ = self.symbols.persist_to_store(*s); // snapshot immediately
        }
        self.bridge.log("THINGOS: symbols ready\n");

        loop {
            self.bridge.log(thing_models::milestones::IDLE_LOOP);
            self.bridge.log("\n");
            self.bridge.idle();
        }
    }
}
