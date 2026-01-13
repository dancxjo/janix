//! Frame transaction types for the Bloom compositor.
//!
//! This module provides the transactional API for frame composition:
//! - `FrameToken`: Exclusive ownership of a frame slot
//! - `FrameBuilder`: Records ops and damage while holding a token
//! - `FrameSpec`: Frame configuration for acquisition
//! - `PresentStats`: Statistics returned from presentation
//! - `AssetGeneration`: Monotonically increasing asset version

use crate::damage::{Damage, Rect};
use crate::drawlist::{DrawCmd, DrawList};

/// Asset generation snapshot - monotonically increasing.
/// Assets with generation > snapshot are invisible in that frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AssetGeneration(pub u64);

impl AssetGeneration {
    pub const ZERO: Self = Self(0);
    
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// Frame specification for acquiring a frame slot.
#[derive(Clone, Debug)]
pub struct FrameSpec {
    pub width: u32,
    pub height: u32,
    pub format: u32,
}

impl FrameSpec {
    pub fn new(width: u32, height: u32, format: u32) -> Self {
        Self { width, height, format }
    }
}

/// Statistics from a present operation.
#[derive(Clone, Debug, Default)]
pub struct PresentStats {
    pub frame_id: u64,
    pub asset_gen: AssetGeneration,
    pub ops_count: usize,
    pub damage_rect_count: usize,
    pub fast_path_taken: bool,
}

/// Token granting exclusive ownership of a frame slot.
/// 
/// # Invariants
/// - Cannot be cloned (exclusive ownership)
/// - Must be consumed by `present_frame()` or dropped
/// - Created only by `Presenter::acquire_frame()`
pub struct FrameToken {
    pub(crate) frame_id: u64,
    pub(crate) asset_gen: AssetGeneration,
    pub(crate) spec: FrameSpec,
    pub(crate) damage: Damage,
    pub(crate) ops: DrawList,
}

impl FrameToken {
    /// Create a new frame token (called by presenter)
    pub(crate) fn new(frame_id: u64, asset_gen: AssetGeneration, spec: FrameSpec) -> Self {
        let bounds = Rect::full(spec.width as i32, spec.height as i32);
        Self {
            frame_id,
            asset_gen,
            spec,
            damage: Damage::empty(bounds),
            ops: DrawList::new(),
        }
    }

    /// Read-only access to the asset generation snapshot
    #[inline]
    pub fn asset_generation(&self) -> AssetGeneration {
        self.asset_gen
    }

    /// Read-only access to the frame id
    #[inline]
    pub fn frame_id(&self) -> u64 {
        self.frame_id
    }

    /// Read-only access to the spec
    #[inline]
    pub fn spec(&self) -> &FrameSpec {
        &self.spec
    }

    #[cfg(debug_assertions)]
    pub fn assert_valid(&self) {
        debug_assert!(self.frame_id > 0, "Invalid frame token: frame_id is 0");
    }
}

/// Builder for recording ops into a frame.
///
/// # Invariants
/// - Exists only while holding a `FrameToken`
/// - `finish()` consumes self and returns the sealed token
/// - Cannot be cloned
pub struct FrameBuilder {
    token: FrameToken,
    finished: bool,
}

impl FrameBuilder {
    /// Create a new builder from a token.
    pub fn new(token: FrameToken) -> Self {
        Self { token, finished: false }
    }

    /// Push a draw command to the frame's op list.
    #[inline]
    pub fn push_op(&mut self, cmd: DrawCmd) {
        debug_assert!(!self.finished, "Cannot push ops after finish()");
        self.token.ops.commands().push(cmd);
    }

    /// Add a damage rectangle.
    #[inline]
    pub fn add_damage(&mut self, rect: Rect) {
        debug_assert!(!self.finished, "Cannot add damage after finish()");
        self.token.damage.add_rect(rect);
    }

    /// Mark entire frame as damaged.
    #[inline]
    pub fn mark_full_damage(&mut self) {
        debug_assert!(!self.finished, "Cannot mark damage after finish()");
        let bounds = Rect::full(
            self.token.spec.width as i32,
            self.token.spec.height as i32,
        );
        self.token.damage = Damage::full(bounds);
    }

    /// Get the asset generation snapshot (read-only).
    #[inline]
    pub fn asset_generation(&self) -> AssetGeneration {
        self.token.asset_gen
    }

    /// Get the frame spec (read-only).
    #[inline]
    pub fn spec(&self) -> &FrameSpec {
        &self.token.spec
    }

    /// Get mutable access to the draw list.
    #[inline]
    pub fn ops(&mut self) -> &mut DrawList {
        debug_assert!(!self.finished, "Cannot access ops after finish()");
        &mut self.token.ops
    }

    /// Get mutable access to the damage tracker.
    #[inline]
    pub fn damage(&mut self) -> &mut Damage {
        debug_assert!(!self.finished, "Cannot access damage after finish()");
        &mut self.token.damage
    }

    /// Seal the ops/damage and return the token for presentation.
    /// Consumes the builder.
    pub fn finish(mut self) -> FrameToken {
        debug_assert!(!self.finished, "Cannot finish() twice");
        self.finished = true;
        self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_generation_ordering() {
        let g0 = AssetGeneration(0);
        let g1 = AssetGeneration(1);
        let g2 = AssetGeneration(2);
        
        assert!(g0 < g1);
        assert!(g1 < g2);
        assert!(g0 <= g0);
        assert_eq!(g1.next(), g2);
    }

    #[test]
    fn test_frame_token_creation() {
        let spec = FrameSpec::new(800, 600, 0);
        let token = FrameToken::new(1, AssetGeneration(5), spec);
        
        assert_eq!(token.frame_id(), 1);
        assert_eq!(token.asset_generation(), AssetGeneration(5));
    }

    #[test]
    fn test_builder_lifecycle() {
        let spec = FrameSpec::new(100, 100, 0);
        let token = FrameToken::new(1, AssetGeneration(0), spec);
        
        let mut builder = FrameBuilder::new(token);
        builder.add_damage(Rect::new(10, 10, 20, 20));
        
        // Should be able to finish
        let token = builder.finish();
        assert_eq!(token.frame_id(), 1);
    }
}
