//! Graph API adapter for read-only introspection
//!
//! Provides JSON representation of graph nodes and their properties.

#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::format;
use alloc::string::String;
use stem::syscall::graph::{find, get_kind, prop_get};
use stem::syscall::root_bytespace_read;

const MAX_BYTESPACE_SIZE: usize = 1024 * 1024; // 1MB limit for safety

#[derive(Debug)]
pub enum GraphError {
    NotFound,
    InvalidId,
    SyscallFailed,
    TooBig,
}

/// Simple JSON builder (avoiding allocations where possible)
pub struct JsonBuilder {
    pub(crate) buf: Vec<u8>,
}

impl JsonBuilder {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn start_object(&mut self) {
        self.buf.push(b'{');
    }

    pub fn end_object(&mut self) {
        if self.buf.last() == Some(&b',') {
            self.buf.pop();
        }
        self.buf.push(b'}');
    }

    pub fn start_array(&mut self) {
        self.buf.push(b'[');
    }

    pub fn end_array(&mut self) {
        if self.buf.last() == Some(&b',') {
            self.buf.pop();
        }
        self.buf.push(b']');
    }

    pub fn key(&mut self, k: &str) {
        self.buf.push(b'"');
        self.buf.extend_from_slice(k.as_bytes());
        self.buf.extend_from_slice(b"\":");
    }

    pub fn string_value(&mut self, v: &str) {
        self.buf.push(b'"');
        // Escape special JSON characters
        for ch in v.bytes() {
            match ch {
                b'"' => self.buf.extend_from_slice(b"\\\""),
                b'\\' => self.buf.extend_from_slice(b"\\\\"),
                b'\n' => self.buf.extend_from_slice(b"\\n"),
                b'\r' => self.buf.extend_from_slice(b"\\r"),
                b'\t' => self.buf.extend_from_slice(b"\\t"),
                ch if ch < 32 => {
                    // Escape control characters as \uXXXX
                    let hex = alloc::format!("\\u{:04x}", ch);
                    self.buf.extend_from_slice(hex.as_bytes());
                }
                _ => self.buf.push(ch),
            }
        }
        self.buf.push(b'"');
        self.buf.push(b',');
    }

    pub fn number_value(&mut self, v: u64) {
        // Format the number manually to avoid needing Write trait
        let s = alloc::format!("{}", v);
        self.buf.extend_from_slice(s.as_bytes());
        self.buf.push(b',');
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn as_string(&self) -> Result<String, core::str::Utf8Error> {
        Ok(String::from(core::str::from_utf8(&self.buf)?))
    }
}

/// Get JSON representation of a Thing by ID
pub fn thing_to_json(thing_id: u64) -> Result<String, GraphError> {
    // Get the kind
    let kind_id = get_kind(thing_id).map_err(|_| GraphError::NotFound)?;
    if kind_id == 0 {
        return Err(GraphError::NotFound);
    }

    let mut json = JsonBuilder::new();
    json.start_object();
    
    json.key("thing_id");
    json.number_value(thing_id);
    
    json.key("kind_id");
    json.number_value(kind_id);
    
    // TODO: Add properties and links when we have a way to enumerate them
    // For now, just return the basic info
    
    json.key("props");
    json.start_object();
    json.end_object();
    json.buf.push(b',');
    
    json.key("links");
    json.start_array();
    json.end_array();
    
    json.end_object();
    
    json.as_string().map_err(|_| GraphError::InvalidId)
}

/// List all Things of a given kind
pub fn list_things_by_kind(kind_id: u64, limit: usize) -> Result<String, GraphError> {
    let mut ids = Vec::new();
    ids.resize(limit, 0u64);
    
    let count = find(kind_id, &mut ids).map_err(|_| GraphError::SyscallFailed)?;
    
    let mut json = JsonBuilder::new();
    json.start_object();
    
    json.key("kind_id");
    json.number_value(kind_id);
    
    json.key("count");
    json.number_value(count as u64);
    
    json.key("things");
    json.start_array();
    
    for i in 0..count.min(limit) {
        json.number_value(ids[i]);
    }
    
    json.end_array();
    
    json.end_object();
    
    json.as_string().map_err(|_| GraphError::InvalidId)
}

/// Get a property value
pub fn get_property(thing_id: u64, key_id: u64) -> Result<u64, GraphError> {
    prop_get(thing_id, key_id).map_err(|_| GraphError::NotFound)
}

/// Read bytespace data with safety limits
pub fn read_bytespace(bytespace_id: u64, max_size: usize) -> Result<Vec<u8>, GraphError> {
    let limit = max_size.min(MAX_BYTESPACE_SIZE);
    let mut buffer = Vec::new();
    buffer.resize(limit, 0);
    
    match root_bytespace_read(bytespace_id as usize, 0, &mut buffer) {
        Ok(n) => {
            buffer.truncate(n);
            Ok(buffer)
        }
        Err(_) => Err(GraphError::SyscallFailed),
    }
}

/// Read bytespace data with offset and limit (for Range header support)
pub fn read_bytespace_ranged(bytespace_id: u64, offset: usize, max_len: usize) -> Result<Vec<u8>, GraphError> {
    let limit = max_len.min(MAX_BYTESPACE_SIZE);
    let mut buffer = Vec::new();
    buffer.resize(limit, 0);
    
    match root_bytespace_read(bytespace_id as usize, offset, &mut buffer) {
        Ok(n) => {
            buffer.truncate(n);
            Ok(buffer)
        }
        Err(_) => Err(GraphError::SyscallFailed),
    }
}

/// Bytespace metadata
pub struct BytespaceMeta {
    pub size: usize,
}

/// Get bytespace metadata
pub fn bytespace_meta(bytespace_id: u64) -> Result<BytespaceMeta, GraphError> {
    use abi::ids::HandleId;
    use stem::thing::sys::bytespace_info;
    use stem::thing::ThingId;
    
    let thing_id = ThingId::from_u64(bytespace_id);
    match bytespace_info(thing_id) {
        Ok(size) => Ok(BytespaceMeta { size }),
        Err(_) => Err(GraphError::SyscallFailed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_builder_simple_object() {
        let mut json = JsonBuilder::new();
        json.start_object();
        json.key("name");
        json.string_value("test");
        json.key("value");
        json.number_value(42);
        json.end_object();
        
        let result = json.as_string().unwrap();
        assert!(result.contains("\"name\":\"test\""));
        assert!(result.contains("\"value\":42"));
    }

    #[test]
    fn test_json_builder_array() {
        let mut json = JsonBuilder::new();
        json.start_array();
        json.number_value(1);
        json.number_value(2);
        json.number_value(3);
        json.end_array();
        
        let result = json.as_string().unwrap();
        assert_eq!(result, "[1,2,3]");
    }

    #[test]
    fn test_json_builder_nested() {
        let mut json = JsonBuilder::new();
        json.start_object();
        json.key("items");
        json.start_array();
        json.number_value(1);
        json.number_value(2);
        json.end_array();
        json.end_object();
        
        let result = json.as_string().unwrap();
        assert!(result.contains("\"items\":[1,2]"));
    }

    #[test]
    fn test_json_string_escaping() {
        let mut json = JsonBuilder::new();
        json.start_object();
        json.key("text");
        json.string_value("Hello \"world\"\nNew line");
        json.end_object();
        
        let result = json.as_string().unwrap();
        assert!(result.contains("\\\""));
        assert!(result.contains("\\n"));
    }

    #[test]
    fn test_json_control_char_escaping() {
        let mut json = JsonBuilder::new();
        json.start_object();
        json.key("ctrl");
        json.string_value("test\x01\x02");
        json.end_object();
        
        let result = json.as_string().unwrap();
        assert!(result.contains("\\u"));
    }
}
