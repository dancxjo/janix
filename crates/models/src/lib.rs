#![no_std]

pub extern crate abi;
extern crate alloc;

pub mod declare;
pub mod diag;
pub mod font;
pub mod intent;
pub mod kind;
pub mod link;
pub mod macros;
pub mod observation;
pub mod prelude;
pub mod result;
pub mod schema;
pub mod symbol;
pub mod font_cache;
pub mod thing;
pub mod thing_builders;
pub mod value;

pub use intent::*;
pub use kind::*;
pub use link::*;
pub use observation::*;
pub use result::*;
pub use schema::*;
pub use symbol::*;
pub use thing::*;
pub use value::*;

pub mod core;
pub mod edge;
pub mod typed;

pub mod builtins;

pub mod milestones {
    pub const KERNEL_ENTRY: &str = "THINGOS: kernel entry";
    pub const BRIDGE_ONLINE: &str = "THINGOS: bridge online";
    pub const IDLE_LOOP: &str = "·";
}
