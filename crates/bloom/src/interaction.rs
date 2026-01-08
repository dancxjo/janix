use abi::ids::ThingId;
use abi::ui::{ResizeEdge, WindowActionKind};
use crate::scene::Rect;
use crate::ui::hittest::{hit_test_scene, HitTarget};
use crate::ui::WindowScene;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub enum Interaction {
    Idle,
    DragMove {
        window: ThingId,
        grab_dx: i32,
        grab_dy: i32,
    },
    DragResize {
        window: ThingId,
        edge: ResizeEdge,
        start_rect: Rect,
        start_px: i32,
        start_py: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InteractionEffect {
    SetFocus(Option<ThingId>),
    Raise(ThingId),
    Close(ThingId),
    UpdateWindowRect { id: ThingId, rect: Rect },
    EmitAction { 
        window: ThingId, 
        kind: WindowActionKind, 
        start_x: i32, 
        start_y: i32, 
        dx: i32, 
        dy: i32, 
        edges: ResizeEdge 
    },
}

#[derive(Debug)]
pub struct InteractionController {
    pub interaction: Interaction,
    pub focused_window: Option<ThingId>,
}

impl InteractionController {
    pub fn new() -> Self {
        Self {
            interaction: Interaction::Idle,
            focused_window: None,
        }
    }

    pub fn handle_pointer_down(&mut self, x: i32, y: i32, scenes: &[WindowScene]) -> Vec<InteractionEffect> {
        let mut effects = Vec::new();
        let target = hit_test_scene(scenes, x, y);

        // Handling Focus
        let new_focus = match target {
            HitTarget::None => None,
            HitTarget::WindowClient { id } |
            HitTarget::WindowTitleBar { id } |
            HitTarget::WindowResize { id, .. } |
            HitTarget::WindowCloseButton { id } => Some(id),
        };

        if new_focus != self.focused_window {
            self.focused_window = new_focus;
            effects.push(InteractionEffect::SetFocus(new_focus));
        }

        match target {
            HitTarget::WindowTitleBar { id } => {
                effects.push(InteractionEffect::Raise(id));
                if let Some(scene) = scenes.iter().find(|s| s.id == id) {
                    self.interaction = Interaction::DragMove {
                        window: id,
                        grab_dx: x - scene.window.x,
                        grab_dy: y - scene.window.y,
                    };
                    effects.push(InteractionEffect::EmitAction {
                        window: id,
                        kind: WindowActionKind::BeginMove,
                        start_x: scene.window.x,
                        start_y: scene.window.y,
                        dx: 0,
                        dy: 0,
                        edges: ResizeEdge::None,
                    });
                }
            }
            HitTarget::WindowResize { id, edge } => {
                effects.push(InteractionEffect::Raise(id));
                if let Some(scene) = scenes.iter().find(|s| s.id == id) {
                    self.interaction = Interaction::DragResize {
                        window: id,
                        edge,
                        start_rect: Rect { x: scene.window.x, y: scene.window.y, w: scene.window.width, h: scene.window.height },
                        start_px: x,
                        start_py: y,
                    };
                    effects.push(InteractionEffect::EmitAction {
                        window: id,
                        kind: WindowActionKind::BeginResize,
                        start_x: scene.window.x,
                        start_y: scene.window.y,
                        dx: 0,
                        dy: 0,
                        edges: edge,
                    });
                }
            }
            HitTarget::WindowClient { id } => {
                effects.push(InteractionEffect::Raise(id));
            }
            HitTarget::WindowCloseButton { id } => {
                effects.push(InteractionEffect::Close(id));
            }
            _ => {}
        }
        effects
    }

    pub fn handle_pointer_move(&mut self, x: i32, y: i32, scenes: &[WindowScene]) -> Vec<InteractionEffect> {
        let mut effects = Vec::new();
        match self.interaction {
            Interaction::DragMove { window, grab_dx, grab_dy } => {
                let new_x = x - grab_dx;
                let new_y = y - grab_dy;
                
                // Fetch current geometry (w/h) to create a valid Rect for update.
                // We could just send what we know (x/y) if UpdateRect supports partial or we merge.
                // But safer to send full rect if we can find it.
                // If not found, we can't really update it safely.
                if let Some(scene) = scenes.iter().find(|s| s.id == window) {
                    effects.push(InteractionEffect::UpdateWindowRect { 
                        id: window, 
                        rect: Rect { x: new_x, y: new_y, w: scene.window.width, h: scene.window.height } 
                    });
                }
            }
            Interaction::DragResize { window, edge, start_rect, start_px, start_py } => {
                let dx = x - start_px;
                let dy = y - start_py;
                
                let mut min_w = 50;
                let mut min_h = 50;
                if let Some(scene) = scenes.iter().find(|s| s.id == window) {
                    min_w = scene.window.min_width.max(50);
                    min_h = scene.window.min_height.max(50);
                }

                let mut new_rect = start_rect;
                
                 match edge {
                     ResizeEdge::Right => {
                         let w = (start_rect.w as i32 + dx).max(min_w as i32);
                         new_rect.w = w as u32;
                     }
                     ResizeEdge::Bottom => {
                        let h = (start_rect.h as i32 + dy).max(min_h as i32);
                        new_rect.h = h as u32;
                     }
                     ResizeEdge::BottomRight => {
                         let w = (start_rect.w as i32 + dx).max(min_w as i32);
                         let h = (start_rect.h as i32 + dy).max(min_h as i32);
                         new_rect.w = w as u32;
                         new_rect.h = h as u32;
                     }
                     ResizeEdge::Left => {
                         let w = (start_rect.w as i32 - dx).max(min_w as i32);
                         new_rect.w = w as u32;
                         if new_rect.w != start_rect.w {
                             new_rect.x = start_rect.x + (start_rect.w as i32 - new_rect.w as i32);
                         }
                     }
                     ResizeEdge::Top => {
                         let h = (start_rect.h as i32 - dy).max(min_h as i32);
                         new_rect.h = h as u32;
                         if new_rect.h != start_rect.h {
                             new_rect.y = start_rect.y + (start_rect.h as i32 - new_rect.h as i32);
                         }
                     }
                     ResizeEdge::TopLeft => {
                         let w = (start_rect.w as i32 - dx).max(min_w as i32);
                         new_rect.w = w as u32;
                         if new_rect.w != start_rect.w {
                             new_rect.x = start_rect.x + (start_rect.w as i32 - new_rect.w as i32);
                         }
                         let h = (start_rect.h as i32 - dy).max(min_h as i32);
                         new_rect.h = h as u32;
                         if new_rect.h != start_rect.h {
                             new_rect.y = start_rect.y + (start_rect.h as i32 - new_rect.h as i32);
                         }
                     }
                     ResizeEdge::TopRight => {
                         let w = (start_rect.w as i32 + dx).max(min_w as i32);
                         new_rect.w = w as u32;
                         let h = (start_rect.h as i32 - dy).max(min_h as i32);
                         new_rect.h = h as u32;
                         if new_rect.h != start_rect.h {
                             new_rect.y = start_rect.y + (start_rect.h as i32 - new_rect.h as i32);
                         }
                     }
                     ResizeEdge::BottomLeft => {
                         let w = (start_rect.w as i32 - dx).max(min_w as i32);
                         new_rect.w = w as u32;
                         if new_rect.w != start_rect.w {
                             new_rect.x = start_rect.x + (start_rect.w as i32 - new_rect.w as i32);
                         }
                         let h = (start_rect.h as i32 + dy).max(min_h as i32);
                         new_rect.h = h as u32;
                     }
                     _ => {}
                 }
                 
                 effects.push(InteractionEffect::UpdateWindowRect { id: window, rect: new_rect });
            }
            _ => {}
        }
        effects
    }

    pub fn handle_pointer_up(&mut self, x: i32, y: i32) -> Vec<InteractionEffect> {
        let mut effects = Vec::new();
        match self.interaction {
            Interaction::DragMove { window, .. } => {
                effects.push(InteractionEffect::EmitAction {
                    window,
                    kind: WindowActionKind::EndMove,
                    start_x: 0, start_y: 0, dx: 0, dy: 0, edges: ResizeEdge::None
                });
                self.interaction = Interaction::Idle;
            }
            Interaction::DragResize { window: _window, start_rect: _start_rect, start_px: _start_px, start_py: _start_py, edge: _edge, .. } => {
                // edge stored in self.interaction
                // But we don't have it easily binded in pattern match if we ignored it?
                // Let's retrieve it.
            }
            _ => {}
        }
        // Properly handle state transition
        if let Interaction::DragResize { window, start_rect, start_px, start_py, edge } = self.interaction {
             effects.push(InteractionEffect::EmitAction {
                    window,
                    kind: WindowActionKind::EndResize,
                    start_x: start_rect.x, start_y: start_rect.y, 
                    dx: x - start_px, dy: y - start_py, 
                    edges: edge
             });
             self.interaction = Interaction::Idle;
        }

        self.interaction = Interaction::Idle;
        effects
    }
}
