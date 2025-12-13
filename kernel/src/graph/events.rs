use abi::Link;
pub use abi::graph_ops::GraphEvent;

pub type GraphListener = fn(&GraphEvent);



#[derive(Clone, Copy)]
struct NodeListener {
    kind: &'static str,
    listener: GraphListener,
}

#[derive(Clone, Copy)]
struct PropListener {
    kind: &'static str,
    key: &'static str,
    listener: GraphListener,
}

#[derive(Clone, Copy)]
struct LinkListener {
    pred: abi::Predicate,
    listener: GraphListener,
}

const MAX_NODE_LISTENERS: usize = 16;
const MAX_PROP_LISTENERS: usize = 16;
const MAX_LINK_LISTENERS: usize = 16;

static mut NODE_CREATED_LISTENERS: [Option<NodeListener>; MAX_NODE_LISTENERS] = [const { None }; MAX_NODE_LISTENERS];
static mut NODE_DELETED_LISTENERS: [Option<NodeListener>; MAX_NODE_LISTENERS] = [const { None }; MAX_NODE_LISTENERS];
static mut PROP_CHANGED_LISTENERS: [Option<PropListener>; MAX_PROP_LISTENERS] = [const { None }; MAX_PROP_LISTENERS];
static mut LINK_ADDED_LISTENERS: [Option<LinkListener>; MAX_LINK_LISTENERS] = [const { None }; MAX_LINK_LISTENERS];
static mut LINK_REMOVED_LISTENERS: [Option<LinkListener>; MAX_LINK_LISTENERS] = [const { None }; MAX_LINK_LISTENERS];

fn dispatch_event_inner(event: &GraphEvent) {
    #[cfg(feature = "journal")]
    crate::journal::push(event.clone());
    unsafe {
        match event {
            GraphEvent::ThingCreated { kind, .. } => {
                let listeners = &raw const NODE_CREATED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::ThingDeleted { kind, .. } => {
                let listeners = &raw const NODE_DELETED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::PropUpdated { kind, key, .. } => {
                let listeners = &raw const PROP_CHANGED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.kind == *kind && slot.key == *key {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::LinkAdded(link) => {
                let listeners = &raw const LINK_ADDED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.pred == link.pred {
                        (slot.listener)(event);
                    }
                }
            }
            GraphEvent::LinkRemoved(link) => {
                let listeners = &raw const LINK_REMOVED_LISTENERS;
                for slot in (*listeners).iter().flatten() {
                    if slot.pred == link.pred {
                        (slot.listener)(event);
                    }
                }
            }
        }
    }
}

pub(crate) fn dispatch_event(event: &GraphEvent) {
    dispatch_event_inner(event)
}

pub fn subscribe_node_created(kind: &'static str, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut NODE_CREATED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(NodeListener { kind, listener });
                return;
            }
        }
    }
}

pub fn subscribe_node_deleted(kind: &'static str, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut NODE_DELETED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(NodeListener { kind, listener });
                return;
            }
        }
    }
}

pub fn subscribe_prop_changed(kind: &'static str, key: &'static str, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut PROP_CHANGED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(PropListener { kind, key, listener });
                return;
            }
        }
    }
}

pub fn subscribe_link_added(pred: abi::Predicate, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut LINK_ADDED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(LinkListener { pred, listener });
                return;
            }
        }
    }
}

pub fn subscribe_link_removed(pred: abi::Predicate, listener: GraphListener) {
    unsafe {
        let listeners = &raw mut LINK_REMOVED_LISTENERS;
        for slot in (*listeners).iter_mut() {
            if slot.is_none() {
                *slot = Some(LinkListener { pred, listener });
                return;
            }
        }
    }
}

pub fn init() {
    unsafe {
        let node_created = &raw mut NODE_CREATED_LISTENERS;
        for slot in (*node_created).iter_mut() {
            *slot = None;
        }
        let node_deleted = &raw mut NODE_DELETED_LISTENERS;
        for slot in (*node_deleted).iter_mut() {
            *slot = None;
        }
        let prop_changed = &raw mut PROP_CHANGED_LISTENERS;
        for slot in (*prop_changed).iter_mut() {
            *slot = None;
        }
        let link_added = &raw mut LINK_ADDED_LISTENERS;
        for slot in (*link_added).iter_mut() {
            *slot = None;
        }
        let link_removed = &raw mut LINK_REMOVED_LISTENERS;
        for slot in (*link_removed).iter_mut() {
            *slot = None;
        }
    }
}
