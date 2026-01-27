#![no_std]
extern crate alloc;
extern crate self as abi;

pub mod debug;
pub mod device;
pub mod display_driver_protocol;
pub mod display_protocol;
pub mod driver_ctx;
pub mod errors;
pub mod geometry;
pub mod kinds;
pub mod module;
pub mod module_manifest;
pub mod names;
pub mod query;
pub mod trace;
pub mod drawlist;
pub mod ui_scene;
pub mod ui_paint;
pub mod schema;
pub mod symbols;
pub mod syscall;
pub mod types;
pub mod root;
pub mod ids;
pub mod vm;
pub mod watch;

pub mod logging;
pub mod hid;
pub mod font;
pub mod font_protocol;
pub mod svg_protocol;
pub mod pixel;

pub mod wire;
pub mod packed;
pub mod wire_schema;
pub mod graphable;
pub mod thing;
pub mod macros;

pub use thing::Thing;
pub use graphable::Graphable;
pub use abi_macros::Graphable;
pub use errors::{Error, Result};
pub use wire::{ThingId, BlobId, SymbolId, KindId, PredicateId, WireSafe};
pub use wire_schema::{Schema, Field, WireType};
