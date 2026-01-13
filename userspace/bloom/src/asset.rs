extern crate alloc;

use stem::thing::ThingId;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;
use stem::info;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Arc<[u32]>,
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

struct WallpaperStorage {
    image: UnsafeCell<Option<Image>>,
    ready: AtomicBool,
}

unsafe impl Sync for WallpaperStorage {}

static WALLPAPER_STORAGE: WallpaperStorage = WallpaperStorage {
    image: UnsafeCell::new(None),
    ready: AtomicBool::new(false),
};

struct CursorStorage {
    cursor: UnsafeCell<Option<CursorAsset>>,
    ready: AtomicBool,
}

unsafe impl Sync for CursorStorage {}

static CURSOR_STORAGE: CursorStorage = CursorStorage {
    cursor: UnsafeCell::new(None),
    ready: AtomicBool::new(false),
};

pub struct AssetBank;

impl AssetBank {
    pub const fn new() -> Self { Self }

    pub fn publish_wallpaper(&self, img: Image) {
        info!("[asset_bank] publish_wallpaper: {}x{}", img.width, img.height);
        unsafe { *WALLPAPER_STORAGE.image.get() = Some(img); }
        WALLPAPER_STORAGE.ready.store(true, Ordering::Release);
        info!("[asset_bank] wallpaper ready flag set to true");
    }

    pub fn get_wallpaper(&self) -> Option<Image> {
        if WALLPAPER_STORAGE.ready.load(Ordering::Acquire) {
            unsafe { (*WALLPAPER_STORAGE.image.get()).clone() }
        } else {
            None
        }
    }

    pub fn publish_cursor(&self, cursor: CursorAsset) {
        info!("[asset_bank] publish_cursor called");
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
        unsafe { *CURSOR_STORAGE.cursor.get() = Some(cursor); }
        CURSOR_STORAGE.ready.store(true, Ordering::Release);
        info!("[asset_bank] cursor ready flag set to true");
    }

    pub fn get_cursor(&self) -> Option<CursorAsset> {
        let ready = CURSOR_STORAGE.ready.load(Ordering::Acquire);
        if ready {
            let result = unsafe { (*CURSOR_STORAGE.cursor.get()).clone() };
            if result.is_some() {
                info!("[asset_bank] get_cursor: returning Some(cursor)");
            }
            result
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
        
        // ICO/CUR check
        if slice.len() > 22 && slice[0]==0 && slice[1]==0 && slice[2]==2 && slice[3]==0 {
            info!("[asset_bank] valid CUR header detected");
            let hx = u16::from_le_bytes([slice[10], slice[11]]);
            let hy = u16::from_le_bytes([slice[12], slice[13]]);
            let img_size = u32::from_le_bytes([slice[14], slice[15], slice[16], slice[17]]) as usize;
            let offset = u32::from_le_bytes([slice[18], slice[19], slice[20], slice[21]]) as usize;
            
            info!("[asset_bank] CUR: hotspot=({}, {}), img_size={}, offset={}", hx, hy, img_size, offset);
            
            if slice.len() >= offset + img_size {
                info!("[asset_bank] decoding embedded BMP at offset {}...", offset);
                if let Ok(bmp) = crate::bmp::decode(&slice[offset..offset+img_size]) {
                    info!("[asset_bank] SUCCESS: cursor BMP decoded {}x{}", bmp.width, bmp.height);
                    let _ = stem::thing::sys::bytespace_unmap(id, ptr);
                    return Some(CursorAsset::Static(CursorFrame {
                        image: Image {
                            width: bmp.width,
                            height: bmp.height,
                            pixels: Arc::from(bmp.pixels.as_slice())
                        },
                        delay_ms: 0,
                        hotspot_x: hx as u32,
                        hotspot_y: hy as u32,
                    }));
                } else {
                    info!("[asset_bank] BMP decode FAILED");
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
