#![no_std]

extern crate alloc;

pub mod graph;
pub mod sched;
pub mod symbols;
pub mod syscalls;
pub mod input;


use hw::HardwareBridge;
use graph::{GraphStore, seed_builtins};

use symbols::{SymbolTable, SymbolError};
use symbols::store::SymbolStore;

use sched::scheduler::Scheduler;

pub struct Kernel<B: HardwareBridge> {
    pub bridge: B, // Make bridge public so syscalls can access it in generic manner? 
    // syscall handler uses `kernel.bridge`. So yes, or make accessors.
    // Right now `bridge` field is private.
    // wait, `impl Kernel` has methods.
    // syscalls/graph.rs had `kernel.bridge.system_now()`.
    // Wait, `bridge` in line 17 IS private.
    // Step 135 `syscalls/graph.rs` uses `kernel.bridge.ticks()`.
    // This implies `bridge` MUST be public or `graph.rs` is failing to compile?
    // Step 137 `cargo check -p kernel_core` SUCCEEDED.
    // How? `syscalls/graph.rs` is a child module of `kernel_core`.
    // Modules in same crate can access private fields if they are in parent?
    // `kernel_core/src/syscalls` is a submodule. It CAN access parent fields if they are pub(crate).
    // `bridge` is private (default). Private fields are only accessible in the module defining the struct.
    // `lib.rs` defines `Kernel`. `syscalls` is separate module.
    // So `bridge` access should FAIL.
    // Why did `cargo check` succeed?
    // Maybe `B` is public? No.
    // Ah, `syscalls/graph.rs` line 6: `pub fn handle_graph_op...`.
    // wait, `syscalls/graph.rs` imported `crate::Kernel`.
    // If access failed, it should error.
    // Maybe I missed the public modifier in my file view?
    // Step 112: `bridge: B,` line 17. No pub.
    // Maybe `cargo check` didn't actually check `syscalls/graph.rs` content fully because it's library?
    // Or I am mistaken about visibility rules.
    // Private fields are visible to child modules? YES!
    // "Functionality in the parent module is visible to child modules." - wrong.
    // "Items in a parent module are visible to child modules, but private items are not... wait."
    // Actually, "In Rust 2018+, child modules can access private items in parent modules." - No.
    // Parent items are private to children unless `pub`.
    // BUT `Kernel` struct is in `lib.rs`. `syscalls` is loaded in `lib.rs`.
    // They are siblings? No `syscalls` is child of `lib`.
    // Wait, visibility is tricky.
    // Regardless, I'll make `bridge` pub to be safe and `scheduler` pub.
    
    pub graph: GraphStore, // Make graph pub too just in case
    pub symbols: SymbolTable,
    pub scheduler: Scheduler,
}

impl<B: HardwareBridge> Kernel<B> {
    pub fn new(bridge: B) -> Self {
        Self { 
            bridge,
            graph: GraphStore::new(),
            symbols: SymbolTable::new(),
            scheduler: Scheduler::new(),
        }
    }



    pub fn boot(&mut self, mut store: Option<&mut dyn SymbolStore>) -> ! {
        crate::input::init();
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
