use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextCacheKey {
    pub face_id: u64,
    pub px: u16,
    pub flags: u8,
    pub text_hash: u64,
    pub text_len: u16,
}

pub struct TextCacheEntry {
    pub w: u16,
    pub h: u16,
    pub offset_x: i16, 
    pub offset_y: i16,
    pub alpha: Vec<u8>,
    pub last_used_ns: u64,
}

pub struct TextRasterCache {
    map: BTreeMap<TextCacheKey, TextCacheEntry>,
    bytes: usize,
}

impl TextRasterCache {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            bytes: 0,
        }
    }

    pub fn get(&mut self, key: &TextCacheKey, now_ns: u64) -> Option<&TextCacheEntry> {
        if let Some(entry) = self.map.get_mut(key) {
            entry.last_used_ns = now_ns;
            return Some(&*entry);
        }
        None
    }
    
    pub fn insert(&mut self, key: TextCacheKey, entry: TextCacheEntry) -> &TextCacheEntry {
        let size = entry.alpha.len();
        self.bytes += size;
        self.map.insert(key.clone(), entry);
        self.map.get(&key).unwrap()
    }

    pub fn evict_if_needed(&mut self, max_bytes: usize, _now_ns: u64) {
        if self.bytes <= max_bytes {
            return;
        }

        let mut entries: Vec<(u64, TextCacheKey)> = self.map.iter()
            .map(|(k, v)| (v.last_used_ns, k.clone()))
            .collect();
        
        entries.sort_by_key(|(t, _)| *t);

        for (_t, key) in entries {
            if self.bytes <= max_bytes {
                break;
            }
            if let Some(v) = self.map.remove(&key) {
                self.bytes = self.bytes.saturating_sub(v.alpha.len());
            }
        }
    }
    
    pub fn byte_count(&self) -> usize {
        self.bytes
    }
}

pub fn hash_str(s: &str) -> u64 {
    // FNV-1a 64-bit
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x10995116282116cd);
    }
    hash
}
