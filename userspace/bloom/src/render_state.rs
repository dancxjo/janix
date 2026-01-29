use alloc::collections::{BTreeMap, BTreeSet};
use alloc::sync::Arc;

use crate::asset::Image;

const DEFAULT_RASTER_CACHE_MAX_BYTES: usize = 64 * 1024 * 1024;

/// Cache key for rasterized UI content.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RasterKey {
    Svg {
        source_id: u64,
        w: u32,
        h: u32,
        content_hash: u64,
    },
    Text {
        w: u32,
        h: u32,
        content_hash: u64,
    },
}

pub struct RenderState {
    raster_cache: RasterCache,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            raster_cache: RasterCache::new(DEFAULT_RASTER_CACHE_MAX_BYTES),
        }
    }

    #[cfg(test)]
    fn with_cache_limit(max_bytes: usize) -> Self {
        Self {
            raster_cache: RasterCache::new(max_bytes),
        }
    }

    /// Lookup a cached raster and refresh its LRU position.
    pub fn get_raster(&mut self, key: &RasterKey) -> Option<Arc<Image>> {
        self.raster_cache.get(key)
    }

    /// Insert a raster into the cache, optionally associating it with a SVG bytespace id.
    pub fn insert_raster(&mut self, key: RasterKey, image: Arc<Image>, svg_source: Option<u64>) {
        self.raster_cache.insert(key, image, svg_source);
    }

    /// Invalidate all cached rasters derived from a particular SVG bytespace.
    pub fn invalidate_svg_source(&mut self, source_id: u64) {
        self.raster_cache.invalidate_svg_source(source_id);
    }

    /// Clear the raster cache when UI content changes.
    pub fn clear_raster_cache(&mut self) {
        self.raster_cache.clear();
    }
}

struct CacheEntry {
    image: Arc<Image>,
    bytes: usize,
    svg_source: Option<u64>,
}

struct RasterCache {
    entries: BTreeMap<RasterKey, CacheEntry>,
    svg_dependents: BTreeMap<u64, BTreeSet<RasterKey>>,
    usage_order: BTreeMap<u64, RasterKey>,
    usage_by_key: BTreeMap<RasterKey, u64>,
    usage_tick: u64,
    total_bytes: usize,
    max_bytes: usize,
}

impl RasterCache {
    fn new(max_bytes: usize) -> Self {
        Self {
            entries: BTreeMap::new(),
            svg_dependents: BTreeMap::new(),
            usage_order: BTreeMap::new(),
            usage_by_key: BTreeMap::new(),
            usage_tick: 0,
            total_bytes: 0,
            max_bytes,
        }
    }

    fn get(&mut self, key: &RasterKey) -> Option<Arc<Image>> {
        let entry = self.entries.get(key)?;
        let image = entry.image.clone();
        self.touch(key.clone());
        Some(image)
    }

    fn insert(&mut self, key: RasterKey, image: Arc<Image>, svg_source: Option<u64>) {
        let bytes = bytes_for_image(&image);
        if let Some(existing) = self.entries.get(&key) {
            self.total_bytes = self.total_bytes.saturating_sub(existing.bytes);
            self.detach_svg_dependency(&key, existing.svg_source);
        }

        self.entries.insert(
            key.clone(),
            CacheEntry {
                image,
                bytes,
                svg_source,
            },
        );
        self.total_bytes = self.total_bytes.saturating_add(bytes);
        self.attach_svg_dependency(&key, svg_source);
        self.touch(key);
        self.evict_to_budget();
    }

    fn invalidate_svg_source(&mut self, source_id: u64) {
        let Some(keys) = self.svg_dependents.remove(&source_id) else {
            return;
        };
        for key in keys {
            if let Some(entry) = self.entries.remove(&key) {
                self.total_bytes = self.total_bytes.saturating_sub(entry.bytes);
            }
            if let Some(tick) = self.usage_by_key.remove(&key) {
                self.usage_order.remove(&tick);
            }
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.svg_dependents.clear();
        self.usage_order.clear();
        self.usage_by_key.clear();
        self.total_bytes = 0;
    }

    fn touch(&mut self, key: RasterKey) {
        if let Some(prev_tick) = self.usage_by_key.remove(&key) {
            self.usage_order.remove(&prev_tick);
        }
        self.usage_tick = self.usage_tick.wrapping_add(1);
        self.usage_by_key.insert(key.clone(), self.usage_tick);
        self.usage_order.insert(self.usage_tick, key);
    }

    fn evict_to_budget(&mut self) {
        while self.total_bytes > self.max_bytes {
            let Some((&tick, key)) = self.usage_order.iter().next() else {
                break;
            };
            let key = key.clone();
            self.usage_order.remove(&tick);
            self.usage_by_key.remove(&key);
            if let Some(entry) = self.entries.remove(&key) {
                self.total_bytes = self.total_bytes.saturating_sub(entry.bytes);
                self.detach_svg_dependency(&key, entry.svg_source);
                crate::trace_counter!("ui.raster_cache.evicted", 1);
            }
        }
    }

    fn attach_svg_dependency(&mut self, key: &RasterKey, svg_source: Option<u64>) {
        let Some(source) = svg_source else { return };
        self.svg_dependents
            .entry(source)
            .or_default()
            .insert(key.clone());
    }

    fn detach_svg_dependency(&mut self, key: &RasterKey, svg_source: Option<u64>) {
        let Some(source) = svg_source else { return };
        if let Some(keys) = self.svg_dependents.get_mut(&source) {
            keys.remove(key);
            if keys.is_empty() {
                self.svg_dependents.remove(&source);
            }
        }
    }
}

fn bytes_for_image(image: &Image) -> usize {
    let width = image.width as usize;
    let height = image.height as usize;
    width
        .checked_mul(height)
        .and_then(|pixels| pixels.checked_mul(4))
        .unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;
    use alloc::vec;

    fn image_of_size(w: u32, h: u32) -> Arc<Image> {
        Arc::new(Image {
            width: w,
            height: h,
            pixels: Arc::from(vec![0u32; (w as usize * h as usize)]),
            gen: crate::frame::AssetGeneration(0),
        })
    }

    #[test]
    fn evicts_when_over_budget() {
        let mut state = RenderState::with_cache_limit(32);
        let key_a = RasterKey::Text {
            w: 2,
            h: 2,
            content_hash: 1,
        };
        let key_b = RasterKey::Text {
            w: 2,
            h: 2,
            content_hash: 2,
        };
        state.insert_raster(key_a.clone(), image_of_size(2, 2), None);
        state.insert_raster(key_b.clone(), image_of_size(2, 2), None);

        assert!(state.get_raster(&key_a).is_none());
        assert!(state.get_raster(&key_b).is_some());
    }

    #[test]
    fn invalidates_svg_by_source() {
        let mut state = RenderState::with_cache_limit(1024);
        let key = RasterKey::Svg {
            source_id: 42,
            w: 4,
            h: 4,
            content_hash: 9,
        };
        state.insert_raster(key.clone(), image_of_size(4, 4), Some(42));
        assert!(state.get_raster(&key).is_some());
        state.invalidate_svg_source(42);
        assert!(state.get_raster(&key).is_none());
    }
}
