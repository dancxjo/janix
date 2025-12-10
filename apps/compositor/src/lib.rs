#![no_std]
extern crate alloc;

#[cfg(test)]
extern crate std;

mod config;
mod fonts;
mod graph;
mod input;
mod layout;
mod model;
mod render;
mod state;

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod mock_tests;

pub use state::run;
