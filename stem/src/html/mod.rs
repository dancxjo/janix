//! HTML parsing and graph ingestion for ThingOS.
//!
//! Provides tolerant HTML parsing that converts HTML documents into
//! navigable graph trees, similar to the XML module.

pub mod ingest;
pub mod model;

pub use ingest::{ingest_html_to_graph, HtmlIngestError, HtmlIngestOptions, HtmlIngestResult};
