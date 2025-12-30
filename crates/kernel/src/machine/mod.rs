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
        use thing_models::builtins::ids::{THING_LINK_KIND};
        use postcard::to_allocvec;

        // Symbols
        let kind_sys_machine = sym("sys.machine");
        let kind_sys_interface = sym("sys.interface");
        let kind_sys_driver = sym("sys.driver");
        let kind_hw_device = sym("hw.device");

        // Namespaced Predicates
        let pred_has_driver = sym("sys.has_driver");
        let pred_implements = sym("sys.implements");
        let pred_provides = sym("sys.provides");
        let pred_driven_by = sym("sys.driven_by");

        // Helper: Ensure a thing exists with a specific ID
        let mut ensure = |g: &mut GraphStore, id: abi::ThingId, kind: abi::ThingId, payload: Vec<u8>| {
             if g.get(id).is_none() {
                 let t = thing_models::Thing {
                     id,
                     kind,
                     body: ThingBody { bytes: payload },
                 };
                 g.insert_seed(t);
             }
        };

        // Note: Kinds are assumed to be seeded by seed_builtins or other mechanism.
        // We do not re-seed them here to avoid conflicts or bad schema.

        // Mixer for link IDs
        let mix64 = |mut x: u64| -> u64 {
            x ^= x >> 30;
            x = x.wrapping_mul(0xbf58476d1ce4e5b9);
            x ^= x >> 27;
            x = x.wrapping_mul(0x94d049bb133111eb);
            x ^= x >> 31;
            x
        };

        // Helper: Ensure Link
        let mut ensure_link = |g: &mut GraphStore, from: abi::ThingId, to: abi::ThingId, pred: abi::SymbolId| {
            let x = from.0 ^ pred.0.rotate_left(21) ^ to.0.rotate_left(42);
            let lid = abi::ThingId(mix64(x));

            #[derive(serde::Serialize)]
            struct LinkBody { from: abi::ThingId, to: abi::ThingId, predicate: abi::SymbolId }
            let body = LinkBody { from, to, predicate: pred };
            ensure(g, lid, THING_LINK_KIND, to_allocvec(&body).unwrap());
        };

        // 1. Machine
        #[derive(serde::Serialize)]
        struct MachineBody { name: &'static str }
        let mach_id = abi::ThingId(sym("machine0").0);
        ensure(graph, mach_id, abi::ThingId(kind_sys_machine.0), to_allocvec(&MachineBody{name: "machine0"}).unwrap());

        // 2. Driver
        #[derive(serde::Serialize)]
        struct DriverBody { name: &'static str, lane: &'static str, state: &'static str }
        let drv_id = abi::ThingId(sym("bridge_builtin").0);
        ensure(graph, drv_id, abi::ThingId(kind_sys_driver.0), to_allocvec(&DriverBody{
            name: "bridge_builtin", lane: "builtin", state: "running"
        }).unwrap());

        ensure_link(graph, mach_id, drv_id, pred_has_driver);

        // 3. Interfaces and Devices
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

             let iface_id = abi::ThingId(iface_sym.0);
             let dev_id = abi::ThingId(dev_sym.0);

             // Interface Node
             ensure(graph, iface_id, abi::ThingId(kind_sys_interface.0), to_allocvec(&InterfaceBody{
                 name: iface_sym, version: 1
             }).unwrap());

             // Device Node
             ensure(graph, dev_id, abi::ThingId(kind_hw_device.0), to_allocvec(&DeviceBody{
                 name: dev_sym, class
             }).unwrap());

             // Links
             ensure_link(graph, drv_id, iface_id, pred_implements);
             ensure_link(graph, drv_id, dev_id, pred_provides);
             ensure_link(graph, dev_id, iface_id, pred_implements);
             ensure_link(graph, dev_id, drv_id, pred_driven_by);
        }
    }
}
