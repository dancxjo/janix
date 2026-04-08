use crate::geometry::Rect;
use crate::surface::Surface;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// A simple compositional scene graph tree.
/// It holds Surfaces directly, and tracks the z-order stacking and damage calculation.
pub struct SceneGraph {
    /// Ordered from back to front (bottom to top).
    /// The backmost surface is at index 0.
    pub ordered_surfaces: Vec<u64>,
    pub surfaces: BTreeMap<u64, Surface>,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            ordered_surfaces: Vec::new(),
            surfaces: BTreeMap::new(),
        }
    }

    pub fn get_surface(&self, id: u64) -> Option<&Surface> {
        self.surfaces.get(&id)
    }

    pub fn get_surface_mut(&mut self, id: u64) -> Option<&mut Surface> {
        self.surfaces.get_mut(&id)
    }

    pub fn insert_surface(&mut self, id: u64, surface: Surface) {
        if !self.surfaces.contains_key(&id) {
            self.ordered_surfaces.push(id);
        }
        self.surfaces.insert(id, surface);
        self.sort_surfaces();
    }

    pub fn remove_surface(&mut self, id: u64) -> Option<Surface> {
        self.ordered_surfaces.retain(|&x| x != id);
        self.surfaces.remove(&id)
    }

    pub fn resort(&mut self) {
        self.sort_surfaces();
    }

    fn sort_surfaces(&mut self) {
        let surfaces = &self.surfaces;
        self.ordered_surfaces.sort_by(|a, b| {
            let z_a = surfaces.get(a).map(|s| s.z_index).unwrap_or(0);
            let z_b = surfaces.get(b).map(|s| s.z_index).unwrap_or(0);
            z_a.cmp(&z_b)
        });
    }

    /// Perform a top-down hit test to find the front-most visible surface under the cursor.
    pub fn hit_test(&self, x: i32, y: i32) -> Option<u64> {
        // Iterate top-to-bottom (reverse order)
        for id in self.ordered_surfaces.iter().rev() {
            if let Some(surf) = self.surfaces.get(id) {
                if surf.visible && surf.rect().contains(x, y) {
                    return Some(*id);
                }
            }
        }
        None
    }

    /// Iterates over all active damage rects attached to surfaces.
    pub fn collect_damage(&mut self) -> Vec<Rect> {
        let mut all_damage = Vec::new();
        for surf in self.surfaces.values_mut() {
            if !surf.visible {
                surf.damage.clear();
                continue;
            }
            if !surf.damage.is_empty() {
                // If the surface moved, we should probably also track the old rect
                // but for now we trust `window_manager` or callers to manually add scene damage
                // if a surface physically moved. Here we just gather internal buffer damage.
                all_damage.append(&mut surf.damage);
            }
        }
        all_damage
    }
}
