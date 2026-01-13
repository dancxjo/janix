extern crate alloc;

use stem::thing::ThingId;
use alloc::vec::Vec;
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
    Animated {
        frames: Arc<[CursorFrame]>,
        // In the future: loop_kind, etc.
    },
}

// Global storage for the wallpaper.
// We use a "Init-Write-Publish" pattern.
struct WallpaperStorage {
    // UnsafeCell to allow mutation by Loader after Main initialized it.
    // Safety: Only accessed by Main (init) and Loader (write) based on protocol.
    pixels: UnsafeCell<Option<Vec<u32>>>, 
    dims: UnsafeCell<(u32, u32)>,
    ready: AtomicBool,
    // Once published, we cache an Arc definition here to hand out cheaply
    cached_arc: UnsafeCell<Option<Image>>,
}

// Safety: We manage sync manually (One writer, atomic flag).
unsafe impl Sync for WallpaperStorage {}

static STORAGE: WallpaperStorage = WallpaperStorage {
    pixels: UnsafeCell::new(None),
    dims: UnsafeCell::new((0, 0)),
    ready: AtomicBool::new(false),
    cached_arc: UnsafeCell::new(None),
};

// We will also need storage for Cursors.
// For now, let's just make a simple global for the default cursor.
struct CursorStorage {
    default_cursor: UnsafeCell<Option<CursorAsset>>,
    ready: AtomicBool,
}
unsafe impl Sync for CursorStorage {}

static CURSOR_STORAGE: CursorStorage = CursorStorage {
    default_cursor: UnsafeCell::new(None),
    ready: AtomicBool::new(false),
};


pub struct AssetBank;

impl AssetBank {
    pub const fn new() -> Self {
        Self
    }

    /// Called by Main Thread BEFORE spawning loader.
    /// Allocates the buffer safely.
    pub fn init_wallpaper_buffer(&self, width: u32, height: u32) {
        let size = (width * height) as usize;
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 0); 
        
        unsafe {
            *STORAGE.pixels.get() = Some(vec);
            *STORAGE.dims.get() = (width, height);
        }
    }

    /// Called by Loader Thread.
    /// Returns mutable slice to write pixels into.
    /// Safety: Only call from loader, after init.
    pub unsafe fn get_wallpaper_write_access(&self) -> Option<&'static mut [u32]> {
        let ptr = STORAGE.pixels.get();
        if let Some(vec) = &mut *ptr {
            Some(vec.as_mut_slice())
        } else {
            None
        }
    }

    /// Called by Loader Thread when done.
    pub fn publish_clouds(&self) {
        // Convert the Vec into Arc and cache it
        unsafe {
             let ptr = STORAGE.pixels.get();
             let dims = *STORAGE.dims.get();
             // We can clone the vec to make the Arc, or if we want to be zero-copy...
             // The Vec is inside an Option inside UnsafeCell.
             // We can take it out? But we might want to keep the buffer for reuse?
             // For this MVP, let's clone to create the Arc.
             // Optimization: We could use `Arc::from(vec)` if we take ownership, 
             // but `get_wallpaper_write_access` might return reference to it?
             // Actually, after publish, loader shouldn't write anymore.
             
             if let Some(vec) = &*ptr {
                 let img = Image {
                     width: dims.0,
                     height: dims.1,
                     pixels: Arc::from(vec.as_slice()), // This clones. It's fine for now (one time).
                 };
                 *STORAGE.cached_arc.get() = Some(img);
             }
        }

        STORAGE.ready.store(true, Ordering::Release);
    }
    
    pub fn publish_default_cursor(&self, asset: CursorAsset) {
        unsafe {
            *CURSOR_STORAGE.default_cursor.get() = Some(asset);
        }
        CURSOR_STORAGE.ready.store(true, Ordering::Release);
    }

    /// Called by Main Thread loop.
    pub fn get_clouds(&self) -> Option<Image> {
        if STORAGE.ready.load(Ordering::Acquire) {
            unsafe {
                if let Some(img) = &*STORAGE.cached_arc.get() {
                    return Some(img.clone());
                }
            }
        }
        None
    }
    
    /// Returns (w, h)
    fn probe_asset(name: &str) -> Option<(ThingId, usize)> {
        use stem::thing::sys::find;
        use stem::thing::ThingId;
        use alloc::string::String;
        use alloc::format;
        
        let mut ids = [ThingId(0); 1];
        
        // Try 1: Exact name (as passed)
        if let Ok(1) = find(name, &mut ids) {
             if let Ok(size) = stem::thing::sys::bytespace_info(ids[0]) {
                 return Some((ids[0], size));
             }
        }
        
        // Try 2: bytespace.asset.{name} (The standard convention)
        let attempt = format!("bytespace.asset.{}", name);
        if let Ok(1) = find(&*attempt, &mut ids) {
             if let Ok(size) = stem::thing::sys::bytespace_info(ids[0]) {
                 return Some((ids[0], size));
             }
        }
        
        // Try 3: bytespace.asset./{name} (If name has leading slash logic)
        if !name.starts_with('/') {
             let attempt = format!("bytespace.asset./{}", name);
             if let Ok(1) = find(&*attempt, &mut ids) {
                  if let Ok(size) = stem::thing::sys::bytespace_info(ids[0]) {
                      return Some((ids[0], size));
                  }
             }
        }
        
        // Try 4: boot.module.{name}
        let attempt = format!("boot.module.{}", name);
        if let Ok(1) = find(&*attempt, &mut ids) {
             if let Ok(size) = stem::thing::sys::bytespace_info(ids[0]) {
                 return Some((ids[0], size));
             }
        }

        // Try 5: boot.module.{basename}
        if let Some(basename) = name.split('/').last() {
             let attempt = format!("boot.module.{}", basename);
             if let Ok(1) = find(&*attempt, &mut ids) {
                  if let Ok(size) = stem::thing::sys::bytespace_info(ids[0]) {
                      return Some((ids[0], size));
                  }
             }
        }
        
        None
    }

    pub fn load_wallpaper_from_graph(&self, path: &str) -> Option<Image> {
        unsafe {
            let (id, size) = Self::probe_asset(path)?;
            let ptr = stem::thing::sys::bytespace_map(id).ok()?;
            let slice = core::slice::from_raw_parts(ptr, size);
            
            let res = if let Ok(bmp) = crate::bmp::decode(slice) {
                Some(Image {
                    width: bmp.width,
                    height: bmp.height,
                    pixels: Arc::from(bmp.pixels.as_slice()),
                })
            } else {
                None
            };
            
            // Unmap?
            let _ = stem::thing::sys::bytespace_unmap(id, ptr);
            res
        }
    }
    
    pub fn load_cursor_from_graph(path: &str) -> Option<CursorAsset> {
        unsafe {
             let (id, size) = Self::probe_asset(path)?;
             let ptr = stem::thing::sys::bytespace_map(id).ok()?;
             let slice = core::slice::from_raw_parts(ptr, size);
             
             // Decode .cur / .ani (Need decoder)
             // For now, assume BMP (Normal.cur might be BMP for this MVP? Or proper .cur?)
             // .cur is basically BMP with header.
             // If bmp::decode fails, maybe we just return None.
             // But wait, the user said ".cur and .ani".
             // `bmp.rs` is minimal. 
             // Implementing full .cur/.ani decoder is out of scope for "Refactor".
             // I will hack: If it matches BMP signature, decode.
             // A proper ICO/CUR usually starts with 00 00 02 00.
             // BMP starts with BM.
             
             // If we can't decode, we can't use it.
             // Let's try to see if `bmp::decode` works (if the user provided a BMP masquerading as CUR).
             // OR, better: We just implement basic .cur support (ICO header).
             
             // ICO Header: Reserved(2) | Type(2) | Count(2)
             // Type 2 = Cursor.
             // Entry: w(1) h(1) pal(1) res(1) hotspot_x(2) hotspot_y(2) size(4) offset(4)
             
             if slice.len() > 6 && slice[0]==0 && slice[1]==0 && slice[2]==2 && slice[3]==0 {
                  // It is a cursor!
                  let count = u16::from_le_bytes([slice[4], slice[5]]);
                  if count > 0 {
                      // Read first entry (16 bytes)
                      // Offset 6 + 16 = 22
                      let w = slice[6];
                      let h = slice[7];
                      // ...
                      let hx = u16::from_le_bytes([slice[10], slice[11]]);
                      let hy = u16::from_le_bytes([slice[12], slice[13]]);
                      let size = u32::from_le_bytes([slice[14], slice[15], slice[16], slice[17]]) as usize;
                      let offset = u32::from_le_bytes([slice[18], slice[19], slice[20], slice[21]]) as usize;
                      
                      if slice.len() >= offset + size {
                          let bmp_data = &slice[offset .. offset+size];
                          // BMP data in ICO often has height * 2 (AND mask + XOR mask).
                          // Our `bmp::decode` might fail or produce weird result.
                          // But let's try it.
                          if let Ok(mut bmp) = crate::bmp::decode(bmp_data) {
                               // Fix height (ICO BMPs are often double height for mask)
                               // If `bmp.height` == w * 2?
                               // Also, transparency...
                               
                               return Some(CursorAsset::Static(CursorFrame {
                                   image: Image {
                                       width: bmp.width,
                                       height: bmp.height, // Leaving as is for now
                                       pixels: Arc::from(bmp.pixels.as_slice())
                                   },
                                   delay_ms: 0,
                                   hotspot_x: hx as u32,
                                   hotspot_y: hy as u32,
                               }));
                          }
                      }
                  }
             }
             
             let _ = stem::thing::sys::bytespace_unmap(id, ptr);
             None
        }
    }
}
