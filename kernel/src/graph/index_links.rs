use abi::{Link, Predicate, ThingId};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
struct LinkSlot {
    link: Link,
    deleted: bool,
}

#[derive(Default, Debug)]
pub struct LinkIndex {
    links: BTreeMap<ThingId, LinkSlot>,
    by_pred_outgoing: BTreeMap<Predicate, BTreeMap<ThingId, Vec<ThingId>>>,
    by_pred_incoming: BTreeMap<Predicate, BTreeMap<ThingId, Vec<ThingId>>>,
}

impl LinkIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.links.clear();
        self.by_pred_outgoing.clear();
        self.by_pred_incoming.clear();
    }

    pub fn insert(&mut self, link: Link) {
        let previous = self
            .links
            .get(&link.id)
            .filter(|slot| !slot.deleted)
            .map(|slot| slot.link);

        if let Some(existing_link) = previous {
            self.remove_from_indexes(link.id, &existing_link);
        }

        let slot = LinkSlot {
            link,
            deleted: false,
        };
        self.add_to_indexes(slot.link.id, &slot.link);
        self.links.insert(slot.link.id, slot);
    }

    pub fn remove(&mut self, id: ThingId) -> bool {
        if let Some(slot) = self.links.get_mut(&id) {
            if slot.deleted {
                return false;
            }
            let link = slot.link;
            slot.deleted = true;
            self.remove_from_indexes(id, &link);
            return true;
        }
        false
    }

    pub fn link(&self, id: ThingId) -> Option<&Link> {
        self.links
            .get(&id)
            .and_then(|slot| (!slot.deleted).then(|| &slot.link))
    }

    pub fn links_from(&self, _src: ThingId) -> &[ThingId] {
        &[]
    }

    pub fn links_to(&self, _dst: ThingId) -> &[ThingId] {
        &[]
    }

    pub fn links_with_pred(&self, _pred: Predicate) -> &[ThingId] {
        &[]
    }

    pub fn links_from_pred(&self, src: ThingId, pred: Predicate) -> &[ThingId] {
        self.by_pred_outgoing
            .get(&pred)
            .and_then(|map| map.get(&src).map(|v| v.as_slice()))
            .unwrap_or(&[])
    }

    pub fn links_to_pred(&self, dst: ThingId, pred: Predicate) -> &[ThingId] {
        self.by_pred_incoming
            .get(&pred)
            .and_then(|map| map.get(&dst).map(|v| v.as_slice()))
            .unwrap_or(&[])
    }

    pub fn neighbor_dsts(
        &self,
        src: ThingId,
        pred: Predicate,
    ) -> impl Iterator<Item = ThingId> + '_ {
        self.links_from_pred(src, pred)
            .iter()
            .filter_map(|id| self.link(*id))
            .map(|link| link.dst)
    }

    pub fn collect_incident_links(&self, id: ThingId, out: &mut Vec<ThingId>) {
        for map in self.by_pred_outgoing.values() {
            if let Some(v) = map.get(&id) {
                out.extend_from_slice(v);
            }
        }
        for map in self.by_pred_incoming.values() {
            if let Some(v) = map.get(&id) {
                out.extend_from_slice(v);
            }
        }
    }

    fn add_to_indexes(&mut self, id: ThingId, link: &Link) {
        self.by_pred_outgoing
            .entry(link.pred)
            .or_default()
            .entry(link.src)
            .or_default()
            .push(id);

        self.by_pred_incoming
            .entry(link.pred)
            .or_default()
            .entry(link.dst)
            .or_default()
            .push(id);
    }

    fn remove_from_indexes(&mut self, id: ThingId, link: &Link) {
        if let Some(map) = self.by_pred_outgoing.get_mut(&link.pred) {
            if let Some(vec) = map.get_mut(&link.src) {
                if let Some(pos) = vec.iter().position(|e| *e == id) {
                    vec.remove(pos);
                }
                if vec.is_empty() {
                    map.remove(&link.src);
                }
            }
            if map.is_empty() {
                self.by_pred_outgoing.remove(&link.pred);
            }
        }

        if let Some(map) = self.by_pred_incoming.get_mut(&link.pred) {
            if let Some(vec) = map.get_mut(&link.dst) {
                if let Some(pos) = vec.iter().position(|e| *e == id) {
                    vec.remove(pos);
                }
                if vec.is_empty() {
                    map.remove(&link.dst);
                }
            }
            if map.is_empty() {
                self.by_pred_incoming.remove(&link.pred);
            }
        }
    }
}

static mut LINK_INDEX: Option<LinkIndex> = None;

pub(crate) fn link_index_mut() -> &'static mut LinkIndex {
    unsafe {
        let link_index = &raw mut LINK_INDEX;
        if (*link_index).is_none() {
            *link_index = Some(LinkIndex::new());
        }
        (*link_index).as_mut().unwrap()
    }
}

pub(crate) fn link_index_ref() -> &'static LinkIndex {
    unsafe {
        let link_index = &raw mut LINK_INDEX;
        if (*link_index).is_none() {
            *link_index = Some(LinkIndex::new());
        }
        (*link_index).as_ref().unwrap()
    }
}

/// Iterate all live links and call `f` for each one.
pub fn for_each_link<F>(mut f: F)
where
    F: FnMut(&Link),
{
    let idx = link_index_ref();
    for slot in idx.links.values() {
        if !slot.deleted {
            f(&slot.link);
        }
    }
}
