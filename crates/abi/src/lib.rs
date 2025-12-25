#![no_std]

extern crate alloc;

pub mod ids;
pub mod memory;
pub mod wire;

pub mod symbols;
pub use ids::{GraphId, ProcessId, ThingId, ThreadId};
pub use symbols::SymbolId;
pub use memory::*;

pub mod syscall_defs;
pub use syscall_defs::*;

pub type SysRet = i64;

