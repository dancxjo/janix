use abi::wire::typed::{Constraints, TypeDef, TypeDesc, TypeId};
use alloc::vec::Vec;
use postcard;
use serde::{Deserialize, Serialize};

pub use thing_macros::ThingType;

/// Trait for types that can be registered as Things
pub trait ThingType: Sized + 'static + Serialize + for<'de> Deserialize<'de> {
    fn type_id() -> TypeId;

    fn type_desc() -> TypeDesc;

    fn typedef() -> TypeDef;

    fn encode(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    fn decode(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }

    fn validate(&self) -> Result<(), ()> {
        Ok(())
    }
}

// Implement ThingType for primitives
macro_rules! impl_primitive_thing_type {
    ($type:ty, $prim_variant:ident) => {
        impl ThingType for $type {
            fn type_id() -> TypeId {
                // For primitives, we can compute hash on the fly or use fixed IDs.
                // Dynamic hash for now to be consistent with typedef.
                Self::typedef().type_id
            }

            fn type_desc() -> TypeDesc {
                TypeDesc::Primitive(abi::wire::typed::PrimitiveType::$prim_variant)
            }

            fn typedef() -> TypeDef {
                // Primitives might not need full TypeDef registration if they are base types,
                // but consistency is good.
                let desc = Self::type_desc();
                let constraints = Constraints::default();
                let codec_id = abi::wire::typed::CodecId::POSTCARD;
                let version = 1;
                let name = abi::symbols::sym(stringify!($type));

                let type_id = TypeDef::compute_hash(name, version, codec_id, &desc, &constraints);

                TypeDef {
                    type_id,
                    name: abi::symbols::sym(stringify!($type)),
                    codec_id: abi::wire::typed::CodecId::POSTCARD,
                    version: 1,
                    desc: Self::type_desc(),
                    constraints: Constraints::default(),
                }
            }
        }
    };
}

impl_primitive_thing_type!(bool, Bool);
impl_primitive_thing_type!(i64, I64);
impl_primitive_thing_type!(u64, U64);
impl_primitive_thing_type!(f64, F64);
// String and Bytes need special handling or just map to String/Vec<u8>

impl ThingType for alloc::string::String {
    fn type_id() -> TypeId {
        Self::typedef().type_id
    }
    fn type_desc() -> TypeDesc {
        TypeDesc::Primitive(abi::wire::typed::PrimitiveType::String)
    }
    fn typedef() -> TypeDef {
        let desc = Self::type_desc();
        let constraints = Constraints::default();
        let codec_id = abi::wire::typed::CodecId::POSTCARD;
        let version = 1;
        let name = abi::symbols::sym("String");

        let type_id = TypeDef::compute_hash(name, version, codec_id, &desc, &constraints);

        TypeDef {
            type_id,
            name,
            codec_id,
            version,
            desc,
            constraints,
        }
    }
}
