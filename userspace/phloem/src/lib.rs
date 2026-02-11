//! phloem: Graph Query Language (GQL) library for Thing-OS
//!
//! Provides parsing and execution of OpenGQL-subset queries against the system graph.

#![no_std]

extern crate alloc;

pub mod executor;
pub mod gql;
#[cfg(test)]
pub mod query_tests;

use crate::alloc::string::ToString;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

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

pub use executor::GraphExecutor;
pub use gql::{parse, Command, NodePattern, Pattern, ReturnExpression, Value};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result_constructors() {
        let res = ExecutionResult::success("test success");
        assert!(res.success);
        assert_eq!(res.message, "test success");
        assert!(res.columns.is_empty());

        let res = ExecutionResult::error("test error");
        assert!(!res.success);
        assert_eq!(res.message, "test error");

        let res = ExecutionResult::message("hello");
        assert!(res.success);
        assert_eq!(res.message, "hello");

        let cols = alloc::vec!["a".to_string(), "b".to_string()];
        let rows = alloc::vec![alloc::vec![
            ResultValue::Number(1),
            ResultValue::String("x".to_string())
        ]];
        let res = ExecutionResult::rows(cols, rows);
        assert!(res.success);
        assert_eq!(res.columns.len(), 2);
        assert_eq!(res.rows.len(), 1);
        assert_eq!(res.message, "ok: 1 rows");
    }
}
