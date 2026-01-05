#![no_std]

extern crate alloc;

pub mod app;
pub mod assets;
pub mod backend;
pub mod input;
pub mod nine_slice;
pub mod pixels;
pub mod scene;
pub mod shadow;
pub mod ui;

pub use app::run;
