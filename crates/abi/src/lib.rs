#![no_std]

extern crate alloc;

pub mod ids;
pub mod memory;
pub mod wire;
pub mod boot;

pub mod symbols;
pub use ids::{GraphId, ProcessId, ThingId, ThreadId};
pub use memory::*;
pub use symbols::SymbolId;
pub use boot::*;

pub mod syscall_defs;
pub use syscall_defs::*;

pub type SysRet = i64;
