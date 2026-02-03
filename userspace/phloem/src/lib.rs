//! phloem: Graph Query Language (GQL) library for Thing-OS
//!
//! Provides parsing and execution of OpenGQL-subset queries against the system graph.

#![no_std]

extern crate alloc;

pub mod gql;
pub mod executor;

use alloc::string::String;
use crate::alloc::string::ToString;
use alloc::vec::Vec;
use alloc::format;

#[derive(Debug, Clone)]
pub enum ResultValue {
    Node(u64),
    String(String),
    Number(u64),
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub message: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<ResultValue>>,
}

impl ExecutionResult {
    pub fn success(msg: &str) -> Self {
        Self {
            success: true,
            message: msg.to_string(),
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            message: msg.to_string(),
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn message(msg: &str) -> Self {
        Self {
            success: true,
            message: msg.to_string(),
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn rows(columns: Vec<String>, rows: Vec<Vec<ResultValue>>) -> Self {
        Self {
            success: true,
            message: format!("ok: {} rows", rows.len()),
            columns,
            rows,
        }
    }
}

pub use gql::{Command, Pattern, Value, NodePattern, parse};
pub use executor::GraphExecutor;
