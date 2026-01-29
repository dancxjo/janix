extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use abi::drawlist::{
    decode_fill_path, decode_fill_rect, DrawCmdTag, DrawListReader, FillRule, PathVerb, PointF,
};

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;
const TILE_SIZE: i32 = 32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectI32 {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl RectI32 {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn intersection(&self, other: &RectI32) -> Option<RectI32> {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x + self.w).min(other.x + other.w);
        let y1 = (self.y + self.h).min(other.y + other.h);
        if x1 > x0 && y1 > y0 {
            Some(RectI32::new(x0, y0, x1 - x0, y1 - y0))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectF {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl RectF {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn union(&self, other: &RectF) -> RectF {
        let x0 = self.x.min(other.x);
        let y0 = self.y.min(other.y);
        let x1 = (self.x + self.w).max(other.x + other.w);
        let y1 = (self.y + self.h).max(other.y + other.h);
        RectF::new(x0, y0, x1 - x0, y1 - y0)
    }

    pub fn to_i32(&self) -> RectI32 {
        let x0 = libm::floorf(self.x);
        let y0 = libm::floorf(self.y);
        let x1 = libm::ceilf(self.x + self.w);
        let y1 = libm::ceilf(self.y + self.h);
        RectI32::new(x0 as i32, y0 as i32, (x1 - x0) as i32, (y1 - y0) as i32)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RenderHashes {
    pub path: u64,
    pub flattened: u64,
    pub edges: u64,
    pub coverage: u64,
    pub raster: u64,
}

#[derive(Clone, Debug)]
pub struct RenderFrame {
    pub items: Vec<RenderHashes>,
    pub cache_hits: u32,
    pub cache_misses: u32,
}

#[derive(Default)]
struct CacheStore {
    paths: BTreeMap<u64, RectF>,
    flattened: BTreeMap<u64, RectF>,
    edges: BTreeMap<u64, RectF>,
    coverage: BTreeMap<u64, Vec<u8>>,
    raster: BTreeMap<u64, Vec<u8>>,
}

pub struct RenderGraphPipeline {
    schema_version: u32,
    cache: CacheStore,
}

impl RenderGraphPipeline {
    /// Create a renderer pipeline with a schema version that participates in hashing.
    ///
    /// The schema version is mixed into every derived artifact hash so caches can
    /// be invalidated when formats change.
    pub fn new(schema_version: u32) -> Self {
        Self {
            schema_version,
            cache: CacheStore::default(),
        }
    }

    /// Render a packed DrawList into hashed artifacts.
    ///
    /// This does not read or write the graph directly; callers can persist the
    /// hashes and optional payloads into graph nodes if desired.
    pub fn render(
        &mut self,
        drawlist_bytes: &[u8],
        viewport: RectI32,
        clip: Option<RectI32>,
    ) -> Option<RenderFrame> {
        let mut reader = DrawListReader::new(drawlist_bytes)?;
        let mut items = Vec::new();
        let mut cache_hits = 0u32;
        let mut cache_misses = 0u32;
        while let Some(cmd) = reader.next() {
            match cmd.tag {
                DrawCmdTag::FillRect => {
                    let (x, y, w, h, color) = decode_fill_rect(cmd.payload)?;
                    let verbs = rect_path(x, y, w, h);
                    let hashes = self.render_item(
                        &verbs,
                        FillRule::NonZero,
                        color,
                        viewport,
                        clip,
                        &mut cache_hits,
                        &mut cache_misses,
                    );
                    items.push(hashes);
                }
                DrawCmdTag::FillPath => {
                    let decoded = decode_fill_path(cmd.payload)?;
                    let hashes = self.render_item(
                        &decoded.verbs,
                        decoded.fill_rule,
                        decoded.color,
                        viewport,
                        clip,
                        &mut cache_hits,
                        &mut cache_misses,
                    );
                    items.push(hashes);
                }
                DrawCmdTag::Line
                | DrawCmdTag::StrokePath
                | DrawCmdTag::TextSpan
                | DrawCmdTag::DrawIcon
                | DrawCmdTag::Save
                | DrawCmdTag::Restore
                | DrawCmdTag::SetClipRect
                | DrawCmdTag::SetTransform
                | DrawCmdTag::DrawImageRect => {}
                DrawCmdTag::Unknown(_) => {}
            }
        }
        Some(RenderFrame {
            items,
            cache_hits,
            cache_misses,
        })
    }

    fn render_item(
        &mut self,
        verbs: &[PathVerb],
        fill_rule: FillRule,
        color: u32,
        viewport: RectI32,
        clip: Option<RectI32>,
        cache_hits: &mut u32,
        cache_misses: &mut u32,
    ) -> RenderHashes {
        let path_hash = hash_path(self.schema_version, verbs, fill_rule);
        let path_bounds = self.cache_path(path_hash, verbs, cache_hits, cache_misses);

        let flattened_hash = hash_flattened(self.schema_version, path_hash);
        let flattened_bounds =
            self.cache_flattened(flattened_hash, path_bounds, cache_hits, cache_misses);

        let edges_hash = hash_edges(self.schema_version, flattened_hash, fill_rule);
        let edges_bounds = self.cache_edges(edges_hash, flattened_bounds, cache_hits, cache_misses);

        let clip_rect = clip.unwrap_or(viewport);
        let coverage_hash = hash_coverage(self.schema_version, edges_hash, clip_rect, TILE_SIZE);
        let coverage_bytes = self.cache_coverage(
            coverage_hash,
            edges_bounds,
            clip_rect,
            cache_hits,
            cache_misses,
        );

        let paint_hash = hash_paint(self.schema_version, color);
        let raster_hash = hash_raster(self.schema_version, coverage_hash, paint_hash);
        self.cache_raster(
            raster_hash,
            color,
            &coverage_bytes,
            cache_hits,
            cache_misses,
        );

        RenderHashes {
            path: path_hash,
            flattened: flattened_hash,
            edges: edges_hash,
            coverage: coverage_hash,
            raster: raster_hash,
        }
    }

    fn cache_path(
        &mut self,
        hash: u64,
        verbs: &[PathVerb],
        cache_hits: &mut u32,
        cache_misses: &mut u32,
    ) -> RectF {
        if let Some(bounds) = self.cache.paths.get(&hash) {
            *cache_hits += 1;
            return *bounds;
        }
        let bounds = compute_path_bounds(verbs);
        self.cache.paths.insert(hash, bounds);
        *cache_misses += 1;
        bounds
    }

    fn cache_flattened(
        &mut self,
        hash: u64,
        bounds: RectF,
        cache_hits: &mut u32,
        cache_misses: &mut u32,
    ) -> RectF {
        if let Some(cached) = self.cache.flattened.get(&hash) {
            *cache_hits += 1;
            return *cached;
        }
        self.cache.flattened.insert(hash, bounds);
        *cache_misses += 1;
        bounds
    }

    fn cache_edges(
        &mut self,
        hash: u64,
        bounds: RectF,
        cache_hits: &mut u32,
        cache_misses: &mut u32,
    ) -> RectF {
        if let Some(cached) = self.cache.edges.get(&hash) {
            *cache_hits += 1;
            return *cached;
        }
        self.cache.edges.insert(hash, bounds);
        *cache_misses += 1;
        bounds
    }

    fn cache_coverage(
        &mut self,
        hash: u64,
        bounds: RectF,
        clip: RectI32,
        cache_hits: &mut u32,
        cache_misses: &mut u32,
    ) -> Vec<u8> {
        if let Some(cached) = self.cache.coverage.get(&hash) {
            *cache_hits += 1;
            return cached.clone();
        }
        let tile_bytes = coverage_for_bounds(bounds, clip);
        self.cache.coverage.insert(hash, tile_bytes.clone());
        *cache_misses += 1;
        tile_bytes
    }

    fn cache_raster(
        &mut self,
        hash: u64,
        color: u32,
        coverage: &[u8],
        cache_hits: &mut u32,
        cache_misses: &mut u32,
    ) {
        if self.cache.raster.contains_key(&hash) {
            *cache_hits += 1;
            return;
        }
        let tile = raster_solid_tile(color, coverage);
        self.cache.raster.insert(hash, tile);
        *cache_misses += 1;
    }
}

fn rect_path(x: i32, y: i32, w: i32, h: i32) -> Vec<PathVerb> {
    let x0 = x as f32;
    let y0 = y as f32;
    let x1 = (x + w) as f32;
    let y1 = (y + h) as f32;
    let mut verbs = Vec::with_capacity(5);
    verbs.push(PathVerb::MoveTo(PointF::new(x0, y0)));
    verbs.push(PathVerb::LineTo(PointF::new(x1, y0)));
    verbs.push(PathVerb::LineTo(PointF::new(x1, y1)));
    verbs.push(PathVerb::LineTo(PointF::new(x0, y1)));
    verbs.push(PathVerb::Close);
    verbs
}

fn compute_path_bounds(verbs: &[PathVerb]) -> RectF {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut update = |p: PointF| {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    };
    for verb in verbs {
        match *verb {
            PathVerb::MoveTo(p) | PathVerb::LineTo(p) => update(p),
            PathVerb::QuadTo(c, p) => {
                update(c);
                update(p);
            }
            PathVerb::CubicTo(c1, c2, p) => {
                update(c1);
                update(c2);
                update(p);
            }
            PathVerb::Close => {}
        }
    }
    if min_x == f32::INFINITY {
        RectF::new(0.0, 0.0, 0.0, 0.0)
    } else {
        RectF::new(
            min_x,
            min_y,
            (max_x - min_x).max(0.0),
            (max_y - min_y).max(0.0),
        )
    }
}

fn coverage_for_bounds(bounds: RectF, clip: RectI32) -> Vec<u8> {
    let tile_px = TILE_SIZE * TILE_SIZE;
    let mut bytes = Vec::new();
    bytes.resize(tile_px as usize, 0u8);
    let bounds_i32 = bounds.to_i32();
    // Placeholder coverage: treat any intersection as fully covered.
    // TODO: replace with edge/scan conversion into per-tile coverage.
    if bounds_i32.intersection(&clip).is_some() {
        for v in &mut bytes {
            *v = 255;
        }
    }
    bytes
}

fn raster_solid_tile(color: u32, coverage: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(coverage.len() * 4);
    let a = ((color >> 24) & 0xFF) as u8;
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    for cov in coverage {
        let alpha = ((*cov as u16 * a as u16) / 255) as u8;
        let pixel = ((alpha as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        out.extend_from_slice(&pixel.to_le_bytes());
    }
    out
}

fn hash_path(schema: u32, verbs: &[PathVerb], fill_rule: FillRule) -> u64 {
    let mut hash = hash_tag(schema, 0x01);
    hash = hash_u32(hash, fill_rule as u32);
    for verb in verbs {
        match *verb {
            PathVerb::MoveTo(p) => {
                hash = hash_u32(hash, 0);
                hash = hash_f32(hash, p.x);
                hash = hash_f32(hash, p.y);
            }
            PathVerb::LineTo(p) => {
                hash = hash_u32(hash, 1);
                hash = hash_f32(hash, p.x);
                hash = hash_f32(hash, p.y);
            }
            PathVerb::QuadTo(c, p) => {
                hash = hash_u32(hash, 2);
                hash = hash_f32(hash, c.x);
                hash = hash_f32(hash, c.y);
                hash = hash_f32(hash, p.x);
                hash = hash_f32(hash, p.y);
            }
            PathVerb::CubicTo(c1, c2, p) => {
                hash = hash_u32(hash, 3);
                hash = hash_f32(hash, c1.x);
                hash = hash_f32(hash, c1.y);
                hash = hash_f32(hash, c2.x);
                hash = hash_f32(hash, c2.y);
                hash = hash_f32(hash, p.x);
                hash = hash_f32(hash, p.y);
            }
            PathVerb::Close => {
                hash = hash_u32(hash, 4);
            }
        }
    }
    hash
}

fn hash_flattened(schema: u32, path_hash: u64) -> u64 {
    let mut hash = hash_tag(schema, 0x02);
    hash = hash_u64(hash, path_hash);
    hash
}

fn hash_edges(schema: u32, flattened_hash: u64, fill_rule: FillRule) -> u64 {
    let mut hash = hash_tag(schema, 0x03);
    hash = hash_u64(hash, flattened_hash);
    hash = hash_u32(hash, fill_rule as u32);
    hash
}

fn hash_coverage(schema: u32, edges_hash: u64, clip: RectI32, tile_size: i32) -> u64 {
    let mut hash = hash_tag(schema, 0x04);
    hash = hash_u64(hash, edges_hash);
    hash = hash_i32(hash, clip.x);
    hash = hash_i32(hash, clip.y);
    hash = hash_i32(hash, clip.w);
    hash = hash_i32(hash, clip.h);
    hash = hash_i32(hash, tile_size);
    hash
}

fn hash_paint(schema: u32, color: u32) -> u64 {
    let mut hash = hash_tag(schema, 0x05);
    hash = hash_u32(hash, color);
    hash
}

fn hash_raster(schema: u32, coverage_hash: u64, paint_hash: u64) -> u64 {
    let mut hash = hash_tag(schema, 0x06);
    hash = hash_u64(hash, coverage_hash);
    hash = hash_u64(hash, paint_hash);
    hash
}

fn hash_tag(schema: u32, tag: u32) -> u64 {
    let mut hash = FNV_OFFSET;
    hash = hash_u32(hash, schema);
    hash = hash_u32(hash, tag);
    hash
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn hash_u64(hash: u64, value: u64) -> u64 {
    hash_bytes(hash, &value.to_le_bytes())
}

fn hash_u32(hash: u64, value: u32) -> u64 {
    hash_bytes(hash, &value.to_le_bytes())
}

fn hash_i32(hash: u64, value: i32) -> u64 {
    hash_bytes(hash, &value.to_le_bytes())
}

fn hash_f32(hash: u64, value: f32) -> u64 {
    hash_u32(hash, value.to_bits())
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use abi::drawlist::DrawListBuilder;

    fn build_rect_drawlist(color: u32) -> Vec<u8> {
        let mut builder = DrawListBuilder::new();
        builder.push_fill_rect(0, 0, 16, 16, color);
        builder.finish()
    }

    #[test]
    fn hash_stability_same_inputs_same_hash() {
        let drawlist = build_rect_drawlist(0xff112233);
        let mut pipeline = RenderGraphPipeline::new(1);
        let viewport = RectI32::new(0, 0, 64, 64);
        let first = pipeline.render(&drawlist, viewport, None).expect("frame");
        let second = pipeline.render(&drawlist, viewport, None).expect("frame");
        assert_eq!(first.items[0].path, second.items[0].path);
        assert_eq!(first.items[0].coverage, second.items[0].coverage);
        assert_eq!(first.items[0].raster, second.items[0].raster);
    }

    #[test]
    fn paint_change_reuses_coverage_hashes() {
        let drawlist_a = build_rect_drawlist(0xff112233);
        let drawlist_b = build_rect_drawlist(0xffaa5533);
        let mut pipeline = RenderGraphPipeline::new(1);
        let viewport = RectI32::new(0, 0, 64, 64);
        let first = pipeline.render(&drawlist_a, viewport, None).expect("frame");
        let second = pipeline.render(&drawlist_b, viewport, None).expect("frame");
        assert_eq!(first.items[0].coverage, second.items[0].coverage);
        assert_ne!(first.items[0].raster, second.items[0].raster);
    }

    #[test]
    fn clip_change_reuses_edges_but_changes_coverage() {
        let drawlist = build_rect_drawlist(0xff112233);
        let mut pipeline = RenderGraphPipeline::new(1);
        let viewport = RectI32::new(0, 0, 64, 64);
        let clip_a = RectI32::new(0, 0, 32, 32);
        let clip_b = RectI32::new(32, 32, 32, 32);
        let first = pipeline
            .render(&drawlist, viewport, Some(clip_a))
            .expect("frame");
        let second = pipeline
            .render(&drawlist, viewport, Some(clip_b))
            .expect("frame");
        assert_eq!(first.items[0].edges, second.items[0].edges);
        assert_ne!(first.items[0].coverage, second.items[0].coverage);
    }

    #[test]
    fn hash_u32_and_u64_are_domain_separated() {
        let base = hash_tag(1, 0x42);
        let hash_u32_val = hash_u32(base, 0xfeed_beefu32);
        let hash_u64_val = hash_u64(base, 0xfeed_beefu32 as u64);
        assert_ne!(hash_u32_val, hash_u64_val);
    }
}
