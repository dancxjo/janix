use abi::wire::typed::{TypeDef, TypeId};
use alloc::collections::BTreeMap;
use spin::RwLock;

pub struct TypeRegistry {
    types: RwLock<BTreeMap<TypeId, TypeDef>>,
}

impl TypeRegistry {
    pub const fn new() -> Self {
        Self {
            types: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn register_typedef(&self, typedef: TypeDef) -> Result<TypeId, ()> {
        let computed_id = TypeDef::compute_hash(
            typedef.name,
            typedef.version,
            typedef.codec_id,
            &typedef.desc,
            &typedef.constraints,
        );

        if computed_id != typedef.type_id {
            // Hash mismatch
            return Err(());
        }

        let mut map = self.types.write();
        if let Some(existing) = map.get(&typedef.type_id) {
            if existing != &typedef {
                return Err(()); // Collision or replacement attempt
            }
            return Ok(typedef.type_id);
        }

        map.insert(typedef.type_id, typedef.clone());
        Ok(typedef.type_id)
    }

    pub fn get_typedef(&self, type_id: TypeId) -> Option<TypeDef> {
        self.types.read().get(&type_id).cloned()
    }
}

pub static TYPE_REGISTRY: TypeRegistry = TypeRegistry::new();
