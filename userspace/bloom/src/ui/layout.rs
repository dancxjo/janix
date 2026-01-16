use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use stem::thing::ThingId;
use crate::damage::Rect;
use crate::ui::snapshot::{UiSnapshot, UiNodeSnapshot, UiNodeKind};
use abi::schema::keys;

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: ThingId,
    pub rect: Rect,
    pub z_index: i32,
    pub children: Vec<LayoutNode>,
}

pub struct LayoutTree {
    pub root: Option<LayoutNode>,
}

pub struct LayoutSolver;

impl LayoutSolver {
    pub fn solve(snapshot: &UiSnapshot, screen_w: i32, screen_h: i32) -> LayoutTree {
        let root_id = match snapshot.root_id {
            Some(id) => id,
            None => return LayoutTree { root: None },
        };

        let root_node = match snapshot.nodes.get(&root_id) {
            Some(n) => n,
            None => return LayoutTree { root: None },
        };

        // For v0, we assume the root covers the whole screen or handles its own layout.
        let mut root_layout = LayoutNode {
            id: root_id,
            rect: Rect::new(0, 0, screen_w, screen_h),
            z_index: 0,
            children: Vec::new(),
        };

        Self::layout_children(snapshot, root_node, &mut root_layout);

        LayoutTree { root: Some(root_layout) }
    }

    fn layout_children(snapshot: &UiSnapshot, node: &UiNodeSnapshot, layout: &mut LayoutNode) {
        for child_id in &node.children {
            if let Some(child_node) = snapshot.nodes.get(child_id) {
                // Determine layout strategy for this node.
                // For v0: Absolute positioning based on ui.x, ui.y, ui.width, ui.height
                
                let x = Self::get_prop(child_node, keys::UI_X) as i32;
                let y = Self::get_prop(child_node, keys::UI_Y) as i32;
                let w = Self::get_prop(child_node, keys::UI_WIDTH) as i32;
                let h = Self::get_prop(child_node, keys::UI_HEIGHT) as i32;
                let z = Self::get_prop(child_node, keys::UI_Z_INDEX) as i32;

                let mut child_layout = LayoutNode {
                    id: *child_id,
                    rect: Rect::new(x, y, w, h),
                    z_index: z,
                    children: Vec::new(),
                };

                Self::layout_children(snapshot, child_node, &mut child_layout);
                layout.children.push(child_layout);
            }
        }

        // Sort children by Z-index
        layout.children.sort_by_key(|n| n.z_index);
    }

    fn get_prop(node: &UiNodeSnapshot, key: &str) -> u64 {
        // We need to intern the key to get the ID for lookup
        if let Ok(id) = stem::thing::sys::intern(key) {
            return *node.props.get(&id).unwrap_or(&0);
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::snapshot::{UiNodeSnapshot, UiNodeKind};
    use alloc::collections::BTreeMap;
    use alloc::vec;
    use abi::schema::keys;

    #[test]
    fn test_layout_absolute_positioning() {
        let mut snapshot = UiSnapshot::new();
        let root_id = ThingId(1);
        snapshot.root_id = Some(root_id);
        
        let mut props = BTreeMap::new();
        // We'll use IDs that would be interned.
        // In a real test we'd need a mock interner.
        // For now this is just a skeleton.
        
        snapshot.nodes.insert(root_id, UiNodeSnapshot {
            id: root_id,
            kind: UiNodeKind::Root,
            props,
            children: vec![],
        });

        let tree = LayoutSolver::solve(&snapshot, 800, 600);
        assert!(tree.root.is_some());
        let root_node = tree.root.unwrap();
        assert_eq!(root_node.id, root_id);
    }

    #[test]
    fn test_z_index_sorting() {
        // TODO: Implement Z-index sorting test
    }
}
