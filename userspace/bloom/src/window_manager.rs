//! Window Manager - Chrome hit testing, move/resize, and input routing.
//!
//! This module is the authoritative source for:
//! - Window geometry contract (window rect vs client rect)
//! - Chrome hit testing (buttons, title bar, resize edges/corners)
//! - Move/resize drag controller with pointer capture
//! - Input routing to apps (client area only)

use crate::geometry::Rect;
use crate::ui::constants::{
    BORDER_THICKNESS, MAXIMIZE_BUTTON_PADDING, MAXIMIZE_BUTTON_SIZE, MIN_WINDOW_HEIGHT,
    MIN_WINDOW_WIDTH, RESIZE_CORNER_SIZE, SHADE_BUTTON_PADDING, SHADE_BUTTON_SIZE,
    TITLE_BAR_HEIGHT,
};
use alloc::vec::Vec;
use stem::thing::ThingId;

/// Edge of a window for resize operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    North,
    South,
    East,
    West,
}

/// Corner of a window for resize operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

/// Result of hit testing a point against a window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hit {
    /// Point is outside the window.
    None,
    /// Point is in the client area (content region).
    ClientArea,
    /// Point is on the title bar (for dragging).
    TitleBar,
    /// Point is on the shade/close button.
    ButtonShade,
    /// Point is on the maximize button.
    ButtonMaximize,
    /// Point is on a resize edge.
    ResizeEdge(Edge),
    /// Point is on a resize corner.
    ResizeCorner(Corner),
}

/// Anchor describing which edges are being resized.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResizeAnchor {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

impl ResizeAnchor {
    pub fn from_edge(edge: Edge) -> Self {
        match edge {
            Edge::North => Self {
                north: true,
                south: false,
                east: false,
                west: false,
            },
            Edge::South => Self {
                north: false,
                south: true,
                east: false,
                west: false,
            },
            Edge::East => Self {
                north: false,
                south: false,
                east: true,
                west: false,
            },
            Edge::West => Self {
                north: false,
                south: false,
                east: false,
                west: true,
            },
        }
    }

    pub fn from_corner(corner: Corner) -> Self {
        match corner {
            Corner::NorthEast => Self {
                north: true,
                south: false,
                east: true,
                west: false,
            },
            Corner::NorthWest => Self {
                north: true,
                south: false,
                east: false,
                west: true,
            },
            Corner::SouthEast => Self {
                north: false,
                south: true,
                east: true,
                west: false,
            },
            Corner::SouthWest => Self {
                north: false,
                south: true,
                east: false,
                west: true,
            },
        }
    }
}

/// Kind of drag operation in progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragKind {
    Move,
    Resize { anchor: ResizeAnchor },
}

/// Active drag state.
#[derive(Clone, Copy, Debug)]
pub struct Drag {
    pub wid: ThingId,
    pub kind: DragKind,
    pub start_mouse: (i32, i32),
    pub start_window_rect: Rect,
}

/// Window state for hit testing.
#[derive(Clone, Debug)]
pub struct WindowState {
    pub id: ThingId,
    pub rect: Rect,
    pub is_shaded: bool,
    pub is_maximized: bool,
    /// Cached pre-maximize rect for restore.
    pub pre_maximize_rect: Option<Rect>,
}

impl WindowState {
    /// Compute the client rect (content area inside chrome).
    pub fn client_rect(&self) -> Rect {
        compute_client_rect(&self.rect, self.is_shaded)
    }
}

/// Compute the client rect from a window rect and shaded state.
///
/// Client rect is the area available for app content, excluding:
/// - Border (BORDER_THICKNESS on all sides)
/// - Title bar (TITLE_BAR_HEIGHT at top)
pub fn compute_client_rect(window_rect: &Rect, is_shaded: bool) -> Rect {
    if is_shaded {
        // Shaded windows have no client area
        return Rect::new(
            window_rect.x() + BORDER_THICKNESS,
            window_rect.y() + TITLE_BAR_HEIGHT,
            0,
            0,
        );
    }

    let content_inset = BORDER_THICKNESS * 2; // Two frame layers
    Rect::new(
        window_rect.x() + content_inset,
        window_rect.y() + content_inset + TITLE_BAR_HEIGHT - content_inset,
        (window_rect.width() - content_inset * 2).max(0),
        (window_rect.height() - TITLE_BAR_HEIGHT - content_inset).max(0),
    )
}

/// Clamp a window rect to screen bounds and minimum size.
pub fn clamp_window_rect(rect: Rect, screen_w: i32, screen_h: i32) -> Rect {
    let mut r = rect;

    // Enforce minimum size
    if r.width() < MIN_WINDOW_WIDTH {
        r.size.width = MIN_WINDOW_WIDTH;
    }
    if r.height() < MIN_WINDOW_HEIGHT {
        r.size.height = MIN_WINDOW_HEIGHT;
    }

    // Keep title bar reachable (at least partially on screen)
    let min_visible = TITLE_BAR_HEIGHT;
    if r.y() + min_visible < 0 {
        r.origin.y = -min_visible + 1;
    }
    if r.y() > screen_h - min_visible {
        r.origin.y = screen_h - min_visible;
    }
    if r.x() + r.width() < min_visible {
        r.origin.x = min_visible - r.width();
    }
    if r.x() > screen_w - min_visible {
        r.origin.x = screen_w - min_visible;
    }

    r
}

/// Hit test a point against a single window.
///
/// Coordinates are in screen space. Window must contain the point for
/// any hit other than None.
pub fn hit_test(screen_x: i32, screen_y: i32, window: &WindowState) -> Hit {
    let r = &window.rect;

    // Outside window entirely?
    if screen_x < r.x()
        || screen_x >= r.x() + r.width()
        || screen_y < r.y()
        || screen_y >= r.y() + r.height()
    {
        return Hit::None;
    }

    let local_x = screen_x - r.x();
    let local_y = screen_y - r.y();

    // Corner resize zones (check first, they overlap edges)
    let corner_size = RESIZE_CORNER_SIZE;

    // NW corner
    if local_x < corner_size && local_y < corner_size {
        return Hit::ResizeCorner(Corner::NorthWest);
    }
    // NE corner
    if local_x >= r.width() - corner_size && local_y < corner_size {
        return Hit::ResizeCorner(Corner::NorthEast);
    }
    // SW corner
    if local_x < corner_size && local_y >= r.height() - corner_size {
        return Hit::ResizeCorner(Corner::SouthWest);
    }
    // SE corner
    if local_x >= r.width() - corner_size && local_y >= r.height() - corner_size {
        return Hit::ResizeCorner(Corner::SouthEast);
    }

    // Edge resize zones
    if local_y < BORDER_THICKNESS {
        return Hit::ResizeEdge(Edge::North);
    }
    if local_y >= r.height() - BORDER_THICKNESS {
        return Hit::ResizeEdge(Edge::South);
    }
    if local_x < BORDER_THICKNESS {
        return Hit::ResizeEdge(Edge::West);
    }
    if local_x >= r.width() - BORDER_THICKNESS {
        return Hit::ResizeEdge(Edge::East);
    }

    // Title bar region (inside border, at top)
    let title_top = BORDER_THICKNESS * 2;
    let title_bottom = TITLE_BAR_HEIGHT;
    if local_y >= title_top && local_y < title_bottom {
        // Check buttons from right to left
        // Button layout: [title text] ... [maximize] [shade]
        let handle_width = 40; // From paint.rs
        let button_region_start =
            r.width() - SHADE_BUTTON_PADDING - SHADE_BUTTON_SIZE - handle_width - 8;

        // Shade button
        let shade_x = button_region_start;
        let shade_y = title_top + (title_bottom - title_top - SHADE_BUTTON_SIZE) / 2;
        if local_x >= shade_x
            && local_x < shade_x + SHADE_BUTTON_SIZE
            && local_y >= shade_y
            && local_y < shade_y + SHADE_BUTTON_SIZE
        {
            return Hit::ButtonShade;
        }

        // Maximize button (left of shade)
        let maximize_x = shade_x - MAXIMIZE_BUTTON_PADDING - MAXIMIZE_BUTTON_SIZE;
        let maximize_y = shade_y;
        if local_x >= maximize_x
            && local_x < maximize_x + MAXIMIZE_BUTTON_SIZE
            && local_y >= maximize_y
            && local_y < maximize_y + MAXIMIZE_BUTTON_SIZE
        {
            return Hit::ButtonMaximize;
        }

        return Hit::TitleBar;
    }

    // If shaded, there's no client area
    if window.is_shaded {
        return Hit::None;
    }

    // Client area
    Hit::ClientArea
}

/// Pick the topmost window at a screen coordinate.
///
/// Windows should be provided in z-order (front to back).
/// Returns the window ID and hit result for the first window hit.
pub fn pick_window(
    screen_x: i32,
    screen_y: i32,
    windows: &[WindowState],
) -> Option<(ThingId, Hit)> {
    for window in windows {
        let hit = hit_test(screen_x, screen_y, window);
        if hit != Hit::None {
            return Some((window.id, hit));
        }
    }
    None
}

/// Apply move delta to a window rect with clamping.
pub fn apply_move_delta(start_rect: Rect, delta: (i32, i32), screen_w: i32, screen_h: i32) -> Rect {
    let new_rect = Rect::new(
        start_rect.x() + delta.0,
        start_rect.y() + delta.1,
        start_rect.width(),
        start_rect.height(),
    );
    clamp_window_rect(new_rect, screen_w, screen_h)
}

/// Apply resize delta to a window rect with min size enforcement.
pub fn apply_resize_delta(
    start_rect: Rect,
    anchor: ResizeAnchor,
    delta: (i32, i32),
    screen_w: i32,
    screen_h: i32,
) -> Rect {
    let mut x = start_rect.x();
    let mut y = start_rect.y();
    let mut w = start_rect.width();
    let mut h = start_rect.height();

    if anchor.west {
        let new_x = x + delta.0;
        let new_w = w - delta.0;
        if new_w >= MIN_WINDOW_WIDTH {
            x = new_x;
            w = new_w;
        } else {
            x = x + w - MIN_WINDOW_WIDTH;
            w = MIN_WINDOW_WIDTH;
        }
    }
    if anchor.east {
        w = (w + delta.0).max(MIN_WINDOW_WIDTH);
    }
    if anchor.north {
        let new_y = y + delta.1;
        let new_h = h - delta.1;
        if new_h >= MIN_WINDOW_HEIGHT {
            y = new_y;
            h = new_h;
        } else {
            y = y + h - MIN_WINDOW_HEIGHT;
            h = MIN_WINDOW_HEIGHT;
        }
    }
    if anchor.south {
        h = (h + delta.1).max(MIN_WINDOW_HEIGHT);
    }

    clamp_window_rect(Rect::new(x, y, w, h), screen_w, screen_h)
}

/// Central window manager state.
pub struct WindowManager {
    /// Currently active drag, if any.
    pub drag: Option<Drag>,
    /// Focused window ID.
    pub focused: Option<ThingId>,
    /// Screen dimensions for clamping.
    pub screen_w: i32,
    pub screen_h: i32,
    /// Damage rects from geometry changes (old_rect, new_rect).
    pending_damage: Vec<Rect>,
}

impl WindowManager {
    pub fn new(screen_w: i32, screen_h: i32) -> Self {
        Self {
            drag: None,
            focused: None,
            screen_w,
            screen_h,
            pending_damage: Vec::new(),
        }
    }

    /// Begin a drag operation.
    pub fn begin_drag(
        &mut self,
        wid: ThingId,
        kind: DragKind,
        mouse: (i32, i32),
        window_rect: Rect,
    ) {
        self.drag = Some(Drag {
            wid,
            kind,
            start_mouse: mouse,
            start_window_rect: window_rect,
        });
        self.focused = Some(wid);
    }

    /// Update drag with current mouse position.
    /// Returns the new window rect if drag is active.
    pub fn update_drag(&mut self, current_mouse: (i32, i32), current_rect: Rect) -> Option<Rect> {
        let drag = self.drag.as_ref()?;
        let delta = (
            current_mouse.0 - drag.start_mouse.0,
            current_mouse.1 - drag.start_mouse.1,
        );

        let new_rect = match drag.kind {
            DragKind::Move => {
                apply_move_delta(drag.start_window_rect, delta, self.screen_w, self.screen_h)
            }
            DragKind::Resize { anchor } => apply_resize_delta(
                drag.start_window_rect,
                anchor,
                delta,
                self.screen_w,
                self.screen_h,
            ),
        };

        if new_rect != current_rect {
            // Add damage for both old and new positions
            self.pending_damage.push(current_rect);
            self.pending_damage.push(new_rect);
            Some(new_rect)
        } else {
            None
        }
    }

    /// End the current drag operation.
    pub fn end_drag(&mut self) -> Option<ThingId> {
        self.drag.take().map(|d| d.wid)
    }

    /// Check if a drag is currently active.
    pub fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }

    /// Get the window being dragged (if any).
    pub fn dragging_window(&self) -> Option<ThingId> {
        self.drag.as_ref().map(|d| d.wid)
    }

    /// Take pending damage rects and clear the buffer.
    pub fn take_damage(&mut self) -> Vec<Rect> {
        core::mem::take(&mut self.pending_damage)
    }

    /// Add damage for a window move/resize/shade operation.
    pub fn add_damage(&mut self, old_rect: Rect, new_rect: Rect) {
        self.pending_damage.push(old_rect);
        self.pending_damage.push(new_rect);
    }

    /// Toggle maximize state for a window.
    /// Returns the new rect (either maximized or restored).
    pub fn toggle_maximize(&mut self, window: &mut WindowState) -> Rect {
        if window.is_maximized {
            // Restore to pre-maximize rect
            let restored = window.pre_maximize_rect.unwrap_or(window.rect);
            self.add_damage(window.rect, restored);
            window.is_maximized = false;
            window.pre_maximize_rect = None;
            restored
        } else {
            // Save current rect and maximize
            window.pre_maximize_rect = Some(window.rect);
            let maximized = Rect::new(0, 0, self.screen_w, self.screen_h);
            self.add_damage(window.rect, maximized);
            window.is_maximized = true;
            maximized
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(n: u8) -> ThingId {
        let mut b = [0u8; 16];
        b[0] = n;
        ThingId(b)
    }

    fn test_window() -> WindowState {
        WindowState {
            id: make_id(1),
            rect: Rect::new(100, 100, 400, 300),
            is_shaded: false,
            is_maximized: false,
            pre_maximize_rect: None,
        }
    }

    #[test]
    fn hit_test_outside_window() {
        let window = test_window();
        assert_eq!(hit_test(50, 50, &window), Hit::None);
        assert_eq!(hit_test(600, 200, &window), Hit::None);
    }

    #[test]
    fn hit_test_client_area() {
        let window = test_window();
        // Client area is inside title bar and borders
        assert_eq!(hit_test(250, 200, &window), Hit::ClientArea);
    }

    #[test]
    fn hit_test_title_bar() {
        let window = test_window();
        // Title bar is between y=108 (BORDER_THICKNESS*2) and y=140 (TITLE_BAR_HEIGHT)
        // And not in button regions
        assert_eq!(hit_test(200, 120, &window), Hit::TitleBar);
    }

    #[test]
    fn hit_test_resize_corners() {
        let window = test_window();
        // NW corner
        assert_eq!(
            hit_test(102, 102, &window),
            Hit::ResizeCorner(Corner::NorthWest)
        );
        // NE corner
        assert_eq!(
            hit_test(498, 102, &window),
            Hit::ResizeCorner(Corner::NorthEast)
        );
        // SW corner
        assert_eq!(
            hit_test(102, 398, &window),
            Hit::ResizeCorner(Corner::SouthWest)
        );
        // SE corner
        assert_eq!(
            hit_test(498, 398, &window),
            Hit::ResizeCorner(Corner::SouthEast)
        );
    }

    #[test]
    fn hit_test_resize_edges() {
        let window = test_window();
        // North edge (not corner)
        assert_eq!(hit_test(250, 101, &window), Hit::ResizeEdge(Edge::North));
        // South edge
        assert_eq!(hit_test(250, 398, &window), Hit::ResizeEdge(Edge::South));
        // West edge
        assert_eq!(hit_test(101, 250, &window), Hit::ResizeEdge(Edge::West));
        // East edge
        assert_eq!(hit_test(498, 250, &window), Hit::ResizeEdge(Edge::East));
    }

    #[test]
    fn move_drag_clamps_to_screen() {
        let start = Rect::new(100, 100, 200, 150);
        // Move far left
        let result = apply_move_delta(start, (-500, 0), 800, 600);
        // Should clamp so title bar is still reachable
        assert!(result.x() + result.width() >= TITLE_BAR_HEIGHT);
    }

    #[test]
    fn resize_drag_enforces_min_size() {
        let start = Rect::new(100, 100, 200, 150);
        let anchor = ResizeAnchor::from_edge(Edge::West);
        // Try to shrink width below minimum
        let result = apply_resize_delta(start, anchor, (500, 0), 800, 600);
        assert_eq!(result.width(), MIN_WINDOW_WIDTH);
    }

    #[test]
    fn compute_client_rect_normal() {
        let window_rect = Rect::new(100, 100, 400, 300);
        let client = compute_client_rect(&window_rect, false);
        // Client should be inset by borders and title bar
        assert!(client.x() > window_rect.x());
        assert!(client.y() > window_rect.y());
        assert!(client.width() < window_rect.width());
        assert!(client.height() < window_rect.height());
    }

    #[test]
    fn compute_client_rect_shaded() {
        let window_rect = Rect::new(100, 100, 400, 300);
        let client = compute_client_rect(&window_rect, true);
        // Shaded window has no client area
        assert_eq!(client.width(), 0);
        assert_eq!(client.height(), 0);
    }

    #[test]
    fn pick_window_z_order() {
        let front = WindowState {
            id: make_id(1),
            rect: Rect::new(100, 100, 200, 200),
            is_shaded: false,
            is_maximized: false,
            pre_maximize_rect: None,
        };
        let back = WindowState {
            id: make_id(2),
            rect: Rect::new(150, 150, 200, 200),
            is_shaded: false,
            is_maximized: false,
            pre_maximize_rect: None,
        };
        // Front window first in array (z-order front to back)
        let windows = [front.clone(), back];
        let result = pick_window(180, 180, &windows);
        // Should hit front window even though back overlaps
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, make_id(1));
    }
}
