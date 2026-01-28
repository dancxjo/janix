//! Damage tracking for efficient compositor redraws.
//!
//! This module provides:
//! - `Rect`: A simple rectangle with geometry helpers
//! - `Damage`: A collection of dirty rectangles (max 8, no allocation)
//! - `DamageTracker`: Frame-by-frame damage accumulation

/// Maximum number of distinct damage rectangles before collapsing to full-frame.
pub const MAX_RECTS: usize = 8;

/// A simple rectangle with i32 coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    /// Create a new rectangle.
    #[inline]
    #[allow(dead_code)]
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    /// Create a rectangle representing full screen bounds.
    #[inline]
    pub const fn full(width: i32, height: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            w: width,
            h: height,
        }
    }

    /// Check if the rectangle is empty (zero or negative area).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.w <= 0 || self.h <= 0
    }

    /// Normalize the rectangle to ensure positive dimensions.
    /// Handles negative width/height by adjusting x/y.
    #[inline]
    pub fn normalize(self) -> Self {
        let (x, w) = if self.w < 0 {
            (self.x + self.w, -self.w)
        } else {
            (self.x, self.w)
        };
        let (y, h) = if self.h < 0 {
            (self.y + self.h, -self.h)
        } else {
            (self.y, self.h)
        };
        Self { x, y, w, h }
    }

    /// Clip this rectangle to the given bounds.
    /// Returns an empty rect if completely outside bounds.
    #[inline]
    pub fn clip(self, bounds: Rect) -> Self {
        let x0 = self.x.max(bounds.x);
        let y0 = self.y.max(bounds.y);
        let x1 = (self.x + self.w).min(bounds.x + bounds.w);
        let y1 = (self.y + self.h).min(bounds.y + bounds.h);

        if x1 <= x0 || y1 <= y0 {
            Self::default()
        } else {
            Self {
                x: x0,
                y: y0,
                w: x1 - x0,
                h: y1 - y0,
            }
        }
    }

    /// Compute the union (bounding box) of two rectangles.
    #[inline]
    pub fn union(self, other: Rect) -> Self {
        if self.is_empty() {
            return other;
        }
        if other.is_empty() {
            return self;
        }

        let x0 = self.x.min(other.x);
        let y0 = self.y.min(other.y);
        let x1 = (self.x + self.w).max(other.x + other.w);
        let y1 = (self.y + self.h).max(other.y + other.h);

        Self {
            x: x0,
            y: y0,
            w: x1 - x0,
            h: y1 - y0,
        }
    }

    /// Compute the intersection of two rectangles.
    /// Returns an empty rect if they don't overlap.
    #[inline]
    #[allow(dead_code)]
    pub fn intersect(self, other: Rect) -> Self {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x + self.w).min(other.x + other.w);
        let y1 = (self.y + self.h).min(other.y + other.h);

        if x1 <= x0 || y1 <= y0 {
            Self::default()
        } else {
            Self {
                x: x0,
                y: y0,
                w: x1 - x0,
                h: y1 - y0,
            }
        }
    }

    /// Expand the rectangle by `px` pixels in all directions.
    #[inline]
    pub fn expand(self, px: i32) -> Self {
        Self {
            x: self.x - px,
            y: self.y - px,
            w: self.w + px * 2,
            h: self.h + px * 2,
        }
    }

    /// Check if this rectangle touches or overlaps another.
    /// Touching means edges are adjacent (within 1 pixel).
    #[inline]
    pub fn touches_or_overlaps(self, other: Rect) -> bool {
        // Expand by 1 to catch adjacent edges
        let a = self.expand(1);
        !a.intersect(other).is_empty()
    }

    /// Compute the area of the rectangle.
    #[inline]
    pub fn area(self) -> i64 {
        if self.is_empty() {
            0
        } else {
            self.w as i64 * self.h as i64
        }
    }
}

/// A collection of damage rectangles for a frame.
///
/// Uses a fixed-size array to avoid allocation.
/// If more than `MAX_RECTS` are added, collapses to full-frame damage.
#[derive(Clone, Debug)]
pub struct Damage {
    rects: [Rect; MAX_RECTS],
    count: usize,
    bounds: Rect,
    /// True if the entire frame is damaged.
    pub is_full: bool,
}

#[allow(dead_code)]
impl Damage {
    /// Create empty damage (nothing needs redrawing).
    pub fn empty(bounds: Rect) -> Self {
        Self {
            rects: [Rect::default(); MAX_RECTS],
            count: 0,
            bounds,
            is_full: false,
        }
    }

    /// Create full-frame damage.
    pub fn full(bounds: Rect) -> Self {
        Self {
            rects: [
                bounds,
                Rect::default(),
                Rect::default(),
                Rect::default(),
                Rect::default(),
                Rect::default(),
                Rect::default(),
                Rect::default(),
            ],
            count: 1,
            bounds,
            is_full: true,
        }
    }

    /// Check if there's no damage at all.
    pub fn is_empty(&self) -> bool {
        !self.is_full && self.count == 0
    }

    /// Get the number of damage rects.
    pub fn rect_count(&self) -> usize {
        if self.is_full {
            1
        } else {
            self.count
        }
    }

    /// Add a rectangle to the damage set.
    /// Clips to bounds, discards empty rects, merges overlapping/touching rects.
    pub fn add_rect(&mut self, r: Rect) {
        if self.is_full {
            return; // Already fully damaged
        }

        let clipped = r.clip(self.bounds);
        if clipped.is_empty() {
            return; // Outside bounds or empty
        }

        // Threshold: if single rect > 25% of screen, collapse to full
        let screen_area = self.bounds.area();
        if clipped.area() * 4 > screen_area {
            self.collapse_to_full();
            return;
        }

        // Try to merge with existing rects
        for i in 0..self.count {
            if self.rects[i].touches_or_overlaps(clipped) {
                self.rects[i] = self.rects[i].union(clipped);
                // After merge, try to consolidate further
                self.consolidate();
                return;
            }
        }

        // No merge possible, add as new rect
        if self.count < MAX_RECTS {
            self.rects[self.count] = clipped;
            self.count += 1;
        } else {
            // Exceeded MAX_RECTS, collapse to full-frame
            self.collapse_to_full();
        }
    }

    /// Consolidate overlapping/touching rectangles.
    fn consolidate(&mut self) {
        if self.count <= 1 {
            return;
        }

        // Simple O(n²) merge pass
        let mut changed = true;
        while changed {
            changed = false;
            'outer: for i in 0..self.count {
                for j in (i + 1)..self.count {
                    if self.rects[i].touches_or_overlaps(self.rects[j]) {
                        self.rects[i] = self.rects[i].union(self.rects[j]);
                        // Check if merged rect exceeds threshold (25% of screen)
                        if self.rects[i].area() * 4 > self.bounds.area() {
                            self.collapse_to_full();
                            return;
                        }
                        // Remove j by swapping with last
                        self.count -= 1;
                        if j < self.count {
                            self.rects[j] = self.rects[self.count];
                        }
                        changed = true;
                        break 'outer;
                    }
                }
            }
        }
    }

    /// Collapse all damage to a single full-frame rect.
    fn collapse_to_full(&mut self) {
        self.rects[0] = self.bounds;
        self.count = 1;
        self.is_full = true;
    }

    /// Iterate over the damage rectangles.
    pub fn iter(&self) -> impl Iterator<Item = Rect> + '_ {
        self.rects[..self.count].iter().copied()
    }

    /// Get the bounding box of all damage.
    #[allow(dead_code)]
    pub fn bounding_box(&self) -> Rect {
        let mut result = Rect::default();
        for r in self.iter() {
            result = result.union(r);
        }
        result
    }

    /// Get the bounds rect (clipping area).
    #[allow(dead_code)]
    pub fn bounds(&self) -> Rect {
        self.bounds
    }
}

/// Tracks damage accumulation for a single frame.
#[allow(dead_code)]
pub struct DamageTracker {
    bounds: Rect,
    damage: Damage,
}

#[allow(dead_code)]
impl DamageTracker {
    /// Create a new tracker (call begin_frame to initialize).
    pub fn new() -> Self {
        Self {
            bounds: Rect::default(),
            damage: Damage::empty(Rect::default()),
        }
    }

    /// Begin a new frame with the given screen dimensions.
    pub fn begin_frame(&mut self, width: i32, height: i32) {
        self.bounds = Rect::full(width, height);
        self.damage = Damage::empty(self.bounds);
    }

    /// Note a bounding box that needs to be redrawn.
    pub fn note_bbox(&mut self, rect: Rect) {
        self.damage.add_rect(rect);
    }

    /// Note cursor movement: damages both old and new cursor positions.
    pub fn note_cursor_move(&mut self, old_bbox: Rect, new_bbox: Rect) {
        // Damage the old position (needs to be redrawn to erase)
        self.damage.add_rect(old_bbox);
        // Damage the new position (needs to be drawn)
        self.damage.add_rect(new_bbox);
    }

    /// Mark the entire frame as damaged.
    pub fn mark_full(&mut self) {
        self.damage = Damage::full(self.bounds);
    }

    /// End the frame and return the accumulated damage.
    pub fn end_frame(&mut self) -> Damage {
        core::mem::replace(&mut self.damage, Damage::empty(self.bounds))
    }
}

impl Default for DamageTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Rect tests ---

    #[test]
    fn test_rect_is_empty() {
        assert!(Rect::new(0, 0, 0, 10).is_empty());
        assert!(Rect::new(0, 0, 10, 0).is_empty());
        assert!(Rect::new(0, 0, -5, 10).is_empty());
        assert!(!Rect::new(0, 0, 10, 10).is_empty());
    }

    #[test]
    fn test_rect_normalize() {
        let r = Rect::new(10, 10, -5, -5);
        let n = r.normalize();
        assert_eq!(n, Rect::new(5, 5, 5, 5));
    }

    #[test]
    fn test_rect_clip() {
        let bounds = Rect::new(0, 0, 100, 100);

        // Fully inside
        let r = Rect::new(10, 10, 20, 20);
        assert_eq!(r.clip(bounds), r);

        // Partially outside
        let r = Rect::new(-10, 50, 30, 20);
        assert_eq!(r.clip(bounds), Rect::new(0, 50, 20, 20));

        // Fully outside
        let r = Rect::new(200, 200, 10, 10);
        assert!(r.clip(bounds).is_empty());
    }

    #[test]
    fn test_rect_union() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(20, 20, 10, 10);
        let u = a.union(b);
        assert_eq!(u, Rect::new(0, 0, 30, 30));

        // Union with empty
        let e = Rect::default();
        assert_eq!(a.union(e), a);
        assert_eq!(e.union(a), a);
    }

    #[test]
    fn test_rect_intersect() {
        let a = Rect::new(0, 0, 20, 20);
        let b = Rect::new(10, 10, 20, 20);
        let i = a.intersect(b);
        assert_eq!(i, Rect::new(10, 10, 10, 10));

        // No overlap
        let c = Rect::new(50, 50, 10, 10);
        assert!(a.intersect(c).is_empty());
    }

    #[test]
    fn test_rect_expand() {
        let r = Rect::new(10, 10, 20, 20);
        let e = r.expand(5);
        assert_eq!(e, Rect::new(5, 5, 30, 30));
    }

    #[test]
    fn test_rect_touches_or_overlaps() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(10, 0, 10, 10); // Adjacent
        let c = Rect::new(5, 5, 10, 10); // Overlapping
        let d = Rect::new(20, 20, 10, 10); // Separate

        assert!(a.touches_or_overlaps(b));
        assert!(a.touches_or_overlaps(c));
        assert!(!a.touches_or_overlaps(d));
    }

    // --- Damage tests ---

    #[test]
    fn test_damage_empty() {
        let bounds = Rect::full(100, 100);
        let d = Damage::empty(bounds);
        assert!(d.is_empty());
        assert!(!d.is_full);
        assert_eq!(d.rect_count(), 0);
    }

    #[test]
    fn test_damage_full() {
        let bounds = Rect::full(100, 100);
        let d = Damage::full(bounds);
        assert!(!d.is_empty());
        assert!(d.is_full);
        assert_eq!(d.rect_count(), 1);
    }

    #[test]
    fn test_damage_add_and_merge() {
        let bounds = Rect::full(100, 100);
        let mut d = Damage::empty(bounds);

        d.add_rect(Rect::new(0, 0, 10, 10));
        assert_eq!(d.rect_count(), 1);

        // Adjacent rect should merge
        d.add_rect(Rect::new(10, 0, 10, 10));
        assert_eq!(d.rect_count(), 1);

        // Separate rect should not merge
        d.add_rect(Rect::new(50, 50, 10, 10));
        assert_eq!(d.rect_count(), 2);
    }

    #[test]
    fn test_damage_clips_to_bounds() {
        let bounds = Rect::full(100, 100);
        let mut d = Damage::empty(bounds);

        d.add_rect(Rect::new(-10, -10, 20, 20));
        let r: alloc::vec::Vec<_> = d.iter().collect();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0], Rect::new(0, 0, 10, 10));
    }

    #[test]
    fn test_damage_collapse_on_overflow() {
        let bounds = Rect::full(1000, 1000);
        let mut d = Damage::empty(bounds);

        // Add MAX_RECTS separate rects
        for i in 0..MAX_RECTS {
            d.add_rect(Rect::new((i * 100) as i32, 0, 10, 10));
        }
        assert!(!d.is_full);

        // One more should collapse
        d.add_rect(Rect::new(900, 0, 10, 10));
        assert!(d.is_full);
    }

    // --- DamageTracker tests ---

    #[test]
    fn test_tracker_cursor_move() {
        let mut t = DamageTracker::new();
        t.begin_frame(100, 100);

        let old = Rect::new(10, 10, 16, 16);
        let new = Rect::new(20, 20, 16, 16);
        t.note_cursor_move(old, new);

        let d = t.end_frame();
        // Should have damaged both old and new (merged since they might touch)
        assert!(!d.is_empty());
    }

    #[test]
    fn test_tracker_clear_is_full() {
        let mut t = DamageTracker::new();
        t.begin_frame(100, 100);

        // Clear damages entire screen
        t.note_bbox(Rect::full(100, 100));

        let d = t.end_frame();
        // Full screen damage should be marked
        let bbox = d.bounding_box();
        assert_eq!(bbox, Rect::full(100, 100));
    }
}
