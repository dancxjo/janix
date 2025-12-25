#![no_std]

pub mod ids;
pub mod memory;

pub use ids::{GraphId, ProcessId, ThingId, ThreadId, SymbolId};
pub use memory::*;

pub type SysRet = i64;
