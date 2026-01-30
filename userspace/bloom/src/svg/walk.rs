//! SVG graph walker that delegates to the generic XmlGraph adapter.
//!
//! This module provides the `SvgGraph` trait for abstracting graph reading
//! (allowing test mocks) and `SysSvgGraph` which is the production implementation
//! that delegates to `stem::xml::graph::XmlGraph`.

use abi::schema::keys;
use abi::wire::ThingId;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use stem::thing::sys;
use stem::xml::graph::XmlGraph;

// === Observability Counters ===
// These track diagnostic events for XML-based SVG traversal

/// Count of times a document had no XML root element
pub static SVG_XML_ROOT_MISSING: AtomicU64 = AtomicU64::new(0);

/// Count of attribute decode failures (name or value missing)
pub static SVG_XML_ATTR_DECODE_FAIL: AtomicU64 = AtomicU64::new(0);

/// Count of unexpected node kinds encountered (non-element where element expected)
pub static SVG_XML_UNEXPECTED_NODE_KIND: AtomicU64 = AtomicU64::new(0);

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
///
/// The adapter reads from the generic XML tree representation in the graph
/// (XML_DOCUMENT, XML_ELEMENT, XML_ATTRIBUTE nodes) and translates to
/// what the SVG walker expects: root, children, tag name, and attributes.
pub struct SysSvgGraph;

impl SysSvgGraph {
    /// Resolve an interned symbol ID to its string representation.
    /// Kept for backward compatibility with get_prop_str fallback.
    fn resolve_symbol(sym_id: u64) -> Option<String> {
        if sym_id == 0 {
            return None;
        }
        // Support up to 4KB for long SVG path data and style attributes
        let mut buf = alloc::vec![0u8; 4096];
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
        let root = XmlGraph::get_document_root(doc);
        if root.is_none() {
            SVG_XML_ROOT_MISSING.fetch_add(1, Ordering::Relaxed);
        }
        root
    }

    fn get_children(&self, elem: ThingId) -> Vec<ThingId> {
        // For SVG, we typically only care about element children (not text nodes)
        // Filter to only XML_ELEMENT nodes, already sorted by XML_ORDER
        let all_children = XmlGraph::get_children(elem);
        let mut elements = Vec::with_capacity(all_children.len());
        
        for id in all_children {
            if XmlGraph::is_element(id) {
                elements.push(id);
            } else if !XmlGraph::is_text(id) {
                // Unexpected node kind (not element, not text)
                SVG_XML_UNEXPECTED_NODE_KIND.fetch_add(1, Ordering::Relaxed);
            }
            // Text nodes are silently filtered - expected for SVG
        }
        
        elements
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;
    use alloc::string::ToString;
    use stem::xml::ingest::{ingest_xml_to_graph, GraphApply, XmlIngestError, XmlIngestOptions};

    // A graph that supports both Ingestion (Write) and SvgGraph (Read)
    struct TestGraph {
        nodes: u64,
        edges: Vec<(ThingId, String, ThingId)>,
        props: BTreeMap<(ThingId, String), String>,
        interner: BTreeMap<String, u64>,
        rev_interner: BTreeMap<u64, String>,
    }

    impl TestGraph {
        fn new() -> Self {
            Self {
                nodes: 0,
                edges: Vec::new(),
                props: BTreeMap::new(),
                interner: BTreeMap::new(),
                rev_interner: BTreeMap::new(),
            }
        }
    }

    impl GraphApply for TestGraph {
        fn create_node(&mut self, _kind: &str) -> Result<ThingId, XmlIngestError> {
            self.nodes += 1;
            Ok(ThingId::from_u64(self.nodes))
        }

        fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), XmlIngestError> {
            self.edges.push((src, rel.to_string(), dst));
            Ok(())
        }

        fn set_prop(&mut self, node: ThingId, key: &str, val: u64) -> Result<(), XmlIngestError> {
            // Resolve the u64 val back to string if possible
            if let Some(s) = self.rev_interner.get(&val) {
                self.props.insert((node, key.to_string()), s.clone());
            } else {
                // Numeric value (like XML_ORDER)
                self.props
                    .insert((node, key.to_string()), alloc::format!("{}", val));
            }
            Ok(())
        }

        fn intern(&mut self, s: &str) -> Result<u64, XmlIngestError> {
            if let Some(&id) = self.interner.get(s) {
                Ok(id)
            } else {
                let id = self.interner.len() as u64 + 1;
                self.interner.insert(s.to_string(), id);
                self.rev_interner.insert(id, s.to_string());
                Ok(id)
            }
        }
    }

    impl SvgGraph for TestGraph {
        fn get_root(&self, doc: ThingId) -> Option<ThingId> {
            self.edges
                .iter()
                .find(|(s, r, _)| *s == doc && r == "HAS_ROOT")
                .map(|(_, _, d)| *d)
        }

        fn get_children(&self, elem: ThingId) -> Vec<ThingId> {
            // Get children and sort by XML_ORDER
            let mut children: Vec<(u64, ThingId)> = self
                .edges
                .iter()
                .filter(|(s, r, _)| *s == elem && r == "HAS_CHILD")
                .map(|(_, _, d)| {
                    let order = self
                        .props
                        .get(&(*d, "xml.order".to_string()))
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(u64::MAX);
                    (order, *d)
                })
                .collect();
            children.sort_by_key(|(order, _)| *order);
            
            // Filter to elements only (check if node has "tag" property)
            children
                .into_iter()
                .filter(|(_, id)| self.props.contains_key(&(*id, "tag".to_string())))
                .map(|(_, id)| id)
                .collect()
        }

        fn get_prop_str(&self, node: ThingId, key: &str) -> Option<String> {
            self.props.get(&(node, key.to_string())).cloned()
        }

        fn get_attributes(&self, elem: ThingId) -> Vec<(String, String)> {
            // Get attrs and sort by XML_ORDER
            let mut attrs: Vec<(u64, String, String)> = Vec::new();
            let attr_nodes: Vec<ThingId> = self
                .edges
                .iter()
                .filter(|(s, r, _)| *s == elem && r == "HAS_ATTR")
                .map(|(_, _, d)| *d)
                .collect();

            for attr in attr_nodes {
                let order = self
                    .props
                    .get(&(attr, "xml.order".to_string()))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(u64::MAX);
                let name = self.get_prop_str(attr, "attr_name");
                let val = self.get_prop_str(attr, "attr_value");
                if let (Some(n), Some(v)) = (name, val) {
                    attrs.push((order, n, v));
                }
            }
            attrs.sort_by_key(|(order, _, _)| *order);
            attrs.into_iter().map(|(_, n, v)| (n, v)).collect()
        }
    }

    /// Test A: Root + children traversal
    /// Given: <svg><g id="layer"><path d="M0 0L1 1"/></g></svg>
    /// Assert: root tag is "svg", has child "g", "g" has child "path"
    #[test]
    fn test_root_children_traversal() {
        let xml = r#"<svg><g id="layer"><path d="M0 0L1 1"/></g></svg>"#;
        let mut graph = TestGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest");

        // Root tag is "svg"
        let root = graph.get_root(res.document).expect("root exists");
        assert_eq!(graph.get_prop_str(root, "tag"), Some("svg".to_string()));

        // Root has one child "g"
        let children = graph.get_children(root);
        assert_eq!(children.len(), 1);
        let g_elem = children[0];
        assert_eq!(graph.get_prop_str(g_elem, "tag"), Some("g".to_string()));

        // "g" has one child "path"
        let g_children = graph.get_children(g_elem);
        assert_eq!(g_children.len(), 1);
        let path_elem = g_children[0];
        assert_eq!(graph.get_prop_str(path_elem, "tag"), Some("path".to_string()));
    }

    /// Test B: Attributes visible
    /// From same tree, "g" has id="layer", "path" has d="M0 0L1 1"
    #[test]
    fn test_attributes_visible() {
        let xml = r#"<svg><g id="layer"><path d="M0 0L1 1"/></g></svg>"#;
        let mut graph = TestGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest");

        let root = graph.get_root(res.document).expect("root");
        let g_elem = graph.get_children(root)[0];
        let path_elem = graph.get_children(g_elem)[0];

        // g has id="layer"
        let g_attrs = graph.get_attributes(g_elem);
        assert!(g_attrs.iter().any(|(n, v)| n == "id" && v == "layer"));

        // path has d="M0 0L1 1"
        let path_attrs = graph.get_attributes(path_elem);
        assert!(path_attrs.iter().any(|(n, v)| n == "d" && v == "M0 0L1 1"));
    }

    /// Test C: Ordering is stable
    /// Given: <svg><a/><b/><c/></svg>
    /// Assert: child tags are a, b, c in that order
    #[test]
    fn test_ordering_stable() {
        let xml = r#"<svg><a/><b/><c/></svg>"#;
        let mut graph = TestGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest");

        let root = graph.get_root(res.document).expect("root");
        let children = graph.get_children(root);
        assert_eq!(children.len(), 3);

        let tags: Vec<String> = children
            .iter()
            .filter_map(|&id| graph.get_prop_str(id, "tag"))
            .collect();
        assert_eq!(tags, vec!["a", "b", "c"]);
    }
}
