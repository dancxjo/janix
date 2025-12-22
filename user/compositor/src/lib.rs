#![no_std]
extern crate alloc;

#[cfg(test)]
extern crate std;

mod config;
pub mod flex;
mod fonts;
mod graph;
mod input;
mod layout;
mod model;
mod render;
mod state;
pub mod widget_layout;
pub mod widgets;

// #[cfg(test)]
// mod mock_tests;
// #[cfg(test)]
// mod test_support;

pub use state::main;

// Re-export the active_framebuffer selection seam so callers can obtain
// the graph-driven primary display buffer without reaching into graph.rs
// directly. This keeps the "which buffer?" decision isolated here.
pub use crate::graph::active_framebuffer;
