//! Wire types only; repr(C) where applicable; no Vec; no traits; no borrowed refs.
//!
//! This module defines the raw wire-compatible types used for syscalls.
//! These types must be stable, layout-compatible with C, and use single-source-of-truth definitions.

pub mod buffers;
pub mod common;
pub mod dev;
pub mod events;
pub mod graph;
pub mod memory;
pub mod process;
pub mod resident;
