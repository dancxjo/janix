#![no_std]
#![allow(unused)]
#![allow(static_mut_refs)]
#![allow(private_interfaces)]
extern crate alloc;

pub mod boot_fs;
pub mod bridge;
pub mod bytespace;
pub mod diag;
pub mod drivers;
pub mod font;
pub mod fs;
pub mod graph;
pub mod input;
pub mod machine;
pub mod sched;
pub mod symbols;
pub mod syscalls;
pub mod time;
pub mod types;

pub mod platform {
    pub mod acpi;
}

use crate::bridge::HardwareBridge;
use graph::{seed_builtins, GraphStore};

use symbols::store::SymbolStore;
use symbols::SymbolTable;

use sched::scheduler::Scheduler;

pub struct Kernel<B: HardwareBridge> {
    pub bridge: B,
    pub graph: GraphStore,
    pub bytespaces: bytespace::ByteSpaceStore,
    pub symbols: SymbolTable,
    pub scheduler: Scheduler<B::Context>,
    pub machine: machine::Machine,
    pub machine_providers: machine::providers::ProviderStorage,
}

impl<B: HardwareBridge> Kernel<B> {
    pub fn new(bridge: B) -> Self {
        Self {
            bridge,
            graph: GraphStore::new(),
            bytespaces: bytespace::ByteSpaceStore::new(),
            symbols: SymbolTable::new(),
            scheduler: Scheduler::new(),
            machine: machine::Machine::new(),
            machine_providers: machine::providers::ProviderStorage::new(),
        }
    }

    pub fn register_machine_providers(&mut self) {
        use abi::symbols::sym;
        use abi::wire::machine::*;

        if self.machine_providers.rtc.is_none() {
            self.machine_providers.rtc = Some(machine::providers::rtc::RtcProvider::new());
        }
        if let Some(provider) = &self.machine_providers.rtc {
            let ctx = provider as *const _ as *const ();
            self.machine.register_provider(
                sym(IFACE_RTC),
                1,
                sym("rtc0"),
                machine::providers::rtc::RtcProvider::META,
                &machine::providers::rtc::RtcProvider::VTABLE,
                ctx,
            );
        }

        if self.machine_providers.ps2.is_none() {
            self.machine_providers.ps2 = Some(machine::providers::ps2::Ps2Provider::new());
        }
        if let Some(provider) = &self.machine_providers.ps2 {
            let ctx = provider as *const _ as *const ();
            self.machine.register_provider(
                sym(IFACE_KEYBOARD),
                1,
                sym("kbd0"),
                machine::providers::ps2::Ps2Provider::META,
                &machine::providers::ps2::Ps2Provider::VTABLE,
                ctx,
            );
            self.machine.register_provider(
                sym(IFACE_MOUSE),
                1,
                sym("mouse0"),
                machine::providers::ps2::Ps2Provider::META,
                &machine::providers::ps2::Ps2Provider::VTABLE,
                ctx,
            );
        }
    }

    pub fn boot(&mut self, mut store: Option<&mut dyn SymbolStore>) -> ! {
        self.bridge.log("BRIDGE: serial::init\n");
        crate::drivers::serial::init(&self.bridge);
        self.bridge.log(thing_models::milestones::KERNEL_ENTRY);

        self.bridge.log("\n");
        self.bridge.log(thing_models::milestones::BRIDGE_ONLINE);
        self.bridge.log("\n");

        self.bridge.log("THINGOS: graph init\n");
        seed_builtins(&mut self.graph);
        crate::input::init();
        self.register_machine_providers();
        self.machine.reflect_into_graph(&mut self.graph);
        self.bridge.log("THINGOS: graph seeded\n");

        self.bridge.log("THINGOS: symbols init\n");
        // Load, Seed, Persist
        if let Some(ref mut s) = store {
            let _ = self.symbols.load_from_store(*s); // simple v0: ignore load error
        }

        for &builtin in crate::symbols::builtins::BUILTIN_SYMBOLS {
            // we panic on builtin seed failure as it implies hash mismatch or logic bug
            self.symbols
                .seed_builtin(builtin)
                .expect("Builtin seed failed");
        }

        if let Some(ref mut s) = store {
            let _ = self.symbols.persist_to_store(*s); // snapshot immediately
        }
        self.bridge.log("THINGOS: symbols ready\n");

        loop {
            self.bridge.log(thing_models::milestones::IDLE_LOOP);
            crate::diag::flusher::flush_diagnostics(self); // Process logs
            self.bridge.idle();
        }
    }
}
