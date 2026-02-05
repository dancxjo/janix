#![no_std]

#[cfg(test)]
extern crate std;

pub mod display;
pub mod font;
pub mod framebuffer;
pub mod parser;
pub mod theme_api;

// Wasm-based boot animation support
pub mod boot_state;
pub mod draw_commands;
#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(test)]
pub mod display_repro;
