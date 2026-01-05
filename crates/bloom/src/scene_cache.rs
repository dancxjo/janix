use abi::ids::{SymbolId, ThingId, WatchId};
use abi::types::{WatchEvent, WatchEventKind};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use thing_std::SyscallGraphClient;

use crate::scene::Rect;
use crate::ui::{read_window_scene, WindowScene};
use crate::watch::WatchSet;

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
}

impl SceneCache {
    pub fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
            dirty: BTreeSet::new(),
            dirty_scene: false,
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
