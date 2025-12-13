use runtime::Sys;

use crate::syscalls::list_things_by_kind;
use thing_models::{Mode, MODE_INDEX_CONSOLE};

/// Return the currently active `Mode` Thing, if one is marked active.
pub fn active_mode<S: Sys>(sys: &mut S) -> Option<Mode> {
    list_things_by_kind::<S, Mode>(sys)
        .into_iter()
        .find(|m| m.active)
}

/// Fallback when no mode is active yet: choose the lowest index mode.
pub fn default_mode<S: Sys>(sys: &mut S) -> Option<Mode> {
    list_things_by_kind::<S, Mode>(sys)
        .into_iter()
        .min_by_key(|m| m.index)
}

/// Convenience guard for deciding if the framebuffer console should own the screen.
pub fn is_console_mode_active<S: Sys>(sys: &mut S) -> bool {
    active_mode(sys)
        .map(|m| m.index == MODE_INDEX_CONSOLE)
        .unwrap_or(false)
}
