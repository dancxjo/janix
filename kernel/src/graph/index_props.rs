extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use abi::{PropKey, PropValue, ThingId};

pub struct PropertyIndex {
    pub by_string: BTreeMap<&'static str, BTreeMap<String, Vec<ThingId>>>,
    pub by_int: BTreeMap<&'static str, BTreeMap<i64, Vec<ThingId>>>,
    pub by_bool: BTreeMap<&'static str, BTreeMap<bool, Vec<ThingId>>>,
}

impl PropertyIndex {
    pub fn new() -> Self {
        Self {
            by_string: BTreeMap::new(),
            by_int: BTreeMap::new(),
            by_bool: BTreeMap::new(),
        }
    }
}

static mut PROPERTY_INDEX: Option<PropertyIndex> = None;

pub(crate) fn add_to_prop_index(id: ThingId, key: PropKey, value: &PropValue) {
    unsafe {
        let index_ptr = &raw mut PROPERTY_INDEX;
        let index = (*index_ptr).get_or_insert_with(PropertyIndex::new);

        match value {
            PropValue::Str(s) => {
                index
                    .by_string
                    .entry(key)
                    .or_default()
                    .entry(s.clone())
                    .or_default()
                    .push(id);
            }
            PropValue::I64(v) => {
                index
                    .by_int
                    .entry(key)
                    .or_default()
                    .entry(*v)
                    .or_default()
                    .push(id);
            }
            PropValue::Bool(v) => {
                index
                    .by_bool
                    .entry(key)
                    .or_default()
                    .entry(*v)
                    .or_default()
                    .push(id);
            }
            _ => {}
        }
    }
}

pub(crate) fn remove_from_prop_index(id: ThingId, key: PropKey, value: &PropValue) {
    unsafe {
        let index_ptr = &raw mut PROPERTY_INDEX;
        if let Some(index) = (*index_ptr).as_mut() {
            match value {
                PropValue::Str(s) => {
                    if let Some(map) = index.by_string.get_mut(key) {
                        if let Some(vec) = map.get_mut(s) {
                            if let Some(pos) = vec.iter().position(|x| *x == id) {
                                vec.swap_remove(pos);
                            }
                        }
                    }
                }
                PropValue::I64(v) => {
                    if let Some(map) = index.by_int.get_mut(key) {
                        if let Some(vec) = map.get_mut(v) {
                            if let Some(pos) = vec.iter().position(|x| *x == id) {
                                vec.swap_remove(pos);
                            }
                        }
                    }
                }
                PropValue::Bool(v) => {
                    if let Some(map) = index.by_bool.get_mut(key) {
                        if let Some(vec) = map.get_mut(v) {
                            if let Some(pos) = vec.iter().position(|x| *x == id) {
                                vec.swap_remove(pos);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn clear() {
    unsafe {
        PROPERTY_INDEX = None;
    }
}
