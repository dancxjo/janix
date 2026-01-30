use crate::thing::ThingId;
use crate::xml::model::{kinds, props, rels};
use crate::xml::{Event, XmlReader};
use alloc::vec::Vec;

#[derive(Debug)]
pub enum XmlIngestError {
    ParseError,
    GraphError,
    LimitExceeded(&'static str),
    Utf8Error,
}

impl From<crate::errors::Errno> for XmlIngestError {
    fn from(_: crate::errors::Errno) -> Self {
        XmlIngestError::GraphError
    }
}

pub struct XmlIngestOptions<'a> {
    pub source_name: &'a str,
    pub attach_under: Option<ThingId>,
    pub keep_whitespace_text: bool,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for XmlIngestOptions<'_> {
    fn default() -> Self {
        Self {
            source_name: "",
            attach_under: None,
            keep_whitespace_text: false,
            max_depth: 32,
            max_nodes: 1024,
        }
    }
}

#[derive(Debug, Default)]
pub struct XmlIngestResult {
    pub document: ThingId,
    pub root_element: ThingId,
    pub element_count: u32,
    pub attribute_count: u32,
    pub text_count: u32,
}

pub trait GraphApply {
    fn create_node(&mut self, kind: &str) -> Result<ThingId, XmlIngestError>;
    fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), XmlIngestError>;
    fn set_prop(&mut self, node: ThingId, key: &str, val: u64) -> Result<(), XmlIngestError>;
    fn intern(&mut self, s: &str) -> Result<u64, XmlIngestError>;
}

/// Real implementation of GraphApply interfacing with ThingOS syscalls
pub struct SysGraphApply;

impl GraphApply for SysGraphApply {
    fn create_node(&mut self, kind: &str) -> Result<ThingId, XmlIngestError> {
        crate::thing::sys::create_node(kind).map_err(|_| XmlIngestError::GraphError)
    }

    fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), XmlIngestError> {
        crate::thing::sys::link(src, rel, dst).map_err(|_| XmlIngestError::GraphError)
    }

    fn set_prop(&mut self, node: ThingId, key: &str, val: u64) -> Result<(), XmlIngestError> {
        crate::thing::sys::prop_set(node, key, val).map_err(|_| XmlIngestError::GraphError)
    }

    fn intern(&mut self, s: &str) -> Result<u64, XmlIngestError> {
        crate::thing::sys::intern(s)
            .map_err(|_| XmlIngestError::GraphError)
            .map(|v| v as u64)
    }
}

pub fn ingest_xml_to_graph(
    bytes: &[u8],
    opts: XmlIngestOptions<'_>,
    graph: &mut dyn GraphApply,
) -> Result<XmlIngestResult, XmlIngestError> {
    let mut reader = XmlReader::new(bytes);
    let mut stack: Vec<ThingId> = Vec::with_capacity(32);
    let mut result = XmlIngestResult::default();

    // Track child order per nesting level (index into stack = nesting depth)
    // Each entry is the next child order to assign at that level
    let mut child_order_stack: Vec<u32> = Vec::with_capacity(32);

    // Create Document Node
    let doc_id = graph.create_node(kinds::DOCUMENT)?;

    // Set source name
    let src_sym = graph.intern(opts.source_name)?;
    graph.set_prop(doc_id, props::SOURCE, src_sym)?;

    // Handle attach_under
    if let Some(parent) = opts.attach_under {
        graph.link(parent, rels::HAS_CHILD, doc_id)?;
    }

    result.document = doc_id;

    let mut nodes_created = 0;

    loop {
        match reader.next() {
            Some(Ok(Event::StartElement { name, attributes })) => {
                if stack.len() >= opts.max_depth {
                    return Err(XmlIngestError::LimitExceeded("max_depth"));
                }
                nodes_created += 1;
                result.element_count += 1;
                if nodes_created > opts.max_nodes {
                    return Err(XmlIngestError::LimitExceeded("max_nodes"));
                }

                let elem_id = graph.create_node(kinds::ELEMENT)?;
                let tag_sym = graph.intern(&name)?;
                graph.set_prop(elem_id, props::TAG, tag_sym)?;

                // Link to parent or document, and set XML_ORDER
                if let Some(parent) = stack.last() {
                    graph.link(*parent, rels::HAS_CHILD, elem_id)?;
                    // Set order and increment parent's child counter
                    if let Some(order) = child_order_stack.last_mut() {
                        graph.set_prop(elem_id, props::XML_ORDER, *order as u64)?;
                        *order += 1;
                    }
                } else {
                    graph.link(doc_id, rels::HAS_ROOT, elem_id)?;
                    // Root element gets order 0
                    graph.set_prop(elem_id, props::XML_ORDER, 0)?;
                    result.root_element = elem_id;
                }

                // Attributes with ordering
                let mut attr_order: u32 = 0;
                for (k, v) in attributes {
                    // Uses IntoIterator
                    nodes_created += 1;
                    result.attribute_count += 1;
                    if nodes_created > opts.max_nodes {
                        return Err(XmlIngestError::LimitExceeded("max_nodes"));
                    }

                    let attr_id = graph.create_node(kinds::ATTRIBUTE)?;
                    let name_sym = graph.intern(&k)?;
                    let val_sym = graph.intern(&v)?;

                    graph.set_prop(attr_id, props::ATTR_NAME, name_sym)?;
                    graph.set_prop(attr_id, props::ATTR_VALUE, val_sym)?;
                    graph.set_prop(attr_id, props::XML_ORDER, attr_order as u64)?;

                    graph.link(elem_id, rels::HAS_ATTR, attr_id)?;
                    attr_order += 1;
                }

                // Push this element onto stack and start counting its children
                stack.push(elem_id);
                child_order_stack.push(0);
            }
            Some(Ok(Event::EndElement { .. })) => {
                stack.pop();
                child_order_stack.pop();
            }
            Some(Ok(Event::Text(text))) => {
                // Fixed variant
                if !opts.keep_whitespace_text && text.trim().is_empty() {
                    continue;
                }

                nodes_created += 1;
                result.text_count += 1;
                if nodes_created > opts.max_nodes {
                    return Err(XmlIngestError::LimitExceeded("max_nodes"));
                }

                let text_id = graph.create_node(kinds::TEXT)?;
                let text_sym = graph.intern(&text)?;
                graph.set_prop(text_id, props::TEXT, text_sym)?;

                if let Some(parent) = stack.last() {
                    graph.link(*parent, rels::HAS_CHILD, text_id)?;
                    // Set order for text node and increment parent's child counter
                    if let Some(order) = child_order_stack.last_mut() {
                        graph.set_prop(text_id, props::XML_ORDER, *order as u64)?;
                        *order += 1;
                    }
                }
            }
            Some(Err(_)) => return Err(XmlIngestError::ParseError),
            None => break,
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::ids::HandleId;
    use alloc::collections::BTreeMap;
    use alloc::format;
    use alloc::string::{String, ToString}; // For from_u64

    struct MockGraph {
        nodes: u64,
        edges: Vec<(ThingId, String, ThingId)>,
        props: BTreeMap<(ThingId, String), String>, // key stringified for mock
        interner: BTreeMap<String, u64>,
    }

    impl MockGraph {
        fn new() -> Self {
            Self {
                nodes: 0,
                edges: Vec::new(),
                props: BTreeMap::new(),
                interner: BTreeMap::new(),
            }
        }
    }

    impl GraphApply for MockGraph {
        fn create_node(&mut self, _kind: &str) -> Result<ThingId, XmlIngestError> {
            self.nodes += 1;
            Ok(ThingId::from_u64(self.nodes))
        }

        fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), XmlIngestError> {
            self.edges.push((src, rel.to_string(), dst));
            Ok(())
        }

        fn set_prop(&mut self, node: ThingId, key: &str, val: u64) -> Result<(), XmlIngestError> {
            // mapping val back for test isn't easy without reverse interner, so we just store the usage
            self.props
                .insert((node, key.to_string()), format!("{}", val));
            Ok(())
        }

        fn intern(&mut self, s: &str) -> Result<u64, XmlIngestError> {
            if let Some(&id) = self.interner.get(s) {
                Ok(id)
            } else {
                let id = self.interner.len() as u64 + 1;
                self.interner.insert(s.to_string(), id);
                Ok(id)
            }
        }
    }

    #[test]
    fn test_ingest_basic_svg() {
        let xml = r#"<svg width="100"><g fill="red"><rect/></g></svg>"#;
        let mut graph = MockGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest failed");

        // Check document created (Node 1)
        assert_eq!(res.document.to_u64_lossy(), 1);

        // Check root element (Node 2)
        assert_eq!(res.root_element.to_u64_lossy(), 2);

        // Doc -> Root
        assert!(graph
            .edges
            .iter()
            .any(|(src, rel, dst)| src.to_u64_lossy() == 1
                && rel == "HAS_ROOT"
                && dst.to_u64_lossy() == 2));

        // Root is <svg>
        // Check props? We need to look up in interner.
        let svg_sym = graph.interner.get("svg").expect("svg interned");
        assert_eq!(
            graph
                .props
                .get(&(res.root_element, "tag".to_string()))
                .expect("has tag"),
            &format!("{}", svg_sym)
        );

        // Root has child <g> (Node 3 or 4 depending on attributes)
        // <svg> attribute width="100" -> Node 3

        // Check element count: svg, g, rect = 3
        assert_eq!(res.element_count, 3);

        // Check attribute count: width="100", fill="red" = 2
        assert_eq!(res.attribute_count, 2);
    }

    #[test]
    fn test_limit_depth() {
        let xml = r#"<a0><a1><a2><a3/></a2></a1></a0>"#;
        let mut graph = MockGraph::new();
        let opts = XmlIngestOptions {
            max_depth: 3,
            ..XmlIngestOptions::default()
        };
        let _err = ingest_xml_to_graph(xml.as_bytes(), opts, &mut graph)
            .err()
            .expect("should fail");
    }

    #[test]
    fn test_minimal_svg() {
        let xml = r#"<svg/>"#;
        let mut graph = MockGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest failed");

        // Exactly one document
        assert_eq!(res.document.to_u64_lossy(), 1);
        
        // Exactly one root element
        assert_eq!(res.element_count, 1);
        assert_eq!(res.root_element.to_u64_lossy(), 2);
        
        // Root element has XML_ORDER = 0
        assert_eq!(
            graph.props.get(&(res.root_element, "xml.order".to_string())),
            Some(&"0".to_string())
        );
    }

    #[test]
    fn test_nested_elements_ordering() {
        let xml = r#"<svg><g><path d="M0 0"/></g></svg>"#;
        let mut graph = MockGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest failed");

        // 3 elements: svg, g, path
        assert_eq!(res.element_count, 3);
        
        // Find g element (should be child of svg with order 0)
        // Nodes: 1=doc, 2=svg, 3=svg.attr(d), wait no - svg has no attrs here
        // Actually: 1=doc, 2=svg, 3=g, 4=path, 5=path.attr(d)
        let g_id = ThingId::from_u64(3);
        let path_id = ThingId::from_u64(4);
        
        // g has order 0 (first child of svg)
        assert_eq!(
            graph.props.get(&(g_id, "xml.order".to_string())),
            Some(&"0".to_string())
        );
        
        // path has order 0 (first child of g)
        assert_eq!(
            graph.props.get(&(path_id, "xml.order".to_string())),
            Some(&"0".to_string())
        );
        
        // Verify HAS_CHILD edges exist
        assert!(graph.edges.iter().any(|(s, r, d)| 
            s.to_u64_lossy() == 2 && r == "HAS_CHILD" && d.to_u64_lossy() == 3
        ));
        assert!(graph.edges.iter().any(|(s, r, d)| 
            s.to_u64_lossy() == 3 && r == "HAS_CHILD" && d.to_u64_lossy() == 4
        ));
    }

    #[test]
    fn test_text_and_whitespace() {
        let xml = r#"<a>Hello <b>world</b></a>"#;
        let mut graph = MockGraph::new();
        let opts = XmlIngestOptions {
            keep_whitespace_text: false, // Default, but explicit
            ..XmlIngestOptions::default()
        };
        let res = ingest_xml_to_graph(xml.as_bytes(), opts, &mut graph)
            .expect("ingest failed");

        // 2 elements: a, b
        assert_eq!(res.element_count, 2);
        
        // 2 text nodes: "Hello " and "world"
        assert_eq!(res.text_count, 2);
        
        // Nodes: 1=doc, 2=a, 3="Hello " text, 4=b, 5="world" text
        let text1_id = ThingId::from_u64(3);
        let b_id = ThingId::from_u64(4);
        let text2_id = ThingId::from_u64(5);
        
        // "Hello " text has order 0 (first child of a)
        assert_eq!(
            graph.props.get(&(text1_id, "xml.order".to_string())),
            Some(&"0".to_string())
        );
        
        // b element has order 1 (second child of a)
        assert_eq!(
            graph.props.get(&(b_id, "xml.order".to_string())),
            Some(&"1".to_string())
        );
        
        // "world" text has order 0 (first child of b)
        assert_eq!(
            graph.props.get(&(text2_id, "xml.order".to_string())),
            Some(&"0".to_string())
        );
    }

    #[test]
    fn test_attribute_ordering() {
        let xml = r#"<x a="1" b="2" c="3"/>"#;
        let mut graph = MockGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest failed");

        // 1 element, 3 attributes
        assert_eq!(res.element_count, 1);
        assert_eq!(res.attribute_count, 3);
        
        // Nodes: 1=doc, 2=x, 3=attr(a), 4=attr(b), 5=attr(c)
        let attr_a = ThingId::from_u64(3);
        let attr_b = ThingId::from_u64(4);
        let attr_c = ThingId::from_u64(5);
        
        // Attributes have XML_ORDER 0, 1, 2 in encounter order
        assert_eq!(
            graph.props.get(&(attr_a, "xml.order".to_string())),
            Some(&"0".to_string())
        );
        assert_eq!(
            graph.props.get(&(attr_b, "xml.order".to_string())),
            Some(&"1".to_string())
        );
        assert_eq!(
            graph.props.get(&(attr_c, "xml.order".to_string())),
            Some(&"2".to_string())
        );
        
        // Verify HAS_ATTR edges
        assert!(graph.edges.iter().any(|(s, r, d)| 
            s.to_u64_lossy() == 2 && r == "HAS_ATTR" && d.to_u64_lossy() == 3
        ));
        assert!(graph.edges.iter().any(|(s, r, d)| 
            s.to_u64_lossy() == 2 && r == "HAS_ATTR" && d.to_u64_lossy() == 4
        ));
        assert!(graph.edges.iter().any(|(s, r, d)| 
            s.to_u64_lossy() == 2 && r == "HAS_ATTR" && d.to_u64_lossy() == 5
        ));
    }
}
