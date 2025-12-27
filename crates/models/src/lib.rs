#![no_std]

extern crate alloc;
pub extern crate abi;

pub mod symbol;
pub mod value;
pub mod thing;
pub mod font;
pub mod link;
pub mod kind;
pub mod schema;
pub mod declare;
pub mod intent;
pub mod observation;
pub mod result;
pub mod diag;
pub mod prelude;
pub mod macros;
pub mod thing_builders;

pub use kind::*;
pub use link::*;
pub use schema::*;
pub use symbol::*;
pub use thing::*;
pub use value::*;
pub use intent::*;
pub use observation::*;
pub use result::*;

pub mod edge;
pub mod core;
pub mod typed;

pub mod builtins;

pub mod milestones {
    pub const KERNEL_ENTRY: &str = "THINGOS: kernel entry";
    pub const BRIDGE_ONLINE: &str = "THINGOS: bridge online";
    pub const IDLE_LOOP: &str = "THINGOS: idle loop";
}
