//! # Bloom - ThingOS Compositor
//!
//! ## Rendering Architecture: Record-Then-Execute
//!
//! Bloom uses a **fully record-then-execute rendering pipeline** for all scene content:
//!
//! ### Frame Pipeline
//!
//! ```text
//! Graph Change → Record DrawCmds → Execute DrawCmds → Present → Overlay Cursor → Sleep
//! ```
//!
//! #### 1. Record Phase (command_recorder.rs)
//! - Walks window scenes from graph
//! - Produces `Vec<DrawCmd>` - no pixel writes
//! - All drawing operations become commands
//!
//! #### 2. Execute Phase (executor.rs, chunked_executor.rs)
//! - Single tight loop processing commands
//! - Only code path that writes pixels to scene buffer
//! - Chunked execution allows cursor updates between chunks
//!
//! #### 3. Compositing (app.rs)
//! - Copy scene_buffer → frame_buffer
//! - No additional drawing
//!
//! #### 4. Cursor Overlay (cursor_overlay.rs)
//! - **THE ONLY IMMEDIATE-MODE EXCEPTION**
//! - Must be immediate for responsiveness during long scene rebuilds
//! - Draws cursor sprite directly to framebuffer
//! - Does not invalidate scene command list
//!
//! ### Why This Architecture?
//!
//! - **Predictable**: All rendering flows through one execution path
//! - **Responsive**: Cursor updates don't wait for scene rebuild
//! - **Damage-trackable**: Command bounds enable efficient dirty regions
//! - **Testable**: Commands can be inspected/validated before execution
//!
//! ### What NOT to do
//!
//! ❌ Don't write pixels during graph traversal  
//! ❌ Don't add immediate-mode drawing in UI rendering  
//! ❌ Don't bypass the command list (except cursor)  
//! ✅ Do record commands in `CommandRecorder`  
//! ✅ Do execute commands in `execute_cmds_into_scene`  
//! ✅ Do use `CursorOverlay` for cursor rendering  

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
pub mod interaction;
pub mod chunked_executor;
pub mod profile;
pub mod present_loop;

pub use app::run;
#[cfg(test)]
mod wallpaper_tests;
#[cfg(test)]
mod cursor_overlay_tests;
pub mod text_pipeline;
pub mod textd_glyph;
