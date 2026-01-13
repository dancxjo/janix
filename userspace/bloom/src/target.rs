//! Hardware-abstracted compositor target
//!
//! Bloom submits DrawLists to a target. The target
//! decides HOW to render them based on available hardware.

use crate::damage::Damage;
use crate::drawlist::DrawList;

/// The contract between Bloom and any rendering backend
pub trait CompositorTarget {
    /// Submit a frame's worth of drawing commands
    fn submit(&mut self, drawlist: &DrawList);

    /// Present the rendered frame to the display
    fn present(&mut self, damage: &Damage);
}
