extern crate alloc;

use alloc::sync::Arc;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use abi::ids::HandleId;
use stem::thing::ThingId;
use stem::{info, thread, warn};

use crate::frame::AssetGeneration;
use crate::reclaimer;

use alloc::collections::{BTreeMap, VecDeque};
use serde::{Deserialize, Serialize};
use spin::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Arc<[u32]>,
    /// Generation when this asset became ready
    pub gen: AssetGeneration,
}

impl Image {
    /// Compute decoded bytes for this image
    pub fn decoded_bytes(&self) -> usize {
        (self.width as usize) * (self.height as usize) * 4
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CursorFrame {
    pub image: Image,
    pub delay_ms: u32,
    pub hotspot_x: u32,
    pub hotspot_y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CursorAsset {
    Static(CursorFrame),
    Animated { frames: Arc<[CursorFrame]> },
}

impl CursorAsset {
    pub fn generation(&self) -> AssetGeneration {
        match self {
            CursorAsset::Static(f) => f.image.gen,
            CursorAsset::Animated { frames } => frames
                .first()
                .map(|f| f.image.gen)
                .unwrap_or(AssetGeneration::ZERO),
        }
    }

    /// Compute decoded bytes for cursor asset
    pub fn decoded_bytes(&self) -> usize {
        match self {
            CursorAsset::Static(f) => f.image.decoded_bytes(),
            CursorAsset::Animated { frames } => {
                frames.iter().map(|f| f.image.decoded_bytes()).sum()
            }
        }
    }
}

/// Wrapper around fontdue::Font for Arc sharing
#[derive(Clone)]
pub struct FontAsset {
    pub font: Arc<fontdue::Font>,
    pub name: Arc<str>,
    pub gen: AssetGeneration,
    pub glyph_cache: Arc<Mutex<BTreeMap<(u32, u16, usize), (fontdue::Metrics, Arc<[u8]>)>>>,
}

impl core::fmt::Debug for FontAsset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FontAsset")
            .field("name", &self.name)
            .field("gen", &self.gen)
            .finish()
    }
}

impl FontAsset {
    /// Estimate decoded bytes for font (rough estimate based on typical glyph cache)
    pub fn decoded_bytes(&self) -> usize {
        // Fonts are relatively small compared to images
        // Estimate ~100KB for a typical font's glyph cache
        let cache_size = self
            .glyph_cache
            .lock()
            .values()
            .map(|(_, b)| b.len())
            .sum::<usize>();
        100 * 1024 + cache_size
    }

    pub fn get_glyph(
        &self,
        config: fontdue::layout::GlyphRasterConfig,
    ) -> (fontdue::Metrics, Arc<[u8]>) {
        let key = (config.px.to_bits(), config.glyph_index, config.font_hash);

        {
            let cache = self.glyph_cache.lock();
            if let Some(cached) = cache.get(&key) {
                return cached.clone();
            }
        }

        // Rasterize if not cached
        let (metrics, bitmap) = self.font.rasterize_config(config);
        let bitmap_arc: Arc<[u8]> = Arc::from(bitmap.into_boxed_slice());

        let mut cache = self.glyph_cache.lock();
        cache.insert(key, (metrics, bitmap_arc.clone()));
        (metrics, bitmap_arc)
    }
}

/// Storage for ready assets with metadata for eviction
struct AssetSlot<T> {
    value: UnsafeCell<Option<T>>,
    ready: AtomicBool,
    last_used_frame: AtomicU64,
    decoded_bytes: AtomicUsize,
    reachable: AtomicBool,
}

unsafe impl<T> Sync for AssetSlot<T> {}

impl<T> AssetSlot<T> {
    const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            ready: AtomicBool::new(false),
            last_used_frame: AtomicU64::new(0),
            decoded_bytes: AtomicUsize::new(0),
            reachable: AtomicBool::new(false),
        }
    }

    fn mark_used(&self, frame_id: u64) {
        self.last_used_frame.store(frame_id, Ordering::Release);
    }

    fn mark_reachable(&self, reachable: bool) {
        self.reachable.store(reachable, Ordering::Release);
    }

    fn get_metadata(&self) -> Option<reclaimer::AssetMetadata> {
        if !self.ready.load(Ordering::Acquire) {
            return None;
        }
        Some(reclaimer::AssetMetadata {
            last_used_frame: self.last_used_frame.load(Ordering::Acquire),
            gen: AssetGeneration(0), // Will be set by caller
            decoded_bytes: self.decoded_bytes.load(Ordering::Acquire),
            reachable: self.reachable.load(Ordering::Acquire),
        })
    }
}

/// Storage for pending assets (published by loaders, promoted on frame boundary)
struct PendingSlot<T> {
    value: UnsafeCell<Option<T>>,
    has_pending: AtomicBool,
}

unsafe impl<T> Sync for PendingSlot<T> {}

impl<T> PendingSlot<T> {
    const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            has_pending: AtomicBool::new(false),
        }
    }
}

// Ready assets (visible to rendering)
static WALLPAPER_READY: AssetSlot<Image> = AssetSlot::new();
static CURSOR_READY: AssetSlot<CursorAsset> = AssetSlot::new();
static FONTS_READY: [AssetSlot<FontAsset>; 8] = [
    AssetSlot::new(),
    AssetSlot::new(),
    AssetSlot::new(),
    AssetSlot::new(),
    AssetSlot::new(),
    AssetSlot::new(),
    AssetSlot::new(),
    AssetSlot::new(),
];

// Pending assets (published by loaders, not yet visible)
static WALLPAPER_PENDING: PendingSlot<Image> = PendingSlot::new();
static CURSOR_PENDING: PendingSlot<CursorAsset> = PendingSlot::new();
static FONTS_PENDING: [PendingSlot<FontAsset>; 8] = [
    PendingSlot::new(),
    PendingSlot::new(),
    PendingSlot::new(),
    PendingSlot::new(),
    PendingSlot::new(),
    PendingSlot::new(),
    PendingSlot::new(),
    PendingSlot::new(),
];

// Global generation counter
static ASSET_GENERATION: AtomicU64 = AtomicU64::new(0);

/// Asset load job for worker threads
enum AssetLoadJob {
    Font {
        id: ThingId,
        size: usize,
        display_name: Arc<str>,
    },
    FontByPath {
        path: Arc<str>,
    },
    Wallpaper {
        path: Arc<str>,
    },
    Cursor {
        path: Arc<str>,
    },
}

static JOB_QUEUE: Mutex<VecDeque<AssetLoadJob>> = Mutex::new(VecDeque::new());
static WORKER_SPAWNED: AtomicBool = AtomicBool::new(false);

extern "C" fn asset_worker_entry() -> ! {
    info!("[asset_bank] worker started (priority bump)");
    let mut idle_spins = 0;
    loop {
        let job = {
            let mut queue = JOB_QUEUE.lock();
            queue.pop_front()
        };

        if let Some(job) = job {
            idle_spins = 0;
            match job {
                AssetLoadJob::Font {
                    id,
                    size,
                    display_name,
                } => {
                    if let Some(font) = AssetBank::load_font_immediate(id, size, &display_name) {
                        AssetBank::new().publish_font(font);
                    }
                }
                AssetLoadJob::FontByPath { path } => {
                    if let Some(font) = AssetBank::load_font_from_graph_by_path(&path) {
                        AssetBank::new().publish_font(font);
                    }
                }
                AssetLoadJob::Wallpaper { path } => {
                    if let Some(img) = AssetBank::new().load_wallpaper_immediate(&path) {
                        AssetBank::new().publish_wallpaper(img);
                    }
                }
                AssetLoadJob::Cursor { path } => {
                    if let Some(cursor) = AssetBank::load_cursor_immediate(&path) {
                        AssetBank::new().publish_cursor(cursor);
                    }
                }
            }
        } else {
            // No jobs: yield aggressively a few times, then short sleep.
            if idle_spins < 5 {
                thread::yield_now();
                idle_spins += 1;
            } else {
                stem::sleep_ms(5);
                idle_spins = 0;
            }
        }
    }
}

/// Asset types for eviction candidate selection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssetType {
    Wallpaper,
    Cursor,
    Font,
}

pub struct AssetBank;

impl AssetBank {
    pub const fn new() -> Self {
        Self
    }

    fn spawn_worker() {
        if WORKER_SPAWNED.swap(true, Ordering::SeqCst) {
            return;
        }

        use stem::stack::{Stack, StackSpec};
        let stack = Stack::alloc_growing_stack(StackSpec {
            reserve_bytes: 256 * 1024,
            initial_commit_bytes: 64 * 1024,
            ..StackSpec::default()
        })
        .expect("asset worker stack");
        match stem::thread::spawn_on(stack, asset_worker_entry) {
            Ok(tid) => {
                // Keep worker responsive but avoid starving device drivers: use Normal (2).
                let _ = stem::thread::set_priority(tid, 2);
                info!("[asset_bank] worker spawned tid={} (priority=2)", tid);
            }
            Err(e) => {
                crate::log!("[asset_bank] ERROR: failed to spawn worker: {:?}", e);
            }
        }
    }

    pub fn enqueue_font_load(&self, id: ThingId, size: usize, display_name: &str) {
        let mut queue = JOB_QUEUE.lock();
        let was_empty = queue.is_empty();
        queue.push_back(AssetLoadJob::Font {
            id,
            size,
            display_name: Arc::from(display_name),
        });
        if was_empty {
            Self::spawn_worker();
        }
    }

    pub fn enqueue_wallpaper_load(&self, path: &str) {
        let mut queue = JOB_QUEUE.lock();
        let was_empty = queue.is_empty();
        queue.push_back(AssetLoadJob::Wallpaper {
            path: Arc::from(path),
        });
        if was_empty {
            Self::spawn_worker();
        }
    }

    pub fn enqueue_cursor_load(&self, path: &str) {
        let mut queue = JOB_QUEUE.lock();
        queue.push_back(AssetLoadJob::Cursor {
            path: Arc::from(path),
        });
        Self::spawn_worker();
    }

    pub fn enqueue_font_load_by_path(&self, path: &str) {
        let mut queue = JOB_QUEUE.lock();
        queue.push_back(AssetLoadJob::FontByPath {
            path: Arc::from(path),
        });
        Self::spawn_worker();
    }

    pub fn probe_asset_exists(&self, path: &str) -> bool {
        Self::probe_asset(path).is_some()
    }

    /// Get current generation (read-only, no side effects)
    pub fn current_generation(&self) -> AssetGeneration {
        AssetGeneration(ASSET_GENERATION.load(Ordering::Acquire))
    }

    /// Promote all pending assets to ready, increment generation if any promoted.
    /// Must be called exactly once per acquire_frame().
    /// Returns the new current generation.
    pub fn publish_pending(&self) -> AssetGeneration {
        let mut promoted = false;

        // Check and promote pending wallpaper
        if WALLPAPER_PENDING.has_pending.load(Ordering::Acquire) {
            let pending = unsafe { (*WALLPAPER_PENDING.value.get()).take() };
            if let Some(mut img) = pending {
                // Increment generation first
                let new_gen = ASSET_GENERATION.fetch_add(1, Ordering::AcqRel) + 1;
                img.gen = AssetGeneration(new_gen);

                // Track memory
                let bytes = img.decoded_bytes();
                reclaimer::add_decoded_bytes(bytes);
                WALLPAPER_READY
                    .decoded_bytes
                    .store(bytes, Ordering::Release);

                info!(
                    "[asset_bank] promoting wallpaper to gen={} ({}b)",
                    new_gen, bytes
                );

                unsafe {
                    *WALLPAPER_READY.value.get() = Some(img);
                }
                WALLPAPER_READY.ready.store(true, Ordering::Release);
                promoted = true;
            }
            WALLPAPER_PENDING
                .has_pending
                .store(false, Ordering::Release);
        }

        // Check and promote pending cursor
        if CURSOR_PENDING.has_pending.load(Ordering::Acquire) {
            let pending = unsafe { (*CURSOR_PENDING.value.get()).take() };
            if let Some(cursor) = pending {
                let new_gen = if !promoted {
                    ASSET_GENERATION.fetch_add(1, Ordering::AcqRel) + 1
                } else {
                    ASSET_GENERATION.load(Ordering::Acquire)
                };

                // Update generation in cursor
                let cursor_with_gen = match cursor {
                    CursorAsset::Static(mut frame) => {
                        frame.image.gen = AssetGeneration(new_gen);
                        CursorAsset::Static(frame)
                    }
                    CursorAsset::Animated { frames } => {
                        let updated: alloc::vec::Vec<_> = frames
                            .iter()
                            .cloned()
                            .map(|mut f| {
                                f.image.gen = AssetGeneration(new_gen);
                                f
                            })
                            .collect();
                        CursorAsset::Animated {
                            frames: Arc::from(updated.as_slice()),
                        }
                    }
                };

                // Track memory
                let bytes = cursor_with_gen.decoded_bytes();
                reclaimer::add_decoded_bytes(bytes);
                CURSOR_READY.decoded_bytes.store(bytes, Ordering::Release);

                info!(
                    "[asset_bank] promoting cursor to gen={} ({}b)",
                    new_gen, bytes
                );
                unsafe {
                    *CURSOR_READY.value.get() = Some(cursor_with_gen);
                }
                CURSOR_READY.ready.store(true, Ordering::Release);
                promoted = true;
            }
            CURSOR_PENDING.has_pending.store(false, Ordering::Release);
        }

        // Check and promote pending fonts
        for i in 0..8 {
            if FONTS_PENDING[i].has_pending.load(Ordering::Acquire) {
                let pending = unsafe { (*FONTS_PENDING[i].value.get()).take() };
                if let Some(mut font) = pending {
                    let new_gen = if !promoted {
                        ASSET_GENERATION.fetch_add(1, Ordering::AcqRel) + 1
                    } else {
                        ASSET_GENERATION.load(Ordering::Acquire)
                    };
                    font.gen = AssetGeneration(new_gen);

                    // Track memory
                    let bytes = font.decoded_bytes();
                    reclaimer::add_decoded_bytes(bytes);
                    FONTS_READY[i].decoded_bytes.store(bytes, Ordering::Release);

                    info!(
                        "[asset_bank] promoting font '{}' to gen={} ({}b) in slot {}",
                        font.name, new_gen, bytes, i
                    );
                    unsafe {
                        *FONTS_READY[i].value.get() = Some(font);
                    }
                    FONTS_READY[i].ready.store(true, Ordering::Release);
                    promoted = true;
                }
                FONTS_PENDING[i].has_pending.store(false, Ordering::Release);
            }
        }

        self.current_generation()
    }

    /// Publish wallpaper to pending (called by loader thread)
    pub fn publish_wallpaper(&self, img: Image) {
        info!(
            "[asset_bank] publish_wallpaper (pending): {}x{}",
            img.width, img.height
        );
        unsafe {
            *WALLPAPER_PENDING.value.get() = Some(img);
        }
        WALLPAPER_PENDING.has_pending.store(true, Ordering::Release);
    }

    /// Get wallpaper if ready and visible at the given generation
    pub fn get_wallpaper_for_gen(&self, snapshot: AssetGeneration) -> Option<Image> {
        if !WALLPAPER_READY.ready.load(Ordering::Acquire) {
            return None;
        }
        let img = unsafe { (*WALLPAPER_READY.value.get()).clone() }?;
        if img.gen <= snapshot {
            Some(img)
        } else {
            None // Asset is newer than snapshot, invisible this frame
        }
    }

    /// Legacy: get wallpaper without generation check
    pub fn get_wallpaper(&self) -> Option<Image> {
        if WALLPAPER_READY.ready.load(Ordering::Acquire) {
            unsafe { (*WALLPAPER_READY.value.get()).clone() }
        } else {
            None
        }
    }

    /// Publish cursor to pending (called by loader thread)
    pub fn publish_cursor(&self, cursor: CursorAsset) {
        info!("[asset_bank] publish_cursor (pending)");
        match &cursor {
            CursorAsset::Static(frame) => {
                info!(
                    "[asset_bank] cursor: Static frame {}x{} hotspot ({}, {})",
                    frame.image.width, frame.image.height, frame.hotspot_x, frame.hotspot_y
                );
            }
            CursorAsset::Animated { frames } => {
                info!("[asset_bank] cursor: Animated with {} frames", frames.len());
            }
        }
        unsafe {
            *CURSOR_PENDING.value.get() = Some(cursor);
        }
        CURSOR_PENDING.has_pending.store(true, Ordering::Release);
    }

    /// Get cursor if ready and visible at the given generation
    pub fn get_cursor_for_gen(&self, snapshot: AssetGeneration) -> Option<CursorAsset> {
        if !CURSOR_READY.ready.load(Ordering::Acquire) {
            return None;
        }
        let cursor = unsafe { (*CURSOR_READY.value.get()).clone() }?;
        if cursor.generation() <= snapshot {
            Some(cursor)
        } else {
            None
        }
    }

    /// Legacy: get cursor without generation check
    pub fn get_cursor(&self) -> Option<CursorAsset> {
        if CURSOR_READY.ready.load(Ordering::Acquire) {
            unsafe { (*CURSOR_READY.value.get()).clone() }
        } else {
            None
        }
    }

    /// Publish font to pending (called by loader thread)
    pub fn publish_font(&self, font: FontAsset) {
        // Find an empty or replaceable pending slot
        for i in 0..8 {
            if !FONTS_PENDING[i].has_pending.load(Ordering::Acquire)
                && !FONTS_READY[i].ready.load(Ordering::Acquire)
            {
                info!(
                    "[asset_bank] publish_font (pending): '{}' in slot {}",
                    font.name, i
                );
                unsafe {
                    *FONTS_PENDING[i].value.get() = Some(font);
                }
                FONTS_PENDING[i].has_pending.store(true, Ordering::Release);
                return;
            }
        }
        warn!(
            "[asset_bank] FONT_PENDING slots exhausted! skipping '{}'",
            font.name
        );
    }

    /// Get font if ready and visible at the given generation
    pub fn get_font_for_gen(&self, snapshot: AssetGeneration) -> Option<FontAsset> {
        // Return first available font (legacy behavior)
        for i in 0..8 {
            if FONTS_READY[i].ready.load(Ordering::Acquire) {
                let font = unsafe { (*FONTS_READY[i].value.get()).clone() }?;
                if font.gen <= snapshot {
                    return Some(font);
                }
            }
        }
        None
    }

    /// Get all fonts ready and visible at the given generation (ordered by slot)
    pub fn get_fonts_for_gen(&self, snapshot: AssetGeneration) -> alloc::vec::Vec<FontAsset> {
        let mut fonts = alloc::vec::Vec::new();
        for i in 0..8 {
            if FONTS_READY[i].ready.load(Ordering::Acquire) {
                if let Some(font) = unsafe { (*FONTS_READY[i].value.get()).as_ref() } {
                    if font.gen <= snapshot {
                        fonts.push(font.clone());
                    }
                }
            }
        }
        fonts
    }

    /// Get all ready fonts without generation check (for rasterizer access)
    pub fn get_fonts(&self) -> alloc::vec::Vec<FontAsset> {
        let mut fonts = alloc::vec::Vec::new();
        for i in 0..8 {
            if FONTS_READY[i].ready.load(Ordering::Acquire) {
                if let Some(font) = unsafe { (*FONTS_READY[i].value.get()).clone() } {
                    fonts.push(font);
                }
            }
        }

        // Priority sorting: NotoSans first, then NotoSerif, then Symbols, then others
        fonts.sort_by_key(|f| {
            if f.name.contains("NotoSans-Regular") {
                0
            } else if f.name.contains("NotoSerif") {
                1
            } else if f.name.contains("NotoSansSymbol") {
                2
            } else if f.name.contains("Hack") {
                5
            }
            // Move Hack to the end
            else if f.name.contains("DSEG") {
                6
            }
            // Move DSEG even further
            else {
                3
            }
        });

        fonts
    }

    /// Get font without generation check (for rasterizer access)
    pub fn get_font(&self) -> Option<FontAsset> {
        for i in 0..8 {
            if FONTS_READY[i].ready.load(Ordering::Acquire) {
                return unsafe { (*FONTS_READY[i].value.get()).clone() };
            }
        }
        None
    }

    /// Mark an asset as used this frame
    pub fn mark_used(&self, asset_type: AssetType, frame_id: u64) {
        match asset_type {
            AssetType::Wallpaper => WALLPAPER_READY.mark_used(frame_id),
            AssetType::Cursor => CURSOR_READY.mark_used(frame_id),
            AssetType::Font => {
                for i in 0..8 {
                    FONTS_READY[i].mark_used(frame_id);
                }
            }
        }
    }

    /// Mark an asset as reachable (in scene graph) or not
    pub fn mark_reachable(&self, asset_type: AssetType, reachable: bool) {
        match asset_type {
            AssetType::Wallpaper => WALLPAPER_READY.mark_reachable(reachable),
            AssetType::Cursor => CURSOR_READY.mark_reachable(reachable),
            AssetType::Font => {
                for i in 0..8 {
                    FONTS_READY[i].mark_reachable(reachable);
                }
            }
        }
    }

    /// Try to evict one asset that is safe to evict (gen < min_live_gen).
    /// Returns the number of bytes freed, or None if nothing can be evicted.
    pub fn try_evict_one(&self, min_live_gen: AssetGeneration) -> Option<usize> {
        // Get metadata for all ready assets
        let mut candidates: [(AssetType, Option<reclaimer::AssetMetadata>, AssetGeneration); 3] = [
            (AssetType::Wallpaper, None, AssetGeneration::ZERO),
            (AssetType::Cursor, None, AssetGeneration::ZERO),
            (AssetType::Font, None, AssetGeneration::ZERO),
        ];

        // Collect metadata
        if WALLPAPER_READY.ready.load(Ordering::Acquire) {
            if let Some(img) = unsafe { (*WALLPAPER_READY.value.get()).as_ref() } {
                let mut meta = WALLPAPER_READY.get_metadata();
                if let Some(ref mut m) = meta {
                    m.gen = img.gen;
                }
                candidates[0] = (AssetType::Wallpaper, meta, img.gen);
            }
        }

        if CURSOR_READY.ready.load(Ordering::Acquire) {
            if let Some(cursor) = unsafe { (*CURSOR_READY.value.get()).as_ref() } {
                let gen = cursor.generation();
                let mut meta = CURSOR_READY.get_metadata();
                if let Some(ref mut m) = meta {
                    m.gen = gen;
                }
                candidates[1] = (AssetType::Cursor, meta, gen);
            }
        }

        for i in 0..8 {
            if FONTS_READY[i].ready.load(Ordering::Acquire) {
                if let Some(font) = unsafe { (*FONTS_READY[i].value.get()).as_ref() } {
                    let mut meta = FONTS_READY[i].get_metadata();
                    if let Some(ref mut m) = meta {
                        m.gen = font.gen;
                    }
                    // For now, simplify eviction candidacy for multi-fonts
                    // We'll only track the first font slot in this simplified loop structure
                    if i == 0 {
                        candidates[2] = (AssetType::Font, meta, font.gen);
                    }
                }
            }
        }

        // Filter to only safe-to-evict assets (gen < min_live_gen)
        let mut evictable: alloc::vec::Vec<(AssetType, reclaimer::AssetMetadata)> = candidates
            .iter()
            .filter_map(|(asset_type, meta, gen)| {
                if *gen < min_live_gen {
                    meta.clone().map(|m| (*asset_type, m))
                } else {
                    None
                }
            })
            .collect();

        if evictable.is_empty() {
            return None;
        }

        // Sort by eviction priority (lowest first = evict first)
        evictable.sort_by_key(|(_, m)| m.eviction_priority());

        // Evict the lowest priority asset
        let (asset_type, meta) = &evictable[0];
        let freed = meta.decoded_bytes;

        match asset_type {
            AssetType::Wallpaper => {
                info!(
                    "[asset_bank] evicting wallpaper (gen={}, {}b)",
                    meta.gen.0, freed
                );
                unsafe {
                    *WALLPAPER_READY.value.get() = None;
                }
                WALLPAPER_READY.ready.store(false, Ordering::Release);
                WALLPAPER_READY.decoded_bytes.store(0, Ordering::Release);
            }
            AssetType::Cursor => {
                info!(
                    "[asset_bank] evicting cursor (gen={}, {}b)",
                    meta.gen.0, freed
                );
                unsafe {
                    *CURSOR_READY.value.get() = None;
                }
                CURSOR_READY.ready.store(false, Ordering::Release);
                CURSOR_READY.decoded_bytes.store(0, Ordering::Release);
            }
            AssetType::Font => {
                info!(
                    "[asset_bank] evicting fonts (gen={}, {}b)",
                    meta.gen.0, freed
                );
                for i in 0..8 {
                    unsafe {
                        *FONTS_READY[i].value.get() = None;
                    }
                    FONTS_READY[i].ready.store(false, Ordering::Release);
                    FONTS_READY[i].decoded_bytes.store(0, Ordering::Release);
                }
            }
        }

        reclaimer::sub_decoded_bytes(freed);
        Some(freed)
    }

    fn probe_asset(name: &str) -> Option<(ThingId, usize)> {
        use stem::abi::schema::kinds;
        use stem::thing::sys::{bytespace_info, describe_thing, find, prop_get};

        // Guard: reject empty search names
        if name.is_empty() {
            return None;
        }

        let mut modules = [ThingId::default(); 64];
        let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);

        for i in 0..count {
            let mod_id = modules[i];
            let mut buf = [0u8; 512];
            let len = match describe_thing(mod_id, &mut buf) {
                Ok(l) => l,
                Err(_) => continue,
            };

            let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
            let mod_name = if let Some(pos) = desc.find("name: \"") {
                let rest = &desc[pos + 7..];
                if let Some(end) = rest.find('"') {
                    &rest[..end]
                } else {
                    continue;
                }
            } else {
                continue;
            };

            // Skip empty module names
            if mod_name.is_empty() {
                continue;
            }

            // Match exact name or proper path suffix
            let is_match = mod_name == name || mod_name.ends_with(name);
            if !is_match {
                continue;
            }

            let bs_id = match prop_get(mod_id, "bytespace") {
                Ok(id) => ThingId::from_u64(id),
                Err(_) => continue,
            };

            if let Ok(size) = bytespace_info(bs_id) {
                return Some((bs_id, size));
            }
        }
        None
    }

    pub fn load_wallpaper_from_graph(&self, path: &str) -> Option<Image> {
        self.load_wallpaper_immediate(path)
    }

    pub fn load_wallpaper_immediate(&self, path: &str) -> Option<Image> {
        info!("[asset_bank] load_wallpaper_immediate: {}", path);
        let (id, size) = Self::probe_asset(path)?;

        info!("[asset_bank] mapping bytespace {} ({} bytes)", id.to_u64_lossy(), size);
        let ptr = stem::thing::sys::bytespace_map(id).ok()?;
        info!("[asset_bank] mapped to {:p}", ptr);
        let slice = unsafe { core::slice::from_raw_parts(ptr, size) };

        info!("[asset_bank] decoding BMP...");
        let res = crate::bmp::decode(slice).ok().map(|bmp| {
            info!("[asset_bank] BMP decoded: {}x{}", bmp.width, bmp.height);
            Image {
                width: bmp.width,
                height: bmp.height,
                pixels: Arc::from(bmp.pixels.as_slice()),
                gen: AssetGeneration::ZERO, // Will be set on promotion
            }
        });

        let _ = stem::thing::sys::bytespace_unmap(id, ptr);
        info!("[asset_bank] bytespace unmapped");
        res
    }

    pub fn load_cursor_from_graph(path: &str) -> Option<CursorAsset> {
        Self::load_cursor_immediate(path)
    }


    #[cfg(feature = "svg-cursors")]
    fn load_svg_cursor(slice: &[u8], path: &str) -> Option<CursorAsset> {
        let svg_content = match core::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => {
                info!("[asset_bank] SVG is not valid UTF-8");
                return None;
            }
        };

        info!("[asset_bank] parsing SVG cursor from {}", path);

        // Rasterize SVG to 32x32 @ 1.0 scale (or scaled up? Windows uses 32x32 usually, large is 48)
        // Let's use 32x32 for now.
        // If we want high-dpi, we might want 64x64 or 96x96 and let the cursor asset handling know.
        // But Image is pixel data.
        // Current cursor.rs implementation uses scale=3.0 hardcoded for SVG.
        // 32 * 3.0 = 96.
        // Let's rasterize at 96x96 to match the "large" look checking cursor.rs logic.
        let scale = 3.0;
        let base_size = 32;
        let size = (base_size as f32 * scale) as i32;

        let pixels_vec = crate::svg::render_to_buffer(svg_content, size, size, scale);


        // Convert Vec<u32> to Arc<[u32]>
        let pixels = Arc::from(pixels_vec.into_boxed_slice());

        info!("[asset_bank] SUCCESS: SVG cursor rasterized {}x{}", size, size);

        // Hotspot: default.svg config says (6,4) at 24px, scale to 96px = (24,16)
        let hotspot_scale = scale;
        let hotspot_x = (6.0 * hotspot_scale) as u32;
        let hotspot_y = (4.0 * hotspot_scale) as u32;

        Some(CursorAsset::Static(CursorFrame {
            image: Image {
                width: size as u32,
                height: size as u32,
                pixels,
                gen: AssetGeneration::ZERO,
            },
            delay_ms: 0,
            hotspot_x,
            hotspot_y,
        }))
    }

    pub fn load_cursor_immediate(path: &str) -> Option<CursorAsset> {
        info!("[asset_bank] load_cursor_immediate: {}", path);
        let (id, size) = Self::probe_asset(path)?;

        info!("[asset_bank] mapping bytespace {} ({} bytes)", id.to_u64_lossy(), size);
        let ptr = stem::thing::sys::bytespace_map(id).ok()?;
        info!("[asset_bank] mapped to {:p}", ptr);
        let slice = unsafe { core::slice::from_raw_parts(ptr, size) };

        // Check for SVG format first (when feature is enabled)
        #[cfg(feature = "svg-cursors")]
        {
            // SVG files start with "<?xml" or "<svg"
            if size > 5 && (
                (slice[0] == b'<' && slice[1] == b'?' && slice[2] == b'x' && slice[3] == b'm' && slice[4] == b'l') ||
                (slice[0] == b'<' && slice[1] == b's' && slice[2] == b'v' && slice[3] == b'g')
            ) {
                info!("[asset_bank] detected SVG format");
                let result = Self::load_svg_cursor(slice, path);
                let _ = stem::thing::sys::bytespace_unmap(id, ptr);
                return result;
            }
        }

        info!("[asset_bank] checking CUR header: len={}", slice.len());

        // ICO/CUR check: type=2 for CUR
        if slice.len() > 22 && slice[0] == 0 && slice[1] == 0 && slice[2] == 2 && slice[3] == 0 {
            info!("[asset_bank] valid CUR header detected");
            let hx = u16::from_le_bytes([slice[10], slice[11]]);
            let hy = u16::from_le_bytes([slice[12], slice[13]]);
            let img_size =
                u32::from_le_bytes([slice[14], slice[15], slice[16], slice[17]]) as usize;
            let offset = u32::from_le_bytes([slice[18], slice[19], slice[20], slice[21]]) as usize;

            info!(
                "[asset_bank] CUR: hotspot=({}, {}), img_size={}, offset={}",
                hx, hy, img_size, offset
            );

            if slice.len() >= offset + img_size {
                info!("[asset_bank] decoding embedded DIB at offset {}...", offset);
                // CUR files embed DIB (no BM header), use decode_dib
                match crate::bmp::decode_dib(&slice[offset..offset + img_size]) {
                    Ok(dib) => {
                        info!(
                            "[asset_bank] SUCCESS: cursor DIB decoded {}x{}",
                            dib.width, dib.height
                        );
                        let _ = stem::thing::sys::bytespace_unmap(id, ptr);
                        return Some(CursorAsset::Static(CursorFrame {
                            image: Image {
                                width: dib.width,
                                height: dib.height,
                                pixels: Arc::from(dib.pixels.as_slice()),
                                gen: AssetGeneration::ZERO,
                            },
                            delay_ms: 0,
                            hotspot_x: hx as u32,
                            hotspot_y: hy as u32,
                        }));
                    }
                    Err(e) => {
                        info!("[asset_bank] DIB decode FAILED: {:?}", e);
                    }
                }
            } else {
                info!(
                    "[asset_bank] CUR data truncated: need {} have {}",
                    offset + img_size,
                    slice.len()
                );
            }
        } else {
            info!("[asset_bank] NOT a valid CUR file (magic bytes don't match)");
            if slice.len() >= 4 {
                info!(
                    "[asset_bank] header bytes: {:02x} {:02x} {:02x} {:02x}",
                    slice[0], slice[1], slice[2], slice[3]
                );
            }
        }

        let _ = stem::thing::sys::bytespace_unmap(id, ptr);
        info!("[asset_bank] bytespace unmapped, returning None");
        None
    }
    /// Load a TTF font from the system graph
    pub fn load_font_from_graph_by_path(path: &str) -> Option<FontAsset> {
        info!("[asset_bank] load_font_from_graph_by_path: {}", path);
        let (id, size) = Self::probe_asset(path)?;
        Self::load_font_from_node_id(id, size, path)
    }

    pub fn load_font_from_node_id(
        id: ThingId,
        size: usize,
        display_name: &str,
    ) -> Option<FontAsset> {
        Self::load_font_immediate(id, size, display_name)
    }


    pub fn load_font_immediate(id: ThingId, size: usize, display_name: &str) -> Option<FontAsset> {
        info!(
            "[asset_bank] mapping font bytespace {} ({} bytes)",
            id.to_u64_lossy(), size
        );
        let ptr = match stem::thing::sys::bytespace_map(id) {
            Ok(p) => p,
            Err(e) => {
                info!("[asset_bank] bytespace_map FAILED: {:?}", e);
                return None;
            }
        };
        info!("[asset_bank] mapped at {:p}", ptr);
        let slice = unsafe { core::slice::from_raw_parts(ptr, size) };

        info!("[asset_bank] parsing font '{}'...", display_name);

        let settings = fontdue::FontSettings::default();
        match fontdue::Font::from_bytes(slice, settings) {
            Ok(font) => {
                info!("[asset_bank] SUCCESS: font parsed");
                // NOTE: We do NOT unmap on success because fontdue keeps a reference to the slice.
                // Extract name from path/display name
                // Handles both raw paths and the debug-printed "module{ name: \"...\" }" format
                let name_part = if display_name.contains("name: \"") {
                    display_name
                        .split("name: \"")
                        .nth(1)
                        .unwrap_or(display_name)
                        .split('"')
                        .next()
                        .unwrap_or(display_name)
                } else {
                    display_name.rsplit('/').next().unwrap_or(display_name)
                };
                let name: Arc<str> = name_part.into();

                Some(FontAsset {
                    font: Arc::new(font),
                    name,
                    gen: AssetGeneration::ZERO,
                    glyph_cache: Arc::new(Mutex::new(BTreeMap::new())),
                })
            }
            Err(e) => {
                info!("[asset_bank] font parse FAILED: {}", e);
                let _ = stem::thing::sys::bytespace_unmap(id, ptr);
                None
            }
        }
    }
}
