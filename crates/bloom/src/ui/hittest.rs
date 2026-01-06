use abi::ui::HitZone;
use crate::ui::WindowScene;
use crate::layout::TITLE_BAR_HEIGHT;

const BORDER_THICKNESS: i32 = 4;
const CORNER_SIZE: i32 = 12;

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
