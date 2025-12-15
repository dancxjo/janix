extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use abi::{PropKey, PropType, PropValue, ThingId};

const MAX_SCHEMA_PROPS: usize = 32;
const MAX_SCHEMAS: usize = 64;

#[derive(Debug, Clone, Copy)]
pub struct Schema {
    pub kind: &'static str,
    pub description: &'static str,
    pub props: [Option<(&'static str, PropType)>; MAX_SCHEMA_PROPS],
    pub indexed_props: [Option<&'static str>; MAX_SCHEMA_PROPS],
}

static mut SCHEMAS: [Option<Schema>; MAX_SCHEMAS] = [None; MAX_SCHEMAS];

static mut KIND_MAP: Option<BTreeMap<&'static str, ThingId>> = None;

static mut KIND_INDEX: Option<BTreeMap<ThingId, Vec<ThingId>>> = None;

use crate::graph_kinds;
use crate::graph::index_props::{add_to_prop_index, remove_from_prop_index};

const ERR_NO_SCHEMA: &str = "No schema registered for this kind";
const ERR_SCHEMA_ALREADY_REGISTERED: &str = "Schema already registered";
const ERR_SCHEMA_STORAGE_FULL: &str = "Schema storage full";
const ERR_TYPE_MISMATCH: &str = "Property type mismatch";
const ERR_PROPERTY_NOT_IN_SCHEMA: &str = "Property not in schema";
const ERR_TOO_MANY_SCHEMA_PROPS: &str = "Too many properties in schema";

pub fn register_schema(
    kind: &'static str,
    description: &'static str,
    props: &'static [(&'static str, PropType)],
    indexed_props: &'static [&'static str],
) -> Result<(), &'static str> {
    unsafe {
        let schemas = &raw mut SCHEMAS;

        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    return Err(ERR_SCHEMA_ALREADY_REGISTERED);
                }
            }
        }

        for slot in (*schemas).iter_mut() {
            if slot.is_none() {
                let mut schema_props = [None; MAX_SCHEMA_PROPS];
                let mut schema_indexed_props = [None; MAX_SCHEMA_PROPS];

                for (i, prop) in props.iter().enumerate() {
                    if i >= MAX_SCHEMA_PROPS {
                        return Err(ERR_TOO_MANY_SCHEMA_PROPS);
                    }
                    schema_props[i] = Some(*prop);
                }

                for (i, prop) in indexed_props.iter().enumerate() {
                    if i >= MAX_SCHEMA_PROPS {
                        return Err(ERR_TOO_MANY_SCHEMA_PROPS);
                    }
                    schema_indexed_props[i] = Some(*prop);
                }

                *slot = Some(Schema {
                    kind,
                    description,
                    props: schema_props,
                    indexed_props: schema_indexed_props,
                });
                return Ok(());
            }
        }

        Err(ERR_SCHEMA_STORAGE_FULL)
    }
}

pub fn get_schema_props(kind: &'static str) -> Option<&'static [Option<(&'static str, PropType)>]> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    return Some(&s.props[..]);
                }
            }
        }
        None
    }
}

pub fn get_schema_indexed_props(kind: &str) -> Option<&'static [Option<&'static str>]> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    return Some(&s.indexed_props[..]);
                }
            }
        }
        None
    }
}

pub fn get_schema_description(kind: &'static str) -> Option<&'static str> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    return Some(s.description);
                }
            }
        }
        None
    }
}

    pub fn validate_props(kind: &'static str, props: &[(PropKey, PropValue)]) -> Result<(), &'static str> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        let schema = (*schemas)
            .iter()
            .find_map(|s| s.as_ref().filter(|s| s.kind == kind));

        let schema = match schema {
            Some(s) => s,
            None => return Err(ERR_NO_SCHEMA),
        };

        for (key, value) in props {
            let mut found = false;
            for prop_def in schema.props.iter() {
                if let Some((schema_key, schema_type)) = prop_def {
                    if *schema_key == *key {
                        found = true;
                        let type_matches = match (schema_type, value) {
                            (PropType::U64, PropValue::U64(_)) => true,
                            (PropType::I64, PropValue::I64(_)) => true,
                            (PropType::Bool, PropValue::Bool(_)) => true,
                            (PropType::Str, PropValue::Str(_)) => true,
                            _ => false,
                        };

                        if !type_matches {
                            return Err(ERR_TYPE_MISMATCH);
                        }
                        break;
                    }
                }
            }

            if !found {
                 let schema_keys: alloc::vec::Vec<alloc::string::String> = schema.props.iter().filter_map(|p| p.map(|(k, _)| alloc::format!("{}", k))).collect();
                 let msg = alloc::format!("Validation failed: key '{}' not found in schema for kind '{}'. Schema keys: {:?}", key, kind, schema_keys);
                 let leaked = alloc::boxed::Box::leak(msg.into_boxed_str());
                 crate::log::log_message(leaked);
                 return Err(ERR_PROPERTY_NOT_IN_SCHEMA);
            }
        }

        Ok(())
    }
}

pub fn ensure_kind_exists(kind: &'static str) -> ThingId {
    unsafe {
        let map_ptr = &raw mut KIND_MAP;
        let map = (*map_ptr).get_or_insert_with(BTreeMap::new);
        if let Some(id) = map.get(kind) {
            return *id;
        }

        if kind == graph_kinds::KIND_KIND {
            let (idx, generation) = crate::graph::store::peek_next_slab_id();
            let id = ThingId::new(idx, generation);
            map.insert(kind, id);

            crate::graph::store::create_thing_internal(
                kind,
                Some(id),
                &[(graph_kinds::PROP_NAME, PropValue::Str(String::from("Kind")))],
                None,
            );
            return id;
        }

        let kind_kind_id = ensure_kind_exists(graph_kinds::KIND_KIND);

        let id = crate::graph::store::create_thing_internal(
            graph_kinds::KIND_KIND,
            Some(kind_kind_id),
            &[(graph_kinds::PROP_NAME, PropValue::Str(String::from(kind)))],
            None,
        )
        .expect("Failed to create kind thing");

        let map_ptr = &raw mut KIND_MAP;
        let map = (*map_ptr).get_or_insert_with(BTreeMap::new);
        map.insert(kind, id);
        id
    }
}

pub fn is_prop_indexed(kind: &str, key: &str) -> bool {
    get_schema_indexed_props(kind)
        .map(|props| props.iter().any(|p| *p == Some(key)))
        .unwrap_or(false)
}

pub(crate) fn add_to_kind_index(id: ThingId, kind_id: ThingId) {
    unsafe {
        let index = &raw mut KIND_INDEX;
        (*index)
            .get_or_insert_with(BTreeMap::new)
            .entry(kind_id)
            .or_default()
            .push(id);
    }
}

pub(crate) fn remove_from_kind_index(id: ThingId, kind_id: ThingId) {
    unsafe {
        let index = &raw mut KIND_INDEX;
        if let Some(map) = (*index).as_mut() {
            if let Some(vec) = map.get_mut(&kind_id) {
                if let Some(pos) = vec.iter().position(|x| *x == id) {
                    vec.swap_remove(pos);
                }
            }
        }
    }
}

pub fn resolve_key(kind: &str, key: &str) -> Option<&'static str> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                if s.kind == kind {
                    for prop_def in s.props.iter() {
                         if let Some((schema_key, _)) = prop_def {
                             if *schema_key == key {
                                 return Some(schema_key);
                             }
                         }
                    }
                }
            }
        }
        None
    }
}

pub fn get_key_from_id(kind: &str, key_id: u32) -> Option<&'static str> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                 if s.kind == kind {
                     if (key_id as usize) < MAX_SCHEMA_PROPS {
                         return s.props[key_id as usize].map(|(k, _)| k);
                     }
                 }
            }
        }
        None
    }
}

pub fn get_id_from_key(kind: &str, key: &str) -> Option<u32> {
    unsafe {
        let schemas = &raw const SCHEMAS;
        for schema in (*schemas).iter() {
            if let Some(s) = schema {
                 if s.kind == kind {
                     for (i, prop_def) in s.props.iter().enumerate() {
                         if let Some((schema_key, _)) = prop_def {
                             if *schema_key == key {
                                 return Some(i as u32);
                             }
                         }
                     }
                 }
            }
        }
        None
    }
}

pub fn init() {
    unsafe {
        let schemas = &raw mut SCHEMAS;
        for slot in (*schemas).iter_mut() {
            *slot = None;
        }
        KIND_MAP = None;
        KIND_INDEX = None;
    }
}

