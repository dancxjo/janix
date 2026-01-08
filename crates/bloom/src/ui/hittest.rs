use abi::ui::{HitZone, ResizeEdge};
use abi::ids::ThingId;
use crate::ui::WindowScene;
use crate::layout::TITLE_BAR_HEIGHT;

const BORDER_THICKNESS: i32 = 4;
const CORNER_SIZE: i32 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitTarget {
    None,
    WindowClient { id: ThingId },
    WindowTitleBar { id: ThingId },
    WindowResize { id: ThingId, edge: ResizeEdge },
    WindowCloseButton { id: ThingId },
}

/// Find the top-most window under (px, py) and return the specific target.
pub fn hit_test_scene(scenes: &[WindowScene], px: i32, py: i32) -> HitTarget {
    // Iterate from top (last) to bottom (first)
    for scene in scenes.iter().rev() {
        let (zone, lx, ly) = hittest_window(scene, px, py);
        if zone == HitZone::None {
            continue;
        }

        // Refine HitZone into HitTarget
        match zone {
            HitZone::Titlebar => {
                // Check close button
                let win_rect = crate::scene::Rect {
                    x: scene.window.x,
                    y: scene.window.y,
                    w: scene.window.width,
                    h: scene.window.height,
                };
                let close_rect = crate::ui::get_close_button_rect(win_rect);
                // Transform to local coordinates for the window logic?
                // hittest_window returns lx, ly relative to top-left of window.
                // get_close_button_rect returns SCREEN coordinates.
                
                if px >= close_rect.x && px < close_rect.x + close_rect.w as i32 &&
                   py >= close_rect.y && py < close_rect.y + close_rect.h as i32 
                {
                    return HitTarget::WindowCloseButton { id: scene.id };
                }
                
                return HitTarget::WindowTitleBar { id: scene.id };
            }
            HitZone::Content => {
                return HitTarget::WindowClient { id: scene.id };
            }
            HitZone::ResizeN => return HitTarget::WindowResize { id: scene.id, edge: ResizeEdge::Top },
            HitZone::ResizeS => return HitTarget::WindowResize { id: scene.id, edge: ResizeEdge::Bottom },
            HitZone::ResizeE => return HitTarget::WindowResize { id: scene.id, edge: ResizeEdge::Right },
            HitZone::ResizeW => return HitTarget::WindowResize { id: scene.id, edge: ResizeEdge::Left },
            HitZone::ResizeNW => return HitTarget::WindowResize { id: scene.id, edge: ResizeEdge::TopLeft },
            HitZone::ResizeNE => return HitTarget::WindowResize { id: scene.id, edge: ResizeEdge::TopRight },
            HitZone::ResizeSW => return HitTarget::WindowResize { id: scene.id, edge: ResizeEdge::BottomLeft },
            HitZone::ResizeSE => return HitTarget::WindowResize { id: scene.id, edge: ResizeEdge::BottomRight },
            _ => {
                // Treat others as Client or Border?
                // Border hits that aren't resize are probably client or nothing?
                // For now, map Border to valid hit so click-to-focus works
                return HitTarget::WindowClient { id: scene.id };
            }
        }
    }
    HitTarget::None
}

pub fn hittest_window(window: &WindowScene, px: i32, py: i32) -> (HitZone, i32, i32) {
    let wx = window.window.x;
    let wy = window.window.y;
    let ww = window.window.width as i32;
    let wh = window.window.height as i32;

    let lx = px - wx;
    let ly = py - wy;

    // Check if outside window bounds (including border/shadow considerations if we had them extended)
    // For now, simple box check
    if lx < 0 || ly < 0 || lx >= ww || ly >= wh {
        return (HitZone::None, 0, 0);
    }

    // Resize handles (borders and corners)
    // Top corners
    if ly < BORDER_THICKNESS {
        if lx < CORNER_SIZE { return (HitZone::ResizeNW, lx, ly); }
        if lx >= ww - CORNER_SIZE { return (HitZone::ResizeNE, lx, ly); }
        return (HitZone::ResizeN, lx, ly);
    }

    // Bottom corners
    if ly >= wh - BORDER_THICKNESS {
        if lx < CORNER_SIZE { return (HitZone::ResizeSW, lx, ly); }
        if lx >= ww - CORNER_SIZE { return (HitZone::ResizeSE, lx, ly); }
        return (HitZone::ResizeS, lx, ly);
    }

    // Left edge (excluding corners already handled)
    if lx < BORDER_THICKNESS {
        return (HitZone::ResizeW, lx, ly);
    }

    // Right edge
    if lx >= ww - BORDER_THICKNESS {
        return (HitZone::ResizeE, lx, ly);
    }

    // Titlebar
    if ly < TITLE_BAR_HEIGHT {
        // We already checked top resize border; titlebar is below that but above content
        // Actually, typically titlebar *includes* the top area, but resize capability overrides it.
        // Let's say top 4px are resize N, next (TITLE_BAR_HEIGHT - 4) are Titlebar.
        return (HitZone::Titlebar, lx, ly);
    }

    // Content
    (HitZone::Content, lx, ly)
}
