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
use spin::Mutex;

#[derive(Clone, Copy)]
pub enum FullRefreshReason {
    FirstFrame,
    WatchOverflow,
    ResyncRequested,
    CacheInvalidated,
    AssetChange,
    BugFallback,
    WatchActivity,
}

impl FullRefreshReason {
    fn as_str(self) -> &'static str {
        match self {
            Self::FirstFrame => "FirstFrame",
            Self::WatchOverflow => "WatchOverflow",
            Self::ResyncRequested => "ResyncRequested",
            Self::CacheInvalidated => "CacheInvalidated",
            Self::AssetChange => "AssetChange",
            Self::BugFallback => "BugFallback",
            Self::WatchActivity => "WatchActivity",
        }
    }
}

struct UiInitState {
    initialized: bool,
    init_frame: u64,
    init_callsite: &'static str,
    keys: Option<UiKeys>,
    kinds: Option<KindIds>,
}

static UI_INIT: Mutex<UiInitState> = Mutex::new(UiInitState {
    initialized: false,
    init_frame: 0,
    init_callsite: "",
    keys: None,
    kinds: None,
});

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
    cached_drawlists: BTreeMap<ThingId, DrawList>,
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
    pending_full_reason: Option<FullRefreshReason>,
    // Temporary position overrides during drag operations
    drag_overrides: BTreeMap<ThingId, (i32, i32)>,
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

    pub fn total_len(&self) -> usize {
        self.props.union(&self.edges).count()
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
            cached_drawlists: BTreeMap::new(),
            solid_text: false,
            cached_keys: None,
            cached_kinds: None,
            asset_cache: AssetCache::new(),
            dirty_nodes: DirtySet::default(),
            last_layout: None,
            render_state: RenderState::new(),
            pending_full_reason: None,
            drag_overrides: BTreeMap::new(),
        }
    }

    /// Ensure keys and kinds are interned (does work only on first call)
    fn ensure_symbols(&mut self, caller: &'static str) -> (&UiKeys, &KindIds) {
        if self.cached_keys.is_none() {
            let mut init = UI_INIT.lock();
            if init.initialized {
                let frame_no = crate::perf::frame_no();
                let reason = "cached_keys_missing";
                crate::log!(
                    "[bloom][ui] INIT_REENTRY frame={} reason={} caller={} first_init_frame={} first_init_caller={}",
                    frame_no,
                    reason,
                    caller,
                    init.init_frame,
                    init.init_callsite
                );
                debug_assert!(false, "ui init ran more than once");
                self.cached_keys = init.keys.clone();
                self.cached_kinds = init.kinds.clone();
            } else {
                crate::log!("[bloom][ui] Initializing cached UI symbols (one-time)");
                let keys = UiKeys::intern();
                let kinds = KindIds::intern();
                init.initialized = true;
                init.init_frame = crate::perf::frame_no();
                init.init_callsite = caller;
                init.keys = Some(keys.clone());
                init.kinds = Some(kinds.clone());
                self.cached_keys = Some(keys);
                self.cached_kinds = Some(kinds);
            }
        }
        (
            self.cached_keys.as_ref().unwrap(),
            self.cached_kinds.as_ref().unwrap(),
        )
    }

    /// Fetch cached kind ids, ensuring they are interned once.
    pub fn kind_ids(&mut self) -> KindIds {
        let (_, kinds) = self.ensure_symbols("UiPipeline::kind_ids");
        kinds.clone()
    }

    /// Fetch cached UI key ids for watch setup.
    pub fn ui_keys(&mut self) -> UiKeys {
        let (keys, _) = self.ensure_symbols("UiPipeline::ui_keys");
        keys.clone()
    }

    pub fn set_root(&mut self, id: ThingId) {
        self.root_id = Some(id);
        self.dirty_full = true;
        self.dirty = true;
        self.dirty_nodes.clear();
        self.cached_drawlists.clear();
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Force a full snapshot rebuild on the next frame.
    pub fn mark_dirty_full(&mut self) {
        self.mark_dirty_full_with_reason(FullRefreshReason::ResyncRequested);
    }

    pub fn mark_dirty_full_with_reason(&mut self, reason: FullRefreshReason) {
        self.dirty_full = true;
        self.dirty = true;
        self.dirty_nodes.clear();
        self.cached_drawlists.clear();
        self.pending_full_reason = Some(reason);
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
        self.render_state.invalidate_svg_source(bs_id);
    }

    /// Clear the entire asset cache (when text bytespaces change)
    pub fn asset_cache_clear(&mut self) {
        self.asset_cache.clear();
    }

    /// Clear the raster cache (pre-rendered text/SVG) when content changes
    pub fn raster_cache_clear(&mut self) {
        self.render_state.clear_raster_cache();
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

        // Fast path: if nothing changed, reuse cached drawlists.
        if !self.dirty {
            if self.emit_cached_drawlists(list) {
                crate::trace_event!("ui.run.path", "fast_path_cached");
                if log_this_frame {
                    let (window_count, windows_hash) = self
                        .prev_snapshot
                        .as_ref()
                        .map(|s| (s.window_count(), s.windows_hash()))
                        .unwrap_or((0, 0));
                    let font_epoch = crate::font_graph::get_epoch();
                    crate::log!("[bloom][ui] ENTER_UI_BUILD dirty=false root_present=true windows_seen={} windows_hash={:#x} font_epoch={} reason=fast_path_cached", 
                        window_count, windows_hash, font_epoch);
                }
                return UiRunResult {
                    changed: false,
                    damage: alloc::vec::Vec::new(),
                    solid_text: self.solid_text,
                };
            }
        }

        // Ensure symbols are cached (no-op after first frame)
        let (keys, kinds) = self.ensure_symbols("UiPipeline::run");
        let keys = keys.clone();
        let kinds = kinds.clone();

        // 1. Snapshot with asset cache (Phase C) and optional incremental (Phase F)
        let had_prev = self.prev_snapshot.is_some();
        let mut snapshot = self.prev_snapshot.take().unwrap_or_else(UiSnapshot::new);
        let mut node_changes: alloc::vec::Vec<NodeChange> = alloc::vec::Vec::new();
        let mut snapshot_changed = false;
        let mut full_snapshot = false;
        let mut full_layout_reason: Option<FullRefreshReason> = None;
        let pending_dirty_count = self.dirty_nodes.total_len();
        let mut dirty_snap_count = 0usize;
        {
            crate::trace_span!("ui.snap");
            if !had_prev || self.dirty_full {
                crate::trace_event!("ui.run.path", "full_snap");
                if !had_prev {
                    full_layout_reason = Some(FullRefreshReason::FirstFrame);
                } else if self.dirty_full {
                    full_layout_reason =
                        Some(self.pending_full_reason.take().unwrap_or(FullRefreshReason::ResyncRequested));
                }
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
                        paint_dirty: true,
                    })
                    .collect();
                snapshot_changed = true;
                dirty_snap_count = snapshot.nodes.len();
            } else {
                let dirty = core::mem::take(&mut self.dirty_nodes);
                if !dirty.is_empty() {
                    crate::trace_event!("ui.run.path", "incremental_snap");
                    node_changes = snapshot.update_dirty(&dirty, &keys, &kinds, &mut self.asset_cache);
                    snapshot_changed = !node_changes.is_empty();
                    dirty_snap_count = pending_dirty_count;

                    // If graph topology changed, garbage collect unreachable nodes
                    // to prevent memory leaks.
                    if !dirty.edges.is_empty() {
                         snapshot.prune();
                    }
                } else {
                    crate::trace_event!("ui.run.path", "snap_reuse");
                }
            }
        }

        crate::trace_counter!("ui.nodes", snapshot.nodes.len());
        crate::trace_counter!("dirty_nodes_snap", dirty_snap_count);

        let changed = snapshot_changed || self.dirty;

        if self.dirty && !snapshot_changed && node_changes.is_empty() && !self.dirty_full {
            if self.emit_cached_drawlists(list) {
                self.dirty = false;
                self.prev_snapshot = Some(snapshot);
                return UiRunResult {
                    changed: false,
                    damage: alloc::vec::Vec::new(),
                    solid_text: self.solid_text,
                };
            }
        }

        crate::trace_event!("ui.run.path", "full_build");

        let prev_layout = self.last_layout.clone();

        // 3. Layout
        let mut force_full_layout = full_snapshot;
        if !force_full_layout {
            if let Some(layout) = self.last_layout.as_ref() {
                if let Some(root) = layout.root.as_ref() {
                    if root.rect.w != screen_w || root.rect.h != screen_h {
                        force_full_layout = true;
                        full_layout_reason = Some(FullRefreshReason::CacheInvalidated);
                    }
                } else {
                    force_full_layout = true;
                    full_layout_reason = Some(FullRefreshReason::BugFallback);
                }
            } else {
                force_full_layout = true;
                full_layout_reason = Some(FullRefreshReason::BugFallback);
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
                            full_layout_reason = Some(FullRefreshReason::BugFallback);
                            // breakdown: don't break, keep marking others
                        }
                    }
                }
            } else {
                force_full_layout = true;
                full_layout_reason = Some(FullRefreshReason::BugFallback);
            }
        }

        // Compute dirty subtrees (ancestors of dirty nodes need to know they contain dirt)
        // This allows us to skip recursion for clean subtrees.
        let mut subtree_layout_dirty = BTreeSet::new();
        if !force_full_layout {
            if let Some(root_id) = snapshot.root_id {
                 let parent_map = build_parent_map(&snapshot, root_id);
                 for &dirty_id in &layout_dirty_nodes {
                     let mut current = dirty_id;
                     subtree_layout_dirty.insert(current);
                     while let Some(parent) = parent_map.get(&current) {
                         if subtree_layout_dirty.contains(parent) {
                             break;
                         }
                         subtree_layout_dirty.insert(*parent);
                         current = *parent;
                     }
                 }
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
        if force_full_layout {
            if let Some(reason) = full_layout_reason {
                crate::trace_event!("ui.full_layout_reason", reason.as_str());
                crate::trace_counter!("ui.full_layout", 1);
                if log_this_frame {
                    crate::log!(
                        "[bloom][ui] full_layout_reason={}",
                        reason.as_str()
                    );
                }
            }
        }
        let layout_dirty_count = if force_full_layout {
            snapshot.nodes.len()
        } else {
            layout_dirty_nodes.len()
        };
        crate::trace_counter!("dirty_nodes_layout", layout_dirty_count);

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
                    &layout_dirty_nodes,
                    &subtree_layout_dirty,
                )
            }
        };
        self.last_layout = Some(layout.clone());

        let mut paint_dirty_nodes = BTreeSet::new();
        for change in &node_changes {
            if change.paint_dirty || change.layout_dirty || change.measure_dirty {
                paint_dirty_nodes.insert(change.id);
            }
        }

        let mut dirty_subtrees = BTreeSet::new();
        let mut paint_all = full_snapshot
            || force_full_layout
            || (self.dirty && node_changes.is_empty());
        if !paint_all && !paint_dirty_nodes.is_empty() {
            if let Some(root_id) = snapshot.root_id {
                if paint_dirty_nodes.contains(&root_id) {
                    paint_all = true;
                } else {
                    let parent_map = build_parent_map(&snapshot, root_id);
                    for id in &paint_dirty_nodes {
                        if let Some(subtree) =
                            find_root_child_ancestor(*id, root_id, &parent_map)
                        {
                            dirty_subtrees.insert(subtree);
                        }
                    }
                }
            } else {
                paint_all = true;
            }
        }

        let mut paint_node_count = 0usize;
        let mut paint_obj_count = 0usize;
        let mut paint_text_count = 0usize;
        let mut rebuilt_any = false;
        let active_window = PaintBuilder::active_window(&layout);

        if let Some(root) = layout.root.as_ref() {
            crate::trace_span!("ui.paint");
            for child in &root.children {
                let needs_rebuild = paint_all
                    || dirty_subtrees.contains(&child.id)
                    || !self.cached_drawlists.contains_key(&child.id);
                if needs_rebuild {
                    let mut node_count = 0usize;
                    let scene = PaintBuilder::build_subtree(
                        &snapshot,
                        child,
                        &resolver,
                        &mut self.render_state,
                        active_window,
                        &mut node_count,
                    );
                    paint_node_count = paint_node_count.saturating_add(node_count);
                    paint_obj_count = paint_obj_count.saturating_add(scene.objects.len());
                    paint_text_count = paint_text_count.saturating_add(
                        scene
                            .objects
                            .iter()
                            .filter(|o| matches!(o, PaintObject::Text { .. }))
                            .count(),
                    );
                    let mut subtree_list = DrawList::new();
                    Self::lower(&scene, &mut subtree_list);
                    self.cached_drawlists.insert(child.id, subtree_list);
                    rebuilt_any = true;
                }
            }
        }

        crate::trace_counter!("dirty_nodes_paint", paint_node_count);
        if rebuilt_any {
            let now_ms = crate::log_ratelimit::now_ms();
            if crate::log_ratelimit::log_every(1000, now_ms) {
                crate::log!(
                    "[bloom][paint] objs={} text={}",
                    paint_obj_count,
                    paint_text_count
                );
            }
        }

        {
            crate::trace_span!("ui.lower");
            if let Some(root) = layout.root.as_ref() {
                let mut active_subtrees = BTreeSet::new();
                for child in &root.children {
                    active_subtrees.insert(child.id);
                    if let Some(drawlist) = self.cached_drawlists.get(&child.id) {
                        drawlist.append_to(list);
                    }
                }
                self.cached_drawlists
                    .retain(|id, _| active_subtrees.contains(id));
            } else {
                self.cached_drawlists.clear();
            }
        }

        self.dirty = false;
        self.prev_snapshot = Some(snapshot);

        let mut damage = alloc::vec::Vec::new();
        let bounds = Rect::full(screen_w, screen_h);
        if paint_all && paint_dirty_nodes.is_empty() {
            damage.push(bounds);
        } else {
            for id in paint_dirty_nodes {
                if let Some(prev) = prev_layout.as_ref().and_then(|l| l.find_rect(id)) {
                    let clipped = prev.clip(bounds);
                    if !clipped.is_empty() {
                        damage.push(clipped);
                    }
                }
                if let Some(next) = layout.find_rect(id) {
                    let clipped = next.clip(bounds);
                    if !clipped.is_empty() {
                        damage.push(clipped);
                    }
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

    pub fn get_cached_drawlist(&self, id: ThingId) -> Option<&DrawList> {
        self.cached_drawlists.get(&id)
    }

    fn emit_cached_drawlists(&self, list: &mut DrawList) -> bool {
        let root = match self.last_layout.as_ref().and_then(|layout| layout.root.as_ref()) {
            Some(root) => root,
            None => return false,
        };
        for child in &root.children {
            let drawlist = match self.cached_drawlists.get(&child.id) {
                Some(drawlist) => drawlist,
                None => return false,
            };
            drawlist.append_to(list);
        }
        true
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

    /// Get all windows for hit testing, in z-order (front to back).
    /// 
    /// Returns (id, rect, is_shaded, is_maximized) for each window.
    pub fn get_windows_for_hit_test(&self) -> alloc::vec::Vec<(ThingId, Rect, bool, bool)> {
        let layout = match self.last_layout.as_ref() {
            Some(l) => l,
            None => return alloc::vec::Vec::new(),
        };
        let root = match layout.root.as_ref() {
            Some(r) => r,
            None => return alloc::vec::Vec::new(),
        };
        let snapshot = match self.prev_snapshot.as_ref() {
            Some(s) => s,
            None => return alloc::vec::Vec::new(),
        };

        let mut windows: alloc::vec::Vec<(ThingId, Rect, bool, bool, i32)> = alloc::vec::Vec::new();
        Self::collect_windows(root, snapshot, &self.cached_keys, &mut windows);

        // Sort by z-index descending (front to back)
        windows.sort_by(|a, b| b.4.cmp(&a.4));

        windows.into_iter().map(|(id, rect, shaded, maximized, _z)| (id, rect, shaded, maximized)).collect()
    }

    fn collect_windows(
        node: &layout::LayoutNode,
        snapshot: &UiSnapshot,
        keys: &Option<UiKeys>,
        out: &mut alloc::vec::Vec<(ThingId, Rect, bool, bool, i32)>,
    ) {
        if node.kind == snapshot::UiNodeKind::Window {
            let is_shaded = if let (Some(k), Some(n)) = (keys, snapshot.nodes.get(&node.id)) {
                if k.window_shaded != 0 {
                    n.props.get(&k.window_shaded).copied().unwrap_or(0) != 0
                } else {
                    false
                }
            } else {
                false
            };
            // Check maximized state (not yet persisted in graph, so always false for now)
            let is_maximized = false;
            out.push((node.id, node.rect, is_shaded, is_maximized, node.z_index));
        }
        for child in &node.children {
            Self::collect_windows(child, snapshot, keys, out);
        }
    }

    /// Set a window's position and size in the graph.
    /// 
    /// This updates the UI_X, UI_Y, UI_WIDTH, UI_HEIGHT properties.
    pub fn set_window_rect(&mut self, window_id: ThingId, rect: Rect) -> bool {
        let keys = match self.cached_keys.as_ref() {
            Some(k) => k,
            None => return false,
        };

        let mut success = true;
        if keys.x != 0 {
            success &= stem::thing::sys::prop_set(window_id, keys.x, rect.x as u64).is_ok();
        }
        if keys.y != 0 {
            success &= stem::thing::sys::prop_set(window_id, keys.y, rect.y as u64).is_ok();
        }
        if keys.w != 0 {
            success &= stem::thing::sys::prop_set(window_id, keys.w, rect.w as u64).is_ok();
        }
        if keys.h != 0 {
            // Subtract title bar height for stored value
            let stored_h = (rect.h - TITLE_BAR_HEIGHT).max(0) as u64;
            success &= stem::thing::sys::prop_set(window_id, keys.h, stored_h).is_ok();
        }

        if success {
            self.mark_node_dirty(window_id);
        }
        success
    }

    /// Get the rect for a window from the layout tree.
    pub fn get_window_rect(&self, window_id: ThingId) -> Option<Rect> {
        self.last_layout.as_ref()?.find_rect(window_id)
    }

    /// Raise a window to the front by updating its z-index.
    pub fn raise_window(&mut self, window_id: ThingId) -> bool {
        let keys = match self.cached_keys.as_ref() {
            Some(k) => k,
            None => return false,
        };

        if keys.z_index == 0 {
            return false;
        }

        // Find the max z-index of all windows
        let windows = self.get_windows_for_hit_test();
        let max_z = windows.iter().filter(|(id, _, _, _)| *id != window_id).map(|(_, _, _, _)| {
            // Read z-index from snapshot
            if let Some(snapshot) = self.prev_snapshot.as_ref() {
                if let Some(node) = snapshot.nodes.get(&window_id) {
                    return node.props.get(&keys.z_index).copied().unwrap_or(0) as i32;
                }
            }
            0
        }).max().unwrap_or(0);

        let new_z = (max_z + 1) as u64;
        if stem::thing::sys::prop_set(window_id, keys.z_index, new_z).is_ok() {
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
                    list.text_font_debug(text, Some(font), rect.x, rect.y, *size, *color, *font_debug);
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

fn find_root_child_ancestor(
    id: ThingId,
    root_id: ThingId,
    parents: &BTreeMap<ThingId, ThingId>,
) -> Option<ThingId> {
    if id == root_id {
        return Some(root_id);
    }
    let mut current = id;
    while let Some(parent) = parents.get(&current) {
        if *parent == root_id {
            return Some(current);
        }
        current = *parent;
    }
    None
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
