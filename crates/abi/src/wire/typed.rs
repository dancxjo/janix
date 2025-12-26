use serde::{Deserialize, Serialize};
use crate::symbols::SymbolId;
use alloc::vec::Vec;
use alloc::boxed::Box;

// TypeId is a stable hash of the canonical TypeDef
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
pub struct TypeId(pub u128);

// CodecId identifies the encoding format (e.g. postcard)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct CodecId(pub u64);

impl CodecId {
    pub const POSTCARD: CodecId = CodecId(0); // Default/Reserved
}

// TypedBytes carries the payload along with type info
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedBytes {
    pub type_id: TypeId,
    pub codec_id: CodecId,
    pub bytes: Vec<u8>,
}

// TypeDef is the schema definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDef {
    pub type_id: TypeId,
    pub name: SymbolId,
    pub codec_id: CodecId,
    pub version: u32,
    pub desc: TypeDesc,
    pub constraints: Constraints,
}

// Canonical description of the type structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeDesc {
    Primitive(PrimitiveType),
    Struct { fields: Vec<Field> },
    Enum { variants: Vec<Variant> },
    Option(Box<TypeDesc>),
    List(Box<TypeDesc>),
    Map { key: Box<TypeDesc>, value: Box<TypeDesc> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    Bool,
    I64,
    U64,
    F64,
    String,
    Bytes,
    ThingId,
    SymbolId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub name: SymbolId,
    pub type_desc: TypeDesc,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variant {
    pub name: SymbolId,
    pub fields: Option<Vec<Field>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constraints {
    // Add fields as needed for validation
    // e.g. min, max, regex, etc.
}

// Canonicalization helper for hashing
impl TypeDef {
    pub fn compute_hash(
        name: SymbolId,
        version: u32,
        codec_id: CodecId,
        desc: &TypeDesc,
        constraints: &Constraints
    ) -> TypeId {
        // Canonical encoding: serialize fields in order using postcard, then hash.
        // We use a temporary buffer. Since we are in abi, we might not have alloc
        // but the file imports alloc::vec::Vec, so we are good.

        // We construct a tuple of relevant fields to ensure canonical order
        let canonical_tuple = (name, version, codec_id, desc, constraints);

        let bytes = postcard::to_allocvec(&canonical_tuple).unwrap_or_default();

        // FNV-1a 128-bit implementation
        let mut hash: u128 = 0x6c62272e07bb014262b821756295c58d;
        let prime: u128 = 0x1000000000000000000001b3;

        for b in bytes {
            hash ^= b as u128;
            hash = hash.wrapping_mul(prime);
        }

        TypeId(hash)
    }
}
