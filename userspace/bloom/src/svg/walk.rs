//! SVG graph walker that delegates to the generic XmlGraph adapter.
//!
//! This module provides the `SvgGraph` trait for abstracting graph reading
//! (allowing test mocks) and `SysSvgGraph` which is the production implementation
//! that delegates to `stem::xml::graph::XmlGraph`.

use abi::schema::keys;
use abi::wire::ThingId;
use alloc::string::String;
use alloc::vec::Vec;
use stem::thing::sys;
use stem::xml::graph::XmlGraph;

/// Trait to abstract graph reading so we can test compiler without syscalls
pub trait SvgGraph {
    fn get_root(&self, doc: ThingId) -> Option<ThingId>;
    fn get_children(&self, elem: ThingId) -> Vec<ThingId>;
    fn get_prop_str(&self, node: ThingId, key: &str) -> Option<String>;

    // For attributes, we need to find HAS_ATTR linked nodes and read ATTR_NAME, ATTR_VALUE
    fn get_attributes(&self, elem: ThingId) -> Vec<(String, String)>;
}

/// SvgGraph implementation that delegates to the generic XmlGraph adapter.
///
/// This provides ordered traversal (by XML_ORDER) and filters children
/// to only return element nodes (skipping text nodes for SVG processing).
pub struct SysSvgGraph;

impl SysSvgGraph {
    /// Resolve an interned symbol ID to its string representation.
    /// Kept for backward compatibility with get_prop_str fallback.
    fn resolve_symbol(sym_id: u64) -> Option<String> {
        if sym_id == 0 {
            return None;
        }
        let mut buf = [0u8; 256];
        match sys::describe_symbol(sym_id as u32, &mut buf) {
            Ok(len) if len > 0 => {
                core::str::from_utf8(&buf[..len]).ok().map(String::from)
            }
            _ => None,
        }
    }
}

impl SvgGraph for SysSvgGraph {
    fn get_root(&self, doc: ThingId) -> Option<ThingId> {
        XmlGraph::get_document_root(doc)
    }

    fn get_children(&self, elem: ThingId) -> Vec<ThingId> {
        // For SVG, we typically only care about element children (not text nodes)
        // Filter to only XML_ELEMENT nodes, already sorted by XML_ORDER
        XmlGraph::get_children(elem)
            .into_iter()
            .filter(|&id| XmlGraph::is_element(id))
            .collect()
    }

    fn get_prop_str(&self, node: ThingId, key: &str) -> Option<String> {
        // Map common property requests to XmlGraph helpers
        if key == keys::TAG {
            XmlGraph::get_tag_name(node)
        } else if key == keys::TEXT {
            XmlGraph::get_text(node)
        } else {
            // Fallback to direct property lookup for other keys
            match sys::prop_get(node, key) {
                Ok(val) if val != 0 => Self::resolve_symbol(val),
                _ => None,
            }
        }
    }

    fn get_attributes(&self, elem: ThingId) -> Vec<(String, String)> {
        // Delegates to XmlGraph which returns attributes sorted by XML_ORDER
        XmlGraph::get_attributes(elem)
    }
}
