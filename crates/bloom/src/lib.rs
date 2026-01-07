#![no_std]

extern crate alloc;

pub mod app;
pub mod assets;
pub mod backend;
pub mod input;
pub mod layout;
pub mod mailbox;
pub mod nine_slice;
pub mod painter;
pub mod pixels;
pub mod scene;
pub mod scene_cache;
pub mod shadow;
pub mod shadow_cache;
pub mod text;
pub mod ui;
pub mod draw_cmd;
pub mod command_recorder;
pub mod executor;
pub mod watch;
pub mod cursor_manager;
pub mod wallpaper_worker;
pub mod boot_fade;
pub mod input_worker;
pub mod cursor_overlay;
pub mod chunked_executor;

pub use app::run;
#[cfg(test)]
mod wallpaper_tests;
