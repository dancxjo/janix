//! Graph storage for fetched network resources

use abi::kinds::*;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use stem::thing::sys::*;
use stem::thing::ThingId;
use stem::xml::ingest::{ingest_xml_to_graph, SysGraphApply, XmlIngestOptions};
use stem::html::ingest::{ingest_html_to_graph, HtmlIngestOptions};
use stem::css::ingest::{ingest_css_to_graph, CssIngestOptions};
use stem::{info, warn};

#[derive(Debug)]
pub enum GraphError {
    CreateFailed,
    StoreFailed,
    XmlParseFailed,
    HtmlParseFailed,
    CssParseFailed,
}

pub fn store_fetch_result(
    url: &str,
    status_code: u16,
    body: &[u8],
) -> Result<ThingId, GraphError> {
    // Create node under /net/fetch/<timestamp>
    let timestamp = stem::time::now_unix_seconds();
    let node_id = create_node(KIND_NET_FETCH_RESOURCE).map_err(|_| GraphError::CreateFailed)?;

    info!("GRAPH: Created node {:?} for fetch result", node_id);

    // Compute URL hash for property storage
    let url_hash = hash_string(url);

    // Set properties
    prop_set(node_id, PROP_NET_URL, url_hash as u64).ok();
    prop_set(node_id, PROP_NET_STATUS, status_code as u64).ok();
    prop_set(node_id, PROP_NET_FETCHED_AT, timestamp).ok();

    // Create bytespace for body
    let bs_id = bytespace_create(body.len(), 0, 0).map_err(|_| GraphError::StoreFailed)?;

    // Write body to bytespace
    bytespace_write(bs_id, 0, body).map_err(|_| GraphError::StoreFailed)?;

    // Link bytespace to node
    link(node_id, intern("bytespace").unwrap_or(0), bs_id).ok();

    info!("GRAPH: Stored {} bytes in bytespace {:?}", body.len(), bs_id);

    // Try document import based on content type detection
    if status_code >= 200 && status_code < 300 {
        let content_type = detect_content_type(url, body);
        
        match content_type {
            ContentType::Css => {
                match try_css_import(node_id, body) {
                    Ok(result) => {
                        info!("GRAPH: CSS import succeeded, {} rules, {} declarations", 
                              result.rule_count, result.declaration_count);
                    }
                    Err(e) => {
                        warn!("GRAPH: CSS import failed: {:?}", e);
                    }
                }
            }
            ContentType::Xml | ContentType::Svg => {
                match try_xml_import(node_id, body) {
                    Ok(result) => {
                        let xml_bytes = result.root_element.0;
                        let xml_hash = hash_bytes(&xml_bytes);
                        prop_set(node_id, PROP_RESOURCE_XML_DOCUMENT, xml_hash as u64).ok();
                        info!("GRAPH: XML import succeeded, root {:?}", result.root_element);
                    }
                    Err(e) => {
                        warn!("GRAPH: XML import failed: {:?}", e);
                    }
                }
            }
            ContentType::Html | ContentType::Unknown => {
                // Try HTML first (most common case for web fetches)
                match try_html_import(node_id, body) {
                    Ok(result) => {
                        info!("GRAPH: HTML import succeeded, {} elements, {} attributes", 
                              result.element_count, result.attribute_count);
                    }
                    Err(_) => {
                        // Fall back to XML parsing for strict XML content
                        match try_xml_import(node_id, body) {
                            Ok(result) => {
                                let xml_bytes = result.root_element.0;
                                let xml_hash = hash_bytes(&xml_bytes);
                                prop_set(node_id, PROP_RESOURCE_XML_DOCUMENT, xml_hash as u64).ok();
                                info!("GRAPH: XML fallback import succeeded, root {:?}", result.root_element);
                            }
                            Err(e) => {
                                warn!("GRAPH: Both HTML and XML import failed: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(node_id)
}

#[derive(Debug, Clone, Copy)]
enum ContentType {
    Html,
    Xml,
    Svg,
    Css,
    Unknown,
}

fn detect_content_type(url: &str, body: &[u8]) -> ContentType {
    // Check URL extension first
    let url_lower = url.to_lowercase();
    if url_lower.ends_with(".css") {
        return ContentType::Css;
    }
    if url_lower.ends_with(".svg") {
        return ContentType::Svg;
    }
    if url_lower.ends_with(".xml") {
        return ContentType::Xml;
    }
    if url_lower.ends_with(".html") || url_lower.ends_with(".htm") {
        return ContentType::Html;
    }
    
    // Content sniffing
    let prefix = if body.len() >= 256 { &body[..256] } else { body };
    let prefix_str = core::str::from_utf8(prefix).unwrap_or("");
    let prefix_lower = prefix_str.to_lowercase();
    
    if prefix_lower.contains("<!doctype html") || 
       prefix_lower.contains("<html") ||
       prefix_lower.contains("<head") ||
       prefix_lower.contains("<body") {
        return ContentType::Html;
    }
    
    if prefix_lower.contains("<?xml") {
        if prefix_lower.contains("<svg") {
            return ContentType::Svg;
        }
        return ContentType::Xml;
    }
    
    if prefix_lower.contains("<svg") {
        return ContentType::Svg;
    }
    
    // Check for CSS-like content (selectors and braces)
    if looks_like_css(prefix_str) {
        return ContentType::Css;
    }
    
    ContentType::Unknown
}

fn looks_like_css(s: &str) -> bool {
    // Simple heuristic: CSS has selectors followed by { } blocks
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return false;
    }
    
    // Check for common CSS patterns
    let has_braces = trimmed.contains('{') && trimmed.contains('}');
    let has_selector = trimmed.contains('.') || trimmed.contains('#') || 
                       trimmed.starts_with('@') || trimmed.contains(':');
    let no_html_tags = !trimmed.contains('<') && !trimmed.contains('>');
    
    has_braces && has_selector && no_html_tags
}

fn try_xml_import(parent: ThingId, xml_data: &[u8]) -> Result<stem::xml::ingest::XmlIngestResult, GraphError> {
    let mut apply = SysGraphApply;
    let options = XmlIngestOptions {
        attach_under: Some(parent),
        ..Default::default()
    };

    ingest_xml_to_graph(xml_data, options, &mut apply).map_err(|_| GraphError::XmlParseFailed)
}

fn try_html_import(parent: ThingId, html_data: &[u8]) -> Result<stem::html::ingest::HtmlIngestResult, GraphError> {
    let mut apply = SysGraphApply;
    let options = HtmlIngestOptions {
        attach_under: Some(parent),
        ..Default::default()
    };

    ingest_html_to_graph(html_data, options, &mut apply).map_err(|_| GraphError::HtmlParseFailed)
}

fn try_css_import(parent: ThingId, css_data: &[u8]) -> Result<stem::css::ingest::CssIngestResult, GraphError> {
    let mut apply = SysGraphApply;
    let options = CssIngestOptions {
        attach_under: Some(parent),
        ..Default::default()
    };

    ingest_css_to_graph(css_data, options, &mut apply).map_err(|_| GraphError::CssParseFailed)
}

fn hash_string(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}

fn hash_bytes(bytes: &[u8]) -> u32 {
    let mut hash: u32 = 5381;
    for &byte in bytes {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}
