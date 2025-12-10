#![no_std]
extern crate alloc;

mod config;
mod fonts;
mod graph;
mod input;
mod layout;
mod model;
mod render;
mod state;

pub use state::run;
