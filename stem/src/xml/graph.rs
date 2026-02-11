//! XmlGraph adapter for traversing XML trees in the Thing-OS graph.
//!
//! This module provides a reusable adapter for navigating XML document
//! structures that have been ingested into the system graph. It handles
//! symbol resolution, edge traversal, and provides ordered access to
//! children and attributes.

use crate::thing::sys;
use crate::thing::ThingId;
use abi::schema::{keys, kinds, rels};
use abi::types::Edge;
use alloc::string::String;
use alloc::vec::Vec;

/// Adapter for traversing XML document trees stored in the graph.
///
/// All methods are static since no state is needed - the adapter
/// provides a consistent interface over the syscall layer.
pub struct XmlGraph;

impl XmlGraph {
    /// Get the root element of an XML document.
    ///
    /// Follows the HAS_ROOT edge from the document node.
    pub fn get_document_root(doc: ThingId) -> Option<ThingId> {
        let mut edges = [Edge::default(); 32];
        if let Ok(count) = sys::get_edges(doc, &mut edges) {
            for i in 0..count {
                let e = &edges[i];
                if Self::edge_has_relation(e, rels::HAS_ROOT) {
                    return Some(e.to);
                }
            }
        }
        None
    }

    /// Get children of an element, sorted by XML_ORDER.
    ///
    /// Returns child nodes (elements, text, comments) in document order.
    pub fn get_children(elem: ThingId) -> Vec<ThingId> {
        let mut children: Vec<(u64, ThingId)> = Vec::new();
        let mut edges = [Edge::default(); 128];

        if let Ok(count) = sys::get_edges(elem, &mut edges) {
            for i in 0..count {
                let e = &edges[i];
                if Self::edge_has_relation(e, rels::HAS_CHILD) {
                    let order = sys::prop_get(e.to, keys::XML_ORDER).unwrap_or(u64::MAX);
                    children.push((order, e.to));
                }
            }
        }

        // Sort by XML_ORDER
        children.sort_by_key(|(order, _)| *order);
        children.into_iter().map(|(_, id)| id).collect()
    }

    /// Get attributes as (name, value) pairs, sorted by XML_ORDER.
    ///
    /// Follows HAS_ATTR edges and reads ATTR_NAME/ATTR_VALUE properties.
    pub fn get_attributes(elem: ThingId) -> Vec<(String, String)> {
        let mut attrs: Vec<(u64, String, String)> = Vec::new();
        let mut edges = [Edge::default(); 64];

        if let Ok(count) = sys::get_edges(elem, &mut edges) {
            for i in 0..count {
                let e = &edges[i];
                if Self::edge_has_relation(e, rels::HAS_ATTR) {
                    let attr_node = e.to;
                    let order = sys::prop_get(attr_node, keys::XML_ORDER).unwrap_or(u64::MAX);

                    if let (Some(name), Some(value)) = (
                        Self::get_prop_str(attr_node, keys::ATTR_NAME),
                        Self::get_prop_str(attr_node, keys::ATTR_VALUE),
                    ) {
                        attrs.push((order, name, value));
                    }
                }
            }
        }

        // Sort by XML_ORDER
        attrs.sort_by_key(|(order, _, _)| *order);
        attrs.into_iter().map(|(_, n, v)| (n, v)).collect()
    }

    /// Get the tag name of an element.
    pub fn get_tag_name(elem: ThingId) -> Option<String> {
        Self::get_prop_str(elem, keys::TAG)
    }

    /// Get the text content of a text node.
    pub fn get_text(node: ThingId) -> Option<String> {
        Self::get_prop_str(node, keys::TEXT)
    }

    /// Check if a node is an XML element.
    pub fn is_element(node: ThingId) -> bool {
        Self::node_has_kind(node, kinds::XML_ELEMENT)
    }

    /// Check if a node is an XML text node.
    pub fn is_text(node: ThingId) -> bool {
        Self::node_has_kind(node, kinds::XML_TEXT)
    }

    /// Check if a node is an XML attribute node.
    pub fn is_attribute(node: ThingId) -> bool {
        Self::node_has_kind(node, kinds::XML_ATTRIBUTE)
    }

    /// Check if a node is an XML document.
    pub fn is_document(node: ThingId) -> bool {
        Self::node_has_kind(node, kinds::XML_DOCUMENT)
    }

    // --- Internal helpers ---

    /// Resolve an interned property value to a string.
    fn get_prop_str(node: ThingId, key: &str) -> Option<String> {
        match sys::prop_get(node, key) {
            Ok(val) if val != 0 => Self::resolve_symbol(val),
            _ => None,
        }
    }

    /// Resolve an interned symbol ID to its string representation.
    fn resolve_symbol(sym_id: u64) -> Option<String> {
        if sym_id == 0 {
            return None;
        }
        // Support up to 4KB for long SVG path data and style attributes
        let mut buf = alloc::vec![0u8; 4096];
        match sys::describe_symbol(sym_id as u32, &mut buf) {
            Ok(len) if len > 0 => core::str::from_utf8(&buf[..len]).ok().map(String::from),
            _ => None,
        }
    }

    /// Check if an edge has the given relation name.
    fn edge_has_relation(edge: &Edge, rel_name: &str) -> bool {
        let pred_id = edge.predicate.to_u64_lossy();
        if pred_id == 0 {
            return false;
        }
        if let Some(resolved) = Self::resolve_symbol(pred_id) {
            resolved == rel_name
        } else {
            false
        }
    }

    /// Check if a node has the given kind.
    fn node_has_kind(node: ThingId, kind_name: &str) -> bool {
        match sys::prop_get(node, keys::KIND) {
            Ok(kind_sym) if kind_sym != 0 => {
                if let Some(resolved) = Self::resolve_symbol(kind_sym) {
                    resolved == kind_name
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests for XmlGraph require either a mock or the real system graph.
    // The compile.rs tests in bloom/src/svg/ already exercise the full
    // ingest -> compile pipeline using TestGraph which implements both
    // GraphApply and SvgGraph traits.
}
