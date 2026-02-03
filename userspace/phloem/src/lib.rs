//! phloem: Graph Query Language (GQL) library for Thing-OS
//!
//! Provides parsing and execution of OpenGQL-subset queries against the system graph.

#![no_std]

extern crate alloc;

pub mod gql;
pub mod executor;

pub use gql::{Command, Pattern, Value, NodePattern, parse};
pub use executor::GraphExecutor;
