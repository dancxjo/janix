extern crate alloc;

use stem::thing::ThingId;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use core::cell::UnsafeCell;
use stem::info;

use crate::frame::AssetGeneration;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Arc<[u32]>,
    /// Generation when this asset became ready
    pub gen: AssetGeneration,
}

#[derive(Debug, Clone)]
pub struct CursorFrame {
    pub image: Image,
    pub delay_ms: u32,
    pub hotspot_x: u32,
    pub hotspot_y: u32,
}

#[derive(Debug, Clone)]
pub enum CursorAsset {
    Static(CursorFrame),
    Animated { frames: Arc<[CursorFrame]> },
}

impl CursorAsset {
    pub fn generation(&self) -> AssetGeneration {
        match self {
            CursorAsset::Static(f) => f.image.gen,
            CursorAsset::Animated { frames } => {
                frames.first().map(|f| f.image.gen).unwrap_or(AssetGeneration::ZERO)
            }
        }
    }
}

/// Storage for ready assets
struct AssetSlot<T> {
    value: UnsafeCell<Option<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for AssetSlot<T> {}

impl<T> AssetSlot<T> {
    const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            ready: AtomicBool::new(false),
        }
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

// Pending assets (published by loaders, not yet visible)
static WALLPAPER_PENDING: PendingSlot<Image> = PendingSlot::new();
static CURSOR_PENDING: PendingSlot<CursorAsset> = PendingSlot::new();

// Global generation counter
static ASSET_GENERATION: AtomicU64 = AtomicU64::new(0);

pub struct AssetBank;

impl AssetBank {
    pub const fn new() -> Self { Self }

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
                info!("[asset_bank] promoting wallpaper to gen={}", new_gen);
                
                unsafe { *WALLPAPER_READY.value.get() = Some(img); }
                WALLPAPER_READY.ready.store(true, Ordering::Release);
                promoted = true;
            }
            WALLPAPER_PENDING.has_pending.store(false, Ordering::Release);
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
                        let updated: alloc::vec::Vec<_> = frames.iter().cloned().map(|mut f| {
                            f.image.gen = AssetGeneration(new_gen);
                            f
                        }).collect();
                        CursorAsset::Animated { frames: Arc::from(updated.as_slice()) }
                    }
                };
                
                info!("[asset_bank] promoting cursor to gen={}", new_gen);
                unsafe { *CURSOR_READY.value.get() = Some(cursor_with_gen); }
                CURSOR_READY.ready.store(true, Ordering::Release);
            }
            CURSOR_PENDING.has_pending.store(false, Ordering::Release);
        }

        self.current_generation()
    }

    /// Publish wallpaper to pending (called by loader thread)
    pub fn publish_wallpaper(&self, img: Image) {
        info!("[asset_bank] publish_wallpaper (pending): {}x{}", img.width, img.height);
        unsafe { *WALLPAPER_PENDING.value.get() = Some(img); }
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
                info!("[asset_bank] cursor: Static frame {}x{} hotspot ({}, {})", 
                    frame.image.width, frame.image.height, 
                    frame.hotspot_x, frame.hotspot_y);
            },
            CursorAsset::Animated { frames } => {
                info!("[asset_bank] cursor: Animated with {} frames", frames.len());
            }
        }
        unsafe { *CURSOR_PENDING.value.get() = Some(cursor); }
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
    
    fn probe_asset(name: &str) -> Option<(ThingId, usize)> {
        use stem::thing::sys::{find, describe_thing, prop_get, bytespace_info};
        use stem::abi::schema::kinds;
        
        info!("[asset_bank] probe_asset: searching for '{}'", name);
        
        let mut modules = [ThingId(0); 64];
        let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
        info!("[asset_bank] probe_asset: found {} boot modules", count);
        
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
                if let Some(end) = rest.find('"') { &rest[..end] } else { continue; }
            } else { continue; };
            
            if !(mod_name == name || mod_name.ends_with(name) || name.ends_with(mod_name)) {
                continue;
            }
            
            info!("[asset_bank] probe_asset: MATCH '{}' -> '{}'", name, mod_name);
            
            let bs_id = match prop_get(mod_id, "bytespace") {
                Ok(id) => ThingId(id),
                Err(e) => {
                    info!("[asset_bank] probe_asset: prop_get failed: {:?}", e);
                    continue;
                },
            };
            
            if let Ok(size) = bytespace_info(bs_id) {
                info!("[asset_bank] probe_asset: bytespace {} size={}b", bs_id.0, size);
                return Some((bs_id, size));
            }
        }
        info!("[asset_bank] probe_asset: '{}' NOT FOUND", name);
        None
    }

    pub fn load_wallpaper_from_graph(&self, path: &str) -> Option<Image> {
        info!("[asset_bank] load_wallpaper_from_graph: {}", path);
        let (id, size) = Self::probe_asset(path)?;
        
        info!("[asset_bank] mapping bytespace {} ({} bytes)", id.0, size);
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
        info!("[asset_bank] load_cursor_from_graph: {}", path);
        let (id, size) = Self::probe_asset(path)?;
        
        info!("[asset_bank] mapping bytespace {} ({} bytes)", id.0, size);
        let ptr = stem::thing::sys::bytespace_map(id).ok()?;
        info!("[asset_bank] mapped to {:p}", ptr);
        let slice = unsafe { core::slice::from_raw_parts(ptr, size) };
        
        info!("[asset_bank] checking CUR header: len={}", slice.len());
        
        // ICO/CUR check: type=2 for CUR
        if slice.len() > 22 && slice[0]==0 && slice[1]==0 && slice[2]==2 && slice[3]==0 {
            info!("[asset_bank] valid CUR header detected");
            let hx = u16::from_le_bytes([slice[10], slice[11]]);
            let hy = u16::from_le_bytes([slice[12], slice[13]]);
            let img_size = u32::from_le_bytes([slice[14], slice[15], slice[16], slice[17]]) as usize;
            let offset = u32::from_le_bytes([slice[18], slice[19], slice[20], slice[21]]) as usize;
            
            info!("[asset_bank] CUR: hotspot=({}, {}), img_size={}, offset={}", hx, hy, img_size, offset);
            
            if slice.len() >= offset + img_size {
                info!("[asset_bank] decoding embedded DIB at offset {}...", offset);
                // CUR files embed DIB (no BM header), use decode_dib
                match crate::bmp::decode_dib(&slice[offset..offset+img_size]) {
                    Ok(dib) => {
                        info!("[asset_bank] SUCCESS: cursor DIB decoded {}x{}", dib.width, dib.height);
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
                    },
                    Err(e) => {
                        info!("[asset_bank] DIB decode FAILED: {:?}", e);
                    }
                }
            } else {
                info!("[asset_bank] CUR data truncated: need {} have {}", offset + img_size, slice.len());
            }
        } else {
            info!("[asset_bank] NOT a valid CUR file (magic bytes don't match)");
            if slice.len() >= 4 {
                info!("[asset_bank] header bytes: {:02x} {:02x} {:02x} {:02x}", slice[0], slice[1], slice[2], slice[3]);
            }
        }
        
        let _ = stem::thing::sys::bytespace_unmap(id, ptr);
        info!("[asset_bank] bytespace unmapped, returning None");
        None
    }
}
