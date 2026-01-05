#![no_std]
extern crate alloc;

pub mod app;
pub mod backend;
pub mod input;
pub mod pixels;
pub mod scene;
pub mod assets;
// cursor is now under assets

// Other modules will be added later

pub fn run() {
    app::run();
}
