use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use abi::{PropKey, PropValue, PropType, ThingId, Predicate, graph_kinds};

#[derive(Debug, Clone)]
pub struct GeoNode {
    pub id: ThingId,
    pub kind: String,
    // We optionally keep props if we want to show them later
    pub props: Vec<(String, PropValue)>,
    
    // Layout
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct GeoEdge {
    pub id: ThingId,
    pub src: ThingId,
    pub dst: ThingId,
    pub pred: Predicate,
}

#[derive(Debug, Default)]
pub struct Scene {
    pub nodes: BTreeMap<ThingId, GeoNode>,
    pub edges: Vec<GeoEdge>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct LinkThing {
    pub id: ThingId,
    pub src: ThingId,
    pub dst: ThingId,
    pub pred: Predicate,
}

impl thing_os::Thing for LinkThing {
    const KIND: &'static str = "Link";
    const DESCRIPTION: &'static str = "Graph Link";
    
    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("link_src", PropType::U64),
            ("link_dst", PropType::U64),
            ("link_pred", PropType::U64),
        ]
    }

    fn to_props(&self, _: &mut Vec<(PropKey, PropValue)>) {}

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut src = ThingId(0);
        let mut dst = ThingId(0);
        let mut pred = Predicate(0);

        for (k, v) in props.iter().flatten() {
            match *k {
                "link_src" => if let PropValue::U64(val) = v { src = ThingId(*val) },
                "link_dst" => if let PropValue::U64(val) = v { dst = ThingId(*val) },
                "link_pred" => if let PropValue::U64(val) = v { pred = Predicate(*val) },
                _ => {}
            }
        }
        LinkThing { id, src, dst, pred }
    }
}
