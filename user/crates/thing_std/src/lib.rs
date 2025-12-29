#![no_std]
extern crate alloc;

pub mod sys;
pub mod syscalls;
pub mod debug;
pub mod graph;
pub mod console;
pub mod rt;
pub mod time;

pub fn init() {}

pub use graph::{GraphClient, GraphError};
pub use console::{Console, StdoutConsole};
