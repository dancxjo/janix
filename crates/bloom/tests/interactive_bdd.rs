use cucumber::{given, when, then, World};
use bloom::interaction::{InteractionController, InteractionEffect};
use bloom::ui::{WindowScene, WidgetKind};
use models::{Window, WindowStyle, Layout, LayoutKind};
use abi::ids::{ThingId, SymbolId};
use bloom::scene::Rect;

#[derive(Debug, World)]
pub struct InteractiveWorld {
    controller: InteractionController,
    scenes: Vec<WindowScene>,
    effects: Vec<InteractionEffect>,
}

impl Default for InteractiveWorld {
    fn default() -> Self {
        Self {
            controller: InteractionController::new(),
            scenes: Vec::new(),
            effects: Vec::new(),
        }
    }
}

// Helper to create a dummy window scene
fn create_dummy_scene(id_val: u64, x: i32, y: i32, w: u32, h: u32) -> WindowScene {
    let id = ThingId::from_parts(1, id_val); // Simple ID
    WindowScene {
        id,
        window: Window {
            title: SymbolId(0),
            x,
            y,
            width: w,
            height: h,
            z: 0,
            focused: false,
            min_width: 50,
            min_height: 50,
            style: WindowStyle {
                bg_rgba: 0xFFFFFFFF,
                radius: 4,
                shadow: 1,
                elevation: 0,
            },
            content_root: ThingId::from_parts(0, 0),
        },
        layout: Layout {
            kind: LayoutKind::Column,
            padding: 0,
            gap: 0,
            align: 0,
        },
        children: Vec::new(),
        title_text: "Test Window".into(),
    }
}

#[given(expr = "a window {int} at {int}, {int} with size {int}x{int}")]
fn given_window(world: &mut InteractiveWorld, id: u64, x: i32, y: i32, w: u32, h: u32) {
    world.scenes.push(create_dummy_scene(id, x, y, w, h));
}

#[when(expr = "I press left mouse at {int}, {int}")]
fn press_left_mouse(world: &mut InteractiveWorld, x: i32, y: i32) {
    let effects = world.controller.handle_pointer_down(x, y, &world.scenes);
    world.effects.extend(effects);
}

#[when(expr = "I move mouse to {int}, {int}")]
fn move_mouse(world: &mut InteractiveWorld, x: i32, y: i32) {
    let effects = world.controller.handle_pointer_move(x, y, &world.scenes);
    world.effects.extend(effects);
}

#[then(expr = "window {int} should receive update rect {int}, {int}, {int}, {int}")]
fn check_update_rect(world: &mut InteractiveWorld, win_id: u64, x: i32, y: i32, w: u32, h: u32) {
    let id = ThingId::from_parts(1, win_id);
    let mut found = false;
    for effect in &world.effects {
        if let InteractionEffect::UpdateWindowRect { id: eid, rect } = effect {
            if *eid == id {
                assert_eq!(rect.x, x, "Rect X mismatch");
                assert_eq!(rect.y, y, "Rect Y mismatch");
                // Only check W/H logic if update rect has them set (handle_pointer_move logic)
                // In my logic, I fetch current scene w/h if dragging, so it should be correct.
                // If resizing, it calculates new w/h.
                assert_eq!(rect.w, w, "Rect W mismatch");
                assert_eq!(rect.h, h, "Rect H mismatch");
                found = true;
            }
        }
    }
    assert!(found, "No UpdateWindowRect effect found for window {}", win_id);
}

#[tokio::main]
async fn main() {
    InteractiveWorld::run("tests/features/interactive_window.feature").await;
}
