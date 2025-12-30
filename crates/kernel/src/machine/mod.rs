use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use crate::bridge::HardwareBridge;
use abi::SymbolId;

pub mod builtin;
pub use builtin::BuiltinEndpoint;

pub struct Machine {
    registry: MachineRegistry,
    pub fb_info: Option<abi::wire::machine::FbGetInfoResp>,
}

pub struct MachineRegistry {
    endpoints: BTreeMap<(SymbolId, u16, SymbolId), Endpoint>,
}

#[derive(Clone, Copy)]
pub enum Endpoint {
    Builtin(BuiltinEndpoint),
}

#[derive(Debug)]
pub enum MachineError {
    NotFound,
    InvalidOp,
    EncodingError,
    InternalError,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registry: MachineRegistry {
                endpoints: BTreeMap::new(),
            },
            fb_info: None,
        }
    }

    pub fn register_builtin(&mut self, iface: SymbolId, ver: u16, instance: SymbolId, ep: BuiltinEndpoint) {
        self.registry.endpoints.insert((iface, ver, instance), Endpoint::Builtin(ep));
    }

    pub fn call<B: HardwareBridge>(
        &self,
        bridge: &B,
        iface: SymbolId,
        ver: u16,
        instance: SymbolId,
        op: u32,
        req: &[u8],
    ) -> Result<Vec<u8>, MachineError> {
        let ep = self.registry.endpoints.get(&(iface, ver, instance)).ok_or(MachineError::NotFound)?;

        match ep {
            Endpoint::Builtin(builtin) => {
                builtin::dispatch(self, bridge, *builtin, op, req)
            }
        }
    }

    pub fn reflect_into_graph(&self, graph: &mut crate::graph::GraphStore) {
        use abi::wire::machine::*;
        use abi::symbols::sym;
        use crate::graph::GraphStore;
        use thing_models::value::ThingBody;
        use thing_models::builtins::ids::{THING_LINK_KIND, THING_KIND_KIND};
        use postcard::to_allocvec;

        // Define symbols
        const KIND_SYS_MACHINE: SymbolId = sym("sys.machine");
        const KIND_SYS_INTERFACE: SymbolId = sym("sys.interface");
        const KIND_SYS_DRIVER: SymbolId = sym("sys.driver");
        const KIND_HW_DEVICE: SymbolId = sym("hw.device");

        const PRED_HAS_DRIVER: SymbolId = sym("HAS_DRIVER");
        const PRED_IMPLEMENTS: SymbolId = sym("IMPLEMENTS");
        const PRED_PROVIDES: SymbolId = sym("PROVIDES");
        const PRED_DRIVEN_BY: SymbolId = sym("DRIVEN_BY");

        // Seed Kinds
        let mut seed_kind = |g: &mut GraphStore, id: SymbolId| {
            let tid = abi::ThingId(id.0);
            if g.get(tid).is_none() {
                 let t = thing_models::Thing {
                     id: tid,
                     kind: THING_KIND_KIND,
                     body: ThingBody { bytes: Vec::new() },
                 };
                 g.insert_seed(t);
            }
        };

        seed_kind(graph, KIND_SYS_MACHINE);
        seed_kind(graph, KIND_SYS_INTERFACE);
        seed_kind(graph, KIND_SYS_DRIVER);
        seed_kind(graph, KIND_HW_DEVICE);

        // Helpers
        let mut create = |g: &mut GraphStore, kind: SymbolId, payload: Vec<u8>| {
             g.create_thing(abi::ThingId(kind.0), ThingBody { bytes: payload })
        };

        let mut link = |g: &mut GraphStore, from: abi::ThingId, to: abi::ThingId, pred: SymbolId| {
            #[derive(serde::Serialize)]
            struct LinkBody { from: abi::ThingId, to: abi::ThingId, predicate: SymbolId }
            let body = LinkBody { from, to, predicate: pred };
            let payload = to_allocvec(&body).unwrap();
            g.create_thing(THING_LINK_KIND, ThingBody { bytes: payload });
        };

        // 1. Create Machine
        #[derive(serde::Serialize)]
        struct MachineBody { name: &'static str }
        let mach_id = create(graph, KIND_SYS_MACHINE, to_allocvec(&MachineBody{name: "machine0"}).unwrap());

        // 2. Create Driver
        #[derive(serde::Serialize)]
        struct DriverBody { name: &'static str, lane: &'static str, state: &'static str }
        let driver_id = create(graph, KIND_SYS_DRIVER, to_allocvec(&DriverBody{
            name: "bridge_builtin", lane: "builtin", state: "running"
        }).unwrap());

        link(graph, mach_id, driver_id, PRED_HAS_DRIVER);

        // 3. Create Interfaces and Devices
        #[derive(serde::Serialize)]
        struct InterfaceBody { name: SymbolId, version: u16 }

        #[derive(serde::Serialize)]
        struct DeviceBody { name: SymbolId, class: &'static str }

        let ifaces = [
            (IFACE_RTC, "rtc", "rtc0"),
            (IFACE_FRAMEBUFFER, "framebuffer", "fb0"),
            (IFACE_KEYBOARD, "keyboard", "kbd0"),
            (IFACE_MOUSE, "mouse", "mouse0"),
        ];

        for (iface_str, class, dev_name) in ifaces {
             let iface_sym = sym(iface_str);
             let dev_sym = sym(dev_name);

             // Interface Node
             let iface_id = create(graph, KIND_SYS_INTERFACE, to_allocvec(&InterfaceBody{
                 name: iface_sym, version: 1
             }).unwrap());

             // Device Node
             let dev_id = create(graph, KIND_HW_DEVICE, to_allocvec(&DeviceBody{
                 name: dev_sym, class
             }).unwrap());

             // Links
             link(graph, driver_id, iface_id, PRED_IMPLEMENTS);
             link(graph, driver_id, dev_id, PRED_PROVIDES);
             link(graph, dev_id, iface_id, PRED_IMPLEMENTS);
             link(graph, dev_id, driver_id, PRED_DRIVEN_BY);
        }
    }
}
