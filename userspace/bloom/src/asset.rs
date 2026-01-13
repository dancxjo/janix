extern crate alloc;

use stem::thing::ThingId;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;

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

static STORAGE: WallpaperStorage = WallpaperStorage {
    image: UnsafeCell::new(None),
    ready: AtomicBool::new(false),
};

pub struct AssetBank;

impl AssetBank {
    pub const fn new() -> Self { Self }

    pub fn publish_image(&self, img: Image) {
        unsafe { *STORAGE.image.get() = Some(img); }
        STORAGE.ready.store(true, Ordering::Release);
    }

    pub fn get_clouds(&self) -> Option<Image> {
        if STORAGE.ready.load(Ordering::Acquire) {
            unsafe { (*STORAGE.image.get()).clone() }
        } else {
            None
        }
    }
    
    fn probe_asset(name: &str) -> Option<(ThingId, usize)> {
        use stem::thing::sys::{find, describe_thing, prop_get, bytespace_info};
        use stem::abi::schema::kinds;
        use stem::info;
        
        let mut modules = [ThingId(0); 64];
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
                if let Some(end) = rest.find('"') { &rest[..end] } else { continue; }
            } else { continue; };
            
            if !(mod_name == name || mod_name.ends_with(name) || name.ends_with(mod_name)) {
                continue;
            }
            
            info!("loader: match '{}' -> '{}'", name, mod_name);
            
            let bs_id = match prop_get(mod_id, "bytespace") {
                Ok(id) => ThingId(id),
                Err(_) => continue,
            };
            
            if let Ok(size) = bytespace_info(bs_id) {
                info!("loader: found bytespace {} ({}b)", bs_id.0, size);
                return Some((bs_id, size));
            }
        }
        None
    }

    pub fn load_wallpaper_from_graph(&self, path: &str) -> Option<Image> {
        use stem::info;
        let (id, size) = Self::probe_asset(path)?;
        
        let ptr = stem::thing::sys::bytespace_map(id).ok()?;
        let slice = unsafe { core::slice::from_raw_parts(ptr, size) };
        
        let res = crate::bmp::decode(slice).ok().map(|bmp| Image {
            width: bmp.width,
            height: bmp.height,
            pixels: Arc::from(bmp.pixels.as_slice()),
        });
        
        let _ = stem::thing::sys::bytespace_unmap(id, ptr);
        res
    }
    
    pub fn load_cursor_from_graph(path: &str) -> Option<CursorAsset> {
        let (id, size) = Self::probe_asset(path)?;
        let ptr = stem::thing::sys::bytespace_map(id).ok()?;
        let slice = unsafe { core::slice::from_raw_parts(ptr, size) };
        
        // ICO/CUR check
        if slice.len() > 22 && slice[0]==0 && slice[1]==0 && slice[2]==2 && slice[3]==0 {
            let hx = u16::from_le_bytes([slice[10], slice[11]]);
            let hy = u16::from_le_bytes([slice[12], slice[13]]);
            let img_size = u32::from_le_bytes([slice[14], slice[15], slice[16], slice[17]]) as usize;
            let offset = u32::from_le_bytes([slice[18], slice[19], slice[20], slice[21]]) as usize;
            
            if slice.len() >= offset + img_size {
                if let Ok(bmp) = crate::bmp::decode(&slice[offset..offset+img_size]) {
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
                }
            }
        }
        
        let _ = stem::thing::sys::bytespace_unmap(id, ptr);
        None
    }
}
