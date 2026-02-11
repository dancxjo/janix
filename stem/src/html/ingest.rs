//! HTML to Graph ingestion.
//!
//! Converts HTML documents into graph nodes using the `htmlparser` crate.

use crate::html::model::{kinds, props, rels};
use crate::thing::ThingId;
use crate::xml::ingest::GraphApply;
use alloc::vec::Vec;
use htmlparser::{Token, Tokenizer};

#[derive(Debug)]
pub enum HtmlIngestError {
    ParseError,
    GraphError,
    CreateNodeFailed,
    LinkFailed,
    SetPropFailed,
    InternFailed,
    LimitExceeded(&'static str),
    Utf8Error,
}

impl From<crate::xml::ingest::XmlIngestError> for HtmlIngestError {
    fn from(e: crate::xml::ingest::XmlIngestError) -> Self {
        match e {
            crate::xml::ingest::XmlIngestError::CreateNodeFailed => {
                HtmlIngestError::CreateNodeFailed
            }
            crate::xml::ingest::XmlIngestError::LinkFailed => HtmlIngestError::LinkFailed,
            crate::xml::ingest::XmlIngestError::SetPropFailed => HtmlIngestError::SetPropFailed,
            crate::xml::ingest::XmlIngestError::InternFailed => HtmlIngestError::InternFailed,
            _ => HtmlIngestError::GraphError,
        }
    }
}

pub struct HtmlIngestOptions<'a> {
    pub source_name: &'a str,
    pub attach_under: Option<ThingId>,
    pub keep_whitespace_text: bool,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for HtmlIngestOptions<'_> {
    fn default() -> Self {
        Self {
            source_name: "",
            attach_under: None,
            keep_whitespace_text: false,
            max_depth: 64,   // HTML can be deeper than XML
            max_nodes: 4096, // HTML pages are typically larger
        }
    }
}

#[derive(Debug, Default)]
pub struct HtmlIngestResult {
    pub document: ThingId,
    pub root_element: ThingId,
    pub element_count: u32,
    pub attribute_count: u32,
    pub text_count: u32,
    pub comment_count: u32,
}

/// Ingest an HTML document into the graph.
///
/// Uses the `htmlparser` crate which is a tolerant HTML tokenizer that
/// handles malformed HTML gracefully.
pub fn ingest_html_to_graph(
    bytes: &[u8],
    opts: HtmlIngestOptions<'_>,
    graph: &mut dyn GraphApply,
) -> Result<HtmlIngestResult, HtmlIngestError> {
    let html_str = core::str::from_utf8(bytes).map_err(|_| HtmlIngestError::Utf8Error)?;

    let mut stack: Vec<ThingId> = Vec::with_capacity(64);
    let mut result = HtmlIngestResult::default();
    let mut child_order_stack: Vec<u32> = Vec::with_capacity(64);

    // Create Document Node
    let doc_id = graph.create_node(kinds::DOCUMENT)?;
    let src_sym = graph.intern(opts.source_name)?;
    graph.set_prop(doc_id, props::SOURCE, src_sym)?;

    if let Some(parent) = opts.attach_under {
        graph.link(parent, rels::HAS_CHILD, doc_id)?;
    }

    result.document = doc_id;

    let mut nodes_created = 0;
    let mut root_set = false;
    let mut current_elem: Option<ThingId> = None;

    for token_result in Tokenizer::from(html_str) {
        // Skip parse errors gracefully - HTML is often malformed
        let token = match token_result {
            Ok(t) => t,
            Err(_) => continue,
        };
        match token {
            Token::ElementStart { local, .. } => {
                if stack.len() >= opts.max_depth {
                    return Err(HtmlIngestError::LimitExceeded("max_depth"));
                }
                nodes_created += 1;
                result.element_count += 1;
                if nodes_created > opts.max_nodes {
                    return Err(HtmlIngestError::LimitExceeded("max_nodes"));
                }

                let elem_id = graph.create_node(kinds::ELEMENT)?;
                let tag_sym = graph.intern(local.as_str())?;
                graph.set_prop(elem_id, props::TAG, tag_sym)?;

                // Link to parent or document
                if let Some(parent) = stack.last() {
                    graph.link(*parent, rels::HAS_CHILD, elem_id)?;
                    if let Some(order) = child_order_stack.last_mut() {
                        graph.set_prop(elem_id, props::ORDER, *order as u64)?;
                        *order += 1;
                    }
                } else {
                    graph.link(doc_id, rels::HAS_ROOT, elem_id)?;
                    graph.set_prop(elem_id, props::ORDER, 0)?;
                    if !root_set {
                        result.root_element = elem_id;
                        root_set = true;
                    }
                }

                // Store current element for attribute processing
                current_elem = Some(elem_id);
            }

            Token::Attribute { local, value, .. } => {
                if let Some(elem_id) = current_elem {
                    nodes_created += 1;
                    result.attribute_count += 1;
                    if nodes_created > opts.max_nodes {
                        return Err(HtmlIngestError::LimitExceeded("max_nodes"));
                    }

                    let attr_id = graph.create_node(kinds::ATTRIBUTE)?;
                    let name_sym = graph.intern(local.as_str())?;
                    let val_str = value.map(|v| v.as_str()).unwrap_or("");
                    let val_sym = graph.intern(val_str)?;

                    graph.set_prop(attr_id, props::ATTR_NAME, name_sym)?;
                    graph.set_prop(attr_id, props::ATTR_VALUE, val_sym)?;

                    graph.link(elem_id, rels::HAS_ATTR, attr_id)?;
                }
            }

            Token::ElementEnd { end, .. } => {
                match end {
                    htmlparser::ElementEnd::Open => {
                        // Element is open, push onto stack for children
                        if let Some(elem_id) = current_elem.take() {
                            stack.push(elem_id);
                            child_order_stack.push(0);
                        }
                    }
                    htmlparser::ElementEnd::Empty => {
                        // Self-closing element, don't push
                        current_elem = None;
                    }
                    htmlparser::ElementEnd::Close(..) => {
                        // Closing tag, pop from stack
                        stack.pop();
                        child_order_stack.pop();
                    }
                }
            }

            Token::Text { text } => {
                let text_str = text.as_str();
                if !opts.keep_whitespace_text && text_str.trim().is_empty() {
                    continue;
                }

                nodes_created += 1;
                result.text_count += 1;
                if nodes_created > opts.max_nodes {
                    return Err(HtmlIngestError::LimitExceeded("max_nodes"));
                }

                let text_id = graph.create_node(kinds::TEXT)?;
                let text_sym = graph.intern(text_str)?;
                graph.set_prop(text_id, props::TEXT, text_sym)?;

                if let Some(parent) = stack.last() {
                    graph.link(*parent, rels::HAS_CHILD, text_id)?;
                    if let Some(order) = child_order_stack.last_mut() {
                        graph.set_prop(text_id, props::ORDER, *order as u64)?;
                        *order += 1;
                    }
                }
            }

            Token::Comment { text, .. } => {
                nodes_created += 1;
                result.comment_count += 1;
                if nodes_created > opts.max_nodes {
                    return Err(HtmlIngestError::LimitExceeded("max_nodes"));
                }

                let comment_id = graph.create_node(kinds::COMMENT)?;
                let text_sym = graph.intern(text.as_str())?;
                graph.set_prop(comment_id, props::TEXT, text_sym)?;

                if let Some(parent) = stack.last() {
                    graph.link(*parent, rels::HAS_CHILD, comment_id)?;
                    if let Some(order) = child_order_stack.last_mut() {
                        graph.set_prop(comment_id, props::ORDER, *order as u64)?;
                        *order += 1;
                    }
                }
            }

            // Skip DOCTYPE, processing instructions, CDATA, etc.
            _ => {}
        }
    }

    Ok(result)
}
