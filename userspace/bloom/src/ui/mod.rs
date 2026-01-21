pub mod layout;
pub mod paint;
pub mod snapshot;

use self::layout::{LayoutSolver, SymbolResolver};
use self::paint::{PaintBuilder, PaintObject, PaintScene};
use self::snapshot::{UiSnapshot, UiKeys, KindIds};
use crate::asset::AssetBank;
use crate::damage::Rect;
use crate::drawlist::DrawList;
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
    cached_scene: Option<PaintScene>,
    pub solid_text: bool,
    // Cached symbol IDs - initialized once, used every frame
    cached_keys: Option<UiKeys>,
    cached_kinds: Option<KindIds>,
}

pub struct UiRunResult {
    pub changed: bool,
    pub damage: alloc::vec::Vec<Rect>,
    pub solid_text: bool,
}

impl UiPipeline {
    pub fn new() -> Self {
        Self {
            root_id: None,
            prev_snapshot: None,
            solver: LayoutSolver::new(),
            dirty: true,
            cached_scene: None,
            solid_text: false,
            cached_keys: None,
            cached_kinds: None,
        }
    }

    /// Ensure keys and kinds are interned (does work only on first call)
    fn ensure_symbols(&mut self) -> (&UiKeys, &KindIds) {
        if self.cached_keys.is_none() {
            crate::log!("[bloom][ui] Initializing cached UI symbols (one-time)");
            self.cached_keys = Some(UiKeys::intern());
            self.cached_kinds = Some(KindIds::intern());
        }
        (self.cached_keys.as_ref().unwrap(), self.cached_kinds.as_ref().unwrap())
    }

    pub fn set_root(&mut self, id: ThingId) {
        self.root_id = Some(id);
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
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
                if log_this_frame {
                    crate::log!("[bloom][ui] ENTER_UI_BUILD dirty={} root_present=false windows_seen=0 reason=no_root_id", self.dirty);
                }
                return UiRunResult {
                    changed: false,
                    damage: alloc::vec::Vec::new(),
                    solid_text: self.solid_text,
                }
            }
        };

        // Fast path: if nothing changed, reuse the last paint scene and only lower.
        if !self.dirty {
            if let Some(scene) = &self.cached_scene {
                crate::trace_event!("ui.run.path", "fast_path_cached");
                if log_this_frame {
                    // Count windows in prev_snapshot if available
                    let window_count = self.prev_snapshot.as_ref().map(|s| {
                        s.nodes.values().filter(|n| matches!(n.kind, snapshot::UiNodeKind::Window)).count()
                    }).unwrap_or(0);
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

        let start = stem::monotonic_ns();

        // Ensure symbols are cached (no-op after first frame)
        let (keys, kinds) = self.ensure_symbols();
        // Clone refs to avoid borrow issues
        let keys = keys.clone();
        let kinds = kinds.clone();

        // 1. Snapshot - now uses cached keys/kinds, no per-frame interning!
        let snapshot = {
            crate::trace_span!("ui.snap");
            UiSnapshot::capture(root_id, &keys, &kinds)
        };

        crate::trace_counter!("ui.nodes", snapshot.nodes.len());

        // 2. Change Detection
        let changed_nodes = {
            crate::trace_span!("ui.diff");
            if let Some(prev) = &self.prev_snapshot {
                snapshot.diff(prev)
            } else {
                snapshot.nodes.keys().cloned().collect()
            }
        };
        let changed = !changed_nodes.is_empty();

        // Store for next frame
        {
            crate::trace_span!("ui.clone");
            self.prev_snapshot = Some(snapshot.clone());
        }

        if !changed {
            crate::trace_event!("ui.run.path", "no_changes");
            if let Some(scene) = &self.cached_scene {
                crate::trace_span!("ui.lower");
                Self::lower(scene, list);
            }
            self.dirty = false;
            return UiRunResult {
                changed: false,
                damage: alloc::vec::Vec::new(),
                solid_text: self.solid_text,
            };
        }

        crate::trace_event!("ui.run.path", "full_build");

        // 3. Layout
        let resolver = SystemSymbolResolver;
        let layout = {
            crate::trace_span!("ui.layout");
            self.solver
                .solve(&snapshot, screen_w, screen_h, assets, &resolver)
        };

        // 4. Paint
        let paint_scene = {
            crate::trace_span!("ui.paint");
            PaintBuilder::build(&snapshot, &layout, &resolver)
        };

        // 5. Lowering
        {
            crate::trace_span!("ui.lower");
            Self::lower(&paint_scene, list);
        }

        // Cache the scene so unchanged frames can skip snapshot/layout/paint.
        self.cached_scene = Some(paint_scene);
        self.dirty = false;

        let mut damage = alloc::vec::Vec::new();
        for id in changed_nodes {
            if let Some(rect) = layout.find_rect(id) {
                if !rect.is_empty() {
                    damage.push(rect);
                }
            }
        }

        UiRunResult { changed, damage, solid_text: self.solid_text }
    }

    fn lower(scene: &PaintScene, list: &mut DrawList) {
        for obj in &scene.objects {
            match obj {
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
            }
        }
    }
}
