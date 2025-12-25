use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeTag(pub u64);

/// Constant FNV-1a 64-bit hash
pub const fn fnv1a64(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}

/// Generate a stable TypeTag from a string identifier.
/// The identifier should be stable, e.g. "thingos.IntentBody.v1".
pub const fn type_tag(name: &str) -> TypeTag {
    TypeTag(fnv1a64(name))
}
