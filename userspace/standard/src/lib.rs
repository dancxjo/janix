#![no_std]

pub mod console;
pub mod macros;
pub mod prelude;
pub mod time;

// Re-export common macros
pub use macros::*;
