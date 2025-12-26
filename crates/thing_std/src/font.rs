use models::font::FontBody;
use crate::ThingType;
use abi::wire::typed::{TypeId, TypeDesc, TypeDef, PrimitiveType, Constraints, CodecId};

// FontBody is a struct wrapper around Vec<u8>
// It serializes to a struct or just bytes?
// models::font::FontBody derives Serialize/Deserialize.
// So it is a Struct.

impl ThingType for FontBody {
    fn type_id() -> TypeId {
        Self::typedef().type_id
    }

    fn type_desc() -> TypeDesc {
        // It's a struct with one field "data" which is Vec<u8> (Bytes)
        // But we can just rely on the derive macro in userspace if we had it.
        // Here we implement manually.
        use abi::wire::typed::{Field, Variant};
        use abi::symbols;

        TypeDesc::Struct {
            fields: alloc::vec![
                Field {
                    name: symbols::sym("data"),
                    type_desc: TypeDesc::Primitive(PrimitiveType::Bytes),
                    optional: false,
                }
            ]
        }
    }

    fn typedef() -> TypeDef {
         let desc = Self::type_desc();
         let constraints = Constraints::default();
         let codec_id = CodecId::POSTCARD;
         let version = 1;
         let name = abi::symbols::sym("FontBody");

         let type_id = TypeDef::compute_hash(
            name,
            version,
            codec_id,
            &desc,
            &constraints
         );

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
