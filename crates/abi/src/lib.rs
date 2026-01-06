#![no_std]

extern crate alloc;

pub mod bodies;
pub mod cap; // Capabilities
pub mod cpu; // CPU capabilities
pub mod display;
pub mod events; // EventStream wire format
pub mod ids;
pub mod machine;
pub mod mouse_ring;
pub mod syscall;
pub mod types;
pub mod draw_cmd;
pub mod wire;
pub mod ui;
