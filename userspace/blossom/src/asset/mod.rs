extern crate alloc;

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};

use crate::asset::cursor::CursorTheme;
use crate::asset::wallpaper::WallpaperSurface;

pub mod cursor;
pub mod wallpaper;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssetKind {
    Wallpaper,
    Cursor,
}

#[derive(Clone, Copy, Debug)]
pub struct AssetJob {
    pub kind: AssetKind,
    pub bytes: &'static [u8],
    pub name: &'static str,
}

pub enum AssetUpdate {
    WallpaperReady(Arc<WallpaperSurface>),
    CursorReady(Arc<CursorTheme>),
}

pub struct AssetPack {
    pub generation: u64,
    pub wallpaper: Arc<WallpaperSurface>,
    pub cursor: Arc<CursorTheme>,
}

impl AssetPack {
    pub fn placeholder() -> Arc<Self> {
        Arc::new(Self {
            generation: 0,
            wallpaper: Arc::new(WallpaperSurface::checkerboard()),
            cursor: Arc::new(CursorTheme::placeholder()),
        })
    }
}

pub struct AssetHub {
    current: AtomicPtr<AssetPack>,
    generation: AtomicU64,
}

impl AssetHub {
    pub fn new(initial: Arc<AssetPack>) -> Self {
        let raw = Arc::into_raw(initial) as *mut AssetPack;
        Self {
            current: AtomicPtr::new(raw),
            generation: AtomicU64::new(0),
        }
    }

    pub fn current_pack(&self) -> Arc<AssetPack> {
        let ptr = self.current.load(Ordering::Acquire);
        unsafe {
            Arc::increment_strong_count(ptr);
            Arc::from_raw(ptr)
        }
    }

    pub fn submit_ready_pack(&self, new_pack: Arc<AssetPack>) {
        let raw = Arc::into_raw(new_pack) as *mut AssetPack;
        let old = self.current.swap(raw, Ordering::AcqRel);
        if !old.is_null() {
            unsafe {
                drop(Arc::from_raw(old));
            }
        }
        self.generation.fetch_add(1, Ordering::Relaxed);
    }

    pub fn next_generation(&self, base: u64) -> u64 {
        base.saturating_add(1)
    }
}

pub fn apply_update(base: &Arc<AssetPack>, update: AssetUpdate, next_gen: u64) -> Arc<AssetPack> {
    match update {
        AssetUpdate::WallpaperReady(wallpaper) => Arc::new(AssetPack {
            generation: next_gen,
            wallpaper,
            cursor: Arc::clone(&base.cursor),
        }),
        AssetUpdate::CursorReady(cursor) => Arc::new(AssetPack {
            generation: next_gen,
            wallpaper: Arc::clone(&base.wallpaper),
            cursor,
        }),
    }
}

pub struct AssetJobQueue {
    inner: SpinLock<VecDeque<AssetJob>>,
}

impl AssetJobQueue {
    pub fn new() -> Self {
        Self {
            inner: SpinLock::new(VecDeque::new()),
        }
    }

    pub fn push(&self, job: AssetJob) {
        let mut guard = self.inner.lock();
        guard.push_back(job);
    }

    pub fn pop(&self) -> Option<AssetJob> {
        let mut guard = self.inner.lock();
        guard.pop_front()
    }
}

pub struct AssetResultQueue {
    inner: SpinLock<VecDeque<AssetUpdate>>,
}

impl AssetResultQueue {
    pub fn new() -> Self {
        Self {
            inner: SpinLock::new(VecDeque::new()),
        }
    }

    pub fn push(&self, update: AssetUpdate) {
        let mut guard = self.inner.lock();
        guard.push_back(update);
    }

    pub fn pop(&self) -> Option<AssetUpdate> {
        let mut guard = self.inner.lock();
        guard.pop_front()
    }
}

struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    fn lock(&self) -> SpinLockGuard<'_, T> {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            stem::thread::yield_now();
        }
        SpinLockGuard { lock: self }
    }
}

struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> core::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> core::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}
