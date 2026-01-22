pub mod constants;
pub mod layout;
pub mod paint;
pub mod snapshot;

use self::layout::{LayoutSolver, SymbolResolver};
use self::paint::{PaintBuilder, PaintObject, PaintScene};
use self::snapshot::{AssetCache, KindIds, NodeChange, UiKeys, UiSnapshot};
use crate::asset::AssetBank;
use crate::damage::Rect;
use crate::drawlist::DrawList;
use crate::ui::constants::{SHADE_BUTTON_PADDING, SHADE_BUTTON_SIZE, TITLE_BAR_HEIGHT};
use crate::render_state::RenderState;
use alloc::collections::{BTreeMap, BTreeSet};
use stem::thing::ThingId;

struct SystemSymbolResolver;

impl SymbolResolver for SystemSymbolResolver {
    fn resolve(&self, key: &str) -> Option<u32> {
        stem::thing::sys::intern(key).ok()
    }
}

pub struct UiPipeline {
    root_id: Option<ThingId>,
    prev_snapshot: Option<UiSnapshot>,
    solver: LayoutSolver,
    dirty: bool,
    dirty_full: bool,
    cached_scene: Option<PaintScene>,
    pub solid_text: bool,
    // Cached symbol IDs - initialized once, used every frame
    cached_keys: Option<UiKeys>,
    cached_kinds: Option<KindIds>,
    // Asset cache - persists across frames (Phase C)
    asset_cache: AssetCache,
    // Dirty node tracking for incremental updates (Phase F)
    pub dirty_nodes: DirtySet,
    // Layout from the last run, used for hit-testing
    pub last_layout: Option<layout::LayoutTree>,
    pub render_state: RenderState,
}

pub struct UiRunResult {
    pub changed: bool,
    pub damage: alloc::vec::Vec<Rect>,
    pub solid_text: bool,
}

#[derive(Default, Clone)]
pub struct DirtySet {
    props: BTreeSet<ThingId>,
    edges: BTreeSet<ThingId>,
}

impl DirtySet {
    pub fn is_empty(&self) -> bool {
        self.props.is_empty() && self.edges.is_empty()
    }

    pub fn mark_prop(&mut self, id: ThingId) {
        self.props.insert(id);
    }

    pub fn mark_edge(&mut self, id: ThingId) {
        self.edges.insert(id);
    }

    pub fn clear(&mut self) {
        self.props.clear();
        self.edges.clear();
    }

    pub fn props(&self) -> impl Iterator<Item = &ThingId> {
        self.props.iter()
    }

    pub fn edges(&self) -> impl Iterator<Item = &ThingId> {
        self.edges.iter()
    }
}

impl UiPipeline {
    pub fn new() -> Self {
        Self {
            root_id: None,
            prev_snapshot: None,
            solver: LayoutSolver::new(),
            dirty: true,
            dirty_full: false,
            cached_scene: None,
            solid_text: false,
            cached_keys: None,
            cached_kinds: None,
            asset_cache: AssetCache::new(),
            dirty_nodes: DirtySet::default(),
            last_layout: None,
            render_state: RenderState::new(),
        }
    }

    /// Ensure keys and kinds are interned (does work only on first call)
    fn ensure_symbols(&mut self) -> (&UiKeys, &KindIds) {
        if self.cached_keys.is_none() {
            crate::log!("[bloom][ui] Initializing cached UI symbols (one-time)");
            self.cached_keys = Some(UiKeys::intern());
            self.cached_kinds = Some(KindIds::intern());
        }
        (
            self.cached_keys.as_ref().unwrap(),
            self.cached_kinds.as_ref().unwrap(),
        )
    }

    /// Fetch cached kind ids, ensuring they are interned once.
    pub fn kind_ids(&mut self) -> KindIds {
        let (_, kinds) = self.ensure_symbols();
        kinds.clone()
    }

    /// Fetch cached UI key ids for watch setup.
    pub fn ui_keys(&mut self) -> UiKeys {
        let (keys, _) = self.ensure_symbols();
        keys.clone()
    }

    pub fn set_root(&mut self, id: ThingId) {
        self.root_id = Some(id);
        self.dirty_full = true;
        self.dirty = true;
        self.dirty_nodes.clear();
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Force a full snapshot rebuild on the next frame.
    pub fn mark_dirty_full(&mut self) {
        self.dirty_full = true;
        self.dirty = true;
        self.dirty_nodes.clear();
    }

    /// Mark a specific node as dirty (for incremental updates from watch events)
    pub fn mark_node_dirty(&mut self, id: ThingId) {
        self.dirty_nodes.mark_prop(id);
        self.dirty = true;
    }

    /// Mark a node's edges as dirty (structure change).
    pub fn mark_node_edges_dirty(&mut self, id: ThingId) {
        self.dirty_nodes.mark_edge(id);
        self.dirty = true;
    }

    /// Invalidate an asset in the cache (when we know it changed)
    pub fn invalidate_asset(&mut self, bs_id: u64) {
        self.asset_cache.invalidate(bs_id);
    }

    pub fn run(
        &mut self,
        screen_w: i32,
        screen_h: i32,
        list: &mut DrawList,
        assets: &AssetBank,
    ) -> UiRunResult {
        let now_ms = crate::log_ratelimit::now_ms();
        let log_this_frame = crate::log_ratelimit::log_every(1000, now_ms);

        let root_id = match self.root_id {
            Some(id) => id,
            None => {
                self.last_layout = None;
                if log_this_frame {
                    crate::log!("[bloom][ui] ENTER_UI_BUILD dirty={} root_present=false windows_seen=0 reason=no_root_id", self.dirty);
                }
                return UiRunResult {
                    changed: false,
                    damage: alloc::vec::Vec::new(),
                    solid_text: self.solid_text,
                };
            }
        };

        // Fast path: if nothing changed, reuse the last paint scene and only lower.
        if !self.dirty {
            if let Some(scene) = &self.cached_scene {
                crate::trace_event!("ui.run.path", "fast_path_cached");
                if log_this_frame {
                    let window_count = self
                        .prev_snapshot
                        .as_ref()
                        .map(|s| {
                            s.nodes
                                .values()
                                .filter(|n| matches!(n.kind, snapshot::UiNodeKind::Window))
                                .count()
                        })
                        .unwrap_or(0);
                    crate::log!("[bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen={} reason=fast_path_cached", window_count);
                }
                Self::lower(scene, list);
                return UiRunResult {
                    changed: false,
                    damage: alloc::vec::Vec::new(),
                    solid_text: self.solid_text,
                };
            }
        }

        // Ensure symbols are cached (no-op after first frame)
        let (keys, kinds) = self.ensure_symbols();
        let keys = keys.clone();
        let kinds = kinds.clone();

        // 1. Snapshot with asset cache (Phase C) and optional incremental (Phase F)
        let had_prev = self.prev_snapshot.is_some();
        let mut snapshot = self.prev_snapshot.take().unwrap_or_else(UiSnapshot::new);
        let mut node_changes: alloc::vec::Vec<NodeChange> = alloc::vec::Vec::new();
        let mut snapshot_changed = false;
        let mut full_snapshot = false;
        {
            crate::trace_span!("ui.snap");
            if !had_prev || self.dirty_full {
                crate::trace_event!("ui.run.path", "full_snap");
                self.dirty_full = false;
                self.dirty_nodes.clear();
                full_snapshot = true;
                snapshot = UiSnapshot::capture_with_cache(root_id, &keys, &kinds, &mut self.asset_cache);
                node_changes = snapshot
                    .nodes
                    .keys()
                    .map(|id| NodeChange {
                        id: *id,
                        layout_dirty: true,
                        measure_dirty: true,
                    })
                    .collect();
                snapshot_changed = true;
            } else {
                let dirty = core::mem::take(&mut self.dirty_nodes);
                if !dirty.is_empty() {
                    crate::trace_event!("ui.run.path", "incremental_snap");
                    node_changes = snapshot.update_dirty(&dirty, &keys, &kinds, &mut self.asset_cache);
                    snapshot_changed = !node_changes.is_empty();
                } else {
                    crate::trace_event!("ui.run.path", "snap_reuse");
                }
            }
        }

        crate::trace_counter!("ui.nodes", snapshot.nodes.len());

        let changed = snapshot_changed || self.dirty;

        crate::trace_event!("ui.run.path", "full_build");

        // 3. Layout
        let changed_nodes: alloc::vec::Vec<ThingId> =
            node_changes.iter().map(|c| c.id).collect();

        let mut force_full_layout = full_snapshot;
        if !force_full_layout {
            if let Some(layout) = self.last_layout.as_ref() {
                if let Some(root) = layout.root.as_ref() {
                    if root.rect.w != screen_w || root.rect.h != screen_h {
                        force_full_layout = true;
                    }
                } else {
                    force_full_layout = true;
                }
            } else {
                force_full_layout = true;
            }
        }

        let mut dirty_windows = BTreeSet::new();
        let mut layout_dirty_nodes = BTreeSet::new();
        if !force_full_layout && !node_changes.is_empty() {
            if let Some(root_id) = snapshot.root_id {
                let parent_map = build_parent_map(&snapshot, root_id);
                for change in &node_changes {
                    let node = match snapshot.nodes.get(&change.id) {
                        Some(node) => node,
                        None => continue,
                    };
                    let mut layout_dirty = change.layout_dirty;
                    if !layout_dirty
                        && change.measure_dirty
                        && node_uses_intrinsic_size(node, &keys)
                    {
                        layout_dirty = true;
                    }
                    if layout_dirty {
                        layout_dirty_nodes.insert(change.id);
                        if let Some(window_id) =
                            find_window_ancestor(change.id, &snapshot, &parent_map)
                        {
                            dirty_windows.insert(window_id);
                        } else {
                            force_full_layout = true;
                            break;
                        }
                    }
                }
            } else {
                force_full_layout = true;
            }
        }

        if log_this_frame {
            crate::log!(
                "[bloom][ui] dirty_nodes_layout={} dirty_windows={} full_layout={}",
                layout_dirty_nodes.len(),
                dirty_windows.len(),
                force_full_layout
            );
        }

        let resolver = SystemSymbolResolver;
        let layout = {
            crate::trace_span!("ui.layout");
            if force_full_layout {
                self.solver
                    .solve(&snapshot, screen_w, screen_h, assets, &resolver)
            } else {
                self.solver.solve_partial(
                    &snapshot,
                    screen_w,
                    screen_h,
                    assets,
                    &resolver,
                    self.last_layout.as_ref(),
                    &dirty_windows,
                )
            }
        };
        self.last_layout = Some(layout.clone());

        // 4. Paint
        let paint_scene = {
            crate::trace_span!("ui.paint");
            PaintBuilder::build(&snapshot, &layout, &resolver, &mut self.render_state)
        };

        // 5. Lowering
        {
            crate::trace_span!("ui.lower");
            Self::lower(&paint_scene, list);
        }

        // Cache the scene so unchanged frames can skip snapshot/layout/paint.
        self.cached_scene = Some(paint_scene);
        self.dirty = false;
        self.prev_snapshot = Some(snapshot);

        let mut damage = alloc::vec::Vec::new();
        for id in changed_nodes {
            if let Some(rect) = layout.find_rect(id) {
                if !rect.is_empty() {
                    damage.push(rect);
                }
            }
        }

        UiRunResult {
            changed,
            damage,
            solid_text: self.solid_text,
        }
    }

    /// Hit-test the shade button for the topmost window under the given point.
    pub fn hit_test_shade_button(&self, x: i32, y: i32) -> Option<ThingId> {
        let layout = self.last_layout.as_ref()?;
        let root = layout.root.as_ref()?;
        let mut best: Option<(ThingId, i32)> = None;
        Self::hit_window_shade(root, x, y, &mut best);
        best.map(|(id, _)| id)
    }

    fn hit_window_shade(
        node: &layout::LayoutNode,
        x: i32,
        y: i32,
        best: &mut Option<(ThingId, i32)>,
    ) {
        if node.kind == snapshot::UiNodeKind::Window {
            let btn_x = node.rect.x + node.rect.w - SHADE_BUTTON_PADDING - SHADE_BUTTON_SIZE;
            let btn_y = node.rect.y + (TITLE_BAR_HEIGHT - SHADE_BUTTON_SIZE) / 2;
            let inside = x >= btn_x
                && x <= btn_x + SHADE_BUTTON_SIZE
                && y >= btn_y
                && y <= btn_y + SHADE_BUTTON_SIZE;
            if inside {
                if best.map(|(_, z)| node.z_index > z).unwrap_or(true) {
                    *best = Some((node.id, node.z_index));
                }
            }
        }

        for child in &node.children {
            Self::hit_window_shade(child, x, y, best);
        }
    }

    /// Toggle the shaded state for a window and mark it dirty.
    pub fn toggle_window_shade(&mut self, window_id: ThingId) -> bool {
        let key_id = match self.cached_keys.as_ref().map(|k| k.window_shaded) {
            Some(k) if k != 0 => k,
            _ => return false,
        };

        let current = stem::thing::sys::prop_get(window_id, key_id).unwrap_or(0);
        let next = if current == 0 { 1 } else { 0 };

        if stem::thing::sys::prop_set(window_id, key_id, next).is_ok() {
            self.mark_node_dirty(window_id);
            true
        } else {
            false
        }
    }

    fn lower(scene: &PaintScene, list: &mut DrawList) {
        use crate::drawlist::DrawCmd;
        for obj in &scene.objects {
            match obj {
                PaintObject::PushClip { rect } => {
                    list.commands().push(DrawCmd::PushClip {
                        rect: crate::geometry::Rect::new(rect.x, rect.y, rect.w, rect.h),
                    });
                }
                PaintObject::PopClip => {
                    list.commands().push(DrawCmd::PopClip);
                }
                PaintObject::Rect {
                    rect,
                    color,
                    radius,
                } => {
                    if *radius > 0 {
                        list.rounded_rect(
                            rect.x,
                            rect.y,
                            rect.w,
                            rect.h,
                            *radius as i32,
                            *color,
                            crate::geometry::EdgeAA::None,
                        );
                    } else {
                        list.rect(rect.x, rect.y, rect.w, rect.h, *color);
                    }
                }
                PaintObject::Text {
                    rect,
                    text,
                    font,
                    size,
                    color,
                    font_debug,
                } => {
                    list.text_font_debug(text, font, rect.x, rect.y, *size, *color, *font_debug);
                }
                PaintObject::Image { rect: _ } => {
                    // TODO: Implement image lowering
                }
                PaintObject::Raster { rect, image } => {
                     list.blit_image(image, rect.x, rect.y);
                }
                PaintObject::Commands { cmds, rect } => {
                    use crate::geometry::Transform;

                    // Translate local 0,0 SVG to node position
                    list.commands().push(DrawCmd::PushTransform {
                        transform: Transform::translate(rect.x as f32, rect.y as f32),
                    });

                    // Append commands
                    // Clone is cheap for Arc<Path> but we are cloning cmds into list
                    // Since cmds is Arc<Vec<DrawCmd>>, we iterate and clone each cmd?
                    // DrawCmd contains Arc<Path>.
                    // DrawCmd is small enum.
                    list.commands().extend(cmds.iter().cloned());

                    list.commands().push(DrawCmd::PopTransform);
                }
            }
        }
    }
}

fn build_parent_map(snapshot: &UiSnapshot, root_id: ThingId) -> BTreeMap<ThingId, ThingId> {
    let mut parents = BTreeMap::new();
    let mut stack = alloc::vec![root_id];
    while let Some(current) = stack.pop() {
        if let Some(node) = snapshot.nodes.get(&current) {
            for child in &node.children {
                parents.insert(*child, current);
                stack.push(*child);
            }
        }
    }
    parents
}

fn find_window_ancestor(
    mut id: ThingId,
    snapshot: &UiSnapshot,
    parents: &BTreeMap<ThingId, ThingId>,
) -> Option<ThingId> {
    loop {
        if let Some(node) = snapshot.nodes.get(&id) {
            if node.kind == snapshot::UiNodeKind::Window {
                return Some(id);
            }
        }
        match parents.get(&id) {
            Some(parent) => id = *parent,
            None => return None,
        }
    }
}

fn node_uses_intrinsic_size(node: &snapshot::UiNodeSnapshot, keys: &UiKeys) -> bool {
    let w = keys.w;
    let h = keys.h;
    let center_x = keys.center_x;
    let center_y = keys.center_y;
    let text_key = keys.text;

    let w = if w != 0 {
        *node.props.get(&w).unwrap_or(&0)
    } else {
        0
    };
    let h = if h != 0 {
        *node.props.get(&h).unwrap_or(&0)
    } else {
        0
    };

    let has_center_x = center_x != 0 && *node.props.get(&center_x).unwrap_or(&0) != 0;
    let has_center_y = center_y != 0 && *node.props.get(&center_y).unwrap_or(&0) != 0;

    let has_text = text_key != 0 && node.strings.contains_key(&text_key);

    (has_center_x || has_center_y) && (w == 0 || h == 0) && has_text
}
