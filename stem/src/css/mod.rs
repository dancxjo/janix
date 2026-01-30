//! CSS parsing and graph ingestion for ThingOS.
//!
//! Provides CSS parsing that converts stylesheets into navigable graph trees.

pub mod ingest;
pub mod model;

pub use ingest::{ingest_css_to_graph, CssIngestError, CssIngestOptions, CssIngestResult};
