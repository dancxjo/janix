#![no_std]

extern crate alloc;

pub mod symbol;
pub mod value;
pub mod thing;
pub mod link;
pub mod kind;
pub mod schema;

pub use symbol::*;
pub use value::*;
pub use thing::*;
pub use link::*;
pub use kind::*;
pub use schema::*;

pub mod milestones {
    pub const KERNEL_ENTRY: &str = "THINGOS: kernel entry";
    pub const BRIDGE_ONLINE: &str = "THINGOS: bridge online";
    pub const IDLE_LOOP: &str = "THINGOS: idle loop";
}
