use abi::{KernelRequest, KernelResponse, NodeId, ThingId, PropKey, PropValue, PropType};

/// Print a line to the kernel log
pub fn println(message: &'static str) {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::Log { message };
    sys.syscall(request);
}

/// Query a node in the graph
pub fn graph_query(node_id: NodeId) -> Option<u64> {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::GraphQuery { node_id };
    match sys.syscall(request) {
        KernelResponse::NodeData { node_id: _, value } => Some(value),
        _ => None,
    }
}

/// Create a transaction
pub fn create_transaction() -> Option<abi::TransactionId> {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::CreateTransaction;
    match sys.syscall(request) {
        KernelResponse::TransactionCreated { tx_id } => Some(tx_id),
        _ => None,
    }
}

/// Commit a transaction
pub fn commit_transaction(tx_id: abi::TransactionId) -> bool {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::CommitTransaction { tx_id };
    matches!(sys.syscall(request), KernelResponse::Success { .. })
}

pub trait Thing: Sized {
    const KIND: &'static str;
    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>);
    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self;
    
    /// Static schema for this Thing, used for registration.
    fn schema() -> &'static [(&'static str, PropType)];
}

pub fn create_thing<T: Thing>(thing: &T) -> Option<ThingId> {
    let sys = userland_rt::get_sys();
    let mut props_vec = Vec::new();
    thing.to_props(&mut props_vec);
    let props_slice = Box::leak(props_vec.into_boxed_slice());
    
    let request = KernelRequest::ThingCreate {
        kind: T::KIND,
        props: props_slice,
    };
    match sys.syscall(request) {
        KernelResponse::ThingCreated { id } => Some(id),
        _ => None,
    }
}

pub fn load_thing<T: Thing>(id: ThingId) -> Option<T> {
    let sys = userland_rt::get_sys();
    let request = KernelRequest::ThingGet { id };
    match sys.syscall(request) {
        KernelResponse::ThingData { id, kind, props } => {
            if kind != T::KIND {
                return None;
            }
            Some(T::from_props(id, props))
        }
        _ => None,
    }
}

/// Register a schema for a Thing type
pub fn register_schema_for<T: Thing>() -> bool {
    let sys = userland_rt::get_sys();
    let schema = T::schema();
    match sys.syscall(KernelRequest::SchemaRegister {
        kind: T::KIND,
        props: schema,
    }) {
        KernelResponse::SchemaRegistered { .. } => true,
        _ => false,
    }
}
