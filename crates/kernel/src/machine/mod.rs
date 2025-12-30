use crate::bridge::{FullMachineBridge, ProviderBridge};
use abi::SymbolId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub mod builtin;
pub mod providers;
pub use builtin::BuiltinEndpoint;

pub struct Machine<B: FullMachineBridge> {
    registry: MachineRegistry<B>,
}

pub struct MachineRegistry<B: FullMachineBridge> {
    endpoints: BTreeMap<(SymbolId, u16, SymbolId), Endpoint<B>>,
}

#[derive(Clone, Copy)]
pub struct ProviderMeta {
    pub name: abi::SymbolId,
    pub kind: abi::SymbolId,
    pub lane: abi::SymbolId,
}

#[derive(Clone, Copy)]
pub struct ProviderVtable<B: ProviderBridge + ?Sized> {
    pub call: fn(
        ctx: *const (),
        bridge: &B,
        op: u32,
        req: &[u8],
    ) -> Result<alloc::vec::Vec<u8>, MachineError>,
}

#[derive(Clone, Copy)]
pub enum Endpoint<B: ProviderBridge + ?Sized> {
    Provider {
        meta: ProviderMeta,
        vtable: ProviderVtable<B>,
        ctx: *const (),
    },
    Builtin(BuiltinEndpoint),
}

unsafe impl<B: ProviderBridge + ?Sized> Send for Endpoint<B> {}
unsafe impl<B: ProviderBridge + ?Sized> Sync for Endpoint<B> {}

#[derive(Debug)]
pub enum MachineError {
    NotFound,
    InvalidOp,
    EncodingError,
    InternalError,
}

impl<B: FullMachineBridge> Default for MachineRegistry<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: FullMachineBridge> MachineRegistry<B> {
    pub fn new() -> Self {
        Self {
            endpoints: BTreeMap::new(),
        }
    }
}

impl<B: FullMachineBridge> Default for Machine<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: FullMachineBridge> Machine<B> {
    pub fn new() -> Self {
        Self {
            registry: MachineRegistry::new(),
        }
    }

    /// Check process buffers for validity.
    ///
    /// Stub implementation for v0.2 MVP. Returns Ok always.
    /// TODO: Implement proper buffer validation when IPC is ready.
    pub fn check_buffers(&self, _pid: abi::ProcessId) -> Result<(), MachineError> {
        Ok(())
    }

    pub fn register_builtin(
        &mut self,
        iface: SymbolId,
        ver: u16,
        instance: SymbolId,
        ep: BuiltinEndpoint,
    ) {
        self.registry
            .endpoints
            .insert((iface, ver, instance), Endpoint::Builtin(ep));
    }

    pub fn register_provider(
        &mut self,
        iface: SymbolId,
        ver: u16,
        instance: SymbolId,
        meta: ProviderMeta,
        vtable: ProviderVtable<B>,
        ctx: *const (),
    ) {
        self.registry.endpoints.insert(
            (iface, ver, instance),
            Endpoint::Provider { meta, vtable, ctx },
        );
    }

    pub fn call(
        &self,
        bridge: &B,
        iface: SymbolId,
        ver: u16,
        instance: SymbolId,
        op: u32,
        req: &[u8],
    ) -> Result<Vec<u8>, MachineError> {
        let ep = self
            .registry
            .endpoints
            .get(&(iface, ver, instance))
            .ok_or(MachineError::NotFound)?;

        match ep {
            Endpoint::Provider {
                meta: _,
                vtable,
                ctx,
            } => (vtable.call)(*ctx, bridge, op, req),
            Endpoint::Builtin(builtin) => builtin::dispatch(bridge, *builtin, op, req),
        }
    }

    pub fn reflect_into_graph(&self, graph: &mut crate::graph::GraphStore) {
        use abi::symbols::sym;
        use abi::wire::machine::*;
        use postcard::to_allocvec;
        use thing_models::builtins::ids::THING_LINK_KIND;
        use thing_models::value::ThingBody;

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
        let mut ensure = |g: &mut crate::graph::GraphStore,
                          id: abi::ThingId,
                          kind: abi::ThingId,
                          payload: Vec<u8>| {
            if g.get(id).is_none() {
                let t = thing_models::Thing {
                    id,
                    kind,
                    body: ThingBody { bytes: payload },
                };
                g.insert_seed(t);
            }
        };

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
        let mut ensure_link = |g: &mut crate::graph::GraphStore,
                               from: abi::ThingId,
                               to: abi::ThingId,
                               pred: abi::SymbolId| {
            let x = from.0 ^ pred.0.rotate_left(21) ^ to.0.rotate_left(42);
            let lid = abi::ThingId(mix64(x));

            #[derive(serde::Serialize)]
            struct LinkBody {
                from: abi::ThingId,
                to: abi::ThingId,
                predicate: abi::SymbolId,
            }
            let body = LinkBody {
                from,
                to,
                predicate: pred,
            };

            let typed = abi::wire::typed::TypedBytes {
                type_id: abi::wire::typed::TypeId(THING_LINK_KIND.0 as u128),
                codec_id: abi::wire::typed::CodecId::POSTCARD,
                bytes: to_allocvec(&body).unwrap(),
            };

            ensure(g, lid, THING_LINK_KIND, to_allocvec(&typed).unwrap());
        };

        // 1. Machine
        #[derive(serde::Serialize)]
        struct MachineBody {
            name: &'static str,
        }
        let mach_id = abi::ThingId(sym("machine0").0);
        ensure(
            graph,
            mach_id,
            abi::ThingId(kind_sys_machine.0),
            to_allocvec(&MachineBody { name: "machine0" }).unwrap(),
        );

        // Collect provider metadata
        let mut drivers: BTreeMap<SymbolId, ProviderMeta> = BTreeMap::new();
        for (_key, ep) in self.registry.endpoints.iter() {
            match ep {
                Endpoint::Provider { meta, .. } => {
                    drivers.entry(meta.name).or_insert(*meta);
                }
                Endpoint::Builtin(builtin) => {
                    let meta = builtin::meta_for(*builtin);
                    drivers.entry(meta.name).or_insert(meta);
                }
            }
        }

        #[derive(serde::Serialize)]
        struct DriverBody {
            name: SymbolId,
            lane: SymbolId,
            kind: SymbolId,
            state: &'static str,
        }

        for meta in drivers.values() {
            let drv_id = abi::ThingId(meta.name.0);
            ensure(
                graph,
                drv_id,
                abi::ThingId(kind_sys_driver.0),
                to_allocvec(&DriverBody {
                    name: meta.name,
                    lane: meta.lane,
                    kind: meta.kind,
                    state: "running",
                })
                .unwrap(),
            );
            ensure_link(graph, mach_id, drv_id, pred_has_driver);
        }

        #[derive(serde::Serialize)]
        struct InterfaceBody {
            name: SymbolId,
            version: u16,
        }

        #[derive(serde::Serialize)]
        struct DeviceBody {
            name: SymbolId,
            class: SymbolId,
        }

        for ((iface, ver, instance), ep) in self.registry.endpoints.iter() {
            let meta = match ep {
                Endpoint::Provider { meta, .. } => *meta,
                Endpoint::Builtin(builtin) => builtin::meta_for(*builtin),
            };

            let iface_id = abi::ThingId(iface.0);
            let dev_id = abi::ThingId(instance.0);
            let drv_id = abi::ThingId(meta.name.0);

            ensure(
                graph,
                iface_id,
                abi::ThingId(kind_sys_interface.0),
                to_allocvec(&InterfaceBody {
                    name: *iface,
                    version: *ver,
                })
                .unwrap(),
            );

            let class_sym = match *iface {
                x if x == sym(IFACE_RTC) => sym("rtc"),
                x if x == sym(IFACE_FRAMEBUFFER) => sym("framebuffer"),
                x if x == sym(IFACE_KEYBOARD) => sym("keyboard"),
                x if x == sym(IFACE_MOUSE) => sym("mouse"),
                _ => *iface,
            };

            ensure(
                graph,
                dev_id,
                abi::ThingId(kind_hw_device.0),
                to_allocvec(&DeviceBody {
                    name: *instance,
                    class: class_sym,
                })
                .unwrap(),
            );

            ensure_link(graph, drv_id, iface_id, pred_implements);
            ensure_link(graph, drv_id, dev_id, pred_provides);
            ensure_link(graph, dev_id, iface_id, pred_implements);
            ensure_link(graph, dev_id, drv_id, pred_driven_by);
        }
    }
}
