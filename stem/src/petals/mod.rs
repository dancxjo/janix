//! # Petals - UI Intent Builder
//!
//! Petals provides a graph-native API for constructing UI intent.
//! Applications write UI nodes and relationships directly into the system graph.
//!
//! ## Architecture
//!
//! - **Apps** use Petals to build UI intent (graph nodes + edges)
//! - **Blossom** reads intent from the graph and performs layout/paint
//! - **Bloom** composites the final rendered output
//!
//! See `docs/UI_INTENT_CONTRACT.md` for the full architectural contract.
//!
//! ## Example
//!
//! ```rust,ignore
//! use stem::petals::Petals;
//!
//! let mut ui = Petals::begin_window(window_id);
//! let root = ui.column(|ui| {
//!     ui.text("Hello, World!")?;
//!     Ok(())
//! })?;
//! ui.set_gap(root, 8)?;
//! ui.set_padding(root, 16)?;
//!
//! ui.finish()?;
//! ```

extern crate alloc;

pub mod drawlist;
pub mod graph;
pub mod viewport;

pub use drawlist::DrawList;
pub use graph::{reduce_window_events_with_graph, Petals, UiKey, UiTreeBuilder};
pub use viewport::{PanZoomController, Viewport, ViewportConstraints, ViewportIntent};
