//! SVG parsing and rendering pipeline for Bloom.

pub mod compile;
pub mod ir;
pub mod parse;
pub mod walk;

mod attributes;
mod geometry;
mod parser;
mod render;
mod shapes;
mod state;

pub use parser::SvgParser;
pub use render::render_to_buffer;

#[cfg(test)]
mod tests;
