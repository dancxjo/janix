extern crate alloc;

use alloc::vec::Vec;
use abi::drawlist::{DrawListBuilder, FillRule, PathVerb, PointF};
use abi::geometry::RectI32Wire;
use abi::schema::keys;
use crate::errors::{Error, Result};
use crate::thing::sys::{bytespace_create, bytespace_write, prop_get, prop_set};
use crate::thing::ThingId;
use abi::errors::Errno;
use abi::ids::HandleId;

/// Helper for creating and updating graph-native drawlists.
///
/// DrawList wraps a ThingId (typically a window or custom UI_DRAWLIST node)
/// and provides convenience methods for publishing drawlist updates via the graph.
///
/// # Example
/// ```no_run
/// use stem::petals::DrawList;
/// use stem::thing::ThingId;
/// use abi::drawlist::{DrawListBuilder, FillRule, PathVerb, PointF};
///
/// let window_id = ThingId::from_u64(42);
/// let mut dl = DrawList::new(window_id);
///
/// // Build a drawlist
/// let mut builder = DrawListBuilder::new();
/// builder.push_save();
/// builder.push_set_clip_rect(0, 0, 100, 100);
/// builder.push_fill_rect(10, 10, 80, 80, 0xff00ff00);
/// builder.push_restore();
///
/// // Publish to graph (updates bytespace and gen counter)
/// dl.publish(builder).expect("publish drawlist");
/// ```
pub struct DrawList {
    /// The ThingId this drawlist is attached to (window, panel, or dedicated drawlist node)
    target: ThingId,
    /// Current generation counter (cached locally, lazily loaded)
    gen: Option<u64>,
}

impl DrawList {
    /// Create a new DrawList for the given target node.
    pub fn new(target: ThingId) -> Self {
        Self { target, gen: None }
    }

    /// Publish a drawlist to the graph.
    ///
    /// This writes the drawlist bytes to a bytespace, updates the UI_DRAWLIST_BYTESPACE
    /// property, and increments the UI_DRAWLIST_GEN counter.
    pub fn publish(&mut self, builder: DrawListBuilder) -> Result<()> {
        let bytes = builder.finish();
        let bs = bytespace_create(bytes.len(), 0, 0).map_err(Error::Errno)?;
        bytespace_write(bs, 0, &bytes).map_err(Error::Errno)?;
        prop_set(
            self.target,
            keys::UI_DRAWLIST_BYTESPACE,
            bs.to_u64_lossy(),
        )
        .map_err(Error::Errno)?;

        let next_gen = self.next_gen();
        prop_set(self.target, keys::UI_DRAWLIST_GEN, next_gen).map_err(Error::Errno)?;
        self.gen = Some(next_gen);
        Ok(())
    }

    /// Set the owner property (optional).
    pub fn set_owner(&self, owner: ThingId) -> Result<()> {
        prop_set(
            self.target,
            keys::UI_DRAWLIST_OWNER,
            owner.to_u64_lossy(),
        )
        .map_err(Error::Errno)
    }

    /// Set the bounds property (optional).
    pub fn set_bounds(&self, x: i32, y: i32, w: i32, h: i32) -> Result<()> {
        let rect = RectI32Wire::new(x, y, w, h);
        let bytes = rect.as_bytes();
        let bs = bytespace_create(bytes.len(), 0, 0).map_err(Error::Errno)?;
        bytespace_write(bs, 0, &bytes).map_err(Error::Errno)?;
        prop_set(
            self.target,
            keys::UI_DRAWLIST_BOUNDS,
            bs.to_u64_lossy(),
        )
        .map_err(Error::Errno)
    }

    /// Set the debug name property (optional).
    pub fn set_debug_name(&self, name: &str) -> Result<()> {
        if name.is_empty() {
            prop_set(self.target, keys::UI_DRAWLIST_DEBUG_NAME, 0).map_err(Error::Errno)?;
            return Ok(());
        }
        let bs = bytespace_create(name.len(), 0, 0).map_err(Error::Errno)?;
        bytespace_write(bs, 0, name.as_bytes()).map_err(Error::Errno)?;
        prop_set(
            self.target,
            keys::UI_DRAWLIST_DEBUG_NAME,
            bs.to_u64_lossy(),
        )
        .map_err(Error::Errno)
    }

    /// Get the current generation counter from the graph, or 0 if not set.
    fn current_gen(&self) -> u64 {
        prop_get(self.target, keys::UI_DRAWLIST_GEN).unwrap_or(0)
    }

    /// Calculate the next generation counter.
    fn next_gen(&mut self) -> u64 {
        if let Some(cached) = self.gen {
            cached.saturating_add(1)
        } else {
            let current = self.current_gen();
            current.saturating_add(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::ids::HandleId;

    #[test]
    fn drawlist_manages_gen_counter() {
        // This test validates the generation counter logic without syscalls.
        // In a real environment, prop_get/prop_set would interact with the kernel.
        let target = ThingId::from_u64(123);
        let mut dl = DrawList::new(target);

        // First publish should use gen 1 (assuming current_gen returns 0)
        let first_gen = dl.next_gen();
        assert_eq!(first_gen, 1);
        dl.gen = Some(first_gen);

        // Second publish should use gen 2
        let second_gen = dl.next_gen();
        assert_eq!(second_gen, 2);
    }
}
