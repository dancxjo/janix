#![allow(dead_code)]

/// Marks a string for localization extraction.
pub const fn mark(text: &'static str) -> &'static str {
    text
}
