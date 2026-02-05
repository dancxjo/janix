//! Stamen: Cursor-Only Overlay Daemon
//!
//! A tiny, purpose-built daemon for "butter smooth" cursor rendering.
//! Receives mouse events from Bristle and renders cursor overlay
//! independently of Bloom's compositing cadence.

#![no_std]

extern crate alloc;

pub mod cursor;
pub mod sprite;
