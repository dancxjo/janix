pub mod snapshot;
pub mod layout;
pub mod paint;

use stem::thing::ThingId;
use crate::drawlist::DrawList;
use self::snapshot::UiSnapshot;
use self::layout::{LayoutSolver, SymbolResolver};
use self::paint::{PaintBuilder, PaintScene, PaintObject};
use crate::asset::AssetBank;

struct SystemSymbolResolver;

impl SymbolResolver for SystemSymbolResolver {
    fn resolve(&self, key: &str) -> Option<u32> {
        stem::thing::sys::intern(key).ok()
    }
}

pub struct UiPipeline {
    pub root_id: Option<ThingId>,
    pub prev_snapshot: Option<UiSnapshot>,
}

impl UiPipeline {
    pub fn new() -> Self {
        Self { 
            root_id: None,
            prev_snapshot: None,
        }
    }

    pub fn set_root(&mut self, id: ThingId) {
        self.root_id = Some(id);
    }

    pub fn run(&mut self, screen_w: i32, screen_h: i32, list: &mut DrawList, assets: &AssetBank) {
        let root_id = match self.root_id {
            Some(id) => id,
            None => return,
        };

        // 1. Snapshot
        let snapshot = UiSnapshot::capture(root_id);
        
        // v0 Damage Tracking: Detect change
        let changed = self.prev_snapshot.as_ref().map(|s| s != &snapshot).unwrap_or(true);
        if changed {
            // In a more advanced version, we'd only repaint dirty regions.
            // For v0, we just note it.
        }
        self.prev_snapshot = Some(snapshot.clone());

        // 2. Layout
        let resolver = SystemSymbolResolver;
        let layout = LayoutSolver::solve(&snapshot, screen_w, screen_h, assets, &resolver);

        // 3. Paint
        let paint_scene = PaintBuilder::build(&snapshot, &layout, &resolver);

        // 4. Lowering
        Self::lower(&paint_scene, list);
    }

    fn lower(scene: &PaintScene, list: &mut DrawList) {
        for obj in &scene.objects {
            match obj {
                PaintObject::Rect { rect, color, radius } => {
                    if *radius > 0 {
                        list.rounded_rect(rect.x, rect.y, rect.w, rect.h, *radius as i32, *color, crate::geometry::EdgeAA::None);
                    } else {
                        list.rect(rect.x, rect.y, rect.w, rect.h, *color);
                    }
                }
                PaintObject::Text { rect, text, font, size, color } => {
                    list.text_font(text, font, rect.x, rect.y, *size, *color);
                }
                PaintObject::Image { rect: _ } => {
                    // TODO: Implement image lowering
                }
            }
        }
    }
}
