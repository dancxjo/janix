extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;

#[derive(Debug, Clone, Copy)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: &'static [u32],
}

// Global storage for the wallpaper to avoid allocation in loader.
// We use a "Init-Write-Publish" pattern.
struct WallpaperStorage {
    // UnsafeCell to allow mutation by Loader after Main initialized it.
    // Safety: Only accessed by Main (init) and Loader (write) based on protocol.
    pixels: UnsafeCell<Option<Vec<u32>>>, 
    dims: UnsafeCell<(u32, u32)>,
    ready: AtomicBool,
}

// Safety: We manage sync manually.
unsafe impl Sync for WallpaperStorage {}

static STORAGE: WallpaperStorage = WallpaperStorage {
    pixels: UnsafeCell::new(None),
    dims: UnsafeCell::new((0, 0)),
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
        // Zero initialize to be safe, though not strictly required if we overwrite
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
        STORAGE.ready.store(true, Ordering::Release);
    }

    /// Called by Main Thread loop.
    pub fn get_clouds(&self) -> Option<Image> {
        if STORAGE.ready.load(Ordering::Acquire) {
            unsafe {
                let (w, h) = *STORAGE.dims.get();
                let ptr = STORAGE.pixels.get();
                if let Some(vec) = &*ptr {
                   return Some(Image {
                       width: w,
                       height: h,
                       pixels: vec.as_slice(),
                   });
                }
            }
        }
        None
    }
}
