#![no_std]
extern crate alloc;
extern crate self as abi;

pub mod debug;
pub mod device;
pub mod display_driver_protocol;
pub mod display_protocol;
pub mod driver_ctx;
pub mod errors;
pub mod ids;
pub mod kinds;
pub mod module;
pub mod module_manifest;
pub mod names;
pub mod query;
pub mod root;
pub mod schema;
pub mod symbols;
pub mod syscall;
pub mod trace;
pub mod types;
pub mod vm;

pub mod font;
pub mod hid;
pub mod logging;

pub mod graphable;
pub mod macros;
pub mod packed;
pub mod thing;
pub mod wire;
pub mod wire_schema;

pub use abi_macros::Graphable;
pub use errors::{Error, Result};
pub use graphable::Graphable;
pub use thing::Thing;
pub use wire::{BlobId, KindId, PredicateId, SymbolId, ThingId, WireSafe};
pub use wire_schema::{Field, Schema, WireType};
