# Window Raster Cache Integration Guide

## Overview

This guide shows how to integrate the generation-based `WindowRasterCache` into Bloom's compositor pipeline to ensure correct, deterministic caching.

## Current State

The existing `PaintPipeline` in `paint_vm.rs` implements a simple per-window cache:
- Stores rasterized buffers in `WindowPaintState`
- Rebuilds when `paint_gen` or `paint_bs` changes
- Rebuilds when geometry (rect/z/hidden) changes

## Integration Steps

### 1. Add RenderState to PaintPipeline

```rust
pub struct PaintPipeline {
    windows: BTreeMap<ThingId, WindowPaintState>,
    render_state: RenderState,  // Add this
}

impl PaintPipeline {
    pub fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
            render_state: RenderState::new(),  // Initialize
        }
    }
}
```

### 2. Get Current Asset Generation

At the start of `process_updates`, capture the current asset generation:

```rust
pub fn process_updates(&mut self, screen_w: i32, screen_h: i32) -> PaintResult {
    use crate::painter_resources::ASSETS;
    let current_asset_gen = ASSETS.current_generation().0;
    
    // ... rest of function
}
```

### 3. Construct Cache Key for Each Window

Before rasterizing a window, construct its cache key:

```rust
use crate::render_state::RasterCacheKey;
use abi::pixel::PixelFormat;

let cache_key = RasterCacheKey::new(
    *id,                      // Window ThingId
    paint_gen,                // From UI_PAINT_GEN property
    entry.geometry_gen,       // Local counter bumped on geometry changes
    current_asset_gen,        // From AssetBank
    1.0,                      // Scale factor (could be from property)
    EdgeAA::None,             // AA mode (could be configurable)
    PixelFormat::Bgra8888,    // Pixel format (from compositor)
);
```

### 4. Check Cache Before Rasterizing

```rust
// Try cache lookup
if let Some(cached_image) = self.render_state.get_window_raster(&cache_key) {
    // Cache hit! Use cached raster
    crate::trace_counter!("bloom.window_paint.cache_hit", 1);
    
    // Copy cached pixels to buffer
    entry.width = cached_image.width;
    entry.height = cached_image.height;
    entry.buffer.clear();
    entry.buffer.extend_from_slice(&cached_image.pixels);
} else {
    // Cache miss - need to rasterize
    crate::trace_counter!("bloom.window_paint.cache_miss", 1);
    
    // Existing rasterization logic here...
    // Build drawlist, execute into buffer, etc.
    
    // After rasterization, insert into cache
    let image = Arc::new(Image {
        width: entry.width,
        height: entry.height,
        pixels: Arc::from(entry.buffer.as_slice()),
        gen: AssetGeneration(current_asset_gen),
        name: Arc::from("window"),
        id: Some(*id),
    });
    
    self.render_state.insert_window_raster(cache_key, image);
}
```

### 5. Update Geometry Generation Tracking

Ensure `geometry_gen` is bumped when geometry changes (already implemented):

```rust
if entry.rect != rect || entry.z != z || entry.hidden != hidden {
    needs_rebuild = true;
    entry.geometry_gen = entry.geometry_gen.wrapping_add(1);
    // ... damage tracking
}
```

### 6. Add Cache Statistics Logging

Periodically log cache statistics:

```rust
// In main loop, every N frames:
if frame_count % 60 == 0 {
    let stats = paint_pipeline.render_state.window_cache_stats();
    stem::info!(
        "Window cache: hits={} misses={} evictions={} bytes={} entries={}",
        stats.hits,
        stats.misses,
        stats.evictions,
        stats.total_bytes,
        stats.entry_count
    );
    
    if stats.misses > 0 {
        stem::info!(
            "  Miss breakdown: paint={} geometry={} asset={} params={} not_found={}",
            stats.miss_paint_gen,
            stats.miss_geometry_gen,
            stats.miss_asset_gen,
            stats.miss_params,
            stats.miss_not_found
        );
    }
}
```

## Advanced: Scale Factor Support

To support HiDPI/scaling:

1. Add a `UI_SCALE_FACTOR` property to windows
2. Read it in `process_updates`:
   ```rust
   let scale = prop_get(*id, keys::UI_SCALE_FACTOR)
       .map(|s| (s as f32) / 65536.0)
       .unwrap_or(1.0);
   ```
3. Use in cache key:
   ```rust
   let cache_key = RasterCacheKey::new(
       *id,
       paint_gen,
       entry.geometry_gen,
       current_asset_gen,
       scale,  // Use actual scale
       EdgeAA::None,
       PixelFormat::Bgra8888,
   );
   ```
4. Scale rasterization accordingly

## Benefits

Once integrated, the cache provides:

1. **Correctness**: No stale frames
   - Paint changes bump `paint_gen` → cache miss
   - Geometry changes bump `geometry_gen` → cache miss
   - Asset changes bump `asset_gen` → cache miss
   - Scale/format changes → cache miss

2. **Performance**: Stable scenes converge to high hit rate
   - Unchanged windows reuse cached rasters
   - Only geometry-only motion requires reraster (if size changes)
   - LRU eviction keeps memory bounded

3. **Visibility**: Clear metrics
   - Hit/miss counters show cache effectiveness
   - Reason tracking shows why rerasters happen
   - Eviction tracking shows memory pressure

4. **Determinism**: Replaying same updates gives same cache behavior
   - Cache key is pure function of observable state
   - No hidden dependencies or timing issues

## Migration Strategy

To minimize risk during migration:

1. **Phase 1** (Done): Implement cache infrastructure
   - `RasterCacheKey` structure
   - `WindowRasterCache` implementation
   - Generation tracking in `WindowPaintState`

2. **Phase 2** (Recommended): Add optional cache usage
   - Add feature flag `use-generation-cache`
   - Keep existing per-window buffer cache as fallback
   - Log cache statistics when enabled

3. **Phase 3**: Monitor and tune
   - Watch hit/miss ratios
   - Adjust cache size limits if needed
   - Add more detailed miss reason tracking

4. **Phase 4**: Make it default
   - Remove feature flag
   - Remove old buffer cache
   - Clean up redundant code

## Testing

Key scenarios to test:

1. **Paint change invalidation**: Modify drawlist, verify reraster
2. **Geometry change invalidation**: Resize window, verify reraster
3. **Asset change invalidation**: Update font/icon, verify reraster
4. **Scale change invalidation**: Change DPI, verify reraster
5. **Stable scene**: No changes for N frames, verify high hit rate
6. **Memory pressure**: Many windows, verify LRU eviction
7. **Determinism**: Replay sequence, verify same hit/miss pattern

## Debug Mode

Add a debug flag to print cache key diffs on invalidation:

```rust
#[cfg(feature = "cache-debug")]
fn debug_cache_invalidation(old_key: &RasterCacheKey, new_key: &RasterCacheKey) {
    if old_key.paint_gen != new_key.paint_gen {
        stem::debug!("Cache invalidated: paint_gen {} -> {}", 
                    old_key.paint_gen, new_key.paint_gen);
    }
    if old_key.geometry_gen != new_key.geometry_gen {
        stem::debug!("Cache invalidated: geometry_gen {} -> {}", 
                    old_key.geometry_gen, new_key.geometry_gen);
    }
    if old_key.asset_gen != new_key.asset_gen {
        stem::debug!("Cache invalidated: asset_gen {} -> {}", 
                    old_key.asset_gen, new_key.asset_gen);
    }
    if old_key.scale_q16 != new_key.scale_q16 {
        stem::debug!("Cache invalidated: scale {} -> {}", 
                    old_key.scale_q16, new_key.scale_q16);
    }
}
```
