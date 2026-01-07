#![no_std]
//! Theme selector - re-exports the active theme as `current`

pub use nebulous as current;

// TODO: This is busted and shows lots of issues
// The boot.rs should consult the theme to get the right colors
// pub use without_a_cause as current;