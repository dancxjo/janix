use abi::ids::{SymbolId, ThingId, WatchId};
use abi::types::{WatchEvent, WatchEventKind};
use alloc::{collections::{BTreeMap, BTreeSet}, format, vec::Vec};
use thing_std::{log_info, memory, SyscallGraphClient, trace_fn};

use crate::scene::Rect;
use crate::ui::{read_window_scene, WindowScene};
use crate::watch::WatchSet;

pub struct MappedRegion {
    pub ptr: *mut u8,
    pub mapped_len: usize,
}

pub enum MapResult<'a> {
    Hit(&'a [u8]),
    Mapped(&'a [u8]),
}

impl<'a> core::ops::Deref for MapResult<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self {
            MapResult::Hit(s) => s,
            MapResult::Mapped(s) => s,
        }
    }
}

pub enum MapResultMut<'a> {
    Hit(&'a mut [u8]),
    Mapped(&'a mut [u8]),
}

impl<'a> core::ops::Deref for MapResultMut<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self {
            MapResultMut::Hit(s) => s,
            MapResultMut::Mapped(s) => s,
        }
    }
}

impl<'a> core::ops::DerefMut for MapResultMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MapResultMut::Hit(s) => s,
            MapResultMut::Mapped(s) => s,
        }
    }
}

pub struct BytespaceMappingCache {
    mappings: BTreeMap<ThingId, MappedRegion>,
    // Debug stats
    frame_maps: usize,
    frame_hits: usize,
    frame_remaps: usize,
}

impl BytespaceMappingCache {
    pub fn new() -> Self {
        Self {
            mappings: BTreeMap::new(),
            frame_maps: 0,
            frame_hits: 0,
            frame_remaps: 0,
        }
    }

    pub fn reset_frame_stats(&mut self) {
        self.frame_maps = 0;
        self.frame_hits = 0;
        self.frame_remaps = 0;
    }

    pub fn log_frame_stats(&self) {
        if self.frame_maps > 0 || self.frame_remaps > 0 {
             log_info(&alloc::format!("BLOOM: frame maps={} hits={} remaps={}", self.frame_maps, self.frame_hits, self.frame_remaps));
        }
    }

    pub fn get_or_map_ro(&mut self, id: ThingId, len: usize) -> Option<MapResult<'_>> {
        let is_new = self.ensure_mapped(id, len);
        let region = self.mappings.get(&id)?;
        let slice = unsafe { core::slice::from_raw_parts(region.ptr, len) };
        Some(if is_new { MapResult::Mapped(slice) } else { MapResult::Hit(slice) })
    }

    pub fn get_or_map_rw(&mut self, id: ThingId, len: usize) -> Option<MapResultMut<'_>> {
         let is_new = self.ensure_mapped(id, len);
         let region = self.mappings.get(&id)?;
         let slice = unsafe { core::slice::from_raw_parts_mut(region.ptr, len) };
         Some(if is_new { MapResultMut::Mapped(slice) } else { MapResultMut::Hit(slice) })
    }

    /// Returns true if a new mapping was created or remapped, false if hit.
    fn ensure_mapped(&mut self, id: ThingId, len: usize) -> bool {
        if let Some(region) = self.mappings.get(&id) {
            if region.mapped_len >= len {
                self.frame_hits += 1;
                return false;
            }
            // Resize needed
            self.frame_remaps += 1;
        } else {
            self.frame_maps += 1;
        }

        // Trace mapping operations (they can be slow)
        trace_fn!("bytespace_map");
        
        let mapped_addr = memory::space_map(id, 0, 0, len as u64);
        
        if mapped_addr != 0 {
             self.mappings.insert(id, MappedRegion {
                 ptr: mapped_addr as *mut u8,
                 mapped_len: len,
             });
             true
        } else {
             log_info("BLOOM: failed to map bytespace");
             false
        }
    }
}

pub struct WindowView {
    pub id: ThingId,
    pub rect: Rect,
    pub z: i32,
    pub title: Option<SymbolId>,
    pub mapped_surface: Option<ThingId>,
    pub flags: u32,
    pub scene: WindowScene,
}

impl Clone for WindowView {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            rect: self.rect,
            z: self.z,
            title: self.title,
            mapped_surface: self.mapped_surface,
            flags: self.flags,
            scene: self.scene.clone(),
        }
    }
}

impl WindowView {
    pub fn from_scene(scene: WindowScene) -> Self {
        let rect = Rect {
            x: scene.window.x,
            y: scene.window.y,
            w: scene.window.width,
            h: scene.window.height,
        };
        Self {
            id: scene.id,
            rect,
            z: 0,
            title: Some(scene.window.title),
            mapped_surface: None,
            flags: 0,
            scene,
        }
    }
}

pub struct SceneCache {
    pub windows: BTreeMap<ThingId, WindowView>,
    pub dirty: BTreeSet<ThingId>,
    pub dirty_scene: bool,
    pub mapping_cache: BytespaceMappingCache,
}

impl SceneCache {
    pub fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
            dirty: BTreeSet::new(),
            dirty_scene: false,
            mapping_cache: BytespaceMappingCache::new(),
        }
    }

    pub fn upsert(&mut self, scene: WindowScene) {
        let view = WindowView::from_scene(scene);
        let id = view.id;
        self.windows.insert(id, view);
        self.dirty.insert(id);
    }

    pub fn remove(&mut self, id: ThingId) {
        if self.windows.remove(&id).is_some() {
            self.dirty_scene = true;
        }
    }

    pub fn mark_scene_dirty(&mut self) {
        self.dirty_scene = true;
    }

    pub fn mark_window_dirty(&mut self, id: ThingId) {
        if self.windows.contains_key(&id) {
            self.dirty.insert(id);
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty_scene || !self.dirty.is_empty()
    }

    pub fn dirty_count(&self) -> usize {
        if self.dirty_scene {
            self.windows.len().max(1)
        } else {
            self.dirty.len()
        }
    }

    pub fn clear_dirty(&mut self) {
        self.dirty.clear();
        self.dirty_scene = false;
    }

    pub fn scenes_in_order(&self) -> Vec<WindowScene> {
        self.windows
            .values()
            .map(|v| v.scene.clone())
            .collect::<Vec<_>>()
    }
}

pub fn apply_watch_event(
    event: &WatchEvent,
    cache: &mut SceneCache,
    watches: &mut WatchSet,
    client: &mut SyscallGraphClient,
) -> Option<(ThingId, WatchId)> {
    match event.kind {
        WatchEventKind::GraphMemberAdded => {
            if let Some(scene) = read_window_scene(client, event.arg0) {
                cache.upsert(scene);
                cache.mark_scene_dirty();
                if let Ok(Some(window_watch)) = watches.ensure_window_watch(event.arg0) {
                    return Some((event.arg0, window_watch));
                }
            }
        }
        WatchEventKind::GraphMemberRemoved => {
            cache.remove(event.arg0);
            cache.mark_scene_dirty();
            watches.remove_window_watch(event.arg0);
        }
        WatchEventKind::ThingUpdated => {
            if let Some(scene) = read_window_scene(client, event.subject) {
                cache.upsert(scene);
                cache.mark_window_dirty(event.subject);
            } else {
                cache.remove(event.subject);
                watches.remove_window_watch(event.subject);
            }
        }
        WatchEventKind::ThingDeleted => {
            cache.remove(event.subject);
            watches.remove_window_watch(event.subject);
        }
    }
    None
}

impl SceneCache {
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }
}
