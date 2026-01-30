//! Damage tracking for efficient compositor redraws.
//!
//! This module provides:
//! - `DamageCause`: Explicit reason for damage
//! - `Rect`: A simple rectangle with geometry helpers
//! - `Damage`: A collection of dirty rectangles (max 8, no allocation)
//! - `DamageTracker`: Frame-by-frame damage accumulation
//! - `DamageJournal`: Debug-only damage history tracking

/// Maximum number of distinct damage rectangles before collapsing to full-frame.
/// Increased from 8 to allow more granular damage tracking before driver-level merging.
pub const MAX_RECTS: usize = 32;

use crate::geometry::Rect;
use crate::snapshot::SnapshotInvalidation;
use alloc::vec::Vec;
use stem::thing::ThingId;

/// Explicit cause for damage invalidation.
///
/// Every damage rect should have at least one cause to make rendering
/// decisions auditable and debuggable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageCause {
    /// Window or surface geometry changed (move, resize)
    GeometryChanged,
    /// Paint properties changed (color, style, etc)
    PaintChanged,
    /// Asset was updated (image, font, cursor)
    AssetUpdated,
    /// Cursor moved to a new position
    CursorMoved,
    /// Full redraw was explicitly requested
    ForceFull,
    /// Content of a window snapshot changed
    ContentChanged,
    /// Font rendering changed (font file, size, etc)
    FontChanged,
    /// Theme settings changed
    ThemeChanged,
    /// Cause is unknown or not specified
    Unknown,
}

impl DamageCause {
    /// Convert from SnapshotInvalidation to DamageCause
    pub fn from_invalidation(inv: SnapshotInvalidation) -> Self {
        match inv {
            SnapshotInvalidation::GeometryChanged => DamageCause::GeometryChanged,
            SnapshotInvalidation::ContentChanged => DamageCause::ContentChanged,
            SnapshotInvalidation::FontChanged => DamageCause::FontChanged,
            SnapshotInvalidation::ThemeChanged => DamageCause::ThemeChanged,
            SnapshotInvalidation::Forced => DamageCause::ForceFull,
        }
    }

    /// Get a short color code for debug visualization
    pub fn debug_color(&self) -> u32 {
        match self {
            DamageCause::GeometryChanged => 0xFF00FFFF, // Cyan
            DamageCause::PaintChanged => 0xFFFF00FF,    // Magenta
            DamageCause::AssetUpdated => 0xFFFFFF00,    // Yellow
            DamageCause::CursorMoved => 0xFF00FF00,     // Green
            DamageCause::ForceFull => 0xFFFF0000,       // Red
            DamageCause::ContentChanged => 0xFFFF8000,  // Orange
            DamageCause::FontChanged => 0xFF87CEEB,     // Light Blue
            DamageCause::ThemeChanged => 0xFFFFB6C1,    // Light Pink
            DamageCause::Unknown => 0xFF808080,         // Gray
        }
    }
}

/// A damage record with an associated cause.
#[derive(Clone, Copy, Debug)]
pub struct DamageRecord {
    pub rect: Rect,
    pub cause: DamageCause,
    pub source: Option<ThingId>,
}

/// A collection of damage rectangles for a frame.
///
/// Uses a fixed-size array to avoid allocation.
/// If more than `MAX_RECTS` are added, collapses to full-frame damage.
#[derive(Clone, Debug)]
pub struct Damage {
    rects: [Rect; MAX_RECTS],
    causes: [DamageCause; MAX_RECTS],
    sources: [Option<ThingId>; MAX_RECTS],
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
            causes: [DamageCause::Unknown; MAX_RECTS],
            sources: [None; MAX_RECTS],
            count: 0,
            bounds,
            is_full: false,
        }
    }

    /// Create damage from a slice of rects WITHOUT consolidation.
    /// Use this when rects are already consolidated and should not be merged.
    pub fn from_rects(bounds: Rect, rects_slice: &[Rect]) -> Self {
        let mut d = Self::empty(bounds);
        let count = rects_slice.len().min(MAX_RECTS);
        for i in 0..count {
            d.rects[i] = rects_slice[i];
        }
        d.count = count;
        d
    }

    /// Create full-frame damage with a cause.
    pub fn full(bounds: Rect) -> Self {
        Self::full_with_cause(bounds, DamageCause::ForceFull, None)
    }

    /// Create full-frame damage with an explicit cause.
    pub fn full_with_cause(bounds: Rect, cause: DamageCause, source: Option<ThingId>) -> Self {
        let mut rects = [Rect::default(); MAX_RECTS];
        rects[0] = bounds;
        let mut causes = [DamageCause::Unknown; MAX_RECTS];
        causes[0] = cause;
        let mut sources = [None; MAX_RECTS];
        sources[0] = source;
        Self {
            rects,
            causes,
            sources,
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
        self.add_rect_with_cause(r, DamageCause::Unknown, None);
    }

    /// Add a rectangle to the damage set with an explicit cause.
    /// Clips to bounds, discards empty rects, merges overlapping/touching rects.
    pub fn add_rect_with_cause(&mut self, r: Rect, cause: DamageCause, source: Option<ThingId>) {
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
            self.collapse_to_full_with_cause(cause, source);
            return;
        }

        // Try to merge with existing rects
        for i in 0..self.count {
            if self.rects[i].touches_or_overlaps(clipped) {
                self.rects[i] = self.rects[i].union(clipped);
                // Keep the more specific cause (prefer non-Unknown)
                if self.causes[i] == DamageCause::Unknown && cause != DamageCause::Unknown {
                    self.causes[i] = cause;
                    self.sources[i] = source;
                }
                // After merge, try to consolidate further
                self.consolidate();
                return;
            }
        }

        // No merge possible, add as new rect
        if self.count < MAX_RECTS {
            self.rects[self.count] = clipped;
            self.causes[self.count] = cause;
            self.sources[self.count] = source;
            self.count += 1;
        } else {
            // Exceeded MAX_RECTS, collapse to full-frame
            self.collapse_to_full_with_cause(cause, source);
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
                        // Keep the more specific cause
                        if self.causes[i] == DamageCause::Unknown && self.causes[j] != DamageCause::Unknown {
                            self.causes[i] = self.causes[j];
                            self.sources[i] = self.sources[j];
                        }
                        // Check if merged rect exceeds threshold (25% of screen)
                        if self.rects[i].area() * 4 > self.bounds.area() {
                            self.collapse_to_full_with_cause(self.causes[i], self.sources[i]);
                            return;
                        }
                        // Remove j by swapping with last
                        self.count -= 1;
                        if j < self.count {
                            self.rects[j] = self.rects[self.count];
                            self.causes[j] = self.causes[self.count];
                            self.sources[j] = self.sources[self.count];
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
        self.collapse_to_full_with_cause(DamageCause::ForceFull, None);
    }

    /// Collapse all damage to a single full-frame rect with a cause.
    fn collapse_to_full_with_cause(&mut self, cause: DamageCause, source: Option<ThingId>) {
        self.rects[0] = self.bounds;
        self.causes[0] = cause;
        self.sources[0] = source;
        self.count = 1;
        self.is_full = true;
    }

    /// Iterate over the damage rectangles.
    pub fn iter(&self) -> impl Iterator<Item = Rect> + '_ {
        self.rects[..self.count].iter().copied()
    }

    /// Iterate over damage records (rect + cause + source).
    pub fn iter_records(&self) -> impl Iterator<Item = DamageRecord> + '_ {
        (0..self.count).map(move |i| DamageRecord {
            rect: self.rects[i],
            cause: self.causes[i],
            source: self.sources[i],
        })
    }

    /// Get the cause for a specific damage rect index.
    pub fn get_cause(&self, index: usize) -> Option<DamageCause> {
        if index < self.count {
            Some(self.causes[index])
        } else {
            None
        }
    }

    /// Get the source for a specific damage rect index.
    pub fn get_source(&self, index: usize) -> Option<ThingId> {
        if index < self.count {
            self.sources[index]
        } else {
            None
        }
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

    /// Note a bounding box with explicit cause and source.
    pub fn note_bbox_with_cause(&mut self, rect: Rect, cause: DamageCause, source: Option<ThingId>) {
        self.damage.add_rect_with_cause(rect, cause, source);
    }

    /// Note cursor movement: damages both old and new cursor positions.
    pub fn note_cursor_move(&mut self, old_bbox: Rect, new_bbox: Rect) {
        // Damage the old position (needs to be redrawn to erase)
        self.damage.add_rect_with_cause(old_bbox, DamageCause::CursorMoved, None);
        // Damage the new position (needs to be drawn)
        self.damage.add_rect_with_cause(new_bbox, DamageCause::CursorMoved, None);
    }

    /// Mark the entire frame as damaged.
    pub fn mark_full(&mut self) {
        self.damage = Damage::full(self.bounds);
    }

    /// Mark the entire frame as damaged with an explicit cause.
    pub fn mark_full_with_cause(&mut self, cause: DamageCause, source: Option<ThingId>) {
        self.damage = Damage::full_with_cause(self.bounds, cause, source);
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
// Damage Journal (Debug-Only)
// ============================================================================

/// Maximum number of frames to keep in the journal.
const JOURNAL_MAX_FRAMES: usize = 60;

/// A record of damage for a single frame.
#[cfg(debug_assertions)]
#[derive(Clone, Debug)]
pub struct FrameDamageRecord {
    pub frame_id: u64,
    pub records: Vec<DamageRecord>,
    pub is_full: bool,
}

/// Rolling damage history for debugging.
///
/// Only compiled in debug builds to avoid overhead in release.
#[cfg(debug_assertions)]
pub struct DamageJournal {
    frames: Vec<FrameDamageRecord>,
    next_frame_id: u64,
}

#[cfg(debug_assertions)]
impl DamageJournal {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            next_frame_id: 1,
        }
    }

    /// Record damage for a frame.
    pub fn record_frame(&mut self, damage: &Damage) {
        let records: Vec<DamageRecord> = damage.iter_records().collect();
        
        let record = FrameDamageRecord {
            frame_id: self.next_frame_id,
            records,
            is_full: damage.is_full,
        };

        self.frames.push(record);

        // Keep only the last JOURNAL_MAX_FRAMES
        if self.frames.len() > JOURNAL_MAX_FRAMES {
            self.frames.remove(0);
        }

        self.next_frame_id = self.next_frame_id.wrapping_add(1);
    }

    /// Get the last N frame records.
    pub fn last_frames(&self, n: usize) -> &[FrameDamageRecord] {
        let start = self.frames.len().saturating_sub(n);
        &self.frames[start..]
    }

    /// Get all recorded frames.
    pub fn all_frames(&self) -> &[FrameDamageRecord] {
        &self.frames
    }

    /// Clear the journal.
    pub fn clear(&mut self) {
        self.frames.clear();
    }
}

#[cfg(debug_assertions)]
impl Default for DamageJournal {
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

        d.add_rect_with_cause(Rect::new(0, 0, 10, 10), DamageCause::GeometryChanged, None);
        assert_eq!(d.rect_count(), 1);

        // Adjacent rect should merge
        d.add_rect_with_cause(Rect::new(10, 0, 10, 10), DamageCause::PaintChanged, None);
        assert_eq!(d.rect_count(), 1);

        // Separate rect should not merge
        d.add_rect_with_cause(Rect::new(50, 50, 10, 10), DamageCause::CursorMoved, None);
        assert_eq!(d.rect_count(), 2);
    }
    
    #[test]
    fn test_damage_causes_preserved() {
        let bounds = Rect::full(100, 100);
        let mut d = Damage::empty(bounds);

        d.add_rect_with_cause(Rect::new(0, 0, 10, 10), DamageCause::GeometryChanged, None);
        d.add_rect_with_cause(Rect::new(50, 50, 10, 10), DamageCause::CursorMoved, None);

        let records: alloc::vec::Vec<_> = d.iter_records().collect();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].cause, DamageCause::GeometryChanged);
        assert_eq!(records[1].cause, DamageCause::CursorMoved);
    }
    
    #[test]
    fn test_damage_full_with_cause() {
        let bounds = Rect::full(100, 100);
        let d = Damage::full_with_cause(bounds, DamageCause::ForceFull, None);
        
        assert!(d.is_full);
        assert_eq!(d.rect_count(), 1);
        let records: alloc::vec::Vec<_> = d.iter_records().collect();
        assert_eq!(records[0].cause, DamageCause::ForceFull);
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
        
        // Check that cursor move causes are recorded
        let records: alloc::vec::Vec<_> = d.iter_records().collect();
        assert!(records.iter().any(|r| r.cause == DamageCause::CursorMoved));
    }
    
    #[test]
    fn test_tracker_with_cause() {
        let mut t = DamageTracker::new();
        t.begin_frame(100, 100);

        let rect = Rect::new(10, 10, 20, 20);
        let source = Some(ThingId::from_u64(42));
        t.note_bbox_with_cause(rect, DamageCause::GeometryChanged, source);

        let d = t.end_frame();
        let records: alloc::vec::Vec<_> = d.iter_records().collect();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].cause, DamageCause::GeometryChanged);
        assert_eq!(records[0].source, source);
    }
    
    #[test]
    fn test_tracker_mark_full_with_cause() {
        let mut t = DamageTracker::new();
        t.begin_frame(100, 100);

        let source = Some(ThingId::from_u64(99));
        t.mark_full_with_cause(DamageCause::ForceFull, source);

        let d = t.end_frame();
        assert!(d.is_full);
        let records: alloc::vec::Vec<_> = d.iter_records().collect();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].cause, DamageCause::ForceFull);
        assert_eq!(records[0].source, source);
    }
    
    #[test]
    fn test_get_cause_accessor() {
        let bounds = Rect::full(100, 100);
        let mut d = Damage::empty(bounds);

        d.add_rect_with_cause(Rect::new(0, 0, 10, 10), DamageCause::GeometryChanged, None);
        d.add_rect_with_cause(Rect::new(50, 50, 10, 10), DamageCause::CursorMoved, None);

        assert_eq!(d.get_cause(0), Some(DamageCause::GeometryChanged));
        assert_eq!(d.get_cause(1), Some(DamageCause::CursorMoved));
        assert_eq!(d.get_cause(2), None); // Out of bounds
    }
    
    #[test]
    fn test_get_source_accessor() {
        let bounds = Rect::full(100, 100);
        let mut d = Damage::empty(bounds);
        
        let source1 = Some(ThingId::from_u64(10));
        let source2 = Some(ThingId::from_u64(20));

        d.add_rect_with_cause(Rect::new(0, 0, 10, 10), DamageCause::GeometryChanged, source1);
        d.add_rect_with_cause(Rect::new(50, 50, 10, 10), DamageCause::CursorMoved, source2);

        assert_eq!(d.get_source(0), source1);
        assert_eq!(d.get_source(1), source2);
        assert_eq!(d.get_source(2), None); // Out of bounds
    }
    
    #[cfg(debug_assertions)]
    #[test]
    fn test_damage_journal_recording() {
        let mut journal = DamageJournal::new();
        let bounds = Rect::full(100, 100);
        
        // Record first frame
        let mut d1 = Damage::empty(bounds);
        d1.add_rect_with_cause(Rect::new(0, 0, 10, 10), DamageCause::GeometryChanged, None);
        journal.record_frame(&d1);
        
        // Record second frame
        let mut d2 = Damage::empty(bounds);
        d2.add_rect_with_cause(Rect::new(20, 20, 10, 10), DamageCause::CursorMoved, None);
        journal.record_frame(&d2);
        
        let frames = journal.all_frames();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].frame_id, 1);
        assert_eq!(frames[1].frame_id, 2);
    }
    
    #[cfg(debug_assertions)]
    #[test]
    fn test_damage_journal_sliding_window() {
        let mut journal = DamageJournal::new();
        let bounds = Rect::full(100, 100);
        
        // Record more than JOURNAL_MAX_FRAMES
        for i in 0..70 {
            let mut d = Damage::empty(bounds);
            d.add_rect_with_cause(Rect::new(i, i, 10, 10), DamageCause::Unknown, None);
            journal.record_frame(&d);
        }
        
        let frames = journal.all_frames();
        assert_eq!(frames.len(), 60); // Should be capped at JOURNAL_MAX_FRAMES
        assert_eq!(frames[0].frame_id, 11); // First 10 frames should be dropped
        assert_eq!(frames[59].frame_id, 70); // Last frame should be 70
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
