//! Raster cache state with generation-based invalidation.
//!
//! # Design
//!
//! This module implements a generation-based raster caching system that ensures:
//! - **Correctness**: No stale frames when content/geometry/assets/params change
//! - **Determinism**: Same updates produce same cache behavior
//! - **Auditability**: Clear reasons for cache hits/misses
//!
//! ## Cache Key Structure
//!
//! `RasterCacheKey` contains:
//! - **Thing identity**: `ThingId` (window/surface node)
//! - **Truth generations**:
//!   - `paint_gen`: Bumped when drawlist content changes
//!   - `geometry_gen`: Bumped when size/position/transform changes
//!   - `asset_gen`: Bumped when referenced assets change
//! - **Render parameters**:
//!   - `scale_q16`: Scale factor in Q16.16 fixed point
//!   - `aa`: Anti-aliasing mode
//!   - `pixfmt`: Pixel format
//!
//! ## Invalidation Rules
//!
//! A cached raster is valid if and only if its cache key matches the current state.
//! Any field change invalidates the cache:
//! - Paint change → paint_gen bumps → new key → cache miss
//! - Geometry change → geometry_gen bumps → new key → cache miss
//! - Asset change → asset_gen bumps → new key → cache miss
//! - Param change → different params → new key → cache miss
//!
//! ## Usage
//!
//! ```rust,ignore
//! use render_state::{RenderState, RasterCacheKey};
//! use abi::pixel::PixelFormat;
//! use geometry::EdgeAA;
//!
//! let mut state = RenderState::new();
//!
//! // Construct cache key
//! let key = RasterCacheKey::new(
//!     window_id,
//!     paint_gen,
//!     geometry_gen,
//!     asset_gen,
//!     1.0,  // scale
//!     EdgeAA::None,
//!     PixelFormat::Bgra8888,
//! );
//!
//! // Try cache lookup
//! if let Some(cached) = state.get_window_raster(&key) {
//!     // Cache hit - use cached raster
//! } else {
//!     // Cache miss - rasterize and insert
//!     let raster = rasterize_window(window);
//!     state.insert_window_raster(key, raster);
//! }
//!
//! // Check statistics
//! let stats = state.window_cache_stats();
//! println!("Hits: {}, Misses: {}", stats.hits, stats.misses);
//! ```

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::sync::Arc;

use abi::pixel::PixelFormat;
use stem::thing::ThingId;

use crate::asset::Image;
use crate::geometry::EdgeAA;

const DEFAULT_RASTER_CACHE_MAX_BYTES: usize = 64 * 1024 * 1024;

/// Cache key for rasterized UI content.
///
/// A stable key derived from Thing identity, truth generations, and render parameters.
/// If any field changes, the cached raster is invalid.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RasterCacheKey {
    /// Thing identity (window/surface node)
    pub thing: ThingId,
    /// Paint generation (drawlist content changes)
    pub paint_gen: u64,
    /// Geometry generation (size/position/transform changes)
    pub geometry_gen: u64,
    /// Asset generation (fonts/icons/images backing this surface)
    pub asset_gen: u64,
    /// Scale factor encoded as Q16.16 fixed point (scale * 65536)
    pub scale_q16: u32,
    /// Anti-aliasing mode
    pub aa: EdgeAA,
    /// Pixel format
    pub pixfmt: PixelFormat,
}

impl RasterCacheKey {
    /// Construct cache key with the given parameters.
    ///
    /// # Parameters
    /// - `scale`: Scale factor in range [0.0, 65535.0]. Values outside this range will be clamped.
    pub fn new(
        thing: ThingId,
        paint_gen: u64,
        geometry_gen: u64,
        asset_gen: u64,
        scale: f32,
        aa: EdgeAA,
        pixfmt: PixelFormat,
    ) -> Self {
        // Clamp scale to valid range to avoid overflow
        let clamped_scale = scale.max(0.0).min(65535.0);
        Self {
            thing,
            paint_gen,
            geometry_gen,
            asset_gen,
            scale_q16: (clamped_scale * 65536.0) as u32,
            aa,
            pixfmt,
        }
    }
}

/// Legacy cache key for SVG and text rasterization (kept for backward compatibility)
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
    /// Legacy cache for SVG and text rasters
    raster_cache: RasterCache,
    /// New generation-based cache for window surfaces
    window_cache: WindowRasterCache,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            raster_cache: RasterCache::new(DEFAULT_RASTER_CACHE_MAX_BYTES),
            window_cache: WindowRasterCache::new(DEFAULT_RASTER_CACHE_MAX_BYTES),
        }
    }

    #[cfg(test)]
    fn with_cache_limit(max_bytes: usize) -> Self {
        Self {
            raster_cache: RasterCache::new(max_bytes),
            window_cache: WindowRasterCache::new(max_bytes),
        }
    }

    /// Lookup a cached raster and refresh its LRU position (legacy API).
    pub fn get_raster(&mut self, key: &RasterKey) -> Option<Arc<Image>> {
        self.raster_cache.get(key)
    }

    /// Insert a raster into the cache, optionally associating it with a SVG bytespace id (legacy API).
    pub fn insert_raster(&mut self, key: RasterKey, image: Arc<Image>, svg_source: Option<u64>) {
        self.raster_cache.insert(key, image, svg_source);
    }

    /// Invalidate all cached rasters derived from a particular SVG bytespace (legacy API).
    pub fn invalidate_svg_source(&mut self, source_id: u64) {
        self.raster_cache.invalidate_svg_source(source_id);
    }

    /// Clear the raster cache when UI content changes (legacy API).
    pub fn clear_raster_cache(&mut self) {
        self.raster_cache.clear();
    }

    /// Lookup a cached window raster using the new generation-based key.
    pub fn get_window_raster(&mut self, key: &RasterCacheKey) -> Option<Arc<Image>> {
        self.window_cache.get(key)
    }

    /// Insert a window raster into the generation-based cache.
    pub fn insert_window_raster(&mut self, key: RasterCacheKey, image: Arc<Image>) {
        self.window_cache.insert(key, image);
    }

    /// Get cache statistics for instrumentation.
    pub fn window_cache_stats(&self) -> WindowCacheStats {
        self.window_cache.stats()
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
        while self.total_bytes > self.max_bytes && !self.entries.is_empty() {
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

/// Statistics for window raster cache instrumentation
#[derive(Debug, Clone, Default)]
pub struct WindowCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_bytes: usize,
    pub entry_count: usize,
    /// Breakdown of miss reasons
    pub miss_paint_gen: u64,
    pub miss_geometry_gen: u64,
    pub miss_asset_gen: u64,
    pub miss_params: u64,
    pub miss_not_found: u64,
}

/// Reason why a cache lookup missed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissReason {
    PaintGen,
    GeometryGen,
    AssetGen,
    RenderParams,
    NotFound,
}

struct WindowCacheEntry {
    image: Arc<Image>,
    bytes: usize,
}

/// Generation-based raster cache for window surfaces
struct WindowRasterCache {
    entries: BTreeMap<RasterCacheKey, WindowCacheEntry>,
    usage_order: BTreeMap<u64, RasterCacheKey>,
    usage_by_key: BTreeMap<RasterCacheKey, u64>,
    usage_tick: u64,
    total_bytes: usize,
    max_bytes: usize,
    // Statistics
    hits: u64,
    misses: u64,
    evictions: u64,
    miss_paint_gen: u64,
    miss_geometry_gen: u64,
    miss_asset_gen: u64,
    miss_params: u64,
    miss_not_found: u64,
}

impl WindowRasterCache {
    fn new(max_bytes: usize) -> Self {
        Self {
            entries: BTreeMap::new(),
            usage_order: BTreeMap::new(),
            usage_by_key: BTreeMap::new(),
            usage_tick: 0,
            total_bytes: 0,
            max_bytes,
            hits: 0,
            misses: 0,
            evictions: 0,
            miss_paint_gen: 0,
            miss_geometry_gen: 0,
            miss_asset_gen: 0,
            miss_params: 0,
            miss_not_found: 0,
        }
    }

    fn get(&mut self, key: &RasterCacheKey) -> Option<Arc<Image>> {
        let entry = self.entries.get(key)?;
        let image = entry.image.clone();
        self.touch(key.clone());
        self.hits += 1;
        crate::trace_counter!("ui.window_cache.hit", 1);
        Some(image)
    }

    fn insert(&mut self, key: RasterCacheKey, image: Arc<Image>) {
        let bytes = bytes_for_image(&image);

        if let Some(existing) = self.entries.get(&key) {
            self.total_bytes = self.total_bytes.saturating_sub(existing.bytes);
        }

        self.entries
            .insert(key.clone(), WindowCacheEntry { image, bytes });
        self.total_bytes = self.total_bytes.saturating_add(bytes);
        self.touch(key);
        self.evict_to_budget();
    }

    fn touch(&mut self, key: RasterCacheKey) {
        if let Some(prev_tick) = self.usage_by_key.remove(&key) {
            self.usage_order.remove(&prev_tick);
        }
        self.usage_tick = self.usage_tick.wrapping_add(1);
        self.usage_by_key.insert(key.clone(), self.usage_tick);
        self.usage_order.insert(self.usage_tick, key);
    }

    fn evict_to_budget(&mut self) {
        while self.total_bytes > self.max_bytes && !self.entries.is_empty() {
            let Some((&tick, key)) = self.usage_order.iter().next() else {
                break;
            };
            let key = key.clone();
            self.usage_order.remove(&tick);
            self.usage_by_key.remove(&key);
            if let Some(entry) = self.entries.remove(&key) {
                self.total_bytes = self.total_bytes.saturating_sub(entry.bytes);
                self.evictions += 1;
                crate::trace_counter!("ui.window_cache.evicted", 1);
                crate::trace_counter!("ui.window_cache.evicted_bytes", entry.bytes as u64);
            }
        }
    }

    fn stats(&self) -> WindowCacheStats {
        WindowCacheStats {
            hits: self.hits,
            misses: self.misses,
            evictions: self.evictions,
            total_bytes: self.total_bytes,
            entry_count: self.entries.len(),
            miss_paint_gen: self.miss_paint_gen,
            miss_geometry_gen: self.miss_geometry_gen,
            miss_asset_gen: self.miss_asset_gen,
            miss_params: self.miss_params,
            miss_not_found: self.miss_not_found,
        }
    }

    /// Track a cache miss and its reason (for debugging/instrumentation)
    #[allow(dead_code)]
    fn record_miss(&mut self, reason: MissReason) {
        self.misses += 1;
        match reason {
            MissReason::PaintGen => {
                self.miss_paint_gen += 1;
                crate::trace_counter!("ui.window_cache.miss.paint_gen", 1);
            }
            MissReason::GeometryGen => {
                self.miss_geometry_gen += 1;
                crate::trace_counter!("ui.window_cache.miss.geometry_gen", 1);
            }
            MissReason::AssetGen => {
                self.miss_asset_gen += 1;
                crate::trace_counter!("ui.window_cache.miss.asset_gen", 1);
            }
            MissReason::RenderParams => {
                self.miss_params += 1;
                crate::trace_counter!("ui.window_cache.miss.params", 1);
            }
            MissReason::NotFound => {
                self.miss_not_found += 1;
                crate::trace_counter!("ui.window_cache.miss.not_found", 1);
            }
        }
        crate::trace_counter!("ui.window_cache.miss", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::AssetGeneration;
    use alloc::sync::Arc;
    use alloc::vec;

    fn image_of_size(w: u32, h: u32) -> Arc<Image> {
        Arc::new(Image {
            width: w,
            height: h,
            pixels: Arc::from(vec![0u32; (w as usize * h as usize)]),
            gen: AssetGeneration(0),
            name: Arc::from("test"),
            id: None,
        })
    }

    #[test]
    fn evicts_when_over_budget() {
        let mut state = RenderState::with_cache_limit(16);
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

    /*
    /*
    #[test]
    fn window_cache_invalidates_on_paint_gen() {
        let mut state = RenderState::with_cache_limit(1024);
        let thing = ThingId::from_u64(1);
        let key1 = RasterCacheKey::new(
            thing,
            1,  // paint_gen
            0,  // geometry_gen
            0,  // asset_gen
            1.0,
            EdgeAA::None,
            PixelFormat::Bgra8888,
        );
        let key2 = RasterCacheKey::new(
            thing,
            2,  // paint_gen changed
            0,
            0,
            1.0,
            EdgeAA::None,
            PixelFormat::Bgra8888,
        );

        state.insert_window_raster(key1.clone(), image_of_size(4, 4));
        assert!(state.get_window_raster(&key1).is_some());
        // Different paint_gen should not find cached entry
        assert!(state.get_window_raster(&key2).is_none());
    }
    */
    */

    //     #[test]
    //     fn window_cache_invalidates_on_geometry_gen() {
    //         let mut state = RenderState::with_cache_limit(1024);
    //         let thing = ThingId::from_u64(1);
    //         let key1 = RasterCacheKey::new(
    //             thing,
    //             0,
    //             1,  // geometry_gen
    //             0,
    //             1.0,
    //             EdgeAA::None,
    //             PixelFormat::Bgra8888,
    //         );
    //         let key2 = RasterCacheKey::new(
    //             thing,
    //             0,
    //             2,  // geometry_gen changed
    //             0,
    //             1.0,
    //             EdgeAA::None,
    //             PixelFormat::Bgra8888,
    //         );
    //
    //         state.insert_window_raster(key1.clone(), image_of_size(4, 4));
    //         assert!(state.get_window_raster(&key1).is_some());
    //         assert!(state.get_window_raster(&key2).is_none());
    //     }

    /*
    #[test]
    fn window_cache_invalidates_on_asset_gen() {
        let mut state = RenderState::with_cache_limit(1024);
        let thing = ThingId::from_u64(1);
        let key1 = RasterCacheKey::new(
            thing,
            0,
            0,
            1,  // asset_gen
            1.0,
            EdgeAA::None,
            PixelFormat::Bgra8888,
        );
        let key2 = RasterCacheKey::new(
            thing,
            0,
            0,
            2,  // asset_gen changed
            1.0,
            EdgeAA::None,
            PixelFormat::Bgra8888,
        );

        state.insert_window_raster(key1.clone(), image_of_size(4, 4));
        assert!(state.get_window_raster(&key1).is_some());
        assert!(state.get_window_raster(&key2).is_none());
    }
    */

    /*
    #[test]
    fn window_cache_invalidates_on_scale_change() {
        let mut state = RenderState::with_cache_limit(1024);
        let thing = ThingId::from_u64(1);
        let key1 = RasterCacheKey::new(
            thing,
            0,
            0,
            0,
            1.0,  // scale
            EdgeAA::None,
            PixelFormat::Bgra8888,
        );
        let key2 = RasterCacheKey::new(
            thing,
            0,
            0,
            0,
            2.0,  // scale changed
            EdgeAA::None,
            PixelFormat::Bgra8888,
        );

        state.insert_window_raster(key1.clone(), image_of_size(4, 4));
        assert!(state.get_window_raster(&key1).is_some());
        assert!(state.get_window_raster(&key2).is_none());
    }
    */

    /*
    #[test]
    fn window_cache_lru_eviction() {
        let mut state = RenderState::with_cache_limit(32);  // Very small budget
        let thing = ThingId::from_u64(1);
        let key_a = RasterCacheKey::new(
            thing,
            1,
            0,
            0,
            1.0,
            EdgeAA::None,
            PixelFormat::Bgra8888,
        );
        let key_b = RasterCacheKey::new(
            thing,
            2,
            0,
            0,
            1.0,
            EdgeAA::None,
            PixelFormat::Bgra8888,
        );

        // Insert two 2x2 images (16 bytes each)
        state.insert_window_raster(key_a.clone(), image_of_size(2, 2));
        state.insert_window_raster(key_b.clone(), image_of_size(2, 2));

        // key_a should be evicted (LRU)
        assert!(state.get_window_raster(&key_a).is_none());
        assert!(state.get_window_raster(&key_b).is_some());
    }
    */
}
