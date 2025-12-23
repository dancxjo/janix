use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use super::props::PropType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SchemaDef {
    pub magic: u32,
    pub kind_name: String,
    pub props: Vec<(String, PropType)>,
    pub indexed_props: Vec<String>,
}

impl SchemaDef {
    pub fn new(kind_name: &str) -> Self {
        Self {
            magic: 0x5C4E4D41, // "SCMA"
            kind_name: String::from(kind_name),
            props: Vec::new(),
            indexed_props: Vec::new(),
        }
    }

    pub fn with_prop(mut self, name: &str, type_: PropType) -> Self {
        self.props.push((String::from(name), type_));
        self
    }

    pub fn with_indexed_prop(mut self, name: &str) -> Self {
        self.indexed_props.push(String::from(name));
        self
    }

    pub fn canonicalize(mut self) -> Self {
        self.props.sort_by(|a, b| a.0.cmp(&b.0));
        self.indexed_props.sort();
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        postcard::to_allocvec(self).unwrap_or_default()
    }

    pub fn fingerprint(&self) -> u64 {
        use core::hash::{Hash, Hasher};
        // Use a deterministic hasher if possible, or simple hashing.
        // For now, we'll use a simple FNV-1a implementation or rely on postcard bytes stability.
        // Postcard is canonical if sorting is done.

        let bytes = self.to_bytes();
        let mut hasher = Fnv1aHasher::new();
        hasher.write(&bytes);
        hasher.finish()
    }
}

struct Fnv1aHasher {
    state: u64,
}

impl Fnv1aHasher {
    fn new() -> Self {
        Self { state: 0xcbf29ce484222325 }
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state ^= b as u64;
            self.state = self.state.wrapping_mul(0x1099511628211);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}
