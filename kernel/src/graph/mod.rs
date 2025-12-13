pub mod events;
pub mod index_links;
pub mod index_props;
pub mod schema;
pub mod store;

// Re-export the public API so other crates/modules can continue to use
// `crate::graph::...` exactly as before.
pub use events::*;
pub use index_links::*;
pub use index_props::*;
pub use schema::*;
pub use store::*;

// Keep module-level constructor convenience
#[derive(Clone, Copy, Default, Debug)]
pub struct Graph;

impl Graph {
    #[inline]
    pub fn new() -> Self {
        Graph
    }

    #[inline]
    pub fn create_thing(
        &mut self,
        kind: &'static str,
        props: &[(abi::PropKey, abi::PropValue)],
    ) -> Option<abi::ThingId> {
        create_thing(kind, props)
    }

    #[inline]
    pub fn update_thing(
        &mut self,
        id: abi::ThingId,
        props: &[(abi::PropKey, abi::PropValue)],
    ) -> bool {
        update_thing(id, props)
    }

    #[inline]
    pub fn get_thing(
        &self,
        id: abi::ThingId,
    ) -> Option<(
        &'static str,
        &'static [Option<(abi::PropKey, abi::PropValue)>],
    )> {
        get_thing(id)
    }

    #[inline]
    pub fn add_link(&mut self, src: abi::ThingId, pred: abi::Predicate, dst: abi::ThingId) -> bool {
        create_link(src, pred, dst).is_some()
    }

    #[inline]
    pub fn remove_link(
        &mut self,
        src: abi::ThingId,
        pred: abi::Predicate,
        dst: abi::ThingId,
    ) -> bool {
        remove_link(src, pred, dst)
    }

    #[inline]
    pub fn neighbors(
        &self,
        src: abi::ThingId,
        pred: abi::Predicate,
        out: &mut [Option<abi::ThingId>],
    ) {
        neighbors(src, pred, out)
    }
}

/// Initialize the graph subsystem (wraps submodule inits)
pub fn init() {
    // Clear storage, schemas, listeners and indexes
    store::init();
    schema::init();
    events::init();
    index_links::link_index_mut().clear();
    index_props::clear();

    static LINK_SCHEMA: &[(&str, abi::PropType)] = &[
        (crate::graph_kinds::PROP_LINK_SRC, abi::PropType::U64),
        (crate::graph_kinds::PROP_LINK_DST, abi::PropType::U64),
        (crate::graph_kinds::PROP_LINK_PRED, abi::PropType::U64),
    ];
    let _ = schema::register_schema(
        crate::graph_kinds::KIND_LINK,
        "A graph link connecting Things by predicate",
        LINK_SCHEMA,
        &[
            crate::graph_kinds::PROP_LINK_DST,
            crate::graph_kinds::PROP_LINK_PRED,
        ],
    );
}
pub mod sink;
