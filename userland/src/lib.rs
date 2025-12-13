#![cfg_attr(target_os = "none", no_std)]

extern crate alloc;

// This crate is now a thin compatibility shim. The real implementation
// of `userland` modules lives in the `thing_os` crate under
// `thing_os::userland`. Re-export the public modules expected by
// existing callers so they don't need to change their imports.

pub use thing_os::thread_info;

pub mod prelude {
    pub use thing_os::userland::prelude::*;
}

pub mod ui {
    pub use thing_os::userland::ui::*;
}
